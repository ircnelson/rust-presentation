/*
    Rust não possui Garbbage Collector! 
    Velocidade de c/c++, porém seguro (memory safety) :-)

    Ownership!

        Regra: Um recurso só possui um dono.
*/

fn main() {

    let a = 1;

    vender(a);
    //vender(a);
}

fn vender(item : i32) {
    // ...

    println!("vender: {:?}", item);
}