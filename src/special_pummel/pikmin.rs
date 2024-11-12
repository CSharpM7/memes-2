use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;

/*
STATUS
*/
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    return catch_attack_main_new(fighter,true);
}

pub fn install() {
    smashline::Agent::new("pikmin")
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
    .install();
}
