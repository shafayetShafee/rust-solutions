// Exercise: 
// Write a small program which constructs a HashMap with pirate ship names as 
// keys and their crew sizes as values. For example, "Black Pearl" could have 3 
// crew members. Iterate over the elements of the hashmap and print each ship with 
// its crew size.Exercise: Write a small program which constructs a HashMap with 
// pirate ship names as keys and their crew sizes as values. For example, "Black Pearl" 
// could have 3 crew members. Iterate over the elements of the hashmap and print each s
// hip with its crew size.

use std::collections::HashMap;

fn main() {
    let ships = vec!["Thousand Sunny", "Oro Jackson", "Moby Dick"];
    let crew_size = vec![10, 16, 12];
    let pirates_ship: HashMap<_, _> = ships.iter().zip(crew_size.iter()).collect();
    
    for (name, size) in &pirates_ship {
        println!("Ship Name: {name} & Crew Size: {size}")
    }
}