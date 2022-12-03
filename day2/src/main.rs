use std::{fs::read_to_string};
use std::collections::HashMap;
pub enum Hand{
    Rock, Paper, Scissors
} 
pub enum Outcome {
    Win, Draw, Lose
}

trait RPCHand {
    fn get_hand(&self, outcome: &Outcome) -> Hand;
}

impl RPCHand for Hand {
    fn get_hand(&self, outcome: &Outcome) -> Hand {
        match self {
            Hand::Rock      => match outcome {
                Outcome::Draw => Hand::Rock ,
                Outcome::Lose => Hand::Scissors,
                Outcome::Win => Hand:: Paper
            }
            Hand::Paper     => match outcome {
                Outcome::Draw => Hand::Paper,
                Outcome::Lose => Hand::Rock,
                Outcome::Win => Hand::Scissors
            }
            Hand::Scissors  => match outcome {
                Outcome::Draw => Hand::Scissors, 
                Outcome::Lose => Hand::Paper,
                Outcome::Win => Hand::Rock
            }
        }
    }
}

fn main() {

    let hands = HashMap::from([
        ("A", Hand::Rock),
        ("B", Hand::Paper),
        ("C", Hand::Scissors),
    ]);

    let outcomes = HashMap::from([
        ("X", Outcome::Lose),
        ("Y", Outcome::Draw),
        ("Z", Outcome::Win),
    ]);

    let filename:&str = "src/input.txt";
    let data = read_to_string(filename)
                    .expect("Couldnt open file!");
    let lines = data.lines();
    let mut my_pnts: u32 = 0;

    for line in lines {
        let mut iter = line.split_whitespace();
        let opp     = hands.get(&iter.next().unwrap()).unwrap();
        let outcome = outcomes.get(&iter.next().unwrap()).unwrap();
        let my_hand = opp.get_hand(outcome);

        match outcome {
            Outcome::Win => my_pnts += 6,
            Outcome::Lose => my_pnts += 0,
            Outcome::Draw => my_pnts += 3
        }
        match my_hand {
            Hand::Rock => my_pnts += 1,
            Hand::Paper => my_pnts += 2,
            Hand::Scissors => my_pnts += 3
        }
    }
    println!("{}", my_pnts)
}
