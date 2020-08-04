use futures::executor::block_on;

async fn hello() {
    println!("Hello World!");
}

fn main() {
    let futures = hello();
    block_on(future);
}