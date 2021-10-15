use std::io::stdin;

#[derive(Debug)]
enum VisitorAction{
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
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
    your_name.trim().to_lowercase()
}

fn main() {

    let mut visitor_list = vec![
        Visitor::new("foo", "The beer is in the fridge"),
        Visitor::new("bar", "Remember to tip the waitstaff"),
        Visitor::new("biz", "who invited you?"),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty to exit)");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty(){
                    break;
                }else {
                    println!("{} is not on the visitor list", name);
                    visitor_list.push(Visitor::new(&name, "new friend"));
                }
            }

        }

    }
    println!("Final list of visitors");
    println!("{:#?}", visitor_list);
}
