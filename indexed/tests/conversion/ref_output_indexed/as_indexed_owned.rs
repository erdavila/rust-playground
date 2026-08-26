#[path = "../../mods/mod.rs"]
mod mods;

use indexed::{Indexed as _, IndexedOwned as _};

use mods::asserts::entries::{NONE_INDEX, VALUES};
use mods::idxd::new_ref_output_indexed;

fn main() {
    let idxd = new_ref_output_indexed(VALUES);

    let idxd_owned = idxd.as_indexed_owned();

    idxd_owned.get(NONE_INDEX);
}
