fn main() {
    println!("Hello, world!");
}

fn tick(env: Environment, proj: Projectile) -> ((f64, f64, f64, f64), (f64, f64, f64, f64)) {
    
}

struct Projectile {
    position: (f64, f64, f64, f64),
    velocity: (f64, f64, f64, f64)
}

struct Environment {
    gravity: (f64, f64, f64, f64),
    wind: (f64, f64, f64, f64),
}

