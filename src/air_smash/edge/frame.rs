use crate::imports::imports_agent::*;

pub unsafe extern "C" fn check_select(boma: *mut BattleObjectModuleAccessor, status: i32) {
}

pub unsafe extern "C" fn edge_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
}

pub fn install() {
    Agent::new("edge")
        .on_line(Main, edge_update)
        .install();
}