use patterns2::newtype::Meters;
fn main() {
    let distance = Meters::new(10.0);
    println!("Distance in meters: {}", distance.value());

    let distance_in_feet = distance.to_feet();
    println!("Distance in feet: {}", distance_in_feet.value());

    let distance_in_meters = distance_in_feet.to_meters();
    println!("Distance in meters: {}", distance_in_meters.value());
}
