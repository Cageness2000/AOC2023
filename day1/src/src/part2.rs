


pub fn parse_line(s:String) -> String{ 
    let num_strings = ["one","two","three","four","five","six","seven","eight","nine"];
    let numeric_strings = ["1","2","3","4","5","6","7","8","9"];
    let mut num:[String;2] = ["".to_string(),"".to_string()];
    let mut j:usize;
    let mut i:usize;
    let mut found;
    
    i = 0;
    'front: loop {
        if i==s.len() {break 'front};
        i +=1;
        j = 0;
        found = false;
        loop {
            if j == num_strings.len()              {break};
            if s[..i].contains(num_strings[j]    ) {found = true; break};
            if s[..i].contains(numeric_strings[j]) {found = true; break};
            j += 1;
        }
        if found {num[0] = numeric_strings[j].to_string(); break 'front}; 
    };
    i = s.len();
    'back: loop {
        if i==0 {break 'back};
        i -=1;
        j = 0;
        found = false;
        loop {
            if j == num_strings.len()              {break};
            if s[i..].contains(num_strings[j]    ) {found = true; break};
            if s[i..].contains(numeric_strings[j]) {found = true; break};
            j += 1;
        }
        if found {num[1] = numeric_strings[j].to_string(); break 'back}; 
    };
    println!("{} {}",num[0],num[1]); 
    let concat = num[0].to_string()+&num[1].to_string();
    return concat;
}

