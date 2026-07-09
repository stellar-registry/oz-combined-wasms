use crate::config::*;
use crate::fragments::{
    access::gen_access_impl, constructor::gen_constructor_and_mint, extensions::gen_extension_impls,
    fungible, imports::gen_imports, nft, pausable::gen_pausable_impl, upgrade::gen_upgradeable_impl,
};

pub fn generate_contract(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    // Doc comment
    out.push_str(&format!("//! {}\n\n", cfg.description));

    // Imports
    out.push_str(&gen_imports(cfg));

    // Contract struct
    out.push_str("#[contract]\n");
    out.push_str("pub struct Contract;\n\n");

    // Constructor + mint
    out.push_str(&gen_constructor_and_mint(cfg));

    // Pausable trait impl (if needed)
    if cfg.has(Feature::Pausable) {
        out.push_str(&gen_pausable_impl(cfg));
    }

    // Token trait + Burnable impls
    match cfg.variant {
        ContractVariant::Fungible(_) => {
            if cfg.has(Feature::Pausable) {
                out.push_str(&fungible::gen_fungible_pausable(cfg));
            } else {
                out.push_str(&fungible::gen_fungible_default(cfg));
            }
        }
        ContractVariant::NonFungible(_) => {
            if cfg.has(Feature::Pausable) {
                out.push_str(&nft::gen_nft_pausable(cfg));
            } else {
                out.push_str(&nft::gen_nft_default(cfg));
            }
        }
    }

    // Extension trait impls (AllowList, BlockList, Royalties, Votes, Vault)
    out.push_str(&gen_extension_impls(cfg));

    // Access control trait impl
    out.push_str(&gen_access_impl(cfg));

    // Upgradeable impl
    out.push_str(&gen_upgradeable_impl(cfg));

    out
}
