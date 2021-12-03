use advent2019;
use std::collections::HashMap;

fn count_orbit(planet: &String, orbits: &HashMap<String, String>) -> u32 {
    let mut count = 0;
    let mut current_planet = planet;
    loop {
        match orbits.get(current_planet) {
            Some(new_planet) => {
                count += 1;
                current_planet = new_planet;
            }
            None => { break }
        }
    }
    count
}

fn set_root(planet: &String, orbits: &mut HashMap<String, String>) {
    let mut current = planet.clone();
    let mut maybe_parent: Option<String> = orbits.remove(&current);
    loop {
        match maybe_parent {
            None => { return }
            Some(parent) => {
                let tmp = parent.clone();
                maybe_parent = orbits.insert(parent, current);
                current = tmp;
            }
        }
    }
}

fn main() {
    let mut orbits: HashMap<_, _> = advent2019::load_input("06.txt")
        .into_iter()
        .map(|x| {
            let idx = x.find(')').unwrap();
            (String::from(&x[idx + 1..]), String::from(&x[..idx]))
        })
        .collect();

    let mut total_orbits: u32 = 0;
    for planet in orbits.keys() {
        total_orbits += count_orbit(planet, &orbits);
    }
    println!("Total orbits: {}", total_orbits);

    set_root(&"SAN".to_string(), &mut orbits);
    let dist_from_you = count_orbit(&"YOU".to_string(), &orbits);
    println!("Orbit changes to Santa: {}", dist_from_you - 2);
}
