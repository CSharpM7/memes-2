use crate::imports::imports_acmd::*;
use crate::koopajr_mech::imports::*;

unsafe extern "C" fn game_specialhilanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_PILOT);
    }
}
unsafe extern "C" fn effect_walkjrslow(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 46.0);
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn sound_walkjrslow(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_koopajr_wait_shake"));
        }
        frame(agent.lua_state_agent, 45.0);
        macros::PLAY_STEP(agent, Hash40::new("se_koopajr_wait_shake"));

        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn expression_walkjrslow(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 48.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);

        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn effect_walkjrfast(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 16.0);
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn sound_walkjrfast(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_koopajr_wait_out"));
        }
        frame(agent.lua_state_agent, 15.0);
        macros::PLAY_STEP(agent, Hash40::new("se_koopajr_wait_out"));

        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn expression_walkjrfast(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        }
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 17.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);

        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn effect_turnjr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_turnjr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_koopajr_wait_shake"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_koopajr_wait_shake"));
    }
}

unsafe extern "C" fn expression_turnjr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x23c33f3bdc));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("effect_walkjrslow",effect_walkjrslow,Priority::Default);
    agent.acmd("sound_walkjrslow",sound_walkjrslow,Priority::Default);
    agent.acmd("expression_walkjrslow",expression_walkjrslow,Priority::Default);
    
    agent.acmd("effect_walkjrfast",effect_walkjrfast,Priority::Default);
    agent.acmd("sound_walkjrfast",sound_walkjrfast,Priority::Default);
    agent.acmd("expression_walkjrfast",expression_walkjrfast,Priority::Default);

    agent.acmd("effect_turnjr",effect_turnjr,Priority::Default);
    agent.acmd("sound_turnjr",sound_turnjr,Priority::Default);
    agent.acmd("expression_turnjr",expression_turnjr,Priority::Default);
}