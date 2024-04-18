pub trait Summary {
    fn summarize(&self) -> String;
}

fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        str::to_string("First")
    } else {
        str::to_string("Second")
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
         result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}