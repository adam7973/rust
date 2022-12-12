#![allow(dead_code)]
use std::fs;
use std::io::Write;
use std::path::Path;

use animal::{Dog, Animal};

mod morris;
mod pyramid;
mod rpg;
mod animal;

fn main() {
    s0075_animal();
}

fn s0027_files() {
    let mydata = "Hi,\nthis is a text file.\nIt has 3 rows.\n";
    let myfile = Path::new("input01.txt");
    let mut file = match fs::File::create(&myfile) {
        Err(_) => panic!("couldn't create file"),
        Ok(file) => file,
    };

    match file.write_all(mydata.as_bytes()) {
        Err(_) => panic!("err"),
        Ok(_) => println!("wrote to file"),
    }
}

fn s0030_morris() {
    let mut me = morris::Morris::new();
    for _i in 1..1001 {
        if _i > 997 {
            me.mine();
        }
        if me.sleepiness > 94 {
            me.sleep();
        } else if me.thirst > 94 {
            me.drink();
        } else if me.hunger > 94 {
            me.eat();
        } else if me.whisky == 0 && me.gold > 0 {
            me.buy_whisky();
        } else {
            me.mine();
        }
    }
    println!("{}", me)
}

fn s0075_animal() {
    let bird = Animal::new(String::from("Bird"), String::from("chirp"), 12.2, 1.2, 2, true);
    println!("{}", bird);
    bird.make_noise();
    let waffie = Dog::new(String::from("Waffie"), 40.0, 7.2, 5, true, 5.8, false);
    let woof = Dog::new(String::from("Woof"), 45.0, 7.3, 6, false, 7.6, true);
    println!("{}", waffie);
    println!("{}", woof);
    waffie.wag_tail();
    let woofie = waffie.mate(&woof).expect("woops");
    println!("{}", woofie)

}

fn pyramid() {
    let start = "1 1";
    let lines = 10;
    pyramid::pyramid2(lines, start)
}
