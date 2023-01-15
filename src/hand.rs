use crate::dice::Die;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Hand {
   pub dice: Vec<Die>,
}

impl Hand {
    pub fn new() -> Hand {
        let tmp = Die::roll();
        
        Hand {
            dice: vec![
                Die::roll(),
                Die::roll(),
                Die::roll(),
                Die::roll(),
                Die::roll(),
            ]
        }
    }
    // pub fn display(&self) {

    //     // iteration 0
    //     println!("--------- --------- --------- --------- ---------");

    //     // iteration 1
    //     for die in self.dice.iter() {
    //         match die.value {
    //             1 => print!("|       | "),
    //             2 => print!("| O     | "),
    //             3 => print!("| O     | "),
    //             4 => print!("| O   O | "),
    //             5 => print!("| O   O | "),
    //             6 => print!("| O   O | "),
    //             _ => print!("|*******| "),
    //         }
    //     }
    //     println!("");

    //     // iteration 2
    //     for die in self.dice.iter() {
    //         match die.value {
    //             1 => print!("|   O   | "),
    //             2 => print!("|       | "),
    //             3 => print!("|   O   | "),
    //             4 => print!("|       | "),
    //             5 => print!("|   O   | "),
    //             6 => print!("| O   O | "),
    //             _ => print!("|*******| "),
    //         }
    //     }
    //     println!("");

    //     // iteration 3
    //     for die in self.dice.iter() {
    //         match die.value {
    //             1 => print!("|       | "),
    //             2 => print!("|     O | "),
    //             3 => print!("|     O | "),
    //             4 => print!("| O   O | "),
    //             5 => print!("| O   O | "),
    //             6 => print!("| O   O | "),
    //             _ => print!("|*******| "),
    //         }
    //     }
    //     println!("");

    //     // iteration 4
    //     println!("--------- --------- --------- --------- ---------");
    //     println!("   [0]        [1]       [2]       [3]       [4]  ")
    // }
}