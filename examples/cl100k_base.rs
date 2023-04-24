use tiktokener::{decode, encode, load_tiktoken};

fn main() {
    let file = "./cl100k_base.tiktoken";
    let special_tokens = vec![100257, 100258, 100259, 100260, 100276];
    let pieces = load_tiktoken(file, &special_tokens);

    println!("count: {}", pieces.len());
    println!("{:?}", encode(&pieces, b"hello world"));
    println!(
        "{}",
        String::from_utf8(decode(&pieces, &encode(&pieces, b"hello world"))).unwrap()
    );
}
