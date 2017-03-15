#[derive(PartialOrd, PartialEq, Debug)]
struct X {
    x: String
}

fn foo<'a, 'b>(first: &'a X, second: &'b X) -> &'a String {

    if second.x == "j" {
        return &"j".to_owned()
    }

    &first.x
}

fn main() {
    let x = X { x: "n".to_string() };
    let v;
    {
        let y = X { x: "j".to_string() };
        v = foo(&x, &y);
    }
    println!("{}", v);
}