use std::fs;

pub struct Pull {
   pub red:u32,
   pub green: u32,
   pub blue: u32
}
pub struct Game {
    pub id:u32,
    pub pulls: Vec<Pull>
}

impl Default for Pull {
    fn default() -> Pull {
        Pull{
            red:0,
            green:0,
            blue:0}
    }
}

impl std::fmt::Display for Pull {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Pull [ R: {}, G: {}, B: {}]",self.red,self.green,self.blue)
    }
}

impl Default for Game{
    fn default() -> Game {
        Game{
            id:0,
            pulls: Vec::new()
        }
    }
}
impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Game[ id: {} \n\tPulls [\n\t\t{}\n\t\t{}\n\t\t{}\n\t]]",self.id,self.pulls[0],self.pulls[1],self.pulls[2])
    }
}

pub fn read(file_path:&str) -> String {
    let contents:String = fs::read_to_string(file_path)
                            .expect("Should have been able to read");
    return contents;
}

fn get_id(left_side:&str) -> u32 {
    let out= left_side
            .split(' ')
            .collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
    return out;
}
fn gen_pull(pull_line:&str) -> Pull {
    let colors:Vec<String> = pull_line.split(',').map(|l| l.to_string()).collect();
    let mut i:usize = 0;
    let mut generated_pull:Pull = Pull{..Default::default()};
    loop {
        if i == colors.len() {break};
        let parsed_color:Vec<&str> = colors[i].split_whitespace().collect();
        let num = parsed_color[0].parse::<u32>().expect("Not a number");
        match parsed_color[1] {
            "green" => generated_pull.green=num,
            "red" => generated_pull.red=num,
            "blue" => generated_pull.blue=num,
            _ => panic!("No parsed Color")
        }
        i+=1;
    }
    return generated_pull;
}

fn gen_game(input: &str) -> Game {
    let mut generated_game = Game{..Default::default()};
    let line:Vec<String> = input.split(':').map(|l| l.to_string()).collect();
    generated_game.id=get_id(&line[0]);
    let generated_game: Game = 
        Game {
            id : get_id(&line[0]),
            pulls: line[1].split(';').map(|pull_line| gen_pull(pull_line)).collect(),
        }; 
    return generated_game; 
}
pub fn gen_games(input: &str) -> Vec<Game> {
    let games:Vec<Game> = input.lines().map(|line| { 
                                            gen_game(&line)}).collect();

    return games
}


