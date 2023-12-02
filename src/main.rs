use std::io::stdin;

use puzzles::day1;

mod puzzles;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, which day and puzzle would you like to test");
    let mut day = String::new();
    stdin().read_line(&mut day)?;

    let mut puzzle = String::new();
    stdin().read_line(&mut puzzle)?;

    let day_number = day.trim_end().parse::<i32>().unwrap();
    let puzzle_number = puzzle.trim_end().parse::<i32>().unwrap();
    println!("day: {}", day);
    println!("puzzle: {}", puzzle);

    find_match_day_puzzle(day_number, puzzle_number);

    Ok(())
}

fn find_match_day_puzzle(day: i32, puzzle: i32) {
    match day {
        1 => switch_puzzles(puzzle, &day1::puzzle_one, &day1::puzzle_two),
        _ => (),
    }
}

fn switch_puzzles(choice: i32, f1: &dyn Fn(), f2: &dyn Fn()) {
    match choice {
        1 => f1(),
        2 => f2(),
        _ => (),
    }
}
