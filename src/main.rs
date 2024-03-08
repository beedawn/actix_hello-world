use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::path::PathBuf;
use std::path::Path;

fn read_files (user_path:String)->String{

    //mutable string to build over course of function
    let mut path_string:String = "".to_owned();
//do we need a veector?
    let mut path_vector: Vec<PathBuf>= vec![];
    //loops through each of the files 
    for entry in fs::read_dir(user_path.clone()).unwrap() {
        //unwraps entry into the path
        let entry_path = entry.unwrap().path();
        //gets a usable string from entry path because we use it alot right now
let entry_path_string = entry_path.display().to_string();
//if entry read is ok ?
        if let Ok(entry) = fs::read_dir(user_path.clone()){
           // println!("{:?}",entry_path);
            // checks if entry is a directory
            if Path::new(&entry_path_string).is_dir(){
                //prints out found directories
                println!("{} is dir", entry_path_string);

                //recursively calls read_files
                //need to figureout if we should move to vector or keep string 
                path_string.push_str(read_files(entry_path_string.clone()).as_str());

            }
            path_vector.push(entry_path.clone());
        }else{
            println!("Error reading file directory.");

        }
        path_string.push_str(format!("<p><a href=\"{}\">{}</a></p>\n",entry_path_string,entry_path_string).as_str());
    
    }
    println!("{:?}",path_vector);
    path_string
}

// slash route returns "irectory of files
#[get("/")]
async fn directory() -> impl Responder {
    let html_paths:String = read_files(String::from("."));
    HttpResponse::Ok().body(html_paths)
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
