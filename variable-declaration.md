
# `_` expressions

> **<sup>Syntax</sup>**\
> _UnderscoreExpression_ :\
> &nbsp;&nbsp; `_`
Underscore expressions, denoted with the symbol `_`, are used to signify a
placeholder in a destructuring assignment. They may only appear in the left-hand
side of an assignment.

An example of an `_` expression:

```rust
let p = (1, 2);
let mut a = 0;
(_, a) = p;
```

```rust
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    loop{
        println!("Please input your guess.");
        let mut guess:String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let _msg = "Please type a number";
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>continue,
        };
    
        println!("You guessed: {}",guess);
        match guess.cmp(&secret_number){
            Ordering::Less=> {println!("Too small");continue;},
            Ordering::Greater=>{println!("Too big");continue;},
            Ordering::Equal =>{
                println!("You win");
                break;
            } 
        }
    }

```

