use actix_web::{web, App, HttpServer, Responder, Scope, HttpResponse };

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(test([String::from("index.html")]))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn test(strings: [String; 1]) -> Scope {
    let mut scope = web::scope("/app");
    for string in strings.into_iter() {
        scope = scope.service(web::resource(string).route(web::get().to(index)))
    }
    scope
}
