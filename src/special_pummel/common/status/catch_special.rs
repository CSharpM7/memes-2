use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;
/*

    FIGHTER_STATUS_CATCH_CUT_WORK_INT_SITUATION: 0x11000005,
    FIGHTER_STATUS_CATCH_DASH_WORK_INT_CATCH_TURN_FRAME: 0x11000005,
    FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT: 0x2100000C,
    FIGHTER_STATUS_CATCH_PULL_FLAG_UNNECESSARY_CLEAR_TRANS: 0x2100000C,
    FIGHTER_STATUS_CATCH_PULL_WORK_INT_MOTION_KIND: 0x11000005,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_IK_LEFT_JOINT_ID: 0x11000007,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_IK_RIGHT_JOINT_ID: 0x11000008,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS: 0x11000006,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND: 0x11000005,
*/

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchAttack)]
unsafe fn status_CatchAttack(fighter: &mut L2CFighterCommon) -> L2CValue {
    return catch_attack_main_inner(fighter);
}

pub unsafe extern "C" fn catch_attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_input = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW);
    let can_special = !WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_FLAG_FORBID_SPECIAL);
    println!("Can special: {can_special}");
    if special_input && can_special {
        println!("Special");
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special"))); //catch_attack
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_FLAG_FORBID_SPECIAL);
        return fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_main_loop as *const () as _));
    }
    else {
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_attack")));
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchAttack_Main as *const () as _));
    }
}
pub unsafe extern "C" fn catch_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    let clatter = ControlModule::get_clatter_time(opponent, 0);
    println!("Clatter: {clatter}");
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_FLAG_ENABLE_CUT) {
        if ControlModule::get_clatter_time(opponent, 0) <= 0.0 {
            println!("Break");
            StatusModule::change_status_request_from_script(opponent, *FIGHTER_STATUS_KIND_CAPTURE_CUT, false);
            return 1.into();
        }
    }
    return fighter.status_CatchAttack_Main();
}


fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_CatchAttack
        );
    }
}
pub fn install() {
    #[cfg(feature = "devhook")]
    skyline::nro::add_hook(nro_hook);
    /*
    Agent::new("fighter")
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_main_inner)
    .install();
    */
}
