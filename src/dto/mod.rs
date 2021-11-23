mod user;

use serde::Serialize;

pub use user::*;

#[derive(Serialize)]
pub struct ListResultDto<T> {
    pub results: Vec<T>,
    pub count: usize,
}

impl<T> ListResultDto<T> {
    pub fn new(results: Vec<T>) -> ListResultDto<T> {
        let count = results.len();
        ListResultDto { results, count }
    }
}
