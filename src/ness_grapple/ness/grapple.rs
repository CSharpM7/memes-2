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

pub unsafe extern "C" fn reach_init(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    if StatusModule::status_kind(owner) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        let owner_pos = *PostureModule::pos(owner);
        PostureModule::set_pos(weapon.module_accessor, &owner_pos);
    
    }
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
    println!("REwind start");
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

pub unsafe extern "C" fn exhaust_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    0.into()
}

unsafe extern "C" fn exhaust_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_FALL);
    MotionModule::set_rate(weapon.module_accessor, 0.0);
    weapon.fastshift(L2CValue::Ptr(exhaust_main_loop as *const () as _))
}
unsafe extern "C" fn exhaust_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn head_lasso_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);

    /*
    LinkModule::remove_model_constraint(weapon.module_accessor,true);
    let mut has_link = LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if has_link {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }

    let yoyo = get_article_boma(owner, GENERATE_ARTICLE_GRAPPLE);
    let yoyo_id = (*yoyo).battle_object_id;
    LinkModule::link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, yoyo_id);


    let parent_bone = Hash40::new("yoyo17");
    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),parent_bone,
    (*CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_ORIENTATION
         | *CONSTRAINT_FLAG_POSITION
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
 */
    weapon.fastshift(L2CValue::Ptr(empty_status as *const () as _))
}

pub fn install() {
    println!("DUDE");
    let agent = &mut smashline::Agent::new("ness_grapple");
    //agent.acmd("effect_hang", effect_hang, Priority::Default);
	agent.status(Pre, WEAPON_NESS_YOYO_STATUS_KIND_REACH, lasso_pre);
	agent.status(Init, WEAPON_NESS_YOYO_STATUS_KIND_REACH, reach_init);
	agent.status(Main, WEAPON_NESS_YOYO_STATUS_KIND_REACH, reach_main);
	agent.status(End, WEAPON_NESS_YOYO_STATUS_KIND_REACH, empty_status);
    
	agent.status(Pre, WEAPON_NESS_YOYO_STATUS_KIND_REWIND, lasso_pre);
	agent.status(Main, WEAPON_NESS_YOYO_STATUS_KIND_REWIND, rewind_main);
	agent.status(End, WEAPON_NESS_YOYO_STATUS_KIND_REWIND, empty_status);
    
	agent.status(Pre, WEAPON_NESS_YOYO_STATUS_KIND_HANG, lasso_pre);
	agent.status(Main, WEAPON_NESS_YOYO_STATUS_KIND_HANG, hang_main);
	agent.status(End, WEAPON_NESS_YOYO_STATUS_KIND_HANG, empty_status);

	agent.status(Pre, WEAPON_NESS_YOYO_STATUS_KIND_EXHAUST, exhaust_pre);
	agent.status(Main, WEAPON_NESS_YOYO_STATUS_KIND_EXHAUST, exhaust_main);
	agent.status(End, WEAPON_NESS_YOYO_STATUS_KIND_EXHAUST, empty_status);

    agent.install();
    /*
    let head = &mut smashline::Agent::new("ness_yoyohead");
	head.status(Pre, WEAPON_NESS_YOYO_HEAD_STATUS_KIND_LASSO, exhaust_pre);
	head.status(Main, WEAPON_NESS_YOYO_HEAD_STATUS_KIND_LASSO, head_lasso_main);
	head.status(End, WEAPON_NESS_YOYO_HEAD_STATUS_KIND_LASSO, empty_status);
    head.install(); 
    */
}