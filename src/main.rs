#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn main() {
    let a = divide(10, 5);

    let b = divide(10, 0);

    match a {
        Res::Thing(v) => println!("val = {}", v),
        _ => {},
    }
    if let Res::Thing(v) = a {
        println!("val = {}", v);
    }
    println!("a = {:?}, b = {:?}", a, b);

    let c = divide2(15, 5);
    let d = divide2(15, 0);
    match c {
        Ok(v) => println!("val = {}", v),
        _ => {}
    }
    if let Ok(v) = c {

        println!("val = {}", v);
    }
    println!("c = {:?}, d = {:?}", c, d);
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannnot Divide by zero".to_string());
    }
    Res::Thing(a / b)
}

fn divide2(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannnot Divide by zero".to_string());
    }
    Ok(a / b)
}
