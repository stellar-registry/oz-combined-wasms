use crate::config::*;
use crate::fragments::fungible::tracks_total_supply;

/// Votes tokens report supply through their voting checkpoints.
fn supply_fn(cfg: &CombinationConfig) -> &'static str {
    if tracks_total_supply(cfg.fungible_type().unwrap()) {
        "total_supply"
    } else {
        "get_total_supply"
    }
}

pub fn generate_test(cfg: &CombinationConfig) -> String {
    match cfg.variant {
        ContractVariant::Fungible(ft) => gen_fungible_test(cfg, ft),
        ContractVariant::NonFungible(nt) => gen_nft_test(cfg, nt),
    }
}

fn gen_fungible_test(cfg: &CombinationConfig, ft: FungibleType) -> String {
    if ft == FungibleType::Vault {
        return gen_vault_test();
    }

    let mut out = String::new();

    out.push_str("extern crate std;\n\n");
    out.push_str("use soroban_sdk::{testutils::Address as _, Address, Env, String};\n\n");
    out.push_str("use crate::contract::{Contract, ContractClient};\n\n");

    // create_client helper
    gen_ft_create_client(&mut out, cfg, ft);

    // Tests
    gen_ft_initial_state(&mut out, cfg, ft);
    gen_ft_transfer_works(&mut out, cfg);
    gen_ft_mint_works(&mut out, cfg, ft);
    gen_ft_burn_works(&mut out, cfg);

    if cfg.has(Feature::Pausable) {
        gen_ft_pausable_tests(&mut out, cfg);
    }

    if cfg.has(Feature::Capped) {
        gen_ft_capped_test(&mut out, cfg, ft);
    }

    if ft == FungibleType::AllowList {
        gen_ft_allowlist_tests(&mut out, cfg);
    }

    if ft == FungibleType::BlockList {
        gen_ft_blocklist_tests(&mut out, cfg);
    }

    out
}

fn gen_ft_create_client(out: &mut String, cfg: &CombinationConfig, ft: FungibleType) {
    match cfg.access {
        AccessModel::Ownable => {
            if ft == FungibleType::Votes {
                out.push_str("fn create_client<'a>(e: &Env, owner: &Address) -> ContractClient<'a> {\n");
                out.push_str("    let name = String::from_str(e, \"Test Token\");\n");
                out.push_str("    let symbol = String::from_str(e, \"TST\");\n");
                let mut args = String::from("name, symbol, 18u32, owner");
                if cfg.has(Feature::Capped) {
                    args.push_str(", 1_000_000i128");
                }
                out.push_str(&format!(
                    "    let address = e.register(Contract, ({args}));\n"
                ));
                out.push_str("    ContractClient::new(e, &address)\n");
                out.push_str("}\n\n");
            } else {
                out.push_str("fn create_client<'a>(e: &Env, owner: &Address, initial_supply: i128) -> ContractClient<'a> {\n");
                out.push_str("    let name = String::from_str(e, \"Test Token\");\n");
                out.push_str("    let symbol = String::from_str(e, \"TST\");\n");
                let mut args = String::from("name, symbol, 18u32, owner");
                if cfg.has(Feature::Capped) {
                    args.push_str(", 1_000_000i128");
                }
                args.push_str(", initial_supply");
                out.push_str(&format!(
                    "    let address = e.register(Contract, ({args}));\n"
                ));
                out.push_str("    ContractClient::new(e, &address)\n");
                out.push_str("}\n\n");
            }
        }
        AccessModel::AccessControl => {
            out.push_str("fn create_client<'a>(e: &Env, admin: &Address, manager: &Address, initial_supply: i128) -> ContractClient<'a> {\n");
            out.push_str("    let name = String::from_str(e, \"Test Token\");\n");
            out.push_str("    let symbol = String::from_str(e, \"TST\");\n");
            let mut args = String::from("name, symbol, 18u32, admin, manager");
            if cfg.has(Feature::Capped) {
                args.push_str(", 1_000_000i128");
            }
            args.push_str(", initial_supply");
            out.push_str(&format!(
                "    let address = e.register(Contract, ({args}));\n"
            ));
            out.push_str("    ContractClient::new(e, &address)\n");
            out.push_str("}\n\n");
        }
    }
}

fn gen_ft_initial_state(out: &mut String, cfg: &CombinationConfig, ft: FungibleType) {
    out.push_str("#[test]\n");
    out.push_str("fn initial_state() {\n");
    out.push_str("    let e = Env::default();\n");

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("    let owner = Address::generate(&e);\n");
            if ft == FungibleType::Votes {
                out.push_str("    let client = create_client(&e, &owner);\n");
                out.push_str(&format!("    assert_eq!(client.{}(), 0);\n", supply_fn(cfg)));
            } else {
                out.push_str("    let client = create_client(&e, &owner, 1000);\n");
                out.push_str(&format!("    assert_eq!(client.{}(), 1000);\n", supply_fn(cfg)));
                out.push_str("    assert_eq!(client.balance(&owner), 1000);\n");
            }
        }
        AccessModel::AccessControl => {
            out.push_str("    let admin = Address::generate(&e);\n");
            out.push_str("    let manager = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &admin, &manager, 1000);\n");
            out.push_str(&format!("    assert_eq!(client.{}(), 1000);\n", supply_fn(cfg)));
            out.push_str("    assert_eq!(client.balance(&admin), 1000);\n");
        }
    }

    out.push_str("    assert_eq!(client.symbol(), String::from_str(&e, \"TST\"));\n");
    out.push_str("    assert_eq!(client.name(), String::from_str(&e, \"Test Token\"));\n");
    out.push_str("    assert_eq!(client.decimals(), 18);\n");

    if cfg.has(Feature::Pausable) {
        out.push_str("    assert!(!client.paused());\n");
    }

    out.push_str("}\n\n");
}

/// Returns (create_call, holder_var) for the test setup.
/// For Votes: creates client + mints tokens. For others: creates with initial_supply.
fn ft_test_setup(cfg: &CombinationConfig, ft: FungibleType, supply: i128) -> (String, &'static str) {
    let mut s = String::new();
    match cfg.access {
        AccessModel::Ownable => {
            s.push_str("    let owner = Address::generate(&e);\n");
            if ft == FungibleType::Votes {
                s.push_str("    let client = create_client(&e, &owner);\n");
                s.push_str("    e.mock_all_auths();\n");
                s.push_str(&format!("    client.mint(&owner, &{supply});\n"));
            } else {
                s.push_str(&format!("    let client = create_client(&e, &owner, {supply});\n"));
                s.push_str("    e.mock_all_auths();\n");
            }
            (s, "owner")
        }
        AccessModel::AccessControl => {
            s.push_str("    let admin = Address::generate(&e);\n");
            s.push_str("    let manager = Address::generate(&e);\n");
            s.push_str(&format!("    let client = create_client(&e, &admin, &manager, {supply});\n"));
            s.push_str("    e.mock_all_auths();\n");
            (s, "admin")
        }
    }
}

fn gen_ft_transfer_works(out: &mut String, cfg: &CombinationConfig) {
    let ft = cfg.fungible_type().unwrap();
    out.push_str("#[test]\n");
    out.push_str("fn transfer_works() {\n");
    out.push_str("    let e = Env::default();\n");

    let (setup, holder) = ft_test_setup(cfg, ft, 1000);
    out.push_str(&setup);
    out.push_str("    let recipient = Address::generate(&e);\n");

    // AllowList needs recipient to be allowed first
    if matches!(cfg.variant, ContractVariant::Fungible(FungibleType::AllowList)) {
        out.push_str("    client.allow_user(&recipient, &manager);\n");
    }

    out.push_str(&format!("    client.transfer(&{holder}, &recipient, &100);\n"));
    out.push_str(&format!("    assert_eq!(client.balance(&{holder}), 900);\n"));
    out.push_str("    assert_eq!(client.balance(&recipient), 100);\n");

    out.push_str("}\n\n");
}

fn gen_ft_mint_works(out: &mut String, cfg: &CombinationConfig, ft: FungibleType) {
    let (setup, holder) = ft_test_setup(cfg, ft, 1000);

    out.push_str("#[test]\n");
    out.push_str("fn mint_works() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str(&setup);
    out.push_str(&format!("    client.mint(&{holder}, &500);\n"));
    out.push_str(&format!("    assert_eq!(client.{}(), 1500);\n", supply_fn(cfg)));
    out.push_str("}\n\n");
}

fn gen_ft_burn_works(out: &mut String, cfg: &CombinationConfig) {
    let ft = cfg.fungible_type().unwrap();
    let (setup, holder) = ft_test_setup(cfg, ft, 1000);

    out.push_str("#[test]\n");
    out.push_str("fn burn_works() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str(&setup);
    out.push_str(&format!("    client.burn(&{holder}, &200);\n"));
    out.push_str(&format!("    assert_eq!(client.{}(), 800);\n", supply_fn(cfg)));
    out.push_str(&format!("    assert_eq!(client.balance(&{holder}), 800);\n"));
    out.push_str("}\n\n");
}

fn gen_ft_pausable_tests(out: &mut String, cfg: &CombinationConfig) {
    let ft = cfg.fungible_type().unwrap();
    let (setup, holder) = ft_test_setup(cfg, ft, 1000);

    out.push_str("#[test]\n");
    out.push_str("#[should_panic(expected = \"Error(Contract, #1000)\")]\n");
    out.push_str("fn transfer_fails_when_paused() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str(&setup);
    out.push_str("    let recipient = Address::generate(&e);\n");
    out.push_str(&format!("    client.pause(&{holder});\n"));
    out.push_str(&format!("    client.transfer(&{holder}, &recipient, &100);\n"));
    out.push_str("}\n\n");

    out.push_str("#[test]\n");
    out.push_str("#[should_panic(expected = \"Error(Contract, #1000)\")]\n");
    out.push_str("fn mint_fails_when_paused() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str(&setup);
    out.push_str(&format!("    client.pause(&{holder});\n"));
    out.push_str(&format!("    client.mint(&{holder}, &500);\n"));
    out.push_str("}\n\n");

    out.push_str("#[test]\n");
    out.push_str("#[should_panic(expected = \"Error(Contract, #1000)\")]\n");
    out.push_str("fn burn_fails_when_paused() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str(&setup);
    out.push_str(&format!("    client.pause(&{holder});\n"));
    out.push_str(&format!("    client.burn(&{holder}, &200);\n"));
    out.push_str("}\n\n");
}

fn gen_ft_capped_test(out: &mut String, cfg: &CombinationConfig, ft: FungibleType) {
    let (setup, holder) = ft_test_setup(cfg, ft, 1000);

    out.push_str("#[test]\n");
    out.push_str("#[should_panic]\n");
    out.push_str("fn mint_exceeds_cap_fails() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str(&setup);
    // Cap is 1_000_000, initial_supply is 1000, so minting 1_000_000 more exceeds cap
    out.push_str(&format!("    client.mint(&{holder}, &1_000_000);\n"));
    out.push_str("}\n\n");
}

fn gen_ft_allowlist_tests(out: &mut String, _cfg: &CombinationConfig) {
    out.push_str("#[test]\n");
    out.push_str("#[should_panic(expected = \"Error(Contract, #113)\")]\n");
    out.push_str("fn cannot_transfer_before_allow() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    let admin = Address::generate(&e);\n");
    out.push_str("    let manager = Address::generate(&e);\n");
    out.push_str("    let user = Address::generate(&e);\n");
    out.push_str("    let client = create_client(&e, &admin, &manager, 1000);\n");
    out.push_str("    e.mock_all_auths();\n");
    out.push_str("    client.transfer(&admin, &user, &100);\n");
    out.push_str("}\n\n");

    out.push_str("#[test]\n");
    out.push_str("fn allow_then_transfer() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    let admin = Address::generate(&e);\n");
    out.push_str("    let manager = Address::generate(&e);\n");
    out.push_str("    let user = Address::generate(&e);\n");
    out.push_str("    let client = create_client(&e, &admin, &manager, 1000);\n");
    out.push_str("    e.mock_all_auths();\n");
    out.push_str("    client.allow_user(&user, &manager);\n");
    out.push_str("    client.transfer(&admin, &user, &100);\n");
    out.push_str("    assert_eq!(client.balance(&user), 100);\n");
    out.push_str("}\n\n");
}

fn gen_ft_blocklist_tests(out: &mut String, _cfg: &CombinationConfig) {
    out.push_str("#[test]\n");
    out.push_str("#[should_panic]\n");
    out.push_str("fn blocked_user_cannot_transfer() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    let admin = Address::generate(&e);\n");
    out.push_str("    let manager = Address::generate(&e);\n");
    out.push_str("    let user = Address::generate(&e);\n");
    out.push_str("    let client = create_client(&e, &admin, &manager, 1000);\n");
    out.push_str("    e.mock_all_auths();\n");
    out.push_str("    // Transfer some tokens to user first\n");
    out.push_str("    client.transfer(&admin, &user, &500);\n");
    out.push_str("    // Block the user\n");
    out.push_str("    client.block_user(&user, &manager);\n");
    out.push_str("    // This should fail\n");
    out.push_str("    client.transfer(&user, &admin, &100);\n");
    out.push_str("}\n\n");
}

// NFT tests

fn gen_nft_test(cfg: &CombinationConfig, nt: NftType) -> String {
    let mut out = String::new();

    out.push_str("extern crate std;\n\n");
    out.push_str("use soroban_sdk::{testutils::Address as _, Address, Env, String};\n\n");
    out.push_str("use crate::contract::{Contract, ContractClient};\n\n");

    // NftType::Votes uses Base-style minting (sequential_mint) from the test perspective
    let test_nt = if nt == NftType::Votes { NftType::Base } else { nt };

    gen_nft_create_client(&mut out, cfg, test_nt);
    gen_nft_mint_test(&mut out, cfg, test_nt);
    gen_nft_transfer_test(&mut out, cfg, test_nt);
    gen_nft_burn_test(&mut out, cfg, test_nt);

    if cfg.has(Feature::Pausable) {
        gen_nft_pausable_tests(&mut out, cfg, test_nt);
    }

    if cfg.has(Feature::Royalties) {
        gen_nft_royalties_test(&mut out, cfg, test_nt);
    }

    out
}

fn gen_nft_create_client(out: &mut String, cfg: &CombinationConfig, _nt: NftType) {
    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("fn create_client<'a>(e: &Env, owner: &Address) -> ContractClient<'a> {\n");
            out.push_str("    let uri = String::from_str(e, \"https://example.com/\");\n");
            out.push_str("    let name = String::from_str(e, \"Test NFT\");\n");
            out.push_str("    let symbol = String::from_str(e, \"TNFT\");\n");
            out.push_str("    let address = e.register(Contract, (uri, name, symbol, owner));\n");
            out.push_str("    ContractClient::new(e, &address)\n");
            out.push_str("}\n\n");
        }
        AccessModel::AccessControl => {
            out.push_str("fn create_client<'a>(e: &Env, admin: &Address, manager: &Address) -> ContractClient<'a> {\n");
            out.push_str("    let uri = String::from_str(e, \"https://example.com/\");\n");
            out.push_str("    let name = String::from_str(e, \"Test NFT\");\n");
            out.push_str("    let symbol = String::from_str(e, \"TNFT\");\n");
            if cfg.has(Feature::Royalties) {
                out.push_str("    let address = e.register(Contract, (uri, name, symbol, admin, manager, 1000u32));\n");
            } else {
                out.push_str("    let address = e.register(Contract, (uri, name, symbol, admin, manager));\n");
            }
            out.push_str("    ContractClient::new(e, &address)\n");
            out.push_str("}\n\n");
        }
    }
}

fn gen_nft_mint_test(out: &mut String, cfg: &CombinationConfig, nt: NftType) {
    out.push_str("#[test]\n");
    out.push_str("fn mint_works() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    e.mock_all_auths();\n");

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("    let owner = Address::generate(&e);\n");
            out.push_str("    let user = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &owner);\n");
            match nt {
                NftType::Consecutive => {
                    out.push_str("    let last_id = client.batch_mint(&user, &3);\n");
                    out.push_str("    assert_eq!(client.balance(&user), 3);\n");
                    out.push_str("    assert_eq!(last_id, 2);\n");
                }
                _ => {
                    out.push_str("    let token_id = client.mint(&user);\n");
                    out.push_str("    assert_eq!(client.balance(&user), 1);\n");
                    out.push_str("    assert_eq!(client.owner_of(&token_id), user);\n");
                }
            }
        }
        AccessModel::AccessControl => {
            out.push_str("    let admin = Address::generate(&e);\n");
            out.push_str("    let manager = Address::generate(&e);\n");
            out.push_str("    let user = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &admin, &manager);\n");
            match nt {
                NftType::Consecutive => {
                    out.push_str("    let last_id = client.batch_mint(&user, &3);\n");
                    out.push_str("    assert_eq!(client.balance(&user), 3);\n");
                    out.push_str("    assert_eq!(last_id, 2);\n");
                }
                _ => {
                    out.push_str("    let token_id = client.mint(&user);\n");
                    out.push_str("    assert_eq!(client.balance(&user), 1);\n");
                    out.push_str("    assert_eq!(client.owner_of(&token_id), user);\n");
                }
            }
        }
    }

    out.push_str("}\n\n");
}

fn gen_nft_transfer_test(out: &mut String, cfg: &CombinationConfig, nt: NftType) {
    out.push_str("#[test]\n");
    out.push_str("fn transfer_works() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    e.mock_all_auths();\n");

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("    let owner = Address::generate(&e);\n");
            out.push_str("    let user1 = Address::generate(&e);\n");
            out.push_str("    let user2 = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &owner);\n");
            match nt {
                NftType::Consecutive => {
                    out.push_str("    client.batch_mint(&user1, &1);\n");
                    out.push_str("    client.transfer(&user1, &user2, &0);\n");
                }
                _ => {
                    out.push_str("    let token_id = client.mint(&user1);\n");
                    out.push_str("    client.transfer(&user1, &user2, &token_id);\n");
                }
            }
        }
        AccessModel::AccessControl => {
            out.push_str("    let admin = Address::generate(&e);\n");
            out.push_str("    let manager = Address::generate(&e);\n");
            out.push_str("    let user1 = Address::generate(&e);\n");
            out.push_str("    let user2 = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &admin, &manager);\n");
            match nt {
                NftType::Consecutive => {
                    out.push_str("    client.batch_mint(&user1, &1);\n");
                    out.push_str("    client.transfer(&user1, &user2, &0);\n");
                }
                _ => {
                    out.push_str("    let token_id = client.mint(&user1);\n");
                    out.push_str("    client.transfer(&user1, &user2, &token_id);\n");
                }
            }
        }
    }

    out.push_str("    assert_eq!(client.balance(&user1), 0);\n");
    out.push_str("    assert_eq!(client.balance(&user2), 1);\n");
    out.push_str("}\n\n");
}

fn gen_nft_burn_test(out: &mut String, cfg: &CombinationConfig, nt: NftType) {
    out.push_str("#[test]\n");
    out.push_str("fn burn_works() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    e.mock_all_auths();\n");

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("    let owner = Address::generate(&e);\n");
            out.push_str("    let user = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &owner);\n");
            match nt {
                NftType::Consecutive => {
                    out.push_str("    client.batch_mint(&user, &1);\n");
                    out.push_str("    client.burn(&user, &0);\n");
                }
                _ => {
                    out.push_str("    let token_id = client.mint(&user);\n");
                    out.push_str("    client.burn(&user, &token_id);\n");
                }
            }
        }
        AccessModel::AccessControl => {
            out.push_str("    let admin = Address::generate(&e);\n");
            out.push_str("    let manager = Address::generate(&e);\n");
            out.push_str("    let user = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &admin, &manager);\n");
            match nt {
                NftType::Consecutive => {
                    out.push_str("    client.batch_mint(&user, &1);\n");
                    out.push_str("    client.burn(&user, &0);\n");
                }
                _ => {
                    out.push_str("    let token_id = client.mint(&user);\n");
                    out.push_str("    client.burn(&user, &token_id);\n");
                }
            }
        }
    }

    out.push_str("    assert_eq!(client.balance(&user), 0);\n");
    out.push_str("}\n\n");
}

fn gen_nft_pausable_tests(out: &mut String, cfg: &CombinationConfig, nt: NftType) {
    let pauser = match cfg.access {
        AccessModel::Ownable => "owner",
        AccessModel::AccessControl => "admin",
    };

    out.push_str("#[test]\n");
    out.push_str("#[should_panic(expected = \"Error(Contract, #1000)\")]\n");
    out.push_str("fn transfer_fails_when_paused() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    e.mock_all_auths();\n");

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("    let owner = Address::generate(&e);\n");
            out.push_str("    let user1 = Address::generate(&e);\n");
            out.push_str("    let user2 = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &owner);\n");
        }
        AccessModel::AccessControl => {
            out.push_str("    let admin = Address::generate(&e);\n");
            out.push_str("    let manager = Address::generate(&e);\n");
            out.push_str("    let user1 = Address::generate(&e);\n");
            out.push_str("    let user2 = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &admin, &manager);\n");
        }
    }

    match nt {
        NftType::Consecutive => {
            out.push_str("    client.batch_mint(&user1, &1);\n");
            out.push_str(&format!("    client.pause(&{pauser});\n"));
            out.push_str("    client.transfer(&user1, &user2, &0);\n");
        }
        _ => {
            out.push_str("    let token_id = client.mint(&user1);\n");
            out.push_str(&format!("    client.pause(&{pauser});\n"));
            out.push_str("    client.transfer(&user1, &user2, &token_id);\n");
        }
    }

    out.push_str("}\n\n");

    out.push_str("#[test]\n");
    out.push_str("#[should_panic(expected = \"Error(Contract, #1000)\")]\n");
    out.push_str("fn mint_fails_when_paused() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    e.mock_all_auths();\n");

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("    let owner = Address::generate(&e);\n");
            out.push_str("    let user = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &owner);\n");
        }
        AccessModel::AccessControl => {
            out.push_str("    let admin = Address::generate(&e);\n");
            out.push_str("    let manager = Address::generate(&e);\n");
            out.push_str("    let user = Address::generate(&e);\n");
            out.push_str("    let client = create_client(&e, &admin, &manager);\n");
        }
    }

    out.push_str(&format!("    client.pause(&{pauser});\n"));
    match nt {
        NftType::Consecutive => {
            out.push_str("    client.batch_mint(&user, &1);\n");
        }
        _ => {
            out.push_str("    client.mint(&user);\n");
        }
    }

    out.push_str("}\n\n");
}

fn gen_nft_royalties_test(out: &mut String, _cfg: &CombinationConfig, nt: NftType) {
    out.push_str("#[test]\n");
    out.push_str("fn royalty_info_works() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    e.mock_all_auths();\n");
    out.push_str("    let admin = Address::generate(&e);\n");
    out.push_str("    let manager = Address::generate(&e);\n");
    out.push_str("    let user = Address::generate(&e);\n");
    out.push_str("    let client = create_client(&e, &admin, &manager);\n");

    match nt {
        NftType::Consecutive => {
            out.push_str("    client.batch_mint(&user, &1);\n");
        }
        _ => {
            out.push_str("    client.mint(&user);\n");
        }
    }

    // Default royalty is 1000 bps (10%), check royalty_info for token 0 with sale price 10000
    out.push_str("    let (receiver, amount) = client.royalty_info(&0, &10000);\n");
    out.push_str("    assert_eq!(receiver, admin);\n");
    out.push_str("    assert_eq!(amount, 1000); // 10% of 10000\n");
    out.push_str("}\n\n");
}

fn gen_vault_test() -> String {
    let mut out = String::new();

    out.push_str("extern crate std;\n\n");
    out.push_str("use soroban_sdk::{\n");
    out.push_str("    contract, contractimpl, testutils::Address as _, Address, Env, MuxedAddress, String,\n");
    out.push_str("};\n");
    out.push_str("use stellar_tokens::fungible::{Base, FungibleToken};\n\n");
    out.push_str("use crate::contract::{Contract, ContractClient};\n\n");

    // MockAssetContract
    out.push_str("#[contract]\n");
    out.push_str("pub struct MockAssetContract;\n\n");
    out.push_str("#[contractimpl]\n");
    out.push_str("impl MockAssetContract {\n");
    out.push_str("    pub fn __constructor(e: &Env, initial_supply: i128, admin: Address) {\n");
    out.push_str("        Base::set_metadata(e, 18, String::from_str(e, \"Mock Asset\"), String::from_str(e, \"MAT\"));\n");
    out.push_str("        Base::mint(e, &admin, initial_supply);\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("#[contractimpl(contracttrait)]\n");
    out.push_str("impl FungibleToken for MockAssetContract {\n");
    out.push_str("    type ContractType = stellar_tokens::fungible::Base;\n");
    out.push_str("}\n\n");

    // Helpers
    out.push_str("fn create_vault_client<'a>(e: &Env, asset_address: &Address, owner: &Address) -> ContractClient<'a> {\n");
    out.push_str("    let name = String::from_str(e, \"Vault Token\");\n");
    out.push_str("    let symbol = String::from_str(e, \"VLT\");\n");
    out.push_str("    let address = e.register(Contract, (name, symbol, asset_address, 0u32, owner));\n");
    out.push_str("    ContractClient::new(e, &address)\n");
    out.push_str("}\n\n");

    out.push_str("fn create_asset_client<'a>(e: &Env, initial_supply: i128, admin: &Address) -> MockAssetContractClient<'a> {\n");
    out.push_str("    let address = e.register(MockAssetContract, (initial_supply, admin));\n");
    out.push_str("    MockAssetContractClient::new(e, &address)\n");
    out.push_str("}\n\n");

    // Tests
    out.push_str("#[test]\n");
    out.push_str("fn vault_deposit_works() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    e.mock_all_auths();\n");
    out.push_str("    let admin = Address::generate(&e);\n");
    out.push_str("    let owner = Address::generate(&e);\n");
    out.push_str("    let user = Address::generate(&e);\n");
    out.push_str("    let asset_client = create_asset_client(&e, 1_000_000, &admin);\n");
    out.push_str("    let vault_client = create_vault_client(&e, &asset_client.address, &owner);\n");
    out.push_str("    asset_client.transfer(&admin, &user, &100_000);\n");
    out.push_str("    let shares = vault_client.deposit(&100_000, &user, &user, &user);\n");
    out.push_str("    assert!(shares > 0);\n");
    out.push_str("    assert_eq!(vault_client.total_assets(), 100_000);\n");
    out.push_str("}\n\n");

    out.push_str("#[test]\n");
    out.push_str("fn vault_withdraw_works() {\n");
    out.push_str("    let e = Env::default();\n");
    out.push_str("    e.mock_all_auths();\n");
    out.push_str("    let admin = Address::generate(&e);\n");
    out.push_str("    let owner = Address::generate(&e);\n");
    out.push_str("    let user = Address::generate(&e);\n");
    out.push_str("    let asset_client = create_asset_client(&e, 1_000_000, &admin);\n");
    out.push_str("    let vault_client = create_vault_client(&e, &asset_client.address, &owner);\n");
    out.push_str("    asset_client.transfer(&admin, &user, &100_000);\n");
    out.push_str("    vault_client.deposit(&100_000, &user, &user, &user);\n");
    out.push_str("    vault_client.withdraw(&50_000, &user, &user, &user);\n");
    out.push_str("    assert_eq!(asset_client.balance(&user), 50_000);\n");
    out.push_str("}\n\n");

    out
}
