use std::sync::Arc;

use axum::{extract, http};

use serde::Deserialize;

use crate::application::{self, queries, State};

use super::parse_user_id;

#[derive(Deserialize)]
pub struct QueryArgs {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub async fn spendings_list_handler(
    extract::State(state): extract::State<Arc<State>>,
    query: extract::Query<QueryArgs>,
    headers: http::HeaderMap,
) ->  Result<application::Response<queries::SpendingsResponse>, application::errors::AppError> {
    let user_id = parse_user_id(headers)?;

    let spendings_result = state
        .spendings_list_query_handler
        .handle(queries::SpendingsListQueryArgs {
            user_id,
            limit: query.limit,
            offset: query.offset
        })
        .await?;
    
    return Ok(spendings_result);
    
}

#[cfg(test)]
mod tests {
    use std::time as stime;
    use time;

    #[test]
    fn it_works() {
        let s = stime::SystemTime::now().duration_since(stime::SystemTime::UNIX_EPOCH);
        let a = s.unwrap();
        println!("{:?}", time::Date::from_ordinal_date(1970, 1).unwrap() + a);
    }
}
