use tiktokener::{decode, encode, load_tiktoken};

fn main() {
    let file = "./cl100k_base.tiktoken";
    let pieces = load_tiktoken(file);

    println!("count: {}", pieces.len());
    println!("{:?}", encode(&pieces, b"hello world"));
    println!(
        "{}",
        String::from_utf8(decode(&pieces, &encode(&pieces, b"hello world"))).unwrap()
    );
}
