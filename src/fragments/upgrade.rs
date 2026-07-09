use crate::config::*;

pub fn gen_upgradeable_impl(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    let guard = match cfg.access {
        AccessModel::Ownable => "only_owner",
        AccessModel::AccessControl => "only_admin",
    };

    out.push_str("#[contractimpl]\n");
    out.push_str("#[allow(unused_variables)]\n");
    out.push_str("impl Upgradeable for Contract {\n");
    out.push_str(&format!("    #[{guard}]\n"));
    out.push_str(
        "    fn upgrade(e: &Env, new_wasm_hash: BytesN<32>, operator: Address) {\n",
    );
    out.push_str("        upgradeable::upgrade(e, &new_wasm_hash);\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    out
}
