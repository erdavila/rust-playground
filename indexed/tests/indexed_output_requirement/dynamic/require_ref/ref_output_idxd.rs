#[path = "../../fixture.rs"]
mod fixture;

use fixture::*;

fn main() {
    ref_output::required_by_dynamic_dispatch(&ref_output::idxd!());
}
