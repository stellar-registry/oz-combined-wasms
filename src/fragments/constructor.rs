use crate::config::*;

pub fn gen_constructor_and_mint(cfg: &CombinationConfig) -> String {
    match cfg.variant {
        ContractVariant::Fungible(ft) => gen_fungible_constructor(cfg, ft),
        ContractVariant::NonFungible(nt) => gen_nft_constructor(cfg, nt),
    }
}

fn gen_fungible_constructor(cfg: &CombinationConfig, ft: FungibleType) -> String {
    let mut out = String::new();

    out.push_str("#[contractimpl]\nimpl Contract {\n");

    // Vault has a special constructor
    if ft == FungibleType::Vault {
        return gen_vault_constructor(cfg);
    }

    // Too many arguments when AccessControl + Capped (8 params)
    if cfg.access == AccessModel::AccessControl && cfg.has(Feature::Capped) {
        out.push_str("    #[allow(clippy::too_many_arguments)]\n");
    }

    // Constructor params
    out.push_str("    pub fn __constructor(\n");
    out.push_str("        e: &Env,\n");
    out.push_str("        name: String,\n");
    out.push_str("        symbol: String,\n");
    out.push_str("        decimals: u32,\n");

    match cfg.access {
        AccessModel::Ownable => out.push_str("        owner: Address,\n"),
        AccessModel::AccessControl => {
            out.push_str("        admin: Address,\n");
            out.push_str("        manager: Address,\n");
        }
    }

    if cfg.has(Feature::Capped) {
        out.push_str("        cap: i128,\n");
    }

    // Votes contracts don't take initial_supply (mint separately)
    if ft != FungibleType::Votes {
        out.push_str("        initial_supply: i128,\n");
    }

    out.push_str("    ) {\n");

    // Constructor body
    out.push_str("        Base::set_metadata(e, decimals, name, symbol);\n");

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("        set_owner(e, &owner);\n");
        }
        AccessModel::AccessControl => {
            out.push_str("        access_control::set_admin(e, &admin);\n");
            out.push_str(
                "        access_control::grant_role_no_auth(e, &manager, &symbol_short!(\"manager\"), &admin);\n",
            );
        }
    }

    if cfg.has(Feature::Capped) {
        out.push_str("        set_cap(e, cap);\n");
    }

    // AllowList: allow admin before minting
    if ft == FungibleType::AllowList {
        out.push_str("        AllowList::allow_user(e, &admin);\n");
    }

    // Mint initial supply
    if ft != FungibleType::Votes {
        let mint_to = if cfg.is_ownable() { "owner" } else { "admin" };
        if cfg.has(Feature::Capped) {
            out.push_str(&format!(
                "        if initial_supply > 0 {{\n            check_cap(e, initial_supply);\n            Base::mint(e, &{mint_to}, initial_supply);\n        }}\n"
            ));
        } else {
            out.push_str(&format!(
                "        Base::mint(e, &{mint_to}, initial_supply);\n"
            ));
        }
    }

    out.push_str("    }\n\n");

    // Mint function
    gen_fungible_mint(&mut out, cfg, ft);

    out.push_str("}\n\n");
    out
}

fn gen_fungible_mint(out: &mut String, cfg: &CombinationConfig, ft: FungibleType) {
    // Guard attributes
    let guard = match cfg.access {
        AccessModel::Ownable => "    #[only_owner]\n",
        AccessModel::AccessControl => "    #[only_admin]\n",
    };
    out.push_str(guard);

    if cfg.has(Feature::Pausable) {
        out.push_str("    #[when_not_paused]\n");
    }

    match ft {
        FungibleType::Votes => {
            out.push_str("    pub fn mint(e: &Env, to: &Address, amount: i128) {\n");
            if cfg.has(Feature::Capped) {
                out.push_str("        check_cap(e, amount);\n");
            }
            out.push_str("        FungibleVotes::mint(e, to, amount);\n");
            out.push_str("    }\n");
        }
        _ => {
            out.push_str("    pub fn mint(e: &Env, to: Address, amount: i128) {\n");
            if cfg.has(Feature::Capped) {
                out.push_str("        check_cap(e, amount);\n");
            }
            out.push_str("        Base::mint(e, &to, amount);\n");
            out.push_str("    }\n");
        }
    }
}

fn gen_vault_constructor(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    out.push_str("#[contractimpl]\nimpl Contract {\n");
    out.push_str("    pub fn __constructor(\n");
    out.push_str("        e: &Env,\n");
    out.push_str("        name: String,\n");
    out.push_str("        symbol: String,\n");
    out.push_str("        asset: Address,\n");
    out.push_str("        decimals_offset: u32,\n");
    out.push_str("        owner: Address,\n");
    out.push_str("    ) {\n");
    out.push_str("        Vault::set_asset(e, asset);\n");
    out.push_str("        Vault::set_decimals_offset(e, decimals_offset);\n");
    out.push_str("        Base::set_metadata(e, Vault::decimals(e), name, symbol);\n");
    out.push_str("        set_owner(e, &owner);\n");
    out.push_str("    }\n");

    // No mint function for vault (deposits mint shares automatically)
    let _ = cfg;

    out.push_str("}\n\n");
    out
}

fn gen_nft_constructor(cfg: &CombinationConfig, nt: NftType) -> String {
    let mut out = String::new();

    out.push_str("#[contractimpl]\nimpl Contract {\n");

    // Constructor
    out.push_str("    pub fn __constructor(\n");
    out.push_str("        e: &Env,\n");
    out.push_str("        uri: String,\n");
    out.push_str("        name: String,\n");
    out.push_str("        symbol: String,\n");

    match cfg.access {
        AccessModel::Ownable => out.push_str("        owner: Address,\n"),
        AccessModel::AccessControl => {
            out.push_str("        admin: Address,\n");
            out.push_str("        manager: Address,\n");
        }
    }

    if cfg.has(Feature::Royalties) && cfg.access == AccessModel::AccessControl {
        out.push_str("        default_royalty_bps: u32,\n");
    }

    out.push_str("    ) {\n");

    out.push_str("        Base::set_metadata(e, uri, name, symbol);\n");

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("        set_owner(e, &owner);\n");
        }
        AccessModel::AccessControl => {
            if cfg.has(Feature::Royalties) {
                out.push_str(
                    "        Base::set_default_royalty(e, &admin, default_royalty_bps);\n",
                );
            }
            out.push_str("        access_control::set_admin(e, &admin);\n");
            out.push_str(
                "        access_control::grant_role_no_auth(e, &manager, &symbol_short!(\"manager\"), &admin);\n",
            );
        }
    }

    out.push_str("    }\n\n");

    // Mint function
    gen_nft_mint(&mut out, cfg, nt);

    out.push_str("}\n\n");
    out
}

fn gen_nft_mint(out: &mut String, cfg: &CombinationConfig, nt: NftType) {
    let guard = match cfg.access {
        AccessModel::Ownable => "    #[only_owner]\n",
        AccessModel::AccessControl => "    #[only_admin]\n",
    };
    out.push_str(guard);

    if cfg.has(Feature::Pausable) {
        out.push_str("    #[when_not_paused]\n");
    }

    match nt {
        NftType::Base | NftType::Enumerable => {
            let minter = match nt {
                NftType::Base => "Base",
                NftType::Enumerable => "Enumerable",
                _ => unreachable!(),
            };
            out.push_str("    pub fn mint(e: &Env, to: Address) -> u32 {\n");
            out.push_str(&format!("        {minter}::sequential_mint(e, &to)\n"));
            out.push_str("    }\n");
        }
        NftType::Consecutive => {
            out.push_str("    pub fn batch_mint(e: &Env, to: Address, amount: u32) -> u32 {\n");
            out.push_str("        Consecutive::batch_mint(e, &to, amount)\n");
            out.push_str("    }\n");
        }
        NftType::Votes => {
            // Use NonFungibleVotes::sequential_mint to update voting units on mint
            out.push_str("    pub fn mint(e: &Env, to: Address) -> u32 {\n");
            out.push_str("        NonFungibleVotes::sequential_mint(e, &to)\n");
            out.push_str("    }\n");
        }
    }
}
