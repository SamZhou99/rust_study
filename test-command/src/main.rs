extern crate test_command;
extern crate tokio;

#[tokio::main]
async fn main() {
    test_command::lock_screen();
    // let url = "https://jsonplaceholder.typicode.com/posts/1";
    // let res = test_command::req::get(url).await;
    // println!("{:?}", res);
}
