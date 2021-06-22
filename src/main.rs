extern crate regex;
extern crate rand;

use std::env;
use rand::Rng;
use rand::thread_rng;
use regex::Regex;

fn roll_dice(format_string: String) {
    // initializers
    let mut rng = thread_rng();
    let mut sum: u32 = 0;

    // Use regex to determine if it is a valid dice formatted string
    let re = Regex::new(r"\d?d\d+(h?l?\d+)?(-\d+)?(\+\d+)?").unwrap();
    if !re.is_match(&format_string) {
        panic!("Usage: must contain standard dice format (ex. 3d4 rolls a 4 sided die 3 times)")
    }

    /* Split valid dice format into a list of the 3 components: 
        -> number of die
        -> number of sides on each die
        -> modifier 
    */
    let val: Vec<&str> = format_string.split(|c| c == 'd' || c == '+').collect::<Vec<&str>>();

    // assign each val to a u32 variable and parse it
    let mut modifier: u32 = 0;
    let num_of_die =    val[0].parse::<u32>().unwrap();
    let sides =         val[1].parse::<u32>().unwrap();

    //if there is a modifier present
    if val.len() == 3 {
        modifier =      val[2].parse::<u32>().unwrap();
    }

    // roll die
    for _i in 0..num_of_die {
        let randu32 = rng.gen_range(0, sides) + modifier + 1; 
        sum = randu32 + sum; 
        println!("{},", randu32);
    }

    println!("Sum of rolls: {:?}", sum);
}

fn main() {
    let dice = env::args().nth(1).expect(&format!("Usage: {} <dice format> (ex. 3d4 rolls 3 4 sided die)", env::args().nth(0).unwrap()));

    roll_dice(dice);
}


