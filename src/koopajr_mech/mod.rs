/*
Bowser Jr Pilot (Up B makes Jr walk on the ground)
*/
pub mod vars;
pub mod imports;
pub mod util;
mod koopajr;

pub fn install() {
    koopajr::install();
}
