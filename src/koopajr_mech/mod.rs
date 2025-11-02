/*
Bowser Shell Shield (like yoshi)
*/
pub mod vars;
pub mod imports;
pub mod util;
mod koopajr;

pub fn install() {
    koopajr::install();
}
