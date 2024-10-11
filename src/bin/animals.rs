use rand::Rng;

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

struct AnimalBase {
    name: String,
    age: i32,
    lifespan: i32,
    group: Group,
    alive: bool,
    stomach_contents: Vec<Animal>,
}
struct Animal(AnimalBase);

impl Animal {
    fn new() -> Animal {
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

    fn kill(&mut self) {
        self.0.alive = false;
    }

    fn age_up(&mut self, years: i32) {
        self.0.age += years;
        if (self.0.age > self.0.lifespan) {
            self.kill();
        }
    }

    fn speciate(&mut self, group: Group) {
        if (self.0.group == Group::Undefined && self.0.alive) {
            self.0.group = group;
        }
    }

    fn consume(&mut self, mut animal: Animal){
        animal.kill();
        self.0.stomach_contents.push(animal);
    }

    fn interact(&mut self, mut animal: Animal) {
        let outcome = Group::interaction(self.get_group(), animal.get_group());
        if outcome == Interaction::Eats {
            self.consume(animal);
        }
    }

    fn pump_stomach(&mut self) -> Vec<Animal>{
        // implement copy
        let contents = Vec::from(self.0.stomach_contents);
        self.0.stomach_contents = Vec::new();
        return contents;
    }
}

fn main() {
    println!("Hello, world!");
}
