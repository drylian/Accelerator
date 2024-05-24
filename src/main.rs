mod argv;
mod http;
mod envs;

#[actix_web::main]
async fn main() {
    argv::arguments_app();
    envs::preset_envs();
    let _ = http::server().await;
    print!("Hello, world!");
}
