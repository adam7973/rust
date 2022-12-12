// Define a class named Animal.
// Each object of this class shall have the attributes name, sound, height, weight, legs, female.
// Name and sound are strings. Height and weight are floating point numbers. Legs is a integer. Female is boolean.

// Add to the class meaningful methods __init__ and __repr__.
// Call these methods to create objects of the class Animal and to print them out in the main program.

// Write a class method named make_noise, which prints out the animal's sound in the console.
// Call this method in the main program.

// Define another class Dog, which inherits from Animal.
// Each object of this class shall have the attributes tail_length and hunts_sheep.
// Tail_length is a floating point number. Hunts_sheep is boolean.

// Add to the class meaningful methods __init__ and __repr__.
// In writing the constructor of Dog, try to reuse code from the class Animal.
// Call these methods to create objects of the class Dog and to print them out in the main program.

// Call the method make_noise on Dog objects in the main program.

// Write a class method named wag_tail for Dog.
// This method prints out into the console something like
// "The dog Snoopy wags its 32cm long tail"
// Call this method in the main program.

// Write a function mate(mother, father). Both parameters are of type Dog.
// This function shall return a new object of type Dog.
// In this function, make meaningful rules for the new dogs attributes.
// Make sure that this function only accepts dogs with the correct sex as arguments.

// In the main program, call this method and print out the new dog.

use std::fmt;
extern crate rand;

pub struct Animal {
    name: String,
    sound: String,
    height: f32,
    weight: f32,
    legs: i32,
    female: bool,
}

impl Animal {
    pub fn new(name: String, sound: String, height: f32, weight: f32, legs: i32, female: bool) -> Animal {
        Animal {name, sound, height, weight, legs, female}
    }
    pub fn make_noise(&self) {
        println!("{}", self.sound)
    }
}

pub struct Dog {
    animal: Animal,
    tail_length: f32,
    hunts_sheep: bool,
}

impl Dog {
    pub fn new(name: String, height: f32, weight: f32, legs: i32, female: bool, tail_length: f32, hunts_sheep: bool) -> Dog {
        Dog { animal: Animal {name, sound: String::from("Bark"), height, weight, legs, female}, tail_length, hunts_sheep }
    }
    pub fn wag_tail(&self) {
        println!("{name} wags its {length} cm tail", name=self.animal.name, length=self.tail_length)
    }
    pub fn mate(&self, other: &Dog) -> Option<Dog> {
        if self.animal.female != other.animal.female {
            Some(Dog::new(String::from("valp"), 10.0, 1.0, 10, rand::random(), 10.0, self.hunts_sheep | other.hunts_sheep))
        } else {
            None
        }
    }
}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut female = "";
        if self.female { female = ", it is female"; }
        write!(f, "A {name} that is {height} tall and weighs {weight}{female}. It sounds like {sound}", name=self.name, height=self.height, weight=self.weight, female=female, sound=self.sound)
    }
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut female = "";
        let mut hunt = "";
        if self.animal.female { female = ", it is female"; }
        if self.hunts_sheep { hunt = " It hunts sheep."; }
        write!(f, "A dog, named {name}, that is {height} tall and weighs {weight}, it has a tail length of {tail_length}{female}. It sounds like {sound}.{hunt}", name=self.animal.name, height=self.animal.height, weight=self.animal.weight, tail_length=self.tail_length, female=female, sound=self.animal.sound, hunt=hunt)
    }
}
