use crate::imports::imports_status::*;
use crate::imports::imports_acmd::*;

/*
HELPER
*/
pub unsafe fn is_uniq_echo(boma: *mut BattleObjectModuleAccessor) -> bool
{
    
    let entry_id = sv_battle_object::entry_id((*boma).battle_object_id) as u32;
    let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
    let color = app::lua_bind::FighterInformation::fighter_color(info) as i32;

    #[cfg(feature = "dev")]
    return color == 0;

    let modded = (120..=127).contains(&color);
    return modded;
}
/*
ACMD
*/
unsafe extern "C" fn game_specialnblank(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn effect_specialnblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn expression_specialnblank(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialsblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.06);
}
unsafe extern "C" fn effect_specialsblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("throw"), 0, 6, 12, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn expression_specialsblank(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        //ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}
/*
FRAME
*/
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
    force_enable_specials(&mut *module_accessor);
}
/*
STATUS
*/
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
    let should_return_original = !is_uniq_echo(fighter.module_accessor) || !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN);
    if should_return_original {
        let to_return = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
        if is_uniq_echo(fighter.module_accessor) {
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        }
        return to_return;
    }

    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_blank") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_blank") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    special_motion_helper(fighter,true);
    fighter.sub_set_ground_correct_by_situation(false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    
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
    let should_return_original = !is_uniq_echo(fighter.module_accessor) || !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY);
    if should_return_original {
        let to_return = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
        if is_uniq_echo(fighter.module_accessor) {
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        }
        return to_return;
    }

    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_blank") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_blank") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    special_motion_helper(fighter,true);

    fighter.sub_set_ground_correct_by_situation(false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    
    return fighter.sub_shift_status_main(L2CValue::Ptr(specialn_main_loop as *const () as _));
}
unsafe extern "C" fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner = smash::app::sv_battle_object::module_accessor(owner_id);
    let ret_original = smashline::original_status(Main, weapon, *WEAPON_DUCKHUNT_GUNMAN_STATUS_KIND_SHOOT)(weapon);
    let should_use_original = !is_uniq_echo(owner);
    if should_use_original {
        return ret_original;
    }

    let end_frame = MotionModule::end_frame(weapon.module_accessor) as i32;
    WorkModule::set_int(weapon.module_accessor, end_frame, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, end_frame, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    
    return ret_original;
}
pub fn install() {   
    Agent::new("duckhunt")
        .acmd("game_specialnblank", game_specialnblank, Priority::Default)
        .acmd("game_specialairnblank", game_specialnblank, Priority::Default)
        .acmd("sound_specialnblank", acmd_stub, Priority::Default)
        .acmd("sound_specialairnblank", acmd_stub, Priority::Default)
        .acmd("effect_specialnblank", effect_specialnblank, Priority::Default)
        .acmd("effect_specialairnblank", effect_specialnblank, Priority::Default)
        .acmd("expression_specialnblank", expression_specialnblank, Priority::Default)
        .acmd("expression_specialairnblank", expression_specialnblank, Priority::Default)
        
        .acmd("game_specialsblank", game_specialsblank, Priority::Default)
        .acmd("game_specialairsblank", game_specialsblank, Priority::Default)
        .acmd("sound_specialsblank", acmd_stub, Priority::Default)
        .acmd("sound_specialairsblank", acmd_stub, Priority::Default)
        .acmd("effect_specialsblank", effect_specialsblank, Priority::Default)
        .acmd("effect_specialairsblank", effect_specialsblank, Priority::Default)
        .acmd("expression_specialsblank", expression_specialsblank, Priority::Default)
        .acmd("expression_specialairsblank", expression_specialsblank, Priority::Default)
        
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_main)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_main)
        //.on_line(Main, fighter_update)
    .install();
    Agent::new("duckhunt_gunman")
        .status(Main, *WEAPON_DUCKHUNT_GUNMAN_STATUS_KIND_SHOOT, shoot_main)
    .install();
}
