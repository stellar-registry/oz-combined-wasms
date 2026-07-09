use crate::config::*;

pub fn generate_cargo_toml(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        r#"[package]
name = "{name}"
edition.workspace = true
license.workspace = true
repository.workspace = true
publish = false
version.workspace = true

[package.metadata.stellar]
cargo_inherit = true
homepage = "stellar-registry.io"
repository = "https://github.com/stellar-registry/oz-combined-wasms/tree/main/generated/{name}"

[lib]
crate-type = ["cdylib"]
doctest = false

[dependencies]
soroban-sdk = {{ workspace = true }}
stellar-access = {{ workspace = true }}
stellar-contract-utils = {{ workspace = true }}
stellar-macros = {{ workspace = true }}
stellar-tokens = {{ workspace = true }}
"#,
        name = cfg.name,
    ));

    // Governance dependency for Votes variants
    let needs_governance = matches!(
        cfg.variant,
        ContractVariant::Fungible(FungibleType::Votes)
            | ContractVariant::NonFungible(NftType::Votes)
    );
    if needs_governance {
        out.push_str("stellar-governance = { workspace = true }\n");
    }

    out.push_str(
        r#"
[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
"#,
    );

    out
}
