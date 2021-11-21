mod user;

use serde::Serialize;

pub use user::*;

#[derive(Serialize)]
pub struct ListResult<T> {
    pub results: Vec<T>,
    pub count: usize,
}

impl<T> ListResult<T> {
    pub fn new(results: Vec<T>) -> Self {
        let count = results.len();
        ListResult { results, count }
    }
}
