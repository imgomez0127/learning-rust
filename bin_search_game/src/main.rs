use std::io;

fn main() {

    println!("Think of a number between 0-100");

    let mut low = 0;

    let mut high = 100;
    
    let mut current_guess;

    loop{ 
        current_guess = (high-low)/2 + low;

        let mut answer = String::new();

        println!("The computer guessed {}",current_guess);
        
        println!("Please enter if your number is, Equal, Less Than, or Greater Than your input");

        io::stdin().read_line(&mut answer).expect("Please enter a correct input");
        
        match answer.trim()
            .to_lowercase().as_ref(){
            "greater than" => low = current_guess,
            "less than" => high = current_guess,
            "equal" => {
                println!("The Computer has won");
                break;
            },
            _ => println!("Your input is incorrect")
        }
    }
}
