use crate::imports::imports_acmd::*;
use crate::ness_grapple::imports::*;

unsafe extern "C" fn sound_aircatchhang(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_cliff_catch"));
    }
}

unsafe extern "C" fn expression_aircatchhang(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("invalid") as i64);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sound_aircatchhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_jack_special_h01"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_common_cliff_catch"), 1);
    }
}
unsafe extern "C" fn expression_aircatchhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("invalid") as i64);
    }
}

unsafe extern "C" fn sound_aircatchpose(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_jack_special_h01"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_jack_special_h01"), 10);
    }
}
unsafe extern "C" fn expression_aircatchpose(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("invalid") as i64);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("sound_aircatchhang", sound_aircatchhang, Priority::Default);
    agent.acmd("expression_aircatchhang", expression_aircatchhang, Priority::Default);

    agent.acmd("sound_aircatchhit", sound_aircatchhit, Priority::Default);
    agent.acmd("expression_aircatchhit", expression_aircatchhit, Priority::Default);

    agent.acmd("sound_aircatchpose", sound_aircatchpose, Priority::Default);
    agent.acmd("expression_aircatchpose", expression_aircatchpose, Priority::Default);
}