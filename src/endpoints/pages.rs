use std::sync::Arc;

use reqwest::Client;

pub mod create;
pub mod retrieve;
#[cfg(test)]
mod tests;
pub mod update;

pub struct PagesEndpoint {
    pub(super) client: Arc<Client>,
}
