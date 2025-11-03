use crate::imports::imports_agent::*;
use crate::ness_grapple::imports::*;

pub unsafe extern "C" fn lasso_fail_sub_status(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    return 0.into();
}
pub unsafe extern "C" fn lasso_fail_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    fighter.status_AirLassoFailure(L2CValue::Ptr( lasso_fail_sub_status as *const () as _),
        hash40("air_catch").into(), WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REWIND.into());
    return 0.into();
}

//NOT NEEDED
pub unsafe extern "C" fn common_airlassohang(fighter: &mut smashline::L2CFighterCommon, param_1: L2CValue, param_3: L2CValue) {
    WorkModule::set_int64(fighter.module_accessor, param_1.get_i64(), *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_ARTICLE_TIP_NODE_ID);
    WorkModule::set_int64(fighter.module_accessor, param_3.get_i64(), *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_CHECK_CLIFF_JOINT_ID);

    let hang_dir = GroundModule::hang_cliff_dir(fighter.module_accessor);
    PostureModule::set_lr(fighter.module_accessor, hang_dir);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    println!("Hang Dir: {hang_dir}");

    let body_angle_intp = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_data"), hash40("body_angle_intp"));
    WorkModule::set_int(fighter.module_accessor, body_angle_intp, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_ANGLE_INTP);
    WorkModule::set_int(fighter.module_accessor, body_angle_intp, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_ANGLE_INTP_MAX);

    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_COUNT);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_air_lasso_hang_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_air_lasso_hang_uniq as *const () as _));

    GroundModule::set_test_coll_stop_status(fighter.module_accessor, true);
    let speed_x = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
    };
    let speed_y = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
    };
    //+0.0?
    WorkModule::set_float(fighter.module_accessor, speed_x, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_FLOAT_SPEED_X);
    WorkModule::set_float(fighter.module_accessor, speed_y, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_FLOAT_SPEED_Y);

    //Fuck a bunch of math...
    let mut rot_pos = VECTOR_ZERO;
    let joint_offset = ModelModule::joint_global_position(fighter.module_accessor, Hash40::new_raw(param_3.get_u64()), &mut rot_pos,false); 
    WorkModule::set_float(fighter.module_accessor, rot_pos.x, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_FLOAT_ROT_X_PREV);

    let mut hip_pos = VECTOR_ZERO;
    let joint_offset = ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("hip"), &mut hip_pos,false); 
    WorkModule::set_float(fighter.module_accessor, hip_pos.x, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_FLOAT_HIP_X_PREV);

    //set hang data
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PFUSHIGISOU_CLIFF_HANG_DATA_AIR_LASSO_HANG as u32);

    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_DOWN);
    GroundModule::set_rhombus_modify_air_lasso(fighter.module_accessor, true);

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_air_lasso_hang_main as *const () as _));
}
////////////////

pub unsafe extern "C" fn lasso_hang_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_HANG_FLAG_NOT_ARM);
    if PostureModule::lr(fighter.module_accessor) < 0.0 {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_BODY_FLIP_Y, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_FLIP);
    }
    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_cliff_catch"), true, false, false, false, enSEType(0));

    fighter.status_AirLassoHang(hash40("vine6").into(), FIGHTER_CLIFF_HANG_DATA_DEFAULT.into(), hash40("rot").into());
    fighter.set_cliff_hangdata(10.0,19.0,1.0,-1.0);
    //common_airlassohang(fighter,hash40("vine6").into(),hash40("rot").into());
    return 0.into();
}
pub unsafe extern "C" fn lasso_landing_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    fighter.status_AirLassoLanding(WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_EXHAUST.into());
    return 0.into();
}
pub unsafe extern "C" fn lasso_reach_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    fighter.status_AirLassoReach(WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REACH.into());
    return 0.into();
}
pub unsafe extern "C" fn lasso_reach_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    WorkModule::set_int(fighter.module_accessor, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REACH, *FIGHTER_STATUS_AIR_LASSO_REACH_WORK_INT_ARTICLE_STATUS);
    fighter.status_end_AirLassoReach();
    return 0.into();
}
pub unsafe extern "C" fn lasso_rewind_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    fighter.status_AirLassoRewind(WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_HANG_REWIND.into());
    return 0.into();
}

pub fn install(agent: &mut smashline::Agent) {
	//agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_FAILURE, lasso_fail_main);
	agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, lasso_hang_main);
	agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING, lasso_landing_main);
	agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, lasso_reach_main);
	agent.status(End, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, lasso_reach_end);
	agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND, lasso_rewind_main);
}