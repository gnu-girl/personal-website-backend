use crate::hand::*;
use std::io;
use std::collections::HashMap;
use crate::dice::Die;
use std::{thread, time};

pub enum ScoreVariant{
    ONES,
    TWOS,
    THREES,
    FOURS,
    FIVES,
    SIXES,
    THREE_OF_A_KIND,
    FOUR_OF_A_KIND,
    YAHTZEE,
    SM_STRAIT,
    LG_STRAIT,
    CHANCE
}

pub struct Player {
    hand: Hand,
    rolls_left: u8,
    score: u8,
    card: Vec<ScoreVariant>,
}

pub fn start() {

}

impl Player {
    pub fn new() -> Player {        
        Player {
            hand: Hand::new(),
            rolls_left: 3,
            score: 0,
            card: Vec::new(),
        }
    }
    // pub fn do_turn(& mut self) {
        
    //     // While Player has rolls left
    //     while self.rolls_left > 0{
        
    //         // display hand
    //         self.hand.display();
    //         let mut ipt = String::new();

    //         println!("\nEnter in numbers that you want to REROLL (0-4):");
            
    //         io::stdin().read_line(&mut ipt)
    //         .expect("Couldn't read line");

    //         for char in ipt.chars() {
    //             match char {
    //                 '0' => self.hand.dice[0] = Die::roll(),
    //                 '1' => self.hand.dice[1] = Die::roll(),
    //                 '2' => self.hand.dice[2] = Die::roll(),
    //                 '3' => self.hand.dice[3] = Die::roll(),
    //                 '4' => self.hand.dice[4] = Die::roll(),
    //                 '\n' => (),
    //                 _ => {
    //                     println!("\nUnrecognized option: {}",ipt);
    //                     thread::sleep(time::Duration::from_secs(2));
    //                     self.do_turn();
    //                 },
    //             }
    //         }
    //         self.rolls_left -= 1;
    //     }
    // }
    // pub fn do_score(&mut self) {
    //     self.hand.display();
    //     println!("How do you want to score this?");


    // }
    
}