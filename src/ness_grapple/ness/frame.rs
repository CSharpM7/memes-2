use crate::imports::imports_status::*;
use crate::ness_grapple::imports::*;

unsafe extern "C" fn ness_frame(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_frame = fighter.global_table[STATUS_FRAME].get_f32();
    let is_lasso = (*FIGHTER_STATUS_KIND_AIR_LASSO..*FIGHTER_STATUS_KIND_AIR_LASSO_LANDING+1).contains(&status);

    if is_lasso && status_frame < 1.0 {
        let status_hex = to_hex(status as u64);
        println!("Status: {status_hex}");
    }
    /*
     FIGHTER_STATUS_KIND_AIR_LASSO: 0xF8,
    FIGHTER_STATUS_KIND_AIR_LASSO_FAILURE: 0xFC,
    FIGHTER_STATUS_KIND_AIR_LASSO_HANG: 0xFA,
    FIGHTER_STATUS_KIND_AIR_LASSO_LANDING: 0xFD,
    FIGHTER_STATUS_KIND_AIR_LASSO_REACH: 0xF9,
    FIGHTER_STATUS_KIND_AIR_LASSO_REWIND: 0xFB,
     */
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main, ness_frame);
}