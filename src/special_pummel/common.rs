use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;

// AGENT
pub unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let status_kind_next = StatusModule::status_kind_next(fighter.module_accessor);

    if !(*FIGHTER_STATUS_KIND_CATCH_WAIT..*FIGHTER_STATUS_KIND_CAPTURE_JUMP).contains(&status_kind_next)
    {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_FLAG_FORBID_SPECIAL);
    }
    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

// STATUS
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchAttack)]
unsafe fn status_CatchAttack(fighter: &mut L2CFighterCommon) -> L2CValue {
    return catch_attack_main_inner(fighter);
}

pub unsafe extern "C"  fn catch_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return catch_attack_main_inner(fighter);
}
pub unsafe fn catch_attack_check_special(fighter: &mut L2CFighterCommon) -> bool {
    let special_input = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW);
    let can_special = !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_FLAG_FORBID_SPECIAL);
    let has_anim = MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("catch_special"));
    //println!("Can special: {can_special} Has special: {has_anim}");
    return special_input && can_special && has_anim;
}
pub unsafe extern "C" fn catch_attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special(fighter) {
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_FLAG_FORBID_SPECIAL);
        return fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_main_loop as *const () as _))
    }
    else {
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_attack")));
    }
    
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchAttack_Main as *const () as _))
}

pub unsafe extern "C" fn catch_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    WorkModule::off_flag(opponent,*FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_JUMP);

    let mut clatter = ControlModule::get_clatter_time(opponent, 0);
    let disable_clatter = WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CLATTER);
    if disable_clatter {
        clatter = WorkModule::get_float(fighter.module_accessor,FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
        ControlModule::set_clatter_time(opponent, clatter,0);
    }
    else {
        WorkModule::set_float(fighter.module_accessor, clatter, FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
    }
    return fighter.status_CatchAttack_Main();
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CatchAttack)]
unsafe fn status_end_CatchAttack(fighter: &mut L2CFighterCommon) -> L2CValue {
    catch_attack_end_inner(fighter)
}

pub unsafe fn catch_attack_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let opponent_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
    if (opponent_id != 0 && opponent_id != OBJECT_ID_NULL) {
        let opponent = &mut *(*get_battle_object_from_id(opponent_id)).module_accessor;
        ControlModule::set_clatter_stop(opponent, false);
        println!("Reset clatter");
    }
    return fighter.status_end_CatchWait();
}
fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_CatchAttack,
            status_end_CatchAttack
        );
    }
}

pub fn install() {
    #[cfg(not(feature = "dev"))]
    skyline::nro::add_hook(nro_hook);

    #[cfg(feature = "dev")] {
    Agent::new("fighter")
        .on_start(agent_start)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_main)
    .install();
    }
}

pub fn uninstall() {
}