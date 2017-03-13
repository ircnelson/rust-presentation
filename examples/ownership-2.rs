fn main() {

    // variable binding
    let numeric_vector = vec![1, 2, 3]; // numeric_vector is allocated

    foo(numeric_vector);

    println!("{:?}", numeric_vector);
}

fn foo(v: Vec<i32>) {
    
    // faz algo usando v ...

} // v é liberado da memória

