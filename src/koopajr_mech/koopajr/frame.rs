use crate::imports::imports_status::*;
use crate::koopajr_mech::imports::*;


unsafe fn set_clown_visibility(fighter: &mut L2CFighterCommon,state: bool) {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_body"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_closemouth"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_closeeye"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_escape"),state);

    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_eyen"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_eye_talk"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_eye_heavyouch"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_eye_ouch"),state);

    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_mouth_talk"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_mouth_heavyouch"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_mouth_ouch"),state);

    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_openmouth"),state);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("koopajr_clown_slide"),state);
}

unsafe fn on_pilot_active(fighter: &mut L2CFighterCommon) {
    let entry_id = sv_battle_object::entry_id(fighter.battle_object_id) as i32;
    let mut param_attributes: Vec<(u64,u64,f32)> = Vec::new();
    param_attributes.push((hash40("walk_speed_max"),0 as u64, 0.75));

    for p in param_attributes {
        param_config::update_attribute_mul_2(*FIGHTER_KIND_KOOPAJR, vec![entry_id], p);
    }
}
unsafe fn on_pilot_deactive(fighter: &mut L2CFighterCommon) {
    let entry_id = sv_battle_object::entry_id(fighter.battle_object_id) as i32;
    let mut param_attributes: Vec<(u64,u64,f32)> = Vec::new();
    param_attributes.push((hash40("walk_speed_max"),0 as u64, 1.0));

    for p in param_attributes {
        param_config::update_attribute_mul_2(*FIGHTER_KIND_KOOPAJR, vec![entry_id], p);
    }
}

unsafe extern "C" fn koopajr_frame(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut change_state = WorkModule::get_int(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_PILOT_CHANGE_STATE) ;
    if change_state == PILOT_STATE_NONE {
        if [*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK,*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL,*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE].contains(&status) {
            WorkModule::set_int(fighter.module_accessor, PILOT_STATE_ACTIVATE, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_PILOT_CHANGE_STATE);
            change_state = PILOT_STATE_ACTIVATE;
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_PILOT);
        }
    }
    if change_state == PILOT_STATE_ACTIVATE {
        WorkModule::set_int(fighter.module_accessor, PILOT_STATE_NONE, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_PILOT_CHANGE_STATE);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_PILOT);
        on_pilot_active(fighter);
    }
    else if change_state == PILOT_STATE_DEACTIVATE {
        WorkModule::set_int(fighter.module_accessor, PILOT_STATE_NONE, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_PILOT_CHANGE_STATE);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_PILOT);
        on_pilot_deactive(fighter);
    }

    if is_pilot(fighter) {
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("clownrot"), &Vector3f::new(0.01, 0.01, 0.01));
        //set_clown_visibility(fighter,false);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main, koopajr_frame);
}