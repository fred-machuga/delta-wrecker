use delta_wrecker::vectors::Vec2;
use delta_wrecker::orbits::KeplerianElements;

fn main() {
    println!("Hello, Delta Wrecker!");

    // Test the library
    let v = Vec2::new(3.0, 4.0);
    println!("Vector: ({}, {})", v.x, v.y);

    let k = KeplerianElements::new(7000.0, 0.1);
    println!("Keplerian: SMA={}, Ecc={}", k.semi_major_axis, k.eccentricity);
}