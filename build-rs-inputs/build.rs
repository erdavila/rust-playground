macro_rules! show_input {
    ($( $name:ident )*) => {
        $(
            build_rs::output::warning(&format!("build_rs::input::{}(): {:?}", stringify!($name), build_rs::input::$name()));
        )*
    };
}

fn main() {
    show_input!(
        cargo
        // cargo_cfg
        cargo_cfg_debug_assertions
        cargo_cfg_feature
        cargo_cfg_panic
        cargo_cfg_proc_macro
        cargo_cfg_target_abi
        cargo_cfg_target_arch
        cargo_cfg_target_endian
        cargo_cfg_target_env
        cargo_cfg_target_feature
        cargo_cfg_target_has_atomic
        cargo_cfg_target_os
        cargo_cfg_target_pointer_width
        cargo_cfg_target_vendor
        cargo_cfg_unix
        cargo_cfg_windows
        cargo_encoded_rustflags
        // cargo_feature
        cargo_makeflags
        cargo_manifest_dir
        cargo_manifest_links
        cargo_manifest_path
        cargo_pkg_authors
        cargo_pkg_description
        cargo_pkg_homepage
        cargo_pkg_license
        cargo_pkg_license_file
        cargo_pkg_name
        cargo_pkg_readme
        cargo_pkg_repository
        cargo_pkg_rust_version
        cargo_pkg_version
        cargo_pkg_version_major
        cargo_pkg_version_minor
        cargo_pkg_version_patch
        cargo_pkg_version_pre
        cargo_target_family
        debug
        // dep_metadata
        host
        num_jobs
        opt_level
        out_dir
        profile
        rustc
        rustc_linker
        rustc_workspace_wrapper
        rustc_wrapper
        rustdoc
        target
    );
}
