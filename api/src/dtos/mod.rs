pub mod orders;
pub mod products;
pub mod users;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestQueryDto {
    #[validate(range(min = 1, message = "Page can only be 1 or more"))]
    #[schema(example = 1)]
    pub page: Option<usize>,

    #[validate(range(min = 1, max = 50, message = "limit can only be between 1 and 50"))]
    #[schema(example = 10)]
    pub limit: Option<usize>,
}
