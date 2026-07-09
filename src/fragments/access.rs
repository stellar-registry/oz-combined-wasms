use crate::config::*;

pub fn gen_access_impl(cfg: &CombinationConfig) -> String {
    let mut out = String::new();

    match cfg.access {
        AccessModel::Ownable => {
            out.push_str("#[contractimpl(contracttrait)]\n");
            out.push_str("impl Ownable for Contract {}\n\n");
        }
        AccessModel::AccessControl => {
            out.push_str("#[contractimpl(contracttrait)]\n");
            out.push_str("impl AccessControl for Contract {}\n\n");
        }
    }

    out
}
