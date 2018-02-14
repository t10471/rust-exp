use std::io;
use std::cmp::Ordering;
use rand::{self, Rng};

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub fn fizzbuzz(n: usize) {
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
pub fn guess_number() {
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
  left: usize,
  right: usize,
}

struct Table {
  forks: Vec<Mutex<()>>,
}

impl Philosopher {
  fn new(name: &str, left: usize, right: usize) -> Philosopher {
    Philosopher {
      name: name.to_string(),
      left: left,
      right: right,
    }
  }
  fn eat(&self, table: &Table) {
    let _left = table.forks[self.left].lock().unwrap();
    thread::sleep(Duration::from_millis(150));
    let _right = table.forks[self.right].lock().unwrap();

    println!("{} is eating.", self.name);

    thread::sleep(Duration::from_millis(1000));

    println!("{} is done eating.", self.name);
  }
}

pub fn philosopher_eat() {
  let table = Arc::new(Table {
    forks: vec![
      Mutex::new(()),
      Mutex::new(()),
      Mutex::new(()),
      Mutex::new(()),
      Mutex::new(()),
    ],
  });
  let philosophers = vec![
    Philosopher::new("Judith Butler", 0, 1),
    Philosopher::new("Gilles Deleuze", 1, 2),
    Philosopher::new("Karl Marx", 2, 3),
    Philosopher::new("Emma Goldman", 3, 4),
    Philosopher::new("Michel Foucault", 0, 4),
  ];
  let handles: Vec<_> = philosophers
    .into_iter()
    .map(|p| {
      let table = table.clone();
      thread::spawn(move || {
        p.eat(&table);
      })
    })
    .collect();

  for h in handles {
    h.join().unwrap();
  }
}
