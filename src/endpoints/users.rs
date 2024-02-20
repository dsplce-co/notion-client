use std::sync::Arc;

use reqwest::Client;

pub mod list;
pub mod retrieve;
#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct UsersEndpoint {
    pub(super) client: Arc<Client>,
}
