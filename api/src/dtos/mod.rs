pub mod orders;
pub mod products;
pub mod users;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestQueryDto {
    #[validate(range(min = 1, message = "Page can only be 1 or more"))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50, message = "limit can only be between 1 and 50"))]
    pub limit: Option<usize>,
}
