#![allow(clippy::all, warnings)]

use chrono::Utc;

pub struct Search;
pub struct ImportNotes;

type DateTime = chrono::DateTime<Utc>;

pub mod search {
    pub const OPERATION_NAME: &str = "Search";
    pub const QUERY : & str = "query Search($query: String!) {\r\n  search(query: $query) {\r\n    id\r\n    content\r\n    url\r\n    title\r\n    isRtl\r\n    username\r\n    created\r\n    tags\r\n  }\r\n}" ;
    use super::*;
    use serde::Deserialize;
    use serde::Serialize;
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTime = super::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub query: String,
    }
    impl Variables {}
    #[derive(Serialize, Deserialize)]
    pub struct ResponseData {
        pub search: Vec<Note>,
    }
    #[derive(Serialize, Deserialize)]
    pub struct Note {
        pub content: String,
        pub url: String,
        pub title: String,
        pub created: DateTime,
        pub tags: Vec<String>,
    }
}

pub mod import_notes {
    pub const OPERATION_NAME: &str = "ImportNotes";
    pub const QUERY: &str =
        "mutation ImportNotes($notes: [NewImportNote!]!) {\r\n  importNotes(notes: $notes)\r\n}";
    use super::*;
    use serde::Deserialize;
    use serde::Serialize;
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTime = super::DateTime;

    #[derive(Serialize)]
    pub struct Variables {
        pub notes: Vec<NewImportNote>,
    }
    impl Variables {}

    #[derive(Serialize, Clone, Debug)]
    pub struct NewImportNote {
        pub content: String,
        pub url: String,
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created: Option<DateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub author: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chapter: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "importNotes")]
        pub import_notes: Boolean,
    }
}

impl graphql_client::GraphQLQuery for Search {
    type Variables = search::Variables;
    type ResponseData = search::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: search::QUERY,
            operation_name: search::OPERATION_NAME,
        }
    }
}

impl graphql_client::GraphQLQuery for ImportNotes {
    type Variables = import_notes::Variables;
    type ResponseData = import_notes::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: import_notes::QUERY,
            operation_name: import_notes::OPERATION_NAME,
        }
    }
}
