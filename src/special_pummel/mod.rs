/*
Air Smash

Allows players to charge a unique aerial smash attack. This can be done in smashline 1 as well, however my sm1 exploded with this plugin
*/

pub mod vars;
pub mod imports;

mod edge;
mod gaogaen;
mod metaknight;
mod packun;
mod snake;
mod tantan;
mod common;

pub fn install() {
    edge::install();
    gaogaen::install();
    metaknight::install();
    packun::install();
    snake::install();
    tantan::install();
    common::install();
}

pub fn uninstall() {
    common::uninstall();
}
