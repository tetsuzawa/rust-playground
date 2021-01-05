use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    // println!("{:?}", args);
    // println!("{:?}", type_of(env::args()));
    // {}を探しています
    // println!("Searching for {}", query);
    // {}というファイルの中

    // println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}

// fn type_of<T>(_: T) -> String {
//     let a = std::any::type_name::<T>();
//     return a.to_string();
// }