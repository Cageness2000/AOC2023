//use std::env;
use std::fs;

fn read(file_path:String) -> String {
    let contents:String = fs::read_to_string(file_path)
                            .expect("Should have been able to read");
    return contents;
}

fn parse_line_part_1(s:String) -> String{ 
    let mut num:String = "".to_string();
    let mut j:u32 = 0;
    loop {
        if j==s.len() as u32 {break;}
        if s.chars().nth(j as usize).unwrap().is_numeric(){
            num+= &s.chars().nth(j as usize).unwrap().to_string();
        }
        j += 1;
    }
    return num;
}

fn parse_line_part_2(s:String) -> String{ 
    let num_strings = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    let numeric_strings = ["0","1","2","3","4","5","6","7","8","9"];
    let mut num:[String;2] = ["".to_string(),"".to_string()];
    let mut j:usize = 0;
    let mut min_index:usize = s.len()+1;
    let mut max_index:usize = 0;

    loop {
        if j==numeric_strings.len() {break;};
        if s.contains(num_strings[j]){
            let location = s.find(num_strings[j]).unwrap();
            if location <= min_index {
                num[0] = numeric_strings[j].to_string();
                min_index = location;
            }
            if max_index <= location {
                num[1] = numeric_strings[j].to_string();
                max_index = location;
            }
        }
        if s.contains(numeric_strings[j]){
            let locationn = s.find(numeric_strings[j]).unwrap();
            if locationn <= min_index {
                num[0] = numeric_strings[j].to_string();
                min_index = locationn;
            }
            if max_index <= locationn {
                num[1] = numeric_strings[j].to_string();
                max_index = locationn;
            }
        }
        j+=1;
    }
    let num1:&String = &num[0];
    let num2:&String = &num[1];
    let concat = num1.to_owned()+num2;
    println!("{}",concat);
    return concat;
}

fn parse_number(num:String) -> u32{
    let newstr:String =   num.chars().nth(0).unwrap().to_string()
                        +&num.chars().last().unwrap().to_string();
    return newstr.parse::<u32>().unwrap();
}

fn main() {
    let test_input:String = "input.txt".to_string();
    let contents = read(test_input);
    let splitted = contents
                    .split("\n")
                    .collect::<Vec<_>>();
    let mut i:u32 = 0;
    let mut sum_part1:u32 = 0;
    let mut sum_part2:u32 = 0;
    loop {
        if i==(splitted.len()-1) as u32 {break;} 
        
//        let num1 = parse_line_part_1(splitted[i as usize].to_string());
        let num2 = parse_line_part_2(splitted[i as usize].to_string());
        
 //       sum_part1+= parse_number(num1);
        sum_part2+= parse_number(num2);
        i += 1;
    }
    println!("Part 1: {}",sum_part1);
    println!("Part 2: {}",sum_part2);
    

}
