#[path = "../../fixture.rs"]
mod fixture;

use fixture::*;

fn main() {
    any_output::required_by_dynamic_dispatch(&owned_output::idxd!());
}
