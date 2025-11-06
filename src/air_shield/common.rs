use crate::imports::imports_agent::*;
use super::imports::*;

unsafe extern "C" fn check_air_escape_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) 
    {
        if ![*FIGHTER_STATUS_KIND_GUARD_ON,*FIGHTER_STATUS_KIND_GUARD,*FIGHTER_STATUS_KIND_GUARD_DAMAGE,*FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status) {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    //fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].assign(&L2CValue::Ptr(check_air_escape_uniq as *const () as _));
}

unsafe extern "C" fn escape_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("NOPE");
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON);
    0.into()
}

pub fn install() {
    let common = &mut Agent::new("fighter");
    common.on_start(agent_init);
    //common.status(Pre,*FIGHTER_STATUS_KIND_ESCAPE_AIR,escape_air_pre);
    common.install();
}