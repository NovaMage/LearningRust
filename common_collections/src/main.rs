fn main() {
    let mut v = Vec::new();
    v.push(8);
    v.push(3);
    v.push(4);
    v.push(17);
    v.iter().for_each(|value| {
        println!("{}", value)
    });
}
