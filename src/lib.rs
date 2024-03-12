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

mod air_smash;
mod marth_fireball;
mod arm_reflect;

mod installer;
mod imports;
use crate::imports::imports_agent::*;

#[skyline::main(name = "smashline2_memes")]
pub fn main() {
    println!("[smashline2_memes::main] Loading");
    installer::install();
    println!("[smashline2_memes::main] Loaded!");
}
