use std::sync::Arc;

use axum::http;
use migration::SimpleExpr;
use serde::{
    Deserialize,
    Serialize,
};

use sea_orm::{
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect
};

use crate::application;
use crate::infrastructure::database::entities;

pub struct SpendingsListQueryArgs {
    pub user_id: uuid::Uuid,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpendingResponse {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub spending_type : String,
    #[serde(rename = "date")]
    pub spending_date: Option<time::Date>,
    pub description: Option<String>,
    pub amount: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpendingsResponse {
    data: Vec<SpendingResponse>,
    count: u64,
}

pub struct SpendingsListQueryHandler {
    db: Arc<sea_orm::DatabaseConnection>,
}

impl SpendingsListQueryHandler {
    pub fn new(db: Arc<sea_orm::DatabaseConnection>) -> SpendingsListQueryHandler {
        SpendingsListQueryHandler{db}
    }

    pub async fn handle(
        self: &Self,
        args: SpendingsListQueryArgs,
    ) -> anyhow::Result<application::Response<SpendingsResponse>> {
        let limit = args.limit.unwrap_or(25);
        let offset = args.offset.unwrap_or_default();

        let user_id_filter = entities::spendings::Column::UserId.eq(args.user_id);
        let count = entities::prelude::Spendings::find()
            .filter(SimpleExpr::clone(&user_id_filter))
            .count(self.db.as_ref())
            .await?;

        let stored_spendings  = entities::prelude::Spendings::find()
            .filter(SimpleExpr::clone(&user_id_filter))
            .limit(limit)
            .offset(offset)
            .order_by(entities::spendings::Column::SpendingDate, Order::Desc)
            .all(self.db.as_ref())
            .await?;


        let mut spendings_response: Vec<SpendingResponse> = Vec::with_capacity(
            stored_spendings.len(),
        );

        for sp in stored_spendings {
            spendings_response.push(SpendingResponse {
                id: sp.id,
                spending_type: sp.spending_type,
                spending_date: sp.spending_date,
                description: sp.description,
                amount: sp.amount.try_into().unwrap(),
            });
        }
        
        let spendings_result = SpendingsResponse{
            data: spendings_response,
            count: count,
        }; 

        Ok(
            application::Response {
                status_code: http::StatusCode::OK,
                data: spendings_result,
            }
        )
    }
}

