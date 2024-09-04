use crate::imports::imports_acmd::*;
use crate::ledge_special::imports::*;

pub unsafe extern "C" fn game_cliffspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        sv_kinetic_energy!(
            set_speed_mul,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.8
        );
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 60, 35, 0, 95, 3.75, 0.0, 3.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 35, 0, 95, 3.75, 0.0, 3.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 70, 35, 0, 95, 3.75, 0.0, 3.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


pub unsafe extern "C" fn effect_cliffspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }

    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_cshot_bullet"), Hash40::new("top"), 0, 2.7, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_RATE(agent,2.0);

        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_samusd_ball1"), Hash40::new("tex_samusd_ball2"), 10, 
        Hash40::new("top"), 0.0, 0.0, -1.5, Hash40::new("top"), 0.0, 7.0, -1.5, 
        true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
        1.0, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);

        macros::BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
    }

    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("samusd_cshot_bullet"), false, true);
    }
    
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
        macros::BURN_COLOR_NORMAL(agent);
    }
}
unsafe extern "C" fn sound_cliffspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samusd_attackdash"));
    }
}

unsafe extern "C" fn expression_cliffspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 20.0); //+14.0
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false,*ATTACH_ITEM_GROUP_ALL as u8);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, true,*ATTACH_ITEM_GROUP_ALL as u8);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
}
pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_cliffspecial", game_cliffspecial);
    agent.acmd("effect_cliffspecial", effect_cliffspecial);
    agent.acmd("sound_cliffspecial", sound_cliffspecial);
    agent.acmd("expression_cliffspecial", expression_cliffspecial);
}