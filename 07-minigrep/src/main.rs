use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("{:?}", type_of(env::args()));
}

fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}