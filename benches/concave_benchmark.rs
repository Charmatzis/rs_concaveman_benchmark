use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{prelude::Distribution, distributions::Standard, Rng};
use rs_concaveman::{location_trait::LocationTrait, concaveman};

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

fn concave(_n: u32) -> () {
    let mut rng = rand::thread_rng();

    let pow_of_ten:i128 = i128::pow(10, _n);
    let raw_points: Vec<Point> = (0..pow_of_ten).map(|_v| rng.gen()).collect();
    concaveman(&raw_points, Some(0.2), None);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create concave hull from 10^6 random points", |b| b.iter(|| concave(black_box(6))));
}

criterion_group!(name=benches; config=Criterion::default().sample_size(10); targets=criterion_benchmark);
criterion_main!(benches);