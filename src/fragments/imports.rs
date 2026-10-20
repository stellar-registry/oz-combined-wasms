use crate::config::*;
use crate::fragments::fungible::tracks_total_supply;

pub fn gen_imports(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    match cfg.variant {
        ContractVariant::Fungible(ft) => gen_fungible_imports(&mut out, cfg, ft),
        ContractVariant::NonFungible(nt) => gen_nft_imports(&mut out, cfg, nt),
    }

    out.push('\n');
    out
}

fn gen_fungible_imports(out: &mut String, cfg: &CombinationConfig, ft: FungibleType) {
    // soroban_sdk imports
    let mut sdk_items = vec!["contract", "contractimpl"];
    if !cfg.is_ownable() {
        sdk_items.push("symbol_short");
    }
    sdk_items.extend_from_slice(&["Address", "BytesN", "Env", "MuxedAddress", "String"]);
    if !cfg.is_ownable() {
        sdk_items.push("Symbol");
        sdk_items.push("Vec");
    }
    out.push_str(&format!("use soroban_sdk::{{{items}}};\n", items = sdk_items.join(", ")));

    // Access control imports
    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("use stellar_access::ownable::{set_owner, Ownable};\n");
        }
        AccessModel::AccessControl => {
            out.push_str(
                "use stellar_access::access_control::{self as access_control, AccessControl};\n",
            );
        }
    }

    // Contract utils imports
    if cfg.has(Feature::Pausable) {
        out.push_str("use stellar_contract_utils::pausable::{self as pausable, Pausable};\n");
    }
    out.push_str(
        "use stellar_contract_utils::upgradeable::{self as upgradeable, Upgradeable};\n",
    );

    // Macros imports
    let mut macro_items = Vec::new();
    match cfg.access {
        AccessModel::Ownable => macro_items.push("only_owner"),
        AccessModel::AccessControl => {
            macro_items.push("only_admin");
            if ft == FungibleType::AllowList || ft == FungibleType::BlockList {
                macro_items.push("only_role");
            }
        }
    }
    if cfg.has(Feature::Pausable) {
        macro_items.push("when_not_paused");
    }
    if !macro_items.is_empty() {
        out.push_str(&format!(
            "use stellar_macros::{{{items}}};\n",
            items = macro_items.join(", ")
        ));
    }

    // Token imports
    let mut token_items = Vec::new();
    if ft != FungibleType::Vault {
        token_items.push("burnable::FungibleBurnable");
    }
    if cfg.has(Feature::Capped) {
        if tracks_total_supply(ft) {
            token_items.push("capped::{Capped, FungibleCapped}");
        } else {
            token_items.push("capped::{check_cap, set_cap}");
        }
    }
    match ft {
        FungibleType::AllowList => token_items.push("allowlist::{AllowList, FungibleAllowList}"),
        FungibleType::BlockList => token_items.push("blocklist::{BlockList, FungibleBlockList}"),
        FungibleType::Votes => token_items.push("votes::FungibleVotes"),
        _ => {}
    }
    if tracks_total_supply(ft) {
        token_items.push("total_supply::{FungibleTotalSupply, TotalSupply}");
    }
    token_items.extend_from_slice(&["Base", "Compose"]);
    // Manual delegation calls the ContractType's trait methods directly
    if cfg.has(Feature::Pausable) {
        token_items.push("ContractOverrides");
    }
    token_items.push("FungibleToken");
    out.push_str(&format!(
        "use stellar_tokens::fungible::{{{items}}};\n",
        items = token_items.join(", ")
    ));
    match ft {
        FungibleType::Votes if cfg.has(Feature::Capped) => {
            out.push_str("use stellar_governance::votes::{get_total_supply, Votes};\n");
        }
        FungibleType::Votes => out.push_str("use stellar_governance::votes::Votes;\n"),
        FungibleType::Vault => out.push_str("use stellar_tokens::vault::{FungibleVault, Vault};\n"),
        _ => {}
    }
}

fn gen_nft_imports(out: &mut String, cfg: &CombinationConfig, nt: NftType) {
    // soroban_sdk imports
    let mut sdk_items = vec!["contract", "contractimpl"];
    if !cfg.is_ownable() {
        sdk_items.push("symbol_short");
    }
    sdk_items.extend_from_slice(&["Address", "BytesN", "Env", "String"]);
    if !cfg.is_ownable() {
        sdk_items.push("Symbol");
        sdk_items.push("Vec");
    }
    out.push_str(&format!("use soroban_sdk::{{{items}}};\n", items = sdk_items.join(", ")));

    // Access control imports
    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("use stellar_access::ownable::{set_owner, Ownable};\n");
        }
        AccessModel::AccessControl => {
            out.push_str(
                "use stellar_access::access_control::{self as access_control, AccessControl};\n",
            );
        }
    }

    // Contract utils imports
    if cfg.has(Feature::Pausable) {
        out.push_str("use stellar_contract_utils::pausable::{self as pausable, Pausable};\n");
    }
    out.push_str(
        "use stellar_contract_utils::upgradeable::{self as upgradeable, Upgradeable};\n",
    );

    // Macros imports
    let mut macro_items = Vec::new();
    match cfg.access {
        AccessModel::Ownable => macro_items.push("only_owner"),
        AccessModel::AccessControl => {
            macro_items.push("only_admin");
            if cfg.has(Feature::Royalties) {
                macro_items.push("only_role");
            }
        }
    }
    if cfg.has(Feature::Pausable) {
        macro_items.push("when_not_paused");
    }
    if !macro_items.is_empty() {
        out.push_str(&format!(
            "use stellar_macros::{{{items}}};\n",
            items = macro_items.join(", ")
        ));
    }

    // Token imports
    let mut nft_items = vec!["burnable::NonFungibleBurnable"];
    match nt {
        NftType::Base => {}
        NftType::Enumerable => nft_items.push("enumerable::{Enumerable, NonFungibleEnumerable}"),
        NftType::Consecutive => nft_items.push("consecutive::{Consecutive, NonFungibleConsecutive}"),
        NftType::Votes => nft_items.push("votes::NonFungibleVotes"),
    }
    if cfg.has(Feature::Royalties) {
        nft_items.push("royalties::{NonFungibleRoyalties, Royalties, RoyaltySupport}");
    }
    nft_items.extend_from_slice(&["Base", "Compose"]);
    // Manual delegation calls the ContractType's trait methods directly; Base has them inherently
    if cfg.has(Feature::Pausable) && nt != NftType::Base {
        nft_items.push("ContractOverrides");
    }
    nft_items.push("NonFungibleToken");
    out.push_str(&format!(
        "use stellar_tokens::non_fungible::{{{items}}};\n",
        items = nft_items.join(", ")
    ));
    if nt == NftType::Votes {
        out.push_str("use stellar_governance::votes::Votes;\n");
    }
}
