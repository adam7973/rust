use std::fs;
use std::io::Write;
use std::path::Path;

mod morris;
mod pyramid;

fn main() {}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn pyramid() {
    let start = "1 1";
    let lines = 10;
    pyramid::pyramid2(lines, start)
}
