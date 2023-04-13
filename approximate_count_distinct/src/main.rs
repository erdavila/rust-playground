use approximate_count_distinct::{ApproximateCountDistinct, Params};

fn main() {
    test_with_params(Params::with_unlimited_set_size());

    for max_set_size in (10..=100).rev().step_by(10) {
        test_with_params(Params::with_max_set_size(max_set_size));
    }

    test_with_params(Params::with_max_set_size(1));

    test_with_params(Params::with_max_set_size(10).set_factor_adjustment(0.1));

    test_with_params(Params::with_max_set_size(10).set_factor_adjustment(0.99));
}

fn test_with_params(params: Params) {
    println!("params = {params:?}");

    const N: u32 = 100;
    let counts: Vec<_> = (0..N)
        .map(|_| {
            let mut counter = ApproximateCountDistinct::with_params(params);
            counter.see_many(1..=100);
            counter.approximate_count_distinct()
        })
        .collect();

    let average = counts.iter().sum::<f64>() / (counts.len() as f64);

    // println!("  counts: {counts:?}");
    println!("  average: {}", average);
    println!();
}
