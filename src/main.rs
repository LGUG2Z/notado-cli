use clap::Parser;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::blocking::ClientBuilder;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;

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
enum SubCommand {
    /// Run a fuzzy search against all notes in your library
    Search(Search),
    /// Export all notes and tags from your library as JSON
    Backup,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let token = std::env::var("NOTADO_API_TOKEN")?;

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
                .post("https://notado.app/graphql")
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
                .get("https://notado.app/export/notes.json")
                .header("X-API-TOKEN", token)
                .send()?;

            let data = res.text()?;
            println!("{data}");
        }
    }

    Ok(())
}
