
pub fn parse_line(s:String) -> String{ 
    let mut num:String = "".to_string();
    let mut j:usize = 0;
    loop {
        if j==s.len(){break;}
        let char_at_j = s.chars().nth(j).unwrap();
        if char_at_j.is_numeric(){
            num+= &char_at_j.to_string();
        }
        j += 1;
    }
    let concat =  num.chars().nth(0).unwrap().to_string()
                +&num.chars().last().unwrap().to_string();
    println!("{} {}",concat.chars().nth(0).unwrap(),concat.chars().nth(1).unwrap());
    return concat;
}
