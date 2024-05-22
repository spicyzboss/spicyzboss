use crate::domain::errors::RepositoryError;
use serde::{Deserialize, Serialize};

pub const DEFAULT_CURSOR: &str = "00000000-0000-0000-0000-000000000000";
pub const DEFAULT_SIZE: usize = 100;
pub const MIN_SIZE: usize = 1;
pub const DEFAULT_SHOW_DELETED: bool = false;

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Serialize, Deserialize)]
pub struct PaginationMetadata {
  pub total: usize,
  pub size: usize,
  pub pages: usize,
  pub cursor: Option<String>,
}

impl PaginationMetadata {
  pub fn new(total: usize, size: usize, cursor: Option<String>) -> Self {
    let pages = Self::calculate_total_pages(total, size);

    Self {
      total,
      size,
      cursor,
      pages,
    }
  }

  fn calculate_total_pages(total: usize, size: usize) -> usize {
    total.div_ceil(size)
  }
}

#[derive(Serialize, Deserialize)]
pub struct PaginationResult<T> {
  pub data: Vec<T>,
  pub meta: PaginationMetadata,
}

impl<T> PaginationResult<T> {
  pub fn new(meta: PaginationMetadata, data: Vec<T>) -> Self {
    Self { data, meta }
  }
}

pub trait QueryParams {
  fn get_size(&self) -> Option<usize>;
  fn get_cursor(&self) -> Option<String>;
  fn get_show_deleted(&self) -> Option<bool>;

  fn size(&self) -> usize {
    self.get_size().unwrap_or(DEFAULT_SIZE).max(MIN_SIZE)
  }

  fn cursor(&self) -> String {
    self.get_cursor().unwrap_or(DEFAULT_CURSOR.to_owned())
  }

  fn show_deleted(&self) -> bool {
    self.get_show_deleted().unwrap_or(DEFAULT_SHOW_DELETED)
  }
}
