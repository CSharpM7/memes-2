use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;

/*
STATUS
*/
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("HM?");
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        ControlModule::clear_command(fighter.module_accessor, true);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CHANGE_PUNCH_R);

        return fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_loop_uniq as *const () as _));
    }
    return catch_attack_main_default(fighter);
}
pub unsafe extern "C" fn catch_attack_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
    if status_frame > 5 {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return 1.into();
    }
    return fighter.status_CatchAttack_Main();
}

pub fn install() {
    smashline::Agent::new("tantan")
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
    .install();
}
