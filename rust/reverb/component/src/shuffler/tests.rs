use super::*;

use assert_approx_eq::assert_approx_eq;
use conformal_component::audio::all_approx_eq;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

#[test]
fn energy_preserving() {
    let shuffler = Shuffler::new(&mut Xoshiro256PlusPlus::seed_from_u64(369));
    let input = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let output = shuffler.shuffle(&input);
    assert_approx_eq!(
        output.iter().map(|x| x * x).sum::<f32>(),
        input.iter().map(|x| x * x).sum::<f32>(),
        1e-6
    );
}

#[test]
fn stateless() {
    let shuffler = Shuffler::new(&mut Xoshiro256PlusPlus::seed_from_u64(369));
    let input = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let output = shuffler.shuffle(&input);
    assert!(all_approx_eq(shuffler.shuffle(&input), output, 1e-6));
}

#[test]
fn seed_matters() {
    let shuffler_a = Shuffler::new(&mut Xoshiro256PlusPlus::seed_from_u64(369));
    let shuffler_b = Shuffler::new(&mut Xoshiro256PlusPlus::seed_from_u64(420));
    let input = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    assert!(!all_approx_eq(
        shuffler_a.shuffle(&input),
        shuffler_b.shuffle(&input),
        1e-6
    ));
}
