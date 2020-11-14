use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting)
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hello Steve, enjoy your treehouse."),
        Visitor::new("fred", "Hello Fred, enjoy your treehouse."),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let known_visitor  = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Sorry, you aren't in the list."),
    }
}
