use actix_web::{post, App, HttpResponse, HttpServer, web, Responder};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use crate::infra::memory_repository::MemoryRepository;
use crate::usecase::simple_usecase::SimpleUsecase;
use crate::usecase::usecase::{TimeRecordWithID, Usecase};

mod domain;
mod usecase;
mod infra;

#[derive(Serialize, Deserialize)]
struct CreateTimeRecordsRequest {
    id: String,
    start: String,
    end: Option<String>,
    memo: String,
}

// #[post("/time_records")]
// async fn create_time_records(handler: Data<&mut SimpleUsecase>, req: web::Json<CreateTimeRecordsRequest>) -> Result<HttpResponse, actix_web::Error> {
//     handler.create_time_record(TimeRecordWithID {
//         id: req.id.clone(),
//         start: req.start.clone(),
//         end: req.end.clone().unwrap_or("".to_string()),
//         memo: req.memo.clone(),
//     })?;
//     // let mut usecase = SimpleUsecase::new(MemoryRepository::new());
//     // usecase.create_time_record(TimeRecordWithID {
//     //     id: req.id.clone(),
//     //     start: req.start.clone(),
//     //     end: req.end.clone().unwrap_or("".to_string()),
//     //     memo: req.memo.clone(),
//     // })?;
//     Ok(HttpResponse::Ok().status(StatusCode::OK).finish())
// }

async fn index() -> impl Responder {
    "Hello world!"
}

// #[actix_web::main]
fn main() {
    let usecase = SimpleUsecase::new(MemoryRepository::new());
    usecase.create_time_record(TimeRecordWithID {
        id: "asdf".to_string(),
        start: "2023-02-15 20:54:00".to_string(),
        end: "2023-02-15 20:54:00".to_string(),
        memo: "asdf".to_string(),
    }).unwrap();

    // let mut data = Data::new(&mut usecase);
    // HttpServer::new(|| {
    //     App::new()
    //         .app_data(Data::clone(&mut data))
    //         .service(create_time_records)
    //         .route("/hey", web::get().to(index))
    // })
    //     .bind(("127.0.0.1", 8080))?
    //     .run()
    //     .await
}
