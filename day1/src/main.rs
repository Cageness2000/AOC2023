mod common;
mod part1;
mod part2;


fn run(location:&String, parser:&dyn Fn(String)->String) -> u32 {
    let contents = common::read(location);
    let splitted = contents
                    .split("\n")
                    .collect::<Vec<_>>();
    let mut i:u32 = 0;
    let mut sum_part:u32 = 0;
    loop {
        if i==(splitted.len()) as u32 {break;} 
        let num = parser(splitted[i as usize].to_string());
        
        sum_part+= common::parse_number(num);
        i += 1;
    }
    return sum_part

}


fn main() {
    let test_input:String = "input".to_string();
    let day2_result = run(&test_input,&part2::parse_line);
    println!("{}",day2_result);
    //solve(&common::read(&test_input));
}


pub fn solve(input: &str) {
    let mut i:u32 = 0;
    let ans: u32 = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_digit(10));
            let first = chars
                .next()
                .expect("the line should have at least one digit");
            let num = match chars.last() {
                Some(last) => format!("{}{}", first, last),
                None => format!("{}{}", first, first),
            };
            println!("{} {} {}",i,num.chars().nth(0 as usize).unwrap(),num.chars().nth(1 as usize).unwrap());
            i+=1;
            num.parse::<u32>().unwrap()
        })
        .sum();
    println!("Answer: {}", ans)
}
