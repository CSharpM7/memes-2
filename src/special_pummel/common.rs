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

pub unsafe extern "C" fn catch_attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_input = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW);
    let can_special = !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_FLAG_FORBID_SPECIAL);
    let has_anim = MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("catch_special"));
    println!("Can special: {can_special} Has special: {has_anim}");
    if special_input && can_special && has_anim {
        println!("Special");
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_FLAG_FORBID_SPECIAL);
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
    
    Agent::new("fighter")
        .on_start(agent_start)
    .install();
}
