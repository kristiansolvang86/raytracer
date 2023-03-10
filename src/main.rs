use raytracer::{add, normalize_vector, point, vector, scalar_multiplication};
use raytracer::canvas::Canvas;
use raytracer::color::Color;
fn main() {

    let start = point((0.0, 1.0, 0.0));
    let velocity = scalar_multiplication(normalize_vector(vector((1.0, 1.8, 0.0))),11.25);

    let p = Projectile::new(start, velocity);

    let gravity = vector((0.0, -0.1, 0.0));
    let wind = vector((-0.01, 0.0, 0.0));
    let e = Environment::new(gravity, wind);

    let canvas = Canvas::new(900, 550);

    let i = 0;
    let (proj, j) = update(&e, &p, i, canvas);

    println!("Final position: {:?} Looped {} times", proj, j);
}

fn update(env: &Environment, proj: &Projectile, mut i: i32, mut canvas: Canvas) -> (Projectile, i32) {
    let current = tick(env, proj);
    if current.position.1 > 0.0 {
        i += 1;
        let color = Color::new((1.0, 0.8, 0.6));
        canvas.write_pixel_f64(current.position.1, current.position.0, color);
        println!("On the y axis: {} On the x axis {}",current.position.0, current.position.1 );
        update(env, &current, i, canvas)
    } else {
        canvas.write_to_ppm();
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
