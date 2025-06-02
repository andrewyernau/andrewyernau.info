use actix_web::{
    get, App, HttpResponse, HttpServer, Responder
    };
    use actix_files::Files;
    use lazy_static::lazy_static;
    use tera::Tera;
    use chrono::Datelike;
    
    lazy_static! {
        pub static ref TEMPLATES: Tera = {
            let mut tera = match Tera::new("templates/**/*") {
                Ok(t) => t,
                Err(e) => {
                    println!("Error parsing templates: {}", e);
                    ::std::process::exit(1);
                }
            };
            tera.autoescape_on(vec![".html", ".htm"]);
            tera
        };
    }
    
    #[get("/")]
    async fn index() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("section", "userinfo");
        let page_content = TEMPLATES.render("index.html", &context).unwrap();
        HttpResponse::Ok().body(page_content)
    }
    
    #[get("/year")]
    async fn year() -> impl Responder {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    HttpResponse::Ok().body(year.to_string())
    }
    
    #[get("/userinfo")]
    async fn userinfo() -> impl Responder {
    let  context = tera::Context::new();
    let page_content = TEMPLATES.render("userinfo.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
    }
    
    #[get("/profession")]
    async fn profession() -> impl Responder {
        let mut ctx = tera::Context::new();
        ctx.insert("section", "profession");
        let page = TEMPLATES.render("profession.html", &ctx).unwrap();
        HttpResponse::Ok().body(page)
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
    
    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
    App::new()
    .service(index)
    .service(userinfo)
    .service(aboutme)
    .service(profession)
    .service(projects)
    .service(misc)
    .service(year)
    .service(Files::new("/static", "./static").prefer_utf8(true))})
    .bind(("127.0.0.1", 8880))?
    .run()
    .await
    }