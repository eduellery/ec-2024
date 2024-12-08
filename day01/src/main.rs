use utils::read_file_to_string;

fn main() {
    let content = read_file_to_string("res/quest1.p3.in").expect("Unable to read file");
    part1(content.clone());
    part2(content.clone());
    part3(content.clone());
}

fn part1(content: String) {
    let mut result = 0;
    for ch in content.chars() {
        result += get_monster_score(ch)
    }
    println!("Part 1: {}", result)
}

fn part2(content: String) {
    let mut result = 0;
    let mut chars = content.chars();
    while let (Some(first), Some(second)) = (chars.next(), chars.next()) {
        if first != 'x' && second != 'x' {
            result += 2
        }
        result += get_monster_score(first);
        result += get_monster_score(second);
    }
    if let Some(_) = chars.next() {
        unimplemented!("Monster ungrouped")
    }
    println!("Part 2: {}", result)
}

fn part3(content: String) {
    let mut result = 0;
    let mut chars = content.chars();
    while let (Some(first), Some(second), Some(third)) = (chars.next(), chars.next(), chars.next()) {
        let under_test = [first, second, third];
        let number_of_x = under_test.iter().filter(|&&ch| ch == 'x').count();
        result += match number_of_x {
            0 => 6,
            1 => 2,
            _ => 0,
        };
        result += get_monster_score(first);
        result += get_monster_score(second);
        result += get_monster_score(third);
    }
    if let Some(_) = chars.next() {
        unimplemented!("Monster(s) ungrouped")
    }
    println!("Part 2: {}", result)
}

fn get_monster_score(monster: char) -> u32 {
    match monster {
        'A' | 'x' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => unimplemented!("Can't find monster {}", monster),
    }
}
