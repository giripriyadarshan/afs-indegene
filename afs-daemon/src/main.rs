use actix_web::{middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};

use models::{AppState, MessageData};
use utils::{amqp_pool::create_amqp_pool, amqp_send::send_message};

mod models;
mod utils;

#[post("/")]
async fn handle_request(
    state: web::Data<AppState>,
    message_data: web::Json<MessageData>,
) -> impl Responder {
    send_message(
        state,
        message_data.message.clone(),
        message_data.run_code.clone(),
    )
    .await
    .unwrap();

    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        amqp_pool: create_amqp_pool("amqp://user:password@172.30.106.6:5672/%2f".into()),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(handle_request)
    })
    .bind(("localhost", 3825))?
    .run()
    .await?;

    Ok(())
}
