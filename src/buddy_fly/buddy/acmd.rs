use crate::imports::imports_acmd::*;
use crate::buddy_fly::imports::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PAD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_DECIDE_LR);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_RESET_CONTROL);
        //WorkModule::on_flag(agent.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_RESET_ANGLE);
        
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_FALL);
    }
}
unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_buddy_swing_l01"));
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_buddy_special_h02"));
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        PLAY_VC(agent, Hash40::new("vc_buddy_special_h03"),0.625);
    }
}

unsafe extern "C" fn effect_specialhifall(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bust"), 5, -5, 0, 0, 0, 0, 0.85, true);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bust"), 5, -5, 0, 0, 0, 0, 0.85, true);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
        }

        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}
unsafe extern "C" fn sound_specialhifall(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_buddy_jump02_01"));
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_buddy_jump02_01"));
        }

        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn effect_specialhilanding(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialhilanding(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_buddy_landing03"));
    }
}

unsafe extern "C" fn expression_specialhilanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhi",game_specialhi,Priority::Default);
    agent.acmd("sound_specialhi",sound_specialhi,Priority::Default);

    agent.acmd("effect_specialhifall",effect_specialhifall,Priority::Default);
    agent.acmd("sound_specialhifall",sound_specialhifall,Priority::Default);
    
    agent.acmd("effect_specialhilanding",effect_specialhilanding,Priority::Default);
    agent.acmd("sound_specialhilanding",sound_specialhilanding,Priority::Default);
    agent.acmd("expression_specialhilanding",expression_specialhilanding,Priority::Default);
}