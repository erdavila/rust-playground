use std::array;

use rand::random;

use crate::algo::Algo;
use crate::enum_map::{EnumMap, EnumMapKey};
use crate::ranks::ranks;
use crate::size::Size;

mod algo;
mod enum_map;
mod ranks;
mod size;

const NUMBER_COUNT: usize = 1000;

mod total_ord;

fn main() {
    // Generate the numbers.
    let numbers: [f32; NUMBER_COUNT] = array::from_fn(|_| random());

    // Calculate the sums.
    let sums =
        EnumMap::from_fn(|size: Size| EnumMap::from_fn(|algo: Algo| algo.sum(size, &numbers)));

    // Print the sums.
    for size in Size::all() {
        println!("{size:?}");
        for algo in Algo::all() {
            println!("  {algo:?} = {}", sums[size][algo]);
        }
        println!();
    }

    // Print the differences among algorithms inside each size group.
    for size in Size::all() {
        let size_sums = &sums[size];

        let diffs: Vec<_> = Algo::all()
            .into_iter()
            .flat_map(|algo_a| Algo::all().into_iter().map(move |algo_b| (algo_a, algo_b)))
            .filter(|(algo_a, algo_b)| algo_a < algo_b)
            .map(|(algo_a, algo_b)| {
                let a = size_sums[algo_a];
                let b = size_sums[algo_b];
                let diff = (a - b).abs();
                (algo_a, algo_b, diff)
            })
            .collect();

        let ranks = ranks(&diffs);

        println!("{size:?} differences");
        for ((algo_a, algo_b, diff), rank) in diffs.into_iter().zip(ranks) {
            println!("  [{rank}] |{algo_a:?} - {algo_b:?}| = {diff}");
        }
        println!();
    }

    // Print the differences of corresponding algorithms for each size.
    let diffs: Vec<_> = Algo::all()
        .into_iter()
        .map(|algo| {
            let a = sums[Size::F32][algo];
            let b = sums[Size::F64][algo];
            let diff = (a - b).abs();
            (algo, diff)
        })
        .collect();
    let ranks = ranks(&diffs);
    println!("{:?} x {:?} differences", Size::F32, Size::F64);
    for ((algo, diff), rank) in diffs.into_iter().zip(ranks) {
        println!("  [{rank}] {algo:?} = {diff}");
    }
}
