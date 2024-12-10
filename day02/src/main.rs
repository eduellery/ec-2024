use regex::Regex;
use utils::read_file_lines;

fn main() {
    let lines = read_file_lines("res/quest02.p2.example").expect("Unable to read file");
    part1(lines.clone());
    part2(lines.clone());
}

fn get_words(input: &str) -> Option<Vec<&str>> {
    let after_colon = input.split_once(':')?.1;
    let words: Vec<&str> = after_colon.split(',').collect();
    Some(words)
}

fn part1(lines: Vec<String>) {
    let phrase = lines[2].as_str();
    let mut count = 0;
    match get_words(lines[0].as_str()) {
        Some(words) => {
            for word in &words {
                let regex = Regex::new(word).expect("Invalid regex pattern");
                count += regex.find_iter(phrase).count();
            }
        }
        None => {}
    }
    println!("Part 1: {}", count)
}
