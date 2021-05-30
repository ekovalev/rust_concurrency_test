#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::sync::mpsc;
use std::thread;
use core::fmt::Debug;
use std::collections::BTreeMap;

#[cfg(test)]
mod tests;

pub trait Executable<T, R, F> {
    fn exec(x: Vec<T>, f: F) -> Vec<R>
    where
        T: Copy,
        R: Default + Clone,
        F: Fn(T) -> R;
}

pub struct SingleThreaded;

impl<T, R, F> Executable<T, R, F> for SingleThreaded {
    fn exec(x: Vec<T>, f: F) -> Vec<R>
    where
        T: Copy,
        R: Default + Clone,
        F: Fn(T) -> R,
    {
        x.into_iter().map(|t| f(t)).collect::<Vec<_>>()
    }
}

pub trait Get<T> {
    fn get() -> T;
}

impl Get<u32> for () {
    fn get() -> u32 {
        Default::default()
    }
}

pub struct MultiThreaded<G>(PhantomData<G>)
where
    G: Get<u32>;

impl<T, R, F, G> Executable<T, R, F> for MultiThreaded<G> where
T: 'static + Send,
R: 'static + Debug + Send,
F: 'static + Send + Copy,
G: Get<u32>
{
    fn exec(x: Vec<T>, f: F) -> Vec<R>
    where
        T: Copy,
        R: Default + Clone,
        F: Fn(T) -> R,
    {
        let threshold = G::get();
        let num_tasks = x.len() as u32;
        let mut num_threads = num_tasks / threshold;
        if num_threads * threshold < num_tasks {
            num_threads = num_threads + 1;
        }
        match num_threads {
            1 => SingleThreaded::exec(x, f),
            n => {
                let to_spawn = n - 1; // At least one
                let (tx, rx) = mpsc::channel();
                for i in 0..to_spawn {
                    let tx_local = tx.clone();
                    let lower = (i * threshold) as usize;
                    let upper = ((i + 1) * threshold) as usize;
                    let x_local: Vec<T> = x[lower..upper].to_vec();
                    thread::spawn(move || {
                        let output = SingleThreaded::exec(x_local, f);
                        tx_local.send((i, output)).expect("Channel should exist");
                    });
                }
                drop(tx); // Dropping the original transmitter's channel to avoid infinite blocking

                // Do the work on the rest of the input data in the main thread
                let start = (threshold * to_spawn) as usize;
                let remainder = SingleThreaded::exec(x[start..].to_vec(), f);
                
                // Collect the results from worker threads
                let mut res_map = BTreeMap::new();
                for (i, values) in rx {
                    res_map.insert(i, values);
                }

                let mut res = vec![];
                for i in 0..to_spawn {
                    res.extend(res_map[&i].clone());
                }
                res.extend(remainder);

                res
            }
        }
    }
}
