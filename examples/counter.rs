#[tokio::main]
async fn main() {
    tokio_data::spawn(|name| async move {
        println!("hello {}!", name);
    });
}
