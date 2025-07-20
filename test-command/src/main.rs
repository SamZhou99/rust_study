extern crate test_command;
extern crate tokio;

#[tokio::main]
async fn main() {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let res = test_command::req::get(url).await;
    println!("{:?}", res);
}
