use crate::imports::imports_agent::*;
use crate::ness_grapple::imports::*;

unsafe extern "C" fn effect_hang(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT_ATTR(agent, Hash40::new("sys_cliff_catch"), Hash40::new("vine5"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
    }
}

pub unsafe extern "C" fn lasso_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_LASSO_STATUS_WORK_KEEP_FLAG_WAIT_FLAG,
        *WEAPON_LASSO_STATUS_WORK_KEEP_FLAG_WAIT_INT,
        *WEAPON_LASSO_STATUS_WORK_KEEP_FLAG_WAIT_FLOAT,
        0
    );
    0.into()
}

pub unsafe extern "C" fn reach_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    VisibilityModule::set_whole(weapon.module_accessor, true);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 0.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(empty_status as *const () as _))
}

unsafe extern "C" fn reach_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn rewind_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    WorkModule::on_flag(weapon.module_accessor, *WEAPON_LASSO_STATUS_HANG_REWIND_WORK_ID_FLAG_END_HIDE);
    weapon.fastshift(L2CValue::Ptr(empty_status as *const () as _))
}

unsafe extern "C" fn hang_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let mut tip_angle_hang = 70.0;
    if PostureModule::lr(weapon.module_accessor) < 0.0 {
        tip_angle_hang += 180.0;
    }
    //WorkModule::set_float(weapon.module_accessor, tip_angle_hang.to_radians(), *WEAPON_LASSO_INSTANCE_WORK_ID_FLOAT_REACH_ANGLE);
    weapon.fastshift(L2CValue::Ptr(empty_status as *const () as _))
}

pub fn install() {
    let agent = &mut smashline::Agent::new("pfushigisou_vine");
    agent.acmd("effect_hang", effect_hang, Priority::Default);
	agent.status(Pre, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REACH, lasso_pre);
	agent.status(Main, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REACH, reach_main);
	agent.status(End, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REACH, empty_status);
    
	agent.status(Pre, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REWIND, lasso_pre);
	agent.status(Main, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REWIND, rewind_main);
	agent.status(End, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_REWIND, empty_status);
    
	agent.status(Pre, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_HANG, lasso_pre);
	agent.status(Main, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_HANG, hang_main);
	agent.status(End, *WEAPON_PFUSHIGISOU_VINE_STATUS_KIND_HANG, empty_status);

    agent.install();
}