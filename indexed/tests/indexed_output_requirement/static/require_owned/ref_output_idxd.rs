#[path = "../../fixture.rs"]
mod fixture;

use fixture::*;

fn main() {
    owned_output::required_by_static_dispatch(&ref_output::idxd!());
}
