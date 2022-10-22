fn get_input() -> &'static str {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2";
}

struct Point {
    x: i32,
    y: i32,
}

fn parse_line(line: &str) -> Point {
    let a = line.split_once(" ")
        
}

fn main() {
    println!("Hello, world!");
}
