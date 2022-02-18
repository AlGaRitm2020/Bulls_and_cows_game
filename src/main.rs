use std::io;


fn main() {
     
    println!("Hello, it's Bulls and Cows game");
    
    let player_name = String::from("Player 1");
    let player_num: u16 = input(&player_name[..]); 
    println!("You have let {}", player_num);


}

fn input(player_name: &str) -> u16 {
    loop {

        println!("{}, Enter 4 digit number that have different digits", player_name);

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

