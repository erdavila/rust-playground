fn main() {
    let mut vars: Vec<_> = std::env::vars().collect();
    vars.sort();

    for (name, value) in vars {
        println!("cargo::warning={name:?}: {value:?}");
    }
}
