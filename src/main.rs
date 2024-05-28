#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}
impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_string(),
            action,
            age,
        }
    }
    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house {}", self.name);
                println!("{note}");
                if self.age < 16 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
            VisitorAction::Probation => println!("{} is a probation member", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name: String = String::new();
    stdin().read_line(&mut your_name).unwrap();
    your_name.trim().to_string()
}
fn main() {
    let mut guest_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 20),
        Visitor::new("chris", VisitorAction::Refuse, 8),
        Visitor::new("mario", VisitorAction::Probation, 47),
        Visitor::new(
            "mike",
            VisitorAction::AcceptWithNote {
                note: "Milk is in the fridge.".to_string(),
            },
            9,
        ),
    ];
    loop {
        println!("Hi, what's your name?");
        let name = what_is_your_name();
        let visitor = guest_list.iter().find(|visitor| visitor.name == name);
        if let Some(it) = visitor {
            it.greet();
        } else {
            if name.is_empty() {
                break;
            }
            println!("Hey, {name} you are new.");
            guest_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
    }
    println!("Our visitors:");
    println!("{guest_list:#?}");
}
