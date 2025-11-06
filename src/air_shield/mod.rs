mod common;
mod hook;
pub mod vars;
pub mod imports;

pub fn install() {
    common::install();
}

pub fn install_hook() {
    hook::install();
}
