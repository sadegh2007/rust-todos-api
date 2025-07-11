use diesel::{prelude::Insertable, AsChangeset};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::todos)]
pub struct UpsertTodoRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 3, max = 500))]
    pub content: String
}