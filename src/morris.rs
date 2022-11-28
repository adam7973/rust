use std::fmt;

#[allow(dead_code)]

pub struct Morris {
    pub sleepiness: i32,
    pub thirst: i32,
    pub hunger: i32,
    pub whisky: i32,
    pub gold: i32,
}

impl Morris {
    pub fn new() -> Morris {
        Morris {
            sleepiness: 0,
            thirst: 0,
            hunger: 0,
            whisky: 0,
            gold: 0,
        }
    }
    fn die(&mut self) {
        self.thirst = 0;
        self.sleepiness = 0;
        self.hunger = 0;
        self.whisky = 0;
        self.gold = 0;
    }
    fn check_health(&mut self, extra: &str) {
        if self.sleepiness > 100 {
            println!("{}", extra);
            println!("he had died of sleep deprivation");
            self.die();
        } else if self.thirst > 100 {
            println!("{}", extra);
            println!("he had diead of thirst");
            self.die()
        } else if self.hunger > 100 {
            println!("{}", extra);
            println!("he had died of hunger");
            self.die()
        }
    }
    fn ensure_zero(&mut self) {
        if self.thirst < 0 {
            self.thirst = 0
        }
        if self.hunger < 0 {
            self.hunger = 0
        }
        if self.sleepiness < 0 {
            self.sleepiness = 0
        }
    }
    pub fn sleep(&mut self) {
        self.sleepiness -= 10;
        self.thirst += 1;
        self.hunger += 1;
        self.check_health("morris would never wake again");
    }
    pub fn mine(&mut self) {
        self.sleepiness += 5;
        self.thirst += 5;
        self.hunger += 5;
        self.gold += 5;
        self.check_health("while mining morris suddenly fell over");
    }
    pub fn eat(&mut self) {
        if self.gold > 1 {
            self.sleepiness += 5;
            self.thirst -= 5;
            self.hunger -= 20;
            self.gold -= 2;
            self.ensure_zero();
            self.check_health("while morris was eating he suddenly fell over");
        } else {
            println!("too poor");
        }
    }
    pub fn buy_whisky(&mut self) {
        if self.whisky == 10 {
            println!("morris already has 10 whisky, he cant carry more");
        } else if self.gold < 1 {
            println!("too poor");
        } else {
            self.sleepiness += 5;
            self.thirst += 1;
            self.hunger += 1;
            self.whisky += 1;
            self.gold -= 1;
            self.check_health("while going to the store to buy whisky morris fell over");
        }
    }
    pub fn drink(&mut self) {
        if self.whisky > 0 {
            self.sleepiness += 5;
            self.thirst -= 15;
            self.hunger -= 1;
            self.whisky -= 1;
            self.ensure_zero();
            self.check_health("While taking a pause to drink morris felt bad");
        } else {
            println!("must have wisky");
        }
    }
}

impl fmt::Display for Morris {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Morris stats:\nHunger: {}\nThirst: {}\nSleepiness: {}\nGold: {}\nWhisky stored: {}",
            self.hunger, self.thirst, self.sleepiness, self.gold, self.whisky
        )
    }
}
