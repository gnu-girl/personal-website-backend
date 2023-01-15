extern crate rand;
use std::fmt;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

#[derive(Debug,Serialize, Deserialize)]
// #[derive(Debug)]
pub struct Die {
   pub value: i8,
}

impl Die {
    pub fn roll() -> Die {
        Die {
            value: rand::thread_rng().gen_range(1..=6)
        }
    }
} 

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let final_output = String::new();
        write!(f, "{}", self.value)
    }
}