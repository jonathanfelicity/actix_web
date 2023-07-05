#[allow(dead_code)]

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState{
    app_name: String,
}

#[get("/state")]
async fn state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}
async fn index() -> impl Responder {
    "Hello World"
}

#[allow(dead_code)]
#[get("/")]
async fn hello()-> impl Responder{
    HttpResponse::Ok().body("hello, world")
}

#[allow(dead_code)]
#[post("/echo")]
async fn echo(req_body: String)-> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[allow(dead_code)]
async fn manual_msg() -> impl Responder{
    HttpResponse::Ok().body("This is so cool!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
//        App::new()
//            .service(hello)
//            .service(echo)
//            .route("/hey", web::get().to(manual_msg))
//
         App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(state)
            .service(
            web::scope("/app")
                .route("index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
