use std::fmt::{Debug, Display};

use deterministic_chooser::{
    DeterministicChooser, deterministic_bool_chooser::DeterministicBoolChooser,
};

fn main() {
    test_deterministic_chooser([('a', 5.0), ('b', 3.0), ('c', 2.0)]);
    test_deterministic_chooser([
        ('A', rand::random::<f64>()),
        ('B', rand::random::<f64>()),
        ('C', rand::random::<f64>()),
    ]);
    test_deterministic_chooser([(true, 5.0)]);
    test_deterministic_chooser([(true, 0.0), (false, 0.0001)]);
    test_deterministic_chooser([(true, 1.0), (false, 1.0)]);

    test_deterministic_bool_generator(0.25);
}

fn test_deterministic_chooser<T: Clone + Display + Debug>(
    values_and_weights: impl IntoIterator<Item = (T, f64)>,
) {
    let values_and_weights: Vec<_> = values_and_weights.into_iter().collect();
    let weights_sum: f64 = values_and_weights.iter().map(|(_, w)| w).sum();
    let normalized_weights: Vec<_> = values_and_weights
        .iter()
        .map(|(_, w)| w / weights_sum)
        .collect();
    println!("{normalized_weights:?}");

    let mut generator = DeterministicChooser::new(values_and_weights);
    for _ in 0..20 {
        let chosen = generator.next().unwrap();

        let stats = generator.stats();
        #[expect(clippy::cast_precision_loss)]
        let total_count = stats.iter().map(|(_, s)| s.count).sum::<usize>() as f64;
        let ratios: Vec<_> = stats
            .iter()
            .map(|(_, s)| {
                #[expect(clippy::cast_precision_loss)]
                let ratio = s.count as f64 / total_count;
                ratio
            })
            .collect();
        let distance = distance(&ratios, &normalized_weights);
        println!("{chosen} {distance} {ratios:?}");
    }

    println!();
}

fn distance(v1: &[f64], v2: &[f64]) -> f64 {
    assert_eq!(v1.len(), v2.len());
    v1.iter()
        .zip(v2.iter())
        .map(|(n1, n2)| (n1 - n2).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn test_deterministic_bool_generator(true_percentage: f64) {
    let mut generator = DeterministicBoolChooser::new(true_percentage);
    println!("{true_percentage}");

    for _ in 0..20 {
        let b = generator.next().unwrap();

        let stats = generator.stats();
        #[expect(clippy::cast_precision_loss)]
        let ratio = stats.trues as f64 / (stats.total as f64);
        let distance = (ratio - true_percentage).abs();

        println!("{b} {distance} {ratio} {stats:?}");
    }

    println!();
}
