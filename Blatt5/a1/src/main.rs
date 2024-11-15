use rand::Rng;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Multitype {
    Integer(i32),
    Float(f32),
    Text(String),
    Coordinate(Point),
}

fn read_value(v: &mut Multitype) {
    let mut rng = rand::thread_rng();
    let type_choice = rng.gen_range(0..4);

    *v = match type_choice {
        0 => Multitype::Integer(rng.gen_range(-100..100)),
        1 => Multitype::Float(rng.gen::<f32>() * 100.0),
        2 => Multitype::Text(String::from("Zufälliger Text")),
        _ => Multitype::Coordinate(Point {
            x: rng.gen_range(-10..10),
            y: rng.gen_range(-10..10),
        }),
    };
}

fn main() {
    let mut value = Multitype::Integer(0);
    
    for i in 1..=5 {
        read_value(&mut value);
        
        println!("Durchlauf {}: ", i);
        match value {
            Multitype::Integer(n) => println!("Integer: {}", n),
            Multitype::Float(f) => println!("Float: {:.2}", f),
            Multitype::Text(ref s) => println!("String: {}", s),
            Multitype::Coordinate(Point { x, y }) => println!("Point: ({}, {})", x, y),
        }
    }
}