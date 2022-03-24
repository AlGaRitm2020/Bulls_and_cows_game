use std::io;


//println!("____        _ _                       _ ");
//println!("| __ ) _   _| | |___    __ _ _ __   __| |");
//println!("|  _ \| | | | | / __|  / _` | '_ \ / _` |");
//println!("| |_) | |_| | | \__ \ | (_| | | | | (_| |");
//println!("|____/ \__,_|_|_|___/  \__,_|_| |_|\__,_|");
//println!("");
//println!("____                                               ");
//println!("/ ___|_____      _____    __ _  __ _ _ __ ___   ___ ");
//println!("| |   / _ \ \ /\ / / __|  / _` |/ _` | '_ ` _ \ / _ \");
//println!("| |__| (_) \ V  V /\__ \ | (_| | (_| | | | | | |  __/");
//println!("\____\___/ \_/\_/ |___/  \__, |\__,_|_| |_| |_|\___|");
//println!("|___/ ");



fn main() {
     
println!("____        _ _                       _ ");
println!("| __ ) _   _| | |___    __ _ _ __   __| |");
println!("|  _ \| | | | | / __|  / _` | '_ \ / _` |");
println!("| |_) | |_| | | \__ \ | (_| | | | | (_| |");
println!("|____/ \__,_|_|_|___/  \__,_|_| |_|\__,_|");
println!("Hello, it's Bulls and Cows game");
    
    let player_1_name: String = input_name("1"); 
    let player_2_name: String = input_name("2"); 

    println!("Hello {} and {}. Good luck!", &player_1_name.as_str(), &player_2_name.as_str());

    let player_1_num: String = input_numbers(&player_1_name.as_str(), "create"); 
    let player_2_num: String = input_numbers(&player_2_name.as_str(), "create"); 

    loop {
        let guess_1: String = input_numbers(&player_2_name.as_str(), "guess");
        
        let response: String = get_response(&player_1_num, &guess_1);
        println!("{}", response.as_str());
        
        let guess_2: String = input_numbers(&player_1_name.as_str(), "guess");

        let response: String = get_response(&player_1_num, &guess_1);
        println!("{}", response.as_str());
        

    }

}

fn get_response(player_num: &String, guess: &String) -> String{
   if player_num == guess {
       return String::from("4 bulls. You are winner!")
   }

   let mut bulls: u8 = 0;
   let mut cows: u8 = 0;
   
   


   for i in 0..4 {
       if player_num.as_bytes()[i] == guess.as_bytes()[i]{
           bulls += 1;
       } else {
           if player_num.as_str().contains(&guess[i..i+1]) {

               cows += 1;
            }
        }
   }
   let response: String = format!("Bulls = {}, Cows = {}", bulls, cows);

   return response
}

fn input_name(num: &str) -> String {
    println!("Player{}, please enter your name \n>>>>>>>>>>", num);

    let mut name = String::with_capacity(30);
    io::stdin().read_line(&mut name)
        .expect("Input error");

    
    name.remove(name.len() - 1);
    return name
}

fn input_numbers(player_name: &str, mode: &str) -> String {
    loop {
        if mode == "create" {
            println!("{}, write your 4 digit number that have different digits. It should be hard to your opponent to guess it \n>>>>", player_name);
        } else {
            println!("{}, guess your opponent number \n>>>>", player_name)
        }
        let mut player_num = String::new();
        io::stdin().read_line(&mut player_num)
            .expect("Input error");

        let int_num: String = match player_num.trim().parse() {
            Ok(int_num) => int_num,
            Err(_) => {
                println!("Number must contain only 4 digits!");
                continue},
        };

        return player_num
        
    }    

}

