pub fn square(s: u32) -> u64 {

    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    let grains = 2u64.pow(s - 1);
    println!("grains of rice on square {}: {}", s, grains);
    grains
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
