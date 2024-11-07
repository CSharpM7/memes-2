use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;

pub unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let status_kind_next = StatusModule::status_kind_next(fighter.module_accessor);

    if !(*FIGHTER_STATUS_KIND_CATCH_WAIT..*FIGHTER_STATUS_KIND_CAPTURE_JUMP).contains(&status_kind_next)
    {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_FLAG_FORBID_SPECIAL);
    }
    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

pub fn install() {
    Agent::new("fighter")
        .on_start(agent_start)
        .install();
}