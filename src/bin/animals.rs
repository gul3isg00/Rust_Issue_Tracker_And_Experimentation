use rand::Rng;
use std::fmt;

#[derive(PartialEq)]
enum Interaction {
    None,
    Eats,
    GetsEaten,
}

#[derive(PartialEq, Clone)]
enum Group {
    Undefined,
    Mammal,
    Reptile,
    Marsupial,
    Bird,
    Fish,
    Invertebrate,
}

// impl Group {
//     fn random() -> Group {
//         const VARIANTS: &[Group] = &[
//             Group::Undefined,
//             Group::Mammal,
//             Group::Reptile,
//             Group::Marsupial,
//             Group::Fish,
//             Group::Bird,
//             Group::Invertebrate,
//         ];
//         return Group::from(VARIANTS[rand::thread_rng().gen_range(0..6)]);
//     }
// }

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group::Undefined => write!(f, "Undefined"),
            Group::Mammal => write!(f, "Mammal"),
            Group::Reptile => write!(f, "Reptile"),
            Group::Marsupial => write!(f, "Marsupial"),
            Group::Bird => write!(f, "Bird"),
            Group::Fish => write!(f, "Fish"),
            Group::Invertebrate => write!(f, "Invertebrate"),
        }
    }
}

impl Group {
    fn interaction(a: Group, b: Group) -> Interaction {
        match a {
            Group::Mammal => match b {
                Group::Bird | Group::Fish | Group::Mammal => Interaction::Eats,
                Group::Reptile => Interaction::GetsEaten,
                _ => Interaction::None,
            },
            Group::Reptile => match b {
                Group::Bird | Group::Fish | Group::Invertebrate => Interaction::Eats,
                Group::Mammal | Group::Marsupial => Interaction::GetsEaten,
                _ => Interaction::None,
            },
            Group::Marsupial => match b {
                Group::Bird | Group::Fish | Group::Invertebrate => Interaction::Eats,
                Group::Reptile => Interaction::GetsEaten,
                _ => Interaction::None,
            },
            Group::Bird => match b {
                Group::Fish | Group::Invertebrate => Interaction::Eats,
                Group::Mammal | Group::Reptile => Interaction::GetsEaten,
                _ => Interaction::None,
            },
            Group::Fish => match b {
                Group::Invertebrate => Interaction::Eats,
                Group::Bird | Group::Mammal | Group::Reptile => Interaction::GetsEaten,
                _ => Interaction::None,
            },
            Group::Invertebrate => match b {
                Group::Invertebrate => Interaction::Eats,
                Group::Bird | Group::Fish | Group::Mammal | Group::Reptile | Group::Marsupial => {
                    Interaction::GetsEaten
                }
                _ => Interaction::None,
            },
            _ => Interaction::None,
        }
    }
}

#[derive(Clone)]
struct AnimalBase {
    name: String,
    age: i32,
    lifespan: i32,
    group: Group,
    alive: bool,
    stomach_contents: Vec<Animal>,
}

#[derive(Clone)]
struct Animal(AnimalBase);

impl Animal {
    fn new() -> Animal {
        println!("A new animal has been brought into this world!");
        return Animal(AnimalBase {
            name: String::from("Unnamed"),
            age: 0,
            lifespan: rand::thread_rng().gen_range(10..100),
            group: Group::Undefined,
            alive: true,
            stomach_contents: Vec::new(),
        });
    }

    fn get_group(&self) -> Group {
        return self.0.group.clone();
    }

    fn get_name(&self) -> String {
        return self.0.name.clone();
    }

    fn kill(&mut self) {
        self.0.alive = false;
        println!("{} has died!", self.0.name);
    }

    fn age_up(&mut self, years: i32) {
        self.0.age += years;
        println!(
            "{} has age up, they're now {} years old!",
            self.0.name, self.0.age
        );
        if (self.0.age > self.0.lifespan) {
            self.kill();
        }
    }

    fn speciate(&mut self, group: Group) {
        if (self.0.group == Group::Undefined && self.0.alive) {
            println!("{} has become a {}!", self.0.name, group);
            self.0.group = group;
        }
    }

    fn consume(&mut self, mut animal: Animal) {
        animal.kill();
        println!("{} has eaten {}!", self.0.name, animal.0.name);
        self.0.stomach_contents.push(animal);
    }

    fn interact(&mut self, mut animal: Animal) {
        let outcome = Group::interaction(self.get_group(), animal.get_group());
        println!("{} is interacting with {}!", self.0.name, animal.0.name);
        if outcome == Interaction::Eats {
            self.consume(animal);
        }
    }

    fn pump_stomach(&mut self) -> Vec<Animal> {
        println!("{} has had it's stomach pumped...", self.0.name);
        let contents = Vec::from(self.0.stomach_contents.as_slice());
        self.0.stomach_contents = Vec::new();
        return contents;
    }

    fn give_name(&mut self, name: String) {
        println!("{} has changed it's name to {}!", self.0.name, name);
        self.0.name = name;
    }
}

fn main() {
    let mut a = Animal::new();
    a.give_name(String::from("Big Bird"));
    a.speciate(Group::Bird);

    let mut b = Animal::new();
    b.give_name(String::from("Little Bug"));
    b.speciate(Group::Invertebrate);

    a.interact(b);

    let mut eaten = a.pump_stomach();
    for animal in eaten {
        println!("{} was in {}'s stomach", animal.get_name(), a.get_name());
    }
}
