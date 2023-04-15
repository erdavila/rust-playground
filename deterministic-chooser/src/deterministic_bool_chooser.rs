use crate::DeterministicChooser;

pub struct DeterministicBoolChooser {
    chooser: DeterministicChooser<bool>,
}

impl DeterministicBoolChooser {
    pub fn new(true_percentage: f64) -> Self {
        DeterministicBoolChooser {
            chooser: DeterministicChooser::new([
                (true, true_percentage),
                (false, 1.0 - true_percentage),
            ]),
        }
    }

    pub fn stats(&self) -> Stats {
        let stats = self.chooser.stats();
        Stats {
            trues: stats[0].1.count,
            total: stats.iter().map(|s| s.1.count).sum(),
        }
    }
}

impl Iterator for DeterministicBoolChooser {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.chooser.next()
    }
}

#[derive(Debug)]
pub struct Stats {
    pub trues: usize,
    pub total: usize,
}
