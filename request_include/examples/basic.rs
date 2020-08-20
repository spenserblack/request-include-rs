use request_include::include_str as request_str;

fn main() {
    /// We're using `"request-include example"` as our user agent, but you
    /// should define your own in your project.
    const VAL: &str = request_str!("https://api.github.com/search/repositories?q=tetris&sort=stars&order=desc", "request-include example");
    println!("{:?}", VAL);
}
