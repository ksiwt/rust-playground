use actix_web::{App, HttpResponse, HttpServer};

#[actix_web::get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8000")?
        .run()
        .await?;
    Ok(())
}
