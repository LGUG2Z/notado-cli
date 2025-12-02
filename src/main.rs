use crate::queries::import_notes::NewImportNote;
use chrono::NaiveDateTime;
use clap::Parser;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use kobo_db_tools::ParseOption;
use reqwest::blocking::ClientBuilder;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

mod queries;

#[derive(Parser)]
#[clap(author, about, version)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
struct Search {
    /// A fuzzy search query
    query: String,
}

#[derive(Parser)]
struct QuoteGen {
    /// Path to a newline-separated file of source URLs from supported commentary sites
    #[arg(long, default_value = "sources")]
    sources: PathBuf,
}

#[derive(Parser)]
struct SaveKoboHighlights {
    /// Path to a Kobo database file
    #[arg(long, default_value = "/Volumes/KOBOeReader/.kobo/KoboReader.sqlite")]
    kobo_database: PathBuf,
    /// Print the highlights as JSON, without saving them to your Notado account
    #[arg(long, short, action)]
    print: bool,
}

#[derive(Parser)]
enum SubCommand {
    /// Run a fuzzy search against all notes in your library
    Search(Search),
    /// Export all notes and tags from your library as JSON
    Backup,
    /// Generate a JSON library of quotes that can be embedded via shortcodes into static site generator pages
    QuoteGen(QuoteGen),
    /// Save highlights from a Kobo database file to your Notado account
    SaveKoboHighlights(SaveKoboHighlights),
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let token = std::env::var("NOTADO_API_TOKEN")?;
    let domain =
        std::env::var("NOTADO_ENDPOINT").unwrap_or_else(|_| String::from("https://notado.app"));
    let graphql_endpoint = format!("{domain}/graphql");

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let mut headers = HeaderMap::new();
    headers.insert("X-API-TOKEN", HeaderValue::from_str(&token)?);
    let client = ClientBuilder::default().default_headers(headers).build()?;

    match opts.subcmd {
        SubCommand::Search(args) => {
            let res = client
                .post(graphql_endpoint)
                .json(&queries::Search::build_query(queries::search::Variables {
                    query: args.query,
                }))
                .header("X-API-TOKEN", token)
                .send()?;

            let response_body: Response<queries::search::ResponseData> = res.json()?;

            if let Some(data) = response_body.data {
                let output = serde_json::to_string_pretty(&data.search)?;
                println!("{output}");
            } else {
                println!("[]");
            }
        }
        SubCommand::Backup => {
            let res = client
                .get(format!("{domain}/export/notes.json"))
                .header("X-API-TOKEN", token)
                .send()?;

            let data = res.text()?;
            println!("{data}");
        }
        SubCommand::QuoteGen(args) => {
            let mut library: BTreeMap<String, QuoteData> = BTreeMap::new();
            if let Ok(library_raw) = std::fs::read_to_string("library.json") {
                std::fs::write("library.json.backup", &library_raw)?;
                library = serde_json::from_str(&library_raw)?
            }

            let sources_raw = std::fs::read_to_string(&args.sources)?;
            let mut sources = sources_raw.lines().collect::<Vec<_>>();

            sources.retain(|s| !s.is_empty());
            sources.sort();
            sources.dedup();

            for s in sources {
                let source = s.trim();
                if Url::parse(source).is_ok() {
                    if !library.contains_key(source) {
                        if source.starts_with("https://notado.app/quote/") {
                            match client
                                .get(format!("{source}/json"))
                                .header("X-API-TOKEN", &token)
                                .send()?
                                .json::<QuoteData>()
                            {
                                Ok(data) => {
                                    println!("Adding quote data for {source} to library.json");
                                    library.insert(source.to_string(), data);
                                }
                                Err(_) => {
                                    println!("Skipping {source} as there wasn't a matching note in your Notado account");
                                }
                            }
                        } else {
                            match client
                                .get(format!("{domain}/quote"))
                                .query(&[("source", source)])
                                .header("X-API-TOKEN", &token)
                                .send()?
                                .json::<QuoteData>()
                            {
                                Ok(data) => {
                                    println!("Adding quote data for {source} to library.json");
                                    library.insert(source.to_string(), data);
                                }
                                Err(_) => {
                                    println!("Skipping {source} as there wasn't a matching note in your Notado account");
                                }
                            }
                        }
                    } else {
                        println!("Skipping {source} as data is already in library.json");
                    }
                }
            }

            std::fs::write("library.json", serde_json::to_string_pretty(&library)?)?;
        }
        SubCommand::SaveKoboHighlights(args) => {
            let analysis =
                kobo_db_tools::Parser::parse_from_str(args.kobo_database, ParseOption::All)?;

            let mut notes = vec![];

            if let Some(bookmarks) = &analysis.bookmarks {
                for b in bookmarks {
                    let mut tags = vec![];
                    if let Some(annotation) = &b.annotation {
                        for possible_tag in annotation.split_whitespace() {
                            if possible_tag.starts_with('.') {
                                tags.push(possible_tag.trim_start_matches('.'));
                            }
                        }
                    }

                    notes.push(NewImportNote {
                        content: b.content.clone(),
                        url: format!(
                            "calibre://search/_?q=title:{} author:{}",
                            b.book_title, b.author
                        ),
                        title: b.book_title.clone(),
                        created: b
                            .create_date
                            .parse::<NaiveDateTime>()
                            .ok()
                            .map(|ntd| ntd.and_utc()),
                        tags: Some(tags.iter().map(|s| s.to_string()).collect()),
                        author: Some(b.author.clone()),
                        chapter: b.chapter_title.clone(),
                    });
                }
            }

            if args.print {
                println!("{}", serde_json::to_string_pretty(&notes)?);
            } else if !notes.is_empty() {
                let len = notes.len();
                let res = client
                    .post(graphql_endpoint)
                    .json(&queries::ImportNotes::build_query(
                        queries::import_notes::Variables { notes },
                    ))
                    .header("X-API-TOKEN", token)
                    .send()?;

                if res.text()?.contains(r#""importNotes":true"#) {
                    println!("Saved {len} highlights to your Notado account");
                }
            }
        }
    }

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct QuoteData {
    title: String,
    source_display: String,
    source_url: Option<String>,
    content: String,
}
