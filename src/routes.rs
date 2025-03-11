use actix_web::{get, web, HttpResponse, Responder};
use lazy_static::lazy_static;
use tera::Tera;
use chrono::Datelike;
use crate::{db, models};
use base64;
use sqlx::MySqlPool;
use std::sync::Mutex;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::new("templates/**/*").unwrap();
        tera.autoescape_on(vec!["html", "htm"]);
        tera
    };
    pub static ref CURRENT_IMAGE_ID: Mutex<i32> = Mutex::new(1);
}

pub fn config_routes(cfg: &mut web:: ServiceConfig) {
    cfg.service(index)
       .service(userinfo)
       .service(aboutme)
       .service(profession)
       .service(projects)
       .service(misc)
       .service(year)
       .service(gallery_current)
       .service(gallery_next)
       .service(gallery_prev);
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("section", "Cargando usuario...");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

// Mantener tus rutas existentes...
#[get("/year")]
async fn year() -> impl Responder {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    HttpResponse::Ok().body(year.to_string())
}

#[get("/userinfo")]
async fn userinfo() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("userinfo.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/profession")]
async fn profession() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("profession.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/projects")]
async fn projects() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("projects.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/misc")]
async fn misc() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("misc.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/aboutme")]
async fn aboutme() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("aboutme.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}


#[get("/api/gallery-image/current")]
async fn gallery_current(db_pool: web::Data<MySqlPool>) -> impl Responder {
    match db::get_current_image(db_pool.get_ref()).await {
        Ok(image) => {
            // Actualizar el ID actual
            let mut current_id = CURRENT_IMAGE_ID.lock().unwrap();
            *current_id = image.id;
            
            // Renderizar el HTML de la imagen
            render_gallery_image(image)
        },
        Err(_) => {
            HttpResponse::Ok().body("<p>No hay imágenes disponibles</p>")
        }
    }
}

#[get("/api/gallery-image/next")]
async fn gallery_next(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let current_id = *CURRENT_IMAGE_ID.lock().unwrap();
    
    match db::get_next_image(db_pool.get_ref(), current_id).await {
        Ok(image) => {
            let mut current_id = CURRENT_IMAGE_ID.lock().unwrap();
            *current_id = image.id;
            render_gallery_image(image)
        },
        Err(_) => {
            HttpResponse::Ok().body("<p>Error al cargar la siguiente imagen</p>")
        }
    }
}

#[get("/api/gallery-image/prev")]
async fn gallery_prev(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let current_id = *CURRENT_IMAGE_ID.lock().unwrap();
    
    match db::get_prev_image(db_pool.get_ref(), current_id).await {
        Ok(image) => {
            let mut current_id = CURRENT_IMAGE_ID.lock().unwrap();
            *current_id = image.id;
            
            render_gallery_image(image)
        },
        Err(_) => {
            HttpResponse::Ok().body("<p>Error al cargar la imagen anterior</p>")
        }
    }
}

fn render_gallery_image(image: models::GalleryImage) -> HttpResponse {
    let base64_image = base64::encode(&image.image_data);
    let data_url = format!("data:{};base64,{}", image.mime_type, base64_image);
    let caption = image.caption.unwrap_or_else(|| "Sin descripción".to_string());
    
    let html = format!(
        r#"<img src="{}" alt="{}" id="gallery-current-image">
           <p id="image-caption">{}</p>"#,
        data_url, caption, caption
    );
    
    HttpResponse::Ok().body(html)
}