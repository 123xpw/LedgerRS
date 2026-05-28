use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub meta: PaginationMeta,
}

/// Query parameters for paginated list endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 { 1 }
fn default_per_page() -> i64 { 20 }

impl PaginationQuery {
    pub fn offset(&self) -> i64 {
        (self.page - 1).max(0) * self.effective_per_page()
    }

    pub fn effective_per_page(&self) -> i64 {
        self.per_page.clamp(1, 100)
    }
}
