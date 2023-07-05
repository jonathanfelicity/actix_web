#[allow(dead_code)]

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


async fn index() -> impl Responder {
    "Hello World"
}

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
//            .service(hello)
//            .service(echo)
//            .route("/hey", web::get().to(manual_msg))
//
//            web::scope("/app")
//              .route("index.html", web::get().to(index)),
    })
    .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
