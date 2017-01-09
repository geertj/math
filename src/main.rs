extern crate rand;

use std::env;
use std::io::{self, Write};
use std::ops::AddAssign;
use std::fmt;

use rand::Rng;

#[derive(Copy, Clone)]
struct Score {
    correct: i32,
    incorrect: i32,
}

impl Score {
    fn new() -> Score {
        Score { correct: 0, incorrect: 0 }
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, other: Score) {
        *self = Score {
            correct: self.correct + other.correct,
            incorrect: self.incorrect + other.incorrect
        }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.correct + self.incorrect == 0 {
            write!(f, "no score yet")
        } else {
            let pct = 100 * self.correct / (self.correct + self.incorrect);
            write!(f, "{}, score {}%", self.correct, pct)
        }
    }
}

fn prompt(message: &str) -> String {
    io::stdout().write(message.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

fn exercise<F, T>(count: u32, score: &mut Score, mut get_problem: F)
            where F: FnMut() -> (String, T),
                  T: std::cmp::Eq + std::str::FromStr {
    let mut s = Score::new();

    println!();
    for _ in 0..count {
        let (problem, answer) = get_problem();
        let response: T;
        loop {
            let input = prompt(problem.as_ref());
            response = match input.parse::<T>() {
                Ok(s) => s,
                Err(_) => { println!("Illegal value: {}", input); continue },
            };
            break
        }
        if response == answer {
            s.correct += 1
        } else {
            s.incorrect += 1
        }
    }
    println!();

    if s.incorrect == 0 {
        println!("Great job! No errors.");
    } else {
        println!("You had {} correct and {} incorrect answers.", s.correct, s.incorrect);
    }

    println!();
    let _ = prompt("Press ENTER to continue.");

    *score += s;
}

fn main() {

    let name = match env::var("NAME") {
        Ok(val) => val,
        Err(_) => String::from("there"),
    };

    println!("Hello, {}! Which excercise do you want to do?", name);

    let mut score = [Score::new(); 10];
    let mut count = 10u32;

    loop {
        println!("Enter a number, or 'q' to quit.");
        println!();
        println!("(0)  Set number of problems [current: {}]", count);
        println!("(1)  Ten's partners [{}]", score[0]);
        println!("(2)  Partners between five and ten [{}]", score[1]);
        println!("(3)  Addition, up to ten [{}]", score[2]);
        println!("(4)  Addition, between 10 and 20 [{}]", score[3]);
        println!("(5)  Addition, over 20 [{}]", score[4]);
        println!("(6)  Subtraction, up to ten [{}]", score[5]);
        println!("(7)  Subtraction, between 10 and 20 [{}]", score[6]);
        println!("(8)  Subtraction, over 20 [{}]", score[7]);
        println!("(9)  Addition and subraction, up to 100 [{}]", score[8]);

        println!();

        let choice = prompt("Enter your choice: ");
        let mut rng = rand::thread_rng();

        match choice.as_ref() {
            // Set count
            "0" => {
                loop {
                    let input = prompt("Number of problems per exercise: ");
                    match input.parse::<u32>() {
                        Ok(u) => { count = u; break }
                        Err(_) => { println!("Illegal value: {}", input); }
                    }
                }
            }

            // Tens partners
            "1" => exercise(count, &mut score[0],
                            || { 
                                let a = rng.gen_range(0, 11);
                                (format!("10 = {} + ", a), 10-a)
                            }),
            // Partners between 5 and ten
            "2" => exercise(count, &mut score[1],
                            || { 
                                let a = rng.gen_range(5, 11);
                                let b = rng.gen_range(0, a+1);
                                (format!("{} = {} + ", a, b), a-b)
                            }),
            // Sum up to 10
            "3" => exercise(count, &mut score[2],
                            || { 
                                let a = rng.gen_range(0, 11);
                                let b = rng.gen_range(0, 11-a);
                                (format!("{} + {} = ", a, b), a+b)
                            }),
            // Sum between 10 and 20
            "4" => exercise(count, &mut score[3],
                            || { 
                                let a = rng.gen_range(0, 11);
                                let b = rng.gen_range(10-a, 11);
                                (format!("{} + {} = ", a, b), a+b)
                            }),
            // Sum up to 100
            "5" => exercise(count, &mut score[4],
                            || { 
                                let a = rng.gen_range(0, 101);
                                let b = rng.gen_range(0, 101-a);
                                (format!("{} + {} = ", a, b), a+b)
                            }),
            // Subtraction, up to 10
            "6" => exercise(count, &mut score[5],
                            || { 
                                let a = rng.gen_range(0, 11);
                                let b = rng.gen_range(0, a+1);
                                (format!("{} - {} = ", a, b), a-b)
                            }),
            // Subtraction, between 10 and 20
            "7" => exercise(count, &mut score[6],
                            || { 
                                let a = rng.gen_range(10, 21);
                                let b = rng.gen_range(a-10, a+1);
                                (format!("{} - {} = ", a, b), a-b)
                            }),
            // Subtraction, up to 100
            "8" => exercise(count, &mut score[7],
                            || { 
                                let a = rng.gen_range(0, 101);
                                let b = rng.gen_range(0, a+1);
                                (format!("{} - {} = ", a, b), a-b)
                            }),
            "9" => exercise(count, &mut score[8],
                            || { 
                                match rng.gen_range(0, 2) {
                                    0 => {
                                        let a = rng.gen_range(0, 101);
                                        let b = rng.gen_range(0, 101-a);
                                        (format!("{} + {} = ", a, b), a+b)
                                    }
                                    1 => {
                                        let a = rng.gen_range(0, 101);
                                        let b = rng.gen_range(0, a+1);
                                        (format!("{} - {} = ", a, b), a-b)
                                    }
                                    _ => panic!("Not reached"),
                                }
                            }),
            "q" => break,
            _ => (),
        }
    }
}
