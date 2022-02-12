use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod file;

#[get("/")]
async fn list_people() -> impl Responder {
    let items = file::lists();
    HttpResponse::Ok().json(items)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .service(list_people)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
