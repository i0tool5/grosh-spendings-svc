//! Domain model for spending.

pub struct Spending {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub spending_date: Option<time::Date>,
    pub spending_types: Vec<String>,
    pub description: Option<String>,
    pub amount: i64,
}
