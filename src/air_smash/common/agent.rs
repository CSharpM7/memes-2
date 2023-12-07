use crate::imports::imports_agent::*;
use crate::air_smash::imports::*;

pub unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let next_status_kind = StatusModule::status_kind_next(fighter.module_accessor);

    if !(&[
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_S4]).contains(&next_status_kind)
    {
        VarModule::set_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE,0);
    }

    //Reset variables
    if (&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]).contains(&next_status_kind) || 
        !sv_information::is_ready_go() || lua_bind::FighterManager::is_result_mode(singletons::FighterManager())
    {
        VarModule::set_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE,0);
    }
    true.into()
}
/* 
pub unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    agent_start(fighter);
}
pub unsafe extern "C" fn agent_reset(fighter: &mut L2CFighterCommon) {
    agent_start(fighter);
}

pub fn install() {
    Agent::new("common")
        .on_init(agent_init)
        .on_start(agent_reset)
        .install();
}*/