use std::io;


fn main() {
     
    println!("Hello, it's Bulls and Cows game");
    
    let player_1_name: String = input_name("1"); 
    let player_2_name: String = input_name("2"); 

    println!("Hello {} and {}. Good luck!", &player_1_name.as_str(), &player_2_name.as_str());

    let player_1_num: u16 = input_numbers(&player_1_name.as_str()); 

    let player_2_num: u16 = input_numbers(&player_2_name.as_str()); 



}

fn input_name(num: &str) -> String {
    println!("Player{}, please enter your name", num);

    let mut name = String::with_capacity(30);
    io::stdin().read_line(&mut name)
        .expect("Input error");

    
    name.remove(name.len() - 1);
    return name
}

fn input_numbers(player_name: &str) -> u16 {
    loop {

        println!("{}, enter 4 digit number that have different digits", player_name);

        let mut player_num = String::new();
        io::stdin().read_line(&mut player_num)
            .expect("Input error");

        let player_num: u16 = match player_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error");
                continue},
        };

        return player_num
    }

}

