use crate::imports::imports_agent::*;
use crate::buddy_fly::imports::*;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
	let status_exited = StatusModule::status_kind(fighter.module_accessor);
	let status_next = StatusModule::status_kind_next(fighter.module_accessor);
    let is_landing_status = [*FIGHTER_STATUS_KIND_LANDING,*FIGHTER_STATUS_KIND_LANDING_LIGHT].contains(&status_next)
    || status_exited == *FIGHTER_STATUS_KIND_ATTACK_AIR;

    if (
        (&[*SITUATION_KIND_CLIFF]).contains(&StatusModule::situation_kind(fighter.module_accessor))
        //&& !is_landing_status
    )
    || is_damage_status(fighter.module_accessor)
    {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
    }

    
    return true.into();
}


unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
	fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}