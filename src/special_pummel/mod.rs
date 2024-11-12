/*
Air Smash

Allows players to charge a unique aerial smash attack. This can be done in smashline 1 as well, however my sm1 exploded with this plugin
*/

pub mod vars;
pub mod imports;

mod edge;
mod gaogaen;
mod koopa;
mod metaknight;
mod packun;
mod pfushigisou;
mod pikmin;
mod reflet;
mod rosetta;
mod snake;
mod tantan;
mod common;

pub fn install() {
    edge::install();
    gaogaen::install();
    koopa::install();
    metaknight::install();
    packun::install();
    pfushigisou::install();
    pikmin::install();
    reflet::install();
    rosetta::install();
    snake::install();
    tantan::install();
    common::install();
}

pub fn uninstall() {
    common::uninstall();
}
