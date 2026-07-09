pub fn generate_lib() -> String {
    r#"#![no_std]
#![allow(dead_code)]

mod contract;
#[cfg(test)]
mod test;
"#
    .to_string()
}
