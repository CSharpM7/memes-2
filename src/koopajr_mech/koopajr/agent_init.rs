use crate::imports::imports_agent::*;
use crate::koopajr_mech::imports::*;

unsafe extern "C" fn on_rebirth(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, PILOT_STATE_DEACTIVATE, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_PILOT_CHANGE_STATE);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_PILOT);
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    on_rebirth(fighter);
}

unsafe extern "C" fn entry_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    on_rebirth(fighter);
    fighter.status_pre_Entry()
}
unsafe extern "C" fn rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    on_rebirth(fighter);
    fighter.status_pre_Rebirth()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
    agent.status(Pre, *FIGHTER_STATUS_KIND_ENTRY, entry_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_pre);
}