use actix_web::{middleware::Logger, post, App, HttpResponse, HttpServer, Responder};
use ssh2::Session;
use std::net::TcpStream;

#[post("/")]
async fn handle_request(svn_url: String) -> impl Responder {
    let tcp = TcpStream::connect("172.30.254.169:22").unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    session.userauth_password("giri", "Welcome@123").unwrap();

    let mut channel = session.channel_session().unwrap();
    let uuid = uuid::Uuid::new_v4();
    let command = format!(
        "cd afs ; nohup afs-checkout-tool {} {} > ~/afs-logs/{}.log &",
        uuid.as_u128(),
        svn_url,
        uuid.as_u128()
    );
    channel.exec(command.as_str()).unwrap();

    session.disconnect(None, "task run", None).unwrap();
    HttpResponse::Ok().body(uuid.as_u128().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().wrap(Logger::default()).service(handle_request))
        .bind(("localhost", 7892))?
        .run()
        .await?;

    Ok(())
}
