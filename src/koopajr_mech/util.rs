use crate::imports::imports_agent::*;
use super::vars::*;

pub unsafe extern "C" fn is_pilot(fighter: &mut smashline::L2CFighterCommon) -> bool {
    return WorkModule::is_flag(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_PILOT);
}