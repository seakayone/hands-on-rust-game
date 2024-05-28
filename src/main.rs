#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}
impl Visitor {
    fn new(name: String, greeting: String) -> Self {
        Self { name, greeting }
    }
}

fn what_is_your_name() -> String {
    let mut your_name: String = String::new();
    stdin().read_line(&mut your_name).unwrap();
    your_name.trim().to_string()
}
fn main() {
    let mut guest_list = vec![
        (Visitor::new("bert".to_string(), "Howdy".to_string())),
        (Visitor::new("chris".to_string(), "Moin".to_string())),
        (Visitor::new("mario".to_string(), "It's me Mario!".to_string())),
    ];
    loop {
        println!("Hi, what's your name?");
        let name = what_is_your_name();
        let visitor = guest_list.iter().find(|visitor| visitor.name == name);
        if let Some(it) = visitor {
            println!("{}: {}", it.name, it.greeting);
        } else {
            if name.is_empty() {
                break;
            }
            println!("Hey, {name} you are new.");
            guest_list.push(Visitor::new(name, "New friend".to_string()));
        }
    }
    println!("Our visitors:");
    println!("{guest_list:#?}");
}
