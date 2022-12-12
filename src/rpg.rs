use std::fmt;

#[derive(PartialEq)]
pub enum Status {
    Normal,
    Problem(u16)
}

pub struct Character {
    pub name: String,
    pub max_health: i32,
    health: i32,
    status: Status,
    turn_num: u16
}

pub struct Attack {
    pub attackpower: i32,
    pub status: Status,
}

impl Character {
    pub fn new(name: String, max_health: i32) -> Character {
        Character { name, max_health, health: max_health, status: Status::Normal, turn_num: 0 }
    }
    fn turn(&mut self) {
        match self.status {
            Status::Normal => {}
            Status::Problem(start_turn) => {
                self.take_damage(1);
                if start_turn+4 == self.turn_num {
                    self.status = Status::Normal
                }
            },
        }
        self.turn_num += 1;
    }
    pub fn get_attacked(&mut self, attack: &Attack) {
        self.take_damage(attack.attackpower);
        match attack.status {
            Status::Normal => {},
            Status::Problem(_) => {
                self.status = Status::Problem(self.turn_num);
            }
        }
    }
    fn take_damage(&mut self, attack: i32) {
        self.health =- attack;
    }
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Character: {}   {}   {}", self.name, self.max_health, self.health)
    }
}
