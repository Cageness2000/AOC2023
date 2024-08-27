
use crate::common;

pub fn solve(games:&Vec<common::Game>) -> u32{
    let mut sum:u32 = 0;
    for game in games{
        let mut can_occur = true;
        for pull in &game.pulls{
            let test:bool =  pull.red<=12 && pull.green<=13 && pull.blue<=14;
            can_occur &=test;
        }
        if can_occur {
            sum += game.id;
        }
    }
    return sum;
}
