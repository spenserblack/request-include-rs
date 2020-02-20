use curl_include::include_str as curl_str;

fn main() {
    let val = curl_str!("https://api.github.com/search/repositories?q=tetris&sort=stars&order=desc");
    println!("{:?}", val);
}
