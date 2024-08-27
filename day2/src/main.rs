mod common;
mod part1;
mod part2; 


fn main() {
    let file = "input";
    let contents = common::read(file);
    let games = common::gen_games(&contents);
    let part1_ans = part1::solve(&games);
    let part2_ans = part2::solve(&games);
    println!("Part 1: {}",part1_ans);
    println!("Part 2: {}",part2_ans);
}
