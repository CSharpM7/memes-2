use crate::imports::imports_status::*;
use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_speciallw19(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0,15.0,7.0);
    frame(agent.lua_state_agent, 5.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 5.0, 180, 50, 35, 0, 4.5, 9.3, 0.0, -2.5, None, None, None, 0.8, 0.7, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("havel"), 5.0, 361, 50, 30, 0, 4.5, 3.7, 0.0, -2.5, None, None, None, 0.8, 0.7, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 180, 50, 50, 0, 6.0, 0.0, 7.0, 7.0, None, None, None, 0.8, 0.7, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 180, 50, 40, 0, 6.0, 0.0, 7.0, 3.0, None, None, None, 0.8, 0.7, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 7.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 7.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 7.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 7.0, false);
        /*
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
         */
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0,30.0,7.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 14.0, 361, 77, 0, 68, 4.5, 9.3, 0.0, -2.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 14.0, 361, 77, 0, 68, 3.5, 3.0, 0.0, -1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 14.0, 361, 77, 0, 68, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 14.0, 361, 77, 0, 68, 10.5, 0.0, 18.0, 16.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    FT_MOTION_RATE_RANGE(agent, 30.0,40.0,7.0);
    frame(agent.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw19(agent: &mut L2CAgentBase) {
    let mut effect_l: u32 = 0;
    let mut effect_r: u32 = 0;
    if macros::is_excute(agent) {

        macros::EFFECT_FOLLOW(agent, Hash40::new("brave_falconsword"), Hash40::new("sword1"), -3.0, 0.0, 0.0, 0, 0, -90, 1, false);
        effect_r = EffectModule::get_last_handle(agent.module_accessor) as u32;
        macros::EFFECT_FOLLOW(agent, Hash40::new("brave_falconsword"), Hash40::new("havel"), -3.0, 0.0, 0.0, 0, 0, -90, 1, false);
        effect_l = EffectModule::get_last_handle(agent.module_accessor) as u32;

        macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("havel"), -3.0, 0, 0, 0, 0, -90, 1, true);
        
        EffectModule::set_rate(agent.module_accessor, effect_r, 1.6);
        EffectModule::set_rate(agent.module_accessor, effect_l, 1.6);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        EffectModule::set_rate(agent.module_accessor, effect_r, 0.6);
        EffectModule::set_rate(agent.module_accessor, effect_l, 0.6);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        EffectModule::set_rate(agent.module_accessor, effect_r, 1.6);
        EffectModule::set_rate(agent.module_accessor, effect_l, 1.6);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("brave_falconsword"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("brave_lightning3_sword"), false, true);
        macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
        macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn sound_speciallw19(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l20"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        PLAY_VC(agent, Hash40::new("vc_brave_smash"),1.6);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l20"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
    }
}

unsafe extern "C" fn expression_speciallw19(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}


pub fn install() {   
    Agent::new("brave")
    .acmd("game_speciallw19",game_speciallw19,Priority::Default)
    .acmd("effect_speciallw19",effect_speciallw19,Priority::Default)
    .acmd("sound_speciallw19",sound_speciallw19,Priority::Default)
    .acmd("expression_speciallw19",expression_speciallw19,Priority::Default)
    .install();
}