use crate::config::*;

pub fn gen_extension_impls(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    match cfg.variant {
        ContractVariant::Fungible(ft) => {
            match ft {
                FungibleType::AllowList => {
                    out.push_str("#[contractimpl(contracttrait)]\n");
                    out.push_str("impl FungibleAllowList for Contract {\n");
                    out.push_str("    #[only_role(operator, \"manager\")]\n");
                    out.push_str(
                        "    fn allow_user(e: &Env, user: Address, operator: Address) {\n",
                    );
                    out.push_str("        AllowList::allow_user(e, &user)\n");
                    out.push_str("    }\n\n");
                    out.push_str("    #[only_role(operator, \"manager\")]\n");
                    out.push_str(
                        "    fn disallow_user(e: &Env, user: Address, operator: Address) {\n",
                    );
                    out.push_str("        AllowList::disallow_user(e, &user)\n");
                    out.push_str("    }\n");
                    out.push_str("}\n\n");
                }
                FungibleType::BlockList => {
                    out.push_str("#[contractimpl(contracttrait)]\n");
                    out.push_str("impl FungibleBlockList for Contract {\n");
                    out.push_str("    #[only_role(operator, \"manager\")]\n");
                    out.push_str(
                        "    fn block_user(e: &Env, user: Address, operator: Address) {\n",
                    );
                    out.push_str("        BlockList::block_user(e, &user)\n");
                    out.push_str("    }\n\n");
                    out.push_str("    #[only_role(operator, \"manager\")]\n");
                    out.push_str(
                        "    fn unblock_user(e: &Env, user: Address, operator: Address) {\n",
                    );
                    out.push_str("        BlockList::unblock_user(e, &user)\n");
                    out.push_str("    }\n");
                    out.push_str("}\n\n");
                }
                FungibleType::Votes => {
                    out.push_str("#[contractimpl(contracttrait)]\n");
                    out.push_str("impl Votes for Contract {}\n\n");
                }
                _ => {}
            }
        }
        ContractVariant::NonFungible(nt) => {
            if cfg.has(Feature::Royalties) {
                out.push_str("#[contractimpl(contracttrait)]\n");
                out.push_str("impl NonFungibleRoyalties for Contract {\n");
                out.push_str("    #[only_role(operator, \"manager\")]\n");
                out.push_str("    fn set_default_royalty(e: &Env, receiver: Address, basis_points: u32, operator: Address) {\n");
                out.push_str(
                    "        Base::set_default_royalty(e, &receiver, basis_points);\n",
                );
                out.push_str("    }\n\n");
                out.push_str("    #[only_role(operator, \"manager\")]\n");
                out.push_str("    fn set_token_royalty(e: &Env, token_id: u32, receiver: Address, basis_points: u32, operator: Address) {\n");
                out.push_str(
                    "        Base::set_token_royalty(e, token_id, &receiver, basis_points);\n",
                );
                out.push_str("    }\n\n");
                out.push_str("    #[only_role(operator, \"manager\")]\n");
                out.push_str(
                    "    fn remove_token_royalty(e: &Env, token_id: u32, operator: Address) {\n",
                );
                out.push_str("        Base::remove_token_royalty(e, token_id);\n");
                out.push_str("    }\n");
                out.push_str("}\n\n");
            }
            if nt == NftType::Votes {
                out.push_str("#[contractimpl(contracttrait)]\n");
                out.push_str("impl Votes for Contract {}\n\n");
            }
        }
    }

    out
}
