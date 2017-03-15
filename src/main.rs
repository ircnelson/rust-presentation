// rustc main.rs

fn main() {

    let a = "a".to_string();
    let b = "b".to_string();

    let c = teste(&a, &b);

    println!("{:?}", c);

    let d = "aa";

    println!("{:p}", &a);

    println!("Usar cargo run --example exemplo"); 
}

fn teste(a : &String, b : &String) -> String {
    a.to_owned()
}