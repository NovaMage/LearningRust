use std::error::Error;
use std::io;
use std::process::ExitCode;
use std::time::Instant;

fn use_iter(vec: &mut Vec<usize>) {
    vec.iter_mut().enumerate().for_each(|(i, v)| *v = i);
}

fn use_for(vec: &mut Vec<usize>) {
    for i in 0..vec.len() {
        vec[i] = i;
    }
}

fn use_while(vec: &mut Vec<usize>) {
    let mut i = 0;
    let len = vec.len();
    while i < len {
        vec[i] = i;
        i += 1;
    }
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let mut vec_size = String::new();
    println!("Please input VEC_SIZE:");
    io::stdin().read_line(&mut vec_size)?;
    let vec_size: usize = vec_size.trim().parse()?;

    let mut vec = vec![0; vec_size];

    // Warm up a little
    for _ in 0..10 {
        use_while(&mut vec);
    }

    let now = Instant::now();
    for _ in 0..10 {
        use_iter(&mut vec);
    }
    let elapsed = now.elapsed();
    println!("time elapsed using iter:  {:?}", elapsed);

    let now = Instant::now();
    for _ in 0..10 {
        use_for(&mut vec);
    }
    let elapsed = now.elapsed();
    println!("time elapsed using for:   {:?}", elapsed);

    let now = Instant::now();
    for _ in 0..10 {
        use_while(&mut vec);
    }
    let elapsed = now.elapsed();
    println!("time elapsed using while: {:?}", elapsed);
    Ok(ExitCode::SUCCESS)
}
