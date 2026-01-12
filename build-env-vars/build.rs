fn main() {
    let mut vars: Vec<_> = std::env::vars().collect();
    vars.sort_by_key(|(name, _value)| name.clone());
    for (name, value) in vars {
        println!("cargo:warning={name:?}: {value:?}");
    }
}
