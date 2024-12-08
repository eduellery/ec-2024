use utils::read_file_to_string;

fn main() {
    let content = read_file_to_string("res/quest1.in").expect("Unable to read file");
    let mut result = 0;
    for ch in content.chars() {
        match ch {
            'B' => result += 1,
            'C' => result += 3,
            _ => {},
        }
    }
    println!("Result: {}", result)
}
