use tiktokener::{decode, encode, load_tiktoken};

fn main() {
    let file = "./p50k_base.tiktoken";
    let special_tokens = vec![50256];
    let pieces = load_tiktoken(file, &special_tokens);

    println!("count: {}", pieces.len());
    println!("{:?}", encode(&pieces, b"hello world"));
    println!(
        "{}",
        String::from_utf8(decode(&pieces, &encode(&pieces, b"hello world"))).unwrap()
    );
    println!("{:?}", encode(&pieces, b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."));
}
