#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused,
    static_mut_refs
)]
#![deny(
    deprecated
)]

use smash::{
    lib::{
        L2CValue,
        LuaConst,
        lua_const::*
    },
    hash40,
    lua2cpp::*,
    phx::*
};
use smashline::*;

//mod air_smash;
//mod marth_fireball;
//mod arm_reflect;
//mod rob_spin;
//mod ike_air;
//mod nana_rocket;
//mod ledge_special;
//mod snake_landmine;
//mod temp;
//mod metroid_crawl;
//mod hero_skill;
//mod burst;
//mod pikmin_charge;
mod duckhunt;

mod imports;
use crate::imports::imports_agent::*;

#[no_mangle]
pub fn smashline_install() {
    install();
}
#[no_mangle]
pub fn smashline_uninstall() {
    uninstall();
}

pub fn install() {
    println!("Loading memes");
    crate::duckhunt::install();
}
pub fn uninstall() {
    println!("Uninstalling...");
}

#[skyline::main(name = "smashline2_memes")]
pub fn main() {
    #[cfg(feature = "devhook")]
    println!("Devhook Loading memes");

    crate::duckhunt::install();

    #[cfg(not(feature = "dev"))]
    install();
    #[cfg(feature = "dev")]
    smashline_install();
}
