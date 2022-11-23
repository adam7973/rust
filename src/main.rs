mod morris;

fn main() {
    let mut me = morris::Morris::new();
    for _i in 1..1001 {
        if me.sleepiness > 50 {
            me.sleep()
        } else if me.thirst > 50 {
            me.buy_whisky();
            me.drink();
        } else if me.hunger > 50 {
            me.eat();
        } else {
            me.mine();
        }
    }
    println!("{}", me)
}

