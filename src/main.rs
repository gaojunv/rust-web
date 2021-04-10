use actix_web::HttpResponse;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index(web::Path(()): web::Path<()>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "<html> 
          <head>
            <meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" />
                <title>gaojun.top</title>
          </head>
         <body><button>确定</button></body>  
        </html>"
    ))
}

#[get("/aa")]
async fn index2(web::Path(()): web::Path<()>) -> impl Responder {
    format!("<html> <body><button>2</button></body>  </html>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(index2))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
