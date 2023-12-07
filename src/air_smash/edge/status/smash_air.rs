use crate::imports::imports_status::*;
use crate::air_smash::imports::*;

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return status::smash_air::attack_air_main(fighter)
}

unsafe extern "C" fn attack_s4_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return status::smash_air::attack_s4_start_pre(fighter)
}
unsafe extern "C" fn attack_s4_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return status::smash_air::attack_s4_hold_pre(fighter)
}
unsafe extern "C" fn attack_s4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return status::smash_air::attack_s4_pre(fighter)
}

pub unsafe extern "C" fn attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return status::smash_air::attack_s4_start_main(fighter)
}

pub unsafe extern "C" fn attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return status::smash_air::attack_s4_hold_main(fighter)
}

pub unsafe extern "C" fn attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return status::smash_air::attack_s4_main(fighter)
}

pub unsafe extern "C" fn landing_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return status::smash_air::landing_attack_air_main(fighter)
}
pub unsafe extern "C" fn landing_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if !(smash_type>0) {
        return 0.into();
    }
    let mut new_rate = 1.0;
    if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F {
        new_rate = (14.0/20.0);
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B {
        new_rate = (14.0/20.0);
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI {
        new_rate = (21.0/20.0);
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW {
        new_rate = (26.0/20.0);
    }
    MotionModule::set_rate(fighter.module_accessor, new_rate);
    return 0.into();
}

pub fn install() {   
    Agent::new("edge")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main)

        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attack_s4_start_pre)
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_pre)
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_pre)

        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attack_s4_start_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_main)

        //.status(Main, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, landing_attack_air_main)
        .status(Exec, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, landing_attack_air_exec)
        .install();
}