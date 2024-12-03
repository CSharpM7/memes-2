use crate::imports::imports_status::*;
pub unsafe fn is_uniq_echo(boma: *mut BattleObjectModuleAccessor) -> bool
{
    #[cfg(feature = "dev")]
    return true;
    
    let entry_id = sv_battle_object::entry_id((*boma).battle_object_id) as u32;
    let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
    let color = app::lua_bind::FighterInformation::fighter_color(info) as i32;
    let modded = [120..=127].contains(&color);
    return modded;
}

unsafe fn force_enable_specials(module_accessor: &mut BattleObjectModuleAccessor) {
    let has_final = WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) &&
    WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ONE_MORE_FINAL_DEAD_FAILED);
    if !has_final {
        WorkModule::unable_transition_term_forbid(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    WorkModule::unable_transition_term_forbid(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
}
pub unsafe extern "C" fn fighter_update(fighter: &mut L2CFighterCommon) {
    let module_accessor= fighter.module_accessor;
    force_enable_specials(module_accessor);
}

pub unsafe extern "C" fn special_motion_helper(fighter: &mut L2CFighterCommon,init: bool) {
    let mot_g =  WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    let mot_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    let mot = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {Hash40::new_raw(mot_g)} else {Hash40::new_raw(mot_a)};

    if init {
        MotionModule::change_motion(fighter.module_accessor, mot,0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, mot, -1.0, 1.0, 0.0, false, false);
    }
}
unsafe extern "C" fn specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let should_use_original = !is_uniq_echo(fighter.module_accessor) || !ArticleModule::is_exist(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN);
    if should_use_original {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    special_motion_helper(fighter,true);
    
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    return fighter.sub_shift_status_main(L2CValue::Ptr(specialn_main_loop as *const () as _));
}

unsafe extern "C" fn specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        special_motion_helper(fighter,false);
    }

    0.into()
} 

unsafe extern "C" fn specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let should_use_original = !is_uniq_echo(fighter.module_accessor) || !ArticleModule::is_exist(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY);
    if should_use_original {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    special_motion_helper(fighter,true);
    
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    return fighter.sub_shift_status_main(L2CValue::Ptr(specialn_main_loop as *const () as _));
}
pub fn install() {   
    Agent::new("duckhunt")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_main)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_main)
        .on_line(Main, fighter_update)
    .install();
}
