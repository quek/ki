#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}
