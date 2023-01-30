use std::process::Output;
use actix_web::{web, App, HttpServer, Responder, Scope, HttpResponse };
use use_cases::use_case::create_generator::CreateGeneratorUseCase;
use use_cases::use_case::{UseCaseA0, UseCaseA1};
use domain::repository::generator_repository::GeneratorRepository;

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
    let createGeneratorUseCase = CreateGeneratorUseCase {
        repository: GeneratorRepository {}
    };

    for string in strings.into_iter() {
        scope = scope.service(web::resource(string).route(web::get().to(
            UseCaseA1Wrapper {
                usecase: createGeneratorUseCase
            }
        )))
    }
    scope
}

struct UseCaseA1Wrapper {
    usecase: dyn UseCaseA1<Output=dyn Any>,
}

impl Responder for UseCaseA1Wrapper {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        &self.usecase.execute(item: web::Json<MyObj>);

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
