use crate::config::*;

pub fn gen_pausable_impl(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    let guard = match cfg.access {
        AccessModel::Ownable => "only_owner",
        AccessModel::AccessControl => "only_admin",
    };

    out.push_str("#[contractimpl]\n");
    out.push_str("#[allow(unused_variables)]\n");
    out.push_str("impl Pausable for Contract {\n");

    out.push_str("    fn paused(e: &Env) -> bool {\n");
    out.push_str("        pausable::paused(e)\n");
    out.push_str("    }\n\n");

    out.push_str(&format!("    #[{guard}]\n"));
    out.push_str("    fn pause(e: &Env, caller: Address) {\n");
    out.push_str("        pausable::pause(e);\n");
    out.push_str("    }\n\n");

    out.push_str(&format!("    #[{guard}]\n"));
    out.push_str("    fn unpause(e: &Env, caller: Address) {\n");
    out.push_str("        pausable::unpause(e);\n");
    out.push_str("    }\n");

    out.push_str("}\n\n");

    out
}
