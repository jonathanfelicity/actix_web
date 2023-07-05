use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello()-> impl Responder{
    HttpResponse::Ok().body("hello, world")
}

#[post("/echo")]
async fn echo(req_body: String)-> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_msg() -> impl Responder{
    HttpResponse::Ok().body("This is so cool!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_msg))
    })
    .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
