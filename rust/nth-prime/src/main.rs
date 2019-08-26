mod lib;

fn main() {
    println!(
        "{:?}",
        (0..15)
            .map(|x: i32| lib::nth(x as u32))
            .collect::<Vec<u32>>()
    );
    println!("{}", lib::nth(10000));
}
