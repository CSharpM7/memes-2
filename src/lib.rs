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
mod ike_air;

mod installer;
mod imports;
use crate::imports::imports_agent::*;

#[skyline::main(name = "smashline2_memes")]
pub fn main() {
    #[cfg(feature = "devhook")]
    return;

    #[cfg(not(feature = "dev"))]
    installer::install();
    #[cfg(feature = "dev")]
    installer::smashline_install();
}
