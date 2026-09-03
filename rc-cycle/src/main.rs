use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

use clap::Parser;
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use rand::{Rng, SeedableRng};

use crate::node::Node;

mod node;

#[derive(Debug, Parser)]
struct Args {
    num_nodes: u16,
    connectivity: u16,
}

fn main() {
    let args = Args::parse();
    let num_nodes: usize = args.num_nodes.into();
    let connectivity: usize = args.connectivity.into();
    assert!(connectivity <= num_nodes);

    let seed = rand::random();

    create_and_connect_and_finalize_nodes(drop, "dropping", num_nodes, connectivity, seed);
    create_and_connect_and_finalize_nodes(clean, "cleaning", num_nodes, connectivity, seed);
}

fn create_and_connect_and_finalize_nodes(
    finalize: fn(Vec<Rc<RefCell<Node>>>),
    finalization_name: &str,
    num_nodes: usize,
    connectivity: usize,
    seed: u64,
) {
    let mut rng = StdRng::seed_from_u64(seed);

    node::count::reset();

    let nodes = create_and_connect_nodes(&mut rng, num_nodes, connectivity);

    println!(
        "node count before {finalization_name}: {}",
        node::count::get()
    );

    finalize(nodes);

    println!(
        "node count after {finalization_name}: {}",
        node::count::get()
    );
}

fn create_and_connect_nodes(
    rng: &mut impl Rng,
    num_nodes: usize,
    connectivity: usize,
) -> Vec<Rc<RefCell<Node>>> {
    let nodes: Vec<_> = iter::from_fn(|| Some(Rc::new(RefCell::new(Node::new()))))
        .take(num_nodes)
        .collect();

    for this_node in &nodes {
        let mut this_node = this_node.borrow_mut();

        for other_node in nodes.sample(rng, connectivity) {
            this_node.connected_to().push(Rc::clone(other_node));
        }
    }

    nodes
}

fn clean(nodes: Vec<Rc<RefCell<Node>>>) {
    for node in nodes {
        node.borrow_mut().connected_to().clear();
    }
}
