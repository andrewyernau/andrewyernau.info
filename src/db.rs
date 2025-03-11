use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct GalleryImage {
    pub id: i32,
    pub caption: Option<String>,
    pub image_data: Vec<u8>,
    pub mime_type: String,
}

#[derive(Deserialize)]
pub struct ImageUpload {
    pub image: Vec<u8>,
    pub caption: Option<String>,
    pub position: Option<i32>,
}

pub async fn create_db_pool() -> MySqlPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn get_current_image(pool: &MySqlPool) -> Result<models::GalleryImage, sqlx::Error> {
    sqlx::query_as::<_, models::GalleryImage>(
        "SELECT id, caption, image_data, mime_type FROM gallery_images ORDER BY position LIMIT 1"
    )
    .fetch_one(pool)
    .await
}

pub async fn get_next_image(pool: &MySqlPool, current_id: i32) -> Result<crate::models::GalleryImage, sqlx::Error> {
    let next_image = sqlx::query_as::<_, crate::models::GalleryImage>(
        "SELECT id, caption, image_data, mime_type FROM gallery_images 
         WHERE position > (SELECT position FROM gallery_images WHERE id = ?) 
         ORDER BY position LIMIT 1"
    )
    .bind(current_id)
    .fetch_one(pool)
    .await;
    
    if let Err(_) = next_image {
        return get_current_image(pool).await;
    }
    
    next_image
}

pub async fn get_prev_image(pool: &MySqlPool, current_id: i32) -> Result<crate::models::GalleryImage, sqlx::Error> {
    let prev_image = sqlx::query_as::<_, crate::models::GalleryImage>(
        "SELECT id, caption, image_data, mime_type FROM gallery_images 
         WHERE position < (SELECT position FROM gallery_images WHERE id = ?) 
         ORDER BY position DESC LIMIT 1"
    )
    .bind(current_id)
    .fetch_one(pool)
    .await;

    if let Err(_) = prev_image {
        return sqlx::query_as::<_, crate::models::GalleryImage>(
            "SELECT id, caption, image_data, mime_type FROM gallery_images 
             ORDER BY position DESC LIMIT 1"
        )
        .fetch_one(pool)
        .await;
    }
    prev_image
}