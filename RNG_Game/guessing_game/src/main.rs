use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    
    let cpu_ans = rand::thread_rng().gen_range(1..=100);


loop {

let mut guess = String::new();

io::stdin().read_line(&mut guess).expect("invalid input");



let guess : u32 = guess.trim().parse().expect("invalid input");

match guess.cmp(&cpu_ans){


    Ordering::Less => println!("too low"),

     Ordering::Greater => println!("too High"),
      Ordering::Equal =>{ println!("YOU win");
        break;
      }
    

    

}

}



}
