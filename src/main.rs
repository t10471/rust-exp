extern crate rand;

use std::thread;
use std::time::Duration;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[allow(dead_code)]
fn fizzbuzz(n: usize) {
  for i in 0..n {
    if i % 15 == 0 {
      println!("FizzBuzz");
    } else if i % 3 == 0 {
      println!("Fizz");
    } else if i % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", i);
    }
  }
}

#[allow(dead_code)]
fn guess_number() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);
  // println!("The secret number is: {}", secret_number);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}

struct Philosopher {
  name: String,
}

impl Philosopher {
  fn new(name: &str) -> Philosopher {
    Philosopher {
      name: name.to_string(),
    }
  }
  fn eat(&self) {
    println!("{} is eating.", self.name);

    thread::sleep(Duration::from_millis(1000));

    println!("{} is done eating.", self.name);
  }
}

fn philosopher_eat() {
  let philosophers = vec![
    Philosopher::new("Judith Butler"),
    Philosopher::new("Gilles Deleuze"),
    Philosopher::new("Karl Marx"),
    Philosopher::new("Emma Goldman"),
    Philosopher::new("Michel Foucault"),
  ];
  let handles: Vec<_> = philosophers
    .into_iter()
    .map(|p| {
      thread::spawn(move || {
        p.eat();
      })
    })
    .collect();

  for h in handles {
    h.join().unwrap();
  }
}

fn main() {
  // fizzbuzz(20);
  // guess_number();
  philosopher_eat();
}
