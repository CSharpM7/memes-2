/*
Air Smash

Allows players to charge a unique aerial smash attack. This can be done in smashline 1 as well, however my sm1 exploded with this plugin
*/

pub mod vars;
pub mod util;
pub mod imports;

mod marth;

pub fn install() {
    marth::install();
}
