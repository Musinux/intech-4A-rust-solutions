fn remove_one_on_two(s: &String) -> String {
    let mut s_out = String::new();
    for (i, s) in s.chars().enumerate() {
        if i % 2 == 1 {
            s_out.push(s);
        }
    }
    s_out
}

fn main() {
    println!("Hello, world!");
    let s = "🙇👨‍🎤hello! 👀".to_string();

    let s = remove_one_on_two(&s);
    println!("result: {}", s);
}
