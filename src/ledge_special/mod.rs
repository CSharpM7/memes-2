/*
Getup Specials
*/

pub mod vars;
pub mod imports;

mod samusd;
mod common;

pub fn install() {
    samusd::install();
    common::install();
}
