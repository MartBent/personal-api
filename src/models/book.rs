use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: u16,
    pub title: String,
    pub page_amount: u16,
    pub pages_read: u16,
    pub cover_img_src: String,
}