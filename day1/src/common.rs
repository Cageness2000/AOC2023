use std::fs;

pub fn parse_number(num:String) -> u32{
    let newstr:String =   num.chars().nth(0).unwrap().to_string()
                        +&num.chars().last().unwrap().to_string();
    return newstr.parse::<u32>().unwrap();
}

pub fn read(file_path:&String) -> String {
    let contents:String = fs::read_to_string(file_path)
                            .expect("Should have been able to read");
    return contents;
}

