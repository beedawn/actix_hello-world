use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/chicken")]
async fn chicken() -> impl Responder {
let chicken = fs::read("./imgs/lost_chicken.jpeg");

    match chicken {
        Ok(chicken) => HttpResponse::Ok().body(chicken),
        Err(err) => HttpResponse::Ok().body("Image not found")

    }


}



#[get("/hel")]
async fn hello3() -> impl Responder {
    let error = "Cannot read file.";
let html = fs::read_to_string("./html/index.html");
let error_page = fs::read_to_string("./html/404.html");
   // println!("hiiiiiii {}", html.as_ref().unwrap());
    match html{
       // "Cannot read file." => HttpResponse::Ok().body("File not found"),
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
     Err(err) => match error_page {
            Ok(error_page) =>HttpResponse::Ok().content_type("text/html").body(error_page),
            Err(err)=>HttpResponse::Ok().body("File not found"),
        }
        }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
async fn pizza_time() -> impl Responder {
    HttpResponse::Ok().body("<a href='http://www.pizza.com'>pizza</a>")
}
async fn hello_html() -> impl Responder {
    let html = fs::read_to_string("./html/index.html").expect("Cannot read file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}



async fn error_page() -> impl Responder {
    let error_file = fs::read_to_string("./html/404.html");

    match error_file {
        Ok(error_file) => HttpResponse::Ok().content_type("text/html").body(error_file),
        Err(err) => HttpResponse::Ok().body("File not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(hello3)
            .service(chicken)
            .route("/hey", web::get().to(manual_hello))
            .route("/pizza", web::get().to(pizza_time))
            .route("/hello2",web::get().to(hello_html))
        .default_service(
        web::route().to(error_page)
            )
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
