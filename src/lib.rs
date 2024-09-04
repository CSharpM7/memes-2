#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
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
mod ledge_special;
//mod temp;

mod imports;
use crate::imports::imports_agent::*;

#[no_mangle]
pub fn smashline_install() {
    install();
}

pub fn install() {
    println!("Loading memes");
    crate::ledge_special::install();
}

#[skyline::main(name = "smashline2_memes")]
pub fn main() {
    #[cfg(not(feature = "dev"))]
    install();
    #[cfg(feature = "dev")]
    smashline_install();
}
