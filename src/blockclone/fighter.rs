use crate::imports::imports_agent::*;
use super::*;

pub const SPECIAL_N_FLAG_IS_AIR: i32 = 0x2100000B;
pub const SPECIAL_N_FLAG_GENERATED: i32 = 0x2100000C;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    let keep_flag_bool = if fighter.is_grounded() {*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG} else {*FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLAG};
    let keep_flag_num = if fighter.is_grounded() {*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT} else {*FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT};

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        keep_flag_bool,
        keep_flag_num,
        keep_flag_num,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        true,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Throw cancel frame: 20. SpecialN3 cancel: 12.

    fighter.sub_change_motion_by_situation(Hash40::new("item_light_throw_lw").into(), Hash40::new("item_light_throw_air_lw").into(), false.into());
    let new_rate = 1.0 / (12.0 / 20.0);
    MotionModule::set_rate(fighter.module_accessor, new_rate);
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(), FIGHTER_KINETIC_TYPE_MOTION_FALL.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    if !fighter.is_grounded() {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_FALL_FLAG_DISABLE_MOTION_INTP);
    }
    WorkModule::set_flag(fighter.module_accessor,!fighter.is_grounded(), SPECIAL_N_FLAG_IS_AIR);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air_start = WorkModule::is_flag(fighter.module_accessor,SPECIAL_N_FLAG_IS_AIR);
    let is_grounded = fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND;

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() 
        || fighter.sub_transition_group_check_air_landing().get_bool() //?
        {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if is_air_start && is_grounded {
            WorkModule::set_float(fighter.module_accessor, 12.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 1.into();
        }
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(),FIGHTER_KINETIC_TYPE_MOTION_FALL.into());
        fighter.sub_change_motion_by_situation(Hash40::new("item_light_throw_lw").into(), Hash40::new("item_light_throw_air_lw").into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    if MotionModule::frame(fighter.module_accessor) >= 6.0
    && !WorkModule::is_flag(fighter.module_accessor, SPECIAL_N_FLAG_GENERATED) {
        WorkModule::on_flag(fighter.module_accessor, SPECIAL_N_FLAG_GENERATED);
        ArticleModule::generate_article(fighter.module_accessor, block::GENERATE_ARTICLE_ID, false, -1);
    }
    else if MotionModule::frame(fighter.module_accessor) == 8.0 {
        if !block::DEBUG_HAS_LVD {
            let eff_pos = *PostureModule::pos(fighter.module_accessor);
            EffectModule::req(fighter.module_accessor, Hash40::new("sys_erace_smoke"), 
            &Vector3f{x: eff_pos.x, y: eff_pos.y+0.0, z: eff_pos.z}, &Vector3f{x: 0.0, y: 300.0, z: 0.0}, 
            1.0, 0,-1,false,0) as u32;
        }
    }
    else if MotionModule::frame(fighter.module_accessor) >= 12.0 {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main)
    .install();
}