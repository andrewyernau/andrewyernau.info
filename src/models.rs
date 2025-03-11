use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GalleryImage {
    pub id: i32,
    pub caption: Option<String>,
    pub image_data: Vec<u8>,
    pub mime_type: String,
}

// Estructura para almacenar el ID de la imagen actual en la sesión
#[derive(Serialize, Deserialize)]
pub struct CurrentImageId {
    pub id: i32,
}