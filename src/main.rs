// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {

    let x = 5;
    let x = x + 3;
    let spaces = "       ";
    let spaces = spaces.len();
    let guess = "4%^jhiutgu5";
    {

        let x = x*2;
        println!("The value of x in the inner scope is: {}",x);
    }
    println!("The value of x is:{} {} {}",x,spaces,guess);

    // println!("Guess the number!");
    // let secret_number = rand::thread_rng().gen_range(1..100);
    // loop{
    //     println!("Please input your guess.");
    //     let mut guess:String = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     let _msg = "Please type a number";
    //     let guess:u32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) =>continue,
    //     };
    
    //     println!("You guessed: {}",guess);
    //     match guess.cmp(&secret_number){
    //         Ordering::Less=> {println!("Too small");continue;},
    //         Ordering::Greater=>{println!("Too big");continue;},
    //         Ordering::Equal =>{
    //             println!("You win");
    //             break;
    //         } 
    //     }
    // }
    
}



// fn last_char(string:String)->char {
//     if string.is_empty(){
//         return '\0';
//     }
//     string.chars().next_back().unwrap()
// }


