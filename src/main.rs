use color::Color;

mod argv;
mod color;
mod envs;
mod http;
mod update;

#[actix_web::main]
async fn main() {
    match update::update().await {
        Ok(()) => {
            println!("Update check {}.", color::color(Color::Green, "complete"));
            argv::arguments_app();
            envs::preset_envs();
            let _ = http::server().await;
        }
        Err(e) => eprintln!("Error during update check: {}", color::color(Color::Red, &e.to_string())),
    }
}
