use crate::imports::imports_status::*;
use crate::ledge_special::imports::*;

pub unsafe extern "C" fn cliff_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CliffAttack();
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
        println!("Special");
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("cliff_special_quick"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("cliff_attack_quick"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(cliff_attack_main_loop as *const () as _))
}
pub unsafe extern "C" fn cliff_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_CliffAttack_Main();
}


pub fn install() {   
    Agent::new("fighter")
        .status(Main, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, cliff_attack_main)
    .install();
}