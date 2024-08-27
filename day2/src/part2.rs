use crate::common;

pub fn solve(games:&Vec<common::Game>) -> u32{
    let mut sum:u32 = 0;
    for game in games{
        let mut min_red:u32 = 1;
        let mut min_blue:u32 = 1;
        let mut min_green:u32 = 1;
        for pull in &game.pulls{
            if min_red<pull.red{
                min_red = pull.red;
            }
            if min_blue<pull.blue{
                min_blue = pull.blue
            }
            if min_green<pull.green{
                min_green = pull.green
            }
        }
        sum += min_red*min_blue*min_green;
    }
    return sum;
}
