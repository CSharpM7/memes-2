use crate::imports::imports_status::*;
use crate::imports::imports_acmd::*;
unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 367, 65, 40, 0, 4.5, 0.0, 13.0, 10.0, Some(0.0), Some(6.0), Some(10.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_ROBOT_REFLECTOR_KIND_ARMSPIN, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 367, 70, 50, 0, 4.5, 0.0, 13.0, -8.0, Some(0.0), Some(6.0), Some(-8.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 367, 65, 40, 0, 6.5, 0.0, 13.0, 12.0, Some(0.0), Some(6.0), Some(12.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 367, 70, 50, 0, 6.5, 0.0, 13.0, -8.0, Some(0.0), Some(6.0), Some(-8.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_ROBOT_REFLECTOR_KIND_ARMSPIN, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
}

pub unsafe extern "C" fn special_s_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    let speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_speed_y"));
    if (speed_y > 0.0) {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
    }
    return fighter.sub_shift_status_main(L2CValue::Ptr(special_s_attack_main_loop as *const () as _));
}

pub unsafe extern "C" fn special_s_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        let frame = MotionModule::frame(fighter.module_accessor);
        let rate = MotionModule::rate(fighter.module_accessor);
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), true.into());
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, frame, true, true, false);
        MotionModule::set_rate(fighter.module_accessor, rate);

        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_ROBOT_SPECIAL_S_ATTACK.into(),FIGHTER_KINETIC_TYPE_ROBOT_SPECIAL_AIR_S_ATTACK.into());
    }

    if fighter.global_table[STATUS_FRAME].get_i32() > 10 && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install() {   
    Agent::new("robot")
        .acmd("game_specials",game_specials)
        .acmd("game_specialairs",game_specials)
        .status(Main, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack_main)
        .install();
}