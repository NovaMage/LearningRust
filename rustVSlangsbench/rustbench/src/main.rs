use std::io;

struct Model {
    value: usize,
}

impl Model {
    fn duplicate(&mut self) {
        self.value = self.value * 2;
    }
}

trait SomeTrait {
    fn some_fun(&self) -> i64;
}

fn do_benchmark(quantity: usize) {
    let mut start_time = std::time::SystemTime::now();
    let mut buffer = Vec::with_capacity(quantity);
    for i in 0..quantity {
        buffer.push(Model {
            value: i
        });
    }
    println!("Time to fill buffer: {:?}", start_time.elapsed().unwrap());
    start_time = std::time::SystemTime::now();
    for i in 0..quantity {
        buffer[i].duplicate();
    }
    println!("Time to duplicate contents of all: {:?}", start_time.elapsed().unwrap());
}

fn something() -> Box<dyn SomeTrait> {
    todo!()
}


fn main() {
    println!("Please input quantity of objects to create:");
    let mut quantity: String = String::new();
    io::stdin().read_line(&mut quantity).unwrap();

    let quantity: usize = quantity.trim().parse().unwrap();

    println!("Cold Start:");
    do_benchmark(quantity);

    println!("\nWarmed up, press ENTER key to continue");
    io::stdin().read_line(&mut String::new()).unwrap();
    do_benchmark(quantity)
}
