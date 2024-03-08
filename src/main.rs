use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;

//let x:i32=0;


//while (x<10){

//#[get("/{}"

//}


// slash route returns "irectory of files
#[get("/")]
async fn directory() -> impl Responder {
let mut path_string:String = "".to_owned();

for entry in fs::read_dir("./html").unwrap() {
let entry_path = entry.unwrap().path();
        if let Ok(entry) = fs::read_dir("./html"){
    println!("{:?}",entry_path);
        }else{
            println!("bye");

        }

        path_string.push_str("<p><a href=\"");
path_string.push_str(entry_path.display().to_string().as_str());
        path_string.push_str("\">");
        path_string.push_str(entry_path.display().to_string().as_str());
        path_string.push_str("</a></p>");
        path_string.push_str("\n");

    }

    HttpResponse::Ok().body(path_string)
}
//provides chicken picture to /chicken end point
// used in 404.html 
#[get("/chicken")]
async fn chicken() -> impl Responder {
    let chicken = fs::read("./imgs/lost_chicken.jpeg");
//check if chicken file read was success
    match chicken {
        //if success, return the chicken
        Ok(chicken) => HttpResponse::Ok().body(chicken),
        //no success, chicken is secret
        Err(err) => HttpResponse::Ok().body("Image not found")
    }
}
//test of error handling if file exists/did not exist
#[get("/gremlin")]
async fn gremlin() -> impl Responder {
    //error message if neither index.html or 404.html file(s) is(are) not found
    let error_var = "<p>Cannot read file.</p>";
    //reads index.html file
    let html = fs::read_to_string("./html/index.html");
    //reads 404 error page file
    let error_page = fs::read_to_string("./html/404.html");
    //check if file read of html variable(index.html) was successful
    match html{
        //no error, index.html path exists
            Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        //if index does not exist, go check if 404 page exists
     Err(err) => match error_page {
            //sends 404 error page
            Ok(error_page) => HttpResponse::Ok().content_type("text/html").body(error_page),
            //text displayed if both 404 page and index cannot be found
            Err(err)=> HttpResponse::Ok().body(error_var),
        }
     }
}
//echos post request
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
// responder for /hey endpoint
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
//responder for /pizza endpoint
async fn pizza_time() -> impl Responder {
    HttpResponse::Ok().body("<a href='http://www.pizza.com'>pizza</a>")
}
//responder for /unsaf_gremlin endpoint
async fn unsaf_gremlin() -> impl Responder {
    //if this cannot read index.html, it will crash the server
    let html = fs::read_to_string("./html/index.html").expect("Cannot read file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}
//404 error page for default service to handle all unaddressed endpoints
async fn error_page() -> impl Responder {
    //read 404.html to String
    let error_file = fs::read_to_string("./html/404.html");
//check if file read was successful or not
    match error_file {
        //404.html exists
        Ok(error_file) => HttpResponse::Ok().content_type("text/html").body(error_file),
        //404.html does not exist
        Err(err) => HttpResponse::Ok().body("File not found")
    }
}
//main server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //HttpServer instantiatiatiation
    HttpServer::new(|| {
        App::new()
            //slash / endpoint
            .service(directory)
            // /echo endpoint
            .service(echo)
            //slash /gremlin endpoint
            .service(gremlin)
            // /chicken endpoint
            .service(chicken)
            .route("/hey", web::get().to(manual_hello))
            .route("/pizza", web::get().to(pizza_time))
            .route("/unsaf_gremlin",web::get().to(unsaf_gremlin))
            //handles all unaddressed endpoints
        .default_service(
        web::route().to(error_page)
            )
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
