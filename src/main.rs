mod config;
mod fragments;
mod gen_cargo;
mod gen_contract;
mod gen_lib;
mod gen_test;

use config::COMBINATIONS;
use std::fs;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let generated_dir = root.join("generated");

    // Clean and recreate
    if generated_dir.exists() {
        fs::remove_dir_all(&generated_dir).expect("Failed to clean generated/");
    }
    fs::create_dir_all(&generated_dir).expect("Failed to create generated/");

    // Write a README so the directory is self-explanatory in source control
    fs::write(
        generated_dir.join("README.md"),
        "# generated/\n\
         \n\
         This directory is **auto-generated**. Do not edit files here manually.\n\
         \n\
         Re-generate with:\n\
         \n\
         ```\n\
         cargo run\n\
         ```\n\
         \n\
         Each subdirectory is a Cargo crate for one pre-built OpenZeppelin Stellar contract\n\
         combination, compiled against the OZ package versions pinned in the root `Cargo.toml`.\n",
    )
    .expect("Failed to write generated/README.md");

    println!("Generating {} combinations...", COMBINATIONS.len());

    for cfg in COMBINATIONS {
        let crate_dir = generated_dir.join(cfg.name);
        let src_dir = crate_dir.join("src");
        fs::create_dir_all(&src_dir).expect("Failed to create directories");

        fs::write(crate_dir.join("Cargo.toml"), gen_cargo::generate_cargo_toml(cfg))
            .expect("Failed to write Cargo.toml");
        fs::write(src_dir.join("lib.rs"), gen_lib::generate_lib())
            .expect("Failed to write lib.rs");
        fs::write(src_dir.join("contract.rs"), gen_contract::generate_contract(cfg))
            .expect("Failed to write contract.rs");
        fs::write(src_dir.join("test.rs"), gen_test::generate_test(cfg))
            .expect("Failed to write test.rs");

        println!("  Generated: {}", cfg.name);
    }

    println!(
        "\nDone! Generated {} combinations in generated/",
        COMBINATIONS.len()
    );
    println!("Run `cargo +nightly fmt` to format the generated code.");
}
