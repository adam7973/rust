fn main() {
    let car1 = Vehicle::new(4, 160);
    let car2 = Vehicle::new(8, 100);
    println!("wheels: {}, maximum speed: {}", car1.wheels, car1.max_speed);
    println!("wheels: {}, maximum speed: {}", car2.wheels, car2.max_speed);
    car1.drive();
}

struct Vehicle {
    wheels: i32,
    max_speed: i32,
}

impl Vehicle {
    fn new(wheels: i32, max_speed: i32) -> Vehicle {
        Vehicle {wheels, max_speed}
    }
    fn drive(self) {
        println!("WROOOOOM!")
    }
}
