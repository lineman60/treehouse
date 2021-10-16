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
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

// impl Visitor {
//     fn new(name: &str, greeting: &str) -> Self {
//         Self {
//             name: name.to_lowercase(),
//             greeting: greeting.to_string(),
//         }
//     }
//     fn greet_visitor(&self) {
//         println!("{}", self.greeting)
//     }
// }

fn greet_visitor(&self){
    match &self.action {
        VisitorAction::Accept => println!("Welcome to the treehouse! {} ", self.name),
        VisitorAction::AcceptWithNote { note} => {
            println!("Welcome to the treehouse! {} ", self.name);
            println!("{}", note);
            if self.age < 21{
                println!("No beer for {}", self.name);
            }
        }
        VisitorAction::Probation => println!("{} is a probation member", self.name),
        VisitorAction::Refuse => println!("tell {} to get lost!", self.name),

    }
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
        Visitor::new("foo", VisitorAction::Accept, 45),
        Visitor::new("bar", VisitorAction::AcceptWithNote {note: String::from("Milks in the fridge")}, 15),
        Visitor::new("biz", VisitorAction::Refuse, 30),
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
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }

        }

    }
    println!("Final list of visitors");
    println!("{:#?}", visitor_list);
}
