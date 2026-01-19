use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {x: x, y: y}
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for Option<u32> {}

impl ComputeNorm for Point {
    fn compute_norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl ComputeNorm for Vec<f64> {
    fn compute_norm(&self) -> f64 {
        self.iter().map(|x| {x * x}).sum::<f64>().sqrt()
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, other: Self) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul for &Point {
    type Output = f64;
    fn mul(self, other:Self) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

fn main() {
    let the_origin = Point::new(0.0, 0.0);
    let mut p = the_origin;
    println!("p: {:?}, the_origin: {:?}", p, the_origin);
    println!("equal? -> {}", p == the_origin);
    p.x += 10.0;
    println!("p: {:?}, the_origin: {:?}", p, the_origin);
    println!("equal? -> {}", p == the_origin);

    let some_opt: Option<u32> = Some(10);
    println!("norm of some_opt: {}", some_opt.compute_norm());

    let lil_vec: Vec<f64> = vec![3.0, 4.0];
    println!("norm of lil_vec: {}", lil_vec.compute_norm());

    println!("origin_p + p: {:?}", &p + &the_origin);
    println!("origin_p * p: {:?}", &p * &the_origin);
}
