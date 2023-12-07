use crate::imports::imports_agent::*;
use crate::air_smash::imports::*;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let next_status_kind = StatusModule::status_kind_next(fighter.module_accessor);

    agent::change_status_callback(fighter);

    true.into()
}

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    GetVarManager::reset_var_module(fighter.battle_object,false);
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}
pub unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    agent_start(fighter);
}
pub unsafe extern "C" fn agent_reset(fighter: &mut L2CFighterCommon) {
    agent_start(fighter);
}

pub fn install() {
    Agent::new("edge")
        .on_init(agent_init)
        .on_start(agent_reset)
        .install();
}