use crate::imports::imports_status::*;
pub unsafe extern "C" fn fighter_update(fighter: &mut L2CFighterCommon) {
    let module_accessor= fighter.module_accessor;
    let has_final = WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) &&
    WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ONE_MORE_FINAL_DEAD_FAILED);
    if !has_final {
        WorkModule::unable_transition_term_forbid(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    WorkModule::unable_transition_term_forbid(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
}

pub fn install() {   
    Agent::new("duckhunt")
    .on_line(Main, fighter_update)
    .install();
}