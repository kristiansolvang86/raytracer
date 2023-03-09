use raytracer::{add, normalize_vector};

fn main() {
    let p = Projectile::new((0.0, 1.0, 0.0, 1.0),normalize_vector((1.0, 1.0, 0.0, 0.0)));
    let e = Environment::new((0.0, -0.1, 0.0, 0.0), (-0.01, 0.0, 0.0, 0.0));

    let i = 0;
    let (proj, j) = update(&e, &p, i);

    println!("Final position: {:?} Looped {} times", proj, j);
}

fn update(env: &Environment, proj: &Projectile, mut i: i32) -> (Projectile, i32) {
    let current = tick(env, proj);
    if current.position.1 > 0.0 {
        i += 1;
        update(env, &current, i)
    } else {
        (current, i)
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = add(proj.position, proj.velocity);
    let v1 = add(proj.velocity, env.gravity);
    let velocity = add(v1, env.wind);

    Projectile { position: position, velocity: velocity }
}

#[derive(Debug)]
struct Projectile {
    position: (f64, f64, f64, f64),
    velocity: (f64, f64, f64, f64)
}

struct Environment {
    gravity: (f64, f64, f64, f64),
    wind: (f64, f64, f64, f64),
}


impl Projectile {
    fn new(position: (f64, f64, f64, f64), velocity: (f64, f64, f64, f64)) -> Projectile {
        Projectile { position, velocity }
    }
    
}

impl Environment {
    fn new(gravity: (f64, f64, f64, f64), wind: (f64, f64, f64, f64)) -> Environment {
        Environment { gravity, wind }
    }
}
