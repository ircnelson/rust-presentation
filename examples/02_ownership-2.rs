fn main() {

    let a = 1;

    //alugar(&a);
    vender(a);
}

fn vender(item : i32) {
    
    println!("vender: {:?}", item);
}

fn alugar(item : &i32) {
    
    println!("alugar: {:?}", item);
}