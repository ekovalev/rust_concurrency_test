use crate::*;
use std::time::{Duration, Instant};

#[test]
fn test_single_threaded_computations() {
    let input_data: Vec<u128> = (0..=20).collect();
    let res = SingleThreaded::exec(input_data.clone(), fibonacci);
    println!("Final result (single_threaded): {:?}", res);

    assert_eq!(
        res,
        [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765]
            .iter()
            .map(|x| *x as u128)
            .collect::<Vec<_>>()
    );
}

pub struct Threshold;
impl Get<u32> for Threshold {
    fn get() -> u32 {
        10u32
    }
}

#[test]
fn test_multi_threaded_computations() {
    let input_data: Vec<u128> = (0..=20).collect();
    let res = MultiThreaded::<Threshold>::exec(input_data, fibonacci);
    println!("Final result (multi_threaded): {:?}", res);

    assert_eq!(
        res,
        [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765]
            .iter()
            .map(|x| *x as u128)
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_timing() {
    let input_data: Vec<u128> = (0..=50).collect();

    let start = Instant::now();
    let _ = SingleThreaded::exec(input_data.clone(), long_computation);
    let elapsed1 = start.elapsed();
    println!("Computed in: {:?} ms", elapsed1.as_millis());

    let start = Instant::now();
    let _ = MultiThreaded::<Threshold>::exec(input_data, long_computation);
    let elapsed2 = start.elapsed();
    println!("Computed in: {:?} ms", elapsed2.as_millis());

    assert!(elapsed1 > elapsed2);
}

pub fn fibonacci(n: u128) -> u128 {
    let res = match n {
        0 => 0,
        1 | 2 => 1,
        _ => (3..=n).fold((1u128, 1u128), |(a, b), _| (b, a + b)).1,
    };
    // println!("fib({}) = {}", n, res);
    res
}

pub fn long_computation(n: u128) -> u128 {
    thread::sleep(Duration::from_millis((n as u64) * 10));
    0
}
