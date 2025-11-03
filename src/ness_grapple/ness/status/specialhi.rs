use crate::imports::imports_status::*;
use crate::ness_grapple::imports::*;

pub unsafe extern "C" fn specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN as u32),
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn specialhi_init(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    0.into()
}
unsafe extern "C" fn specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot_g = hash40("special_hi_start");
    let mot_a = hash40("special_air_hi_start");
    //let mot_g = hash40("special_hi");
    //let mot_a = hash40("special_air_hi");
    fighter.sub_change_motion_by_situation(Hash40::new_raw(mot_g).into(), Hash40::new_raw(mot_a).into(), false.into());

    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_hi").into());
    //KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 1.0, y: 0.75, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    fighter.set_cliff_hangdata(72.0,65.0,-18.0,-52.0);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);

    WorkModule::set_int(fighter.module_accessor, GENERATE_ARTICLE_GRAPPLE, *FIGHTER_STATUS_AIR_LASSO_WORK_INT_ARTICLE_ID);
    WorkModule::set_int(fighter.module_accessor, *ARTICLE_ID_NONE, *FIGHTER_STATUS_AIR_LASSO_WORK_INT_ARTICLE2_ID);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_AIR_LASSO_FAIL);

	fighter.sub_shift_status_main(L2CValue::Ptr( specialhi_main_loop as *const () as _)) 
}

unsafe extern "C" fn specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_AIR_LASSO_FAIL) {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            println!("Air cliff caught");
            return 1.into();
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        let mot_g = hash40("special_hi_start");
        let mot_a = hash40("special_air_hi_start");
        //let mot_g = hash40("special_hi");
        //let mot_a = hash40("special_air_hi");
        fighter.sub_change_motion_by_situation(Hash40::new_raw(mot_g).into(), Hash40::new_raw(mot_a).into(), true.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
    }

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation == *SITUATION_KIND_AIR {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK) {
            println!("Lasso check");
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_IS_CHECK);
            let found_cliff = fighter.sub_fighter_general_term_is_cliff_check_pos().get_bool();
            if found_cliff {
                println!("Gottem");
                fighter.change_status(FIGHTER_STATUS_KIND_AIR_LASSO_REACH.into(), false.into());
            }
        }
        else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_IS_CHECK) {
            println!("MISS!");
            //hang data
            GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_CLIFF_HANG_DATA_DEFAULT as u32);
            GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_IS_CHECK);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_AIR_LASSO_FAIL);
        }
    }
    0.into()
}


pub unsafe extern "C" fn specialhi_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_AIR_LASSO_REACH {
        fighter.status_end_AirLasso();
        ArticleModule::remove_exist(fighter.module_accessor, GENERATE_ARTICLE_GRAPPLE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, empty_status);
	agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_pre);
	agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_main);
	agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, empty_status);
	agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_end);
}