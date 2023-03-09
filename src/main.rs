use rand::Rng;
use rand::distributions::{Distribution, Standard};
use rs_concaveman::concaveman;
use rs_concaveman::location_trait::LocationTrait;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

impl LocationTrait for Point {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    const N:i128 = i128::pow(10, 6);
    println!("{}", N);
    let raw_points: Vec<Point> = (0..N).map(|_v| rng.gen()).collect();
    let hull = concaveman(&raw_points, None, None);
    println!("Polygon{:?}", hull);
}

