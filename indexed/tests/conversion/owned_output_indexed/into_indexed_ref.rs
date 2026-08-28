#[path = "../../mods/mod.rs"]
mod mods;

use indexed::{Indexed as _, IndexedOwned as _};

use mods::asserts::entries::{NONE_INDEX, VALUES};
use mods::idxd::new_owned_output_indexed;

fn main() {
    let idxd = new_owned_output_indexed(VALUES);

    let idxd_ref = idxd.into_indexed_ref();

    idxd_ref.get(NONE_INDEX);
}
