use std::io;
<<<<<<< HEAD

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    name.pop();
    println!("Hello {}!", name);
=======
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop{    

        println!("Please input your guess.");
    
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = guess.trim().parse()
            .expect("Please enter a number");

        println!("You guessed: {}", guess);
        
        println!("The correct answer is {}", secret_number);
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You were correct!");
                break;
            },
            Ordering::Greater => println!("Too Big!"),

        }
    }
>>>>>>> 4f6b902362828597cd1d36211a63c249a6bc341d
}
