use request_include::include_str as request_str;

fn main() {
    const VAL: &str = request_str!("https://api.github.com/search/repositories?q=tetris&sort=stars&order=desc");
    println!("{:?}", VAL);
}
