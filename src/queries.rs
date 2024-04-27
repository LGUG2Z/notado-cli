#![allow(clippy::all, warnings)]

use chrono::Utc;

pub struct Search;
type DateTime = chrono::DateTime<Utc>;
pub mod search {
    #![allow(dead_code)]
    use std::result::Result;
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
