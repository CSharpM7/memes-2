use crate::imports::imports_acmd::*;
use super::*;


pub const LW4_FRAME_OFFSET: f32 = 19.0;

pub const ATTACK_LW4_WORK_FLAG_MINE_PLANTED: i32 = 0x0; //FIGHTER_EDGE_STATUS_ATTACK_LW4_WORK_FLAG_IS_HIT_FLOOR


unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    //frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //ArticleModule::generate_article(agent.module_accessor, ground::GENERATE_ARTICLE_ID, false, -1);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    //Spawns on 13? so 19.0+13.0?
    frame(agent.lua_state_agent, LW4_FRAME_OFFSET+13.0);
    if macros::is_excute(agent) {
        //Here the landmine itself gets generated
        ArticleModule::generate_article(agent.module_accessor, landmine::GENERATE_ARTICLE_ID, false, -1);
        ArticleModule::change_status_exist(agent.module_accessor, landmine::GENERATE_ARTICLE_ID, landmine::STATUS_KIND_HAVED);
    }
    frame(agent.lua_state_agent, LW4_FRAME_OFFSET+41.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status_exist(agent.module_accessor, landmine::GENERATE_ARTICLE_ID, landmine::STATUS_KIND_READY);
    }
}
unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toel"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13, -1, 35, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 1.5, 15, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 12.5, 0, 35, -180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 1.5, -15, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    }
}

unsafe extern "C" fn sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_smash_l02"));
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_smash_l02"));
    }

    frame(agent.lua_state_agent, LW4_FRAME_OFFSET);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_smash_l01"));
    }
    wait(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_special_l01"));
    }
}

unsafe extern "C" fn expression_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn effect_attacklw4charge(agent: &mut L2CAgentBase) {
    /*
    frame(agent.lua_state_agent, 5.0);
    loop {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("toer"), 0, 0, -2, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
    */
    //Dig effects
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, true);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..3 {
        wait(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    for _ in 0..3 {
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_attacklw4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
    //Dig effects
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
    for _ in 0..3 {
        wait(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_smash_start"));
        }
    }
    for _ in 0..3 {
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_smash_start"));
        }
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
}

unsafe extern "C" fn expression_attacklw4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, 0x50000000);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, 0x50000000);
    }
}


pub unsafe extern "C" fn attacklw4_start_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    attacklw4_common_exit(fighter);
    fighter.status_end_AttackLw4Start()
}
pub unsafe extern "C" fn attacklw4_hold_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    attacklw4_common_exit(fighter);
    fighter.status_end_AttackLw4Hold()
}
pub unsafe extern "C" fn attacklw4_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    attacklw4_common_exit(fighter);
    fighter.status_end_AttackLw4()
}
pub unsafe extern "C" fn attacklw4_common_exit(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let status_interupt = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,*FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_interupt) {
        ArticleModule::remove_exist(fighter.module_accessor, ground::GENERATE_ARTICLE_ID, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

pub fn install() {
    Agent::new("snake")
        .acmd("game_attacklw4", game_attacklw4, Priority::Default)
        .acmd("effect_attacklw4", effect_attacklw4, Priority::Default)
        .acmd("sound_attacklw4", sound_attacklw4, Priority::Default)
        .acmd("expression_attacklw4", expression_attacklw4, Priority::Default)
        
        .acmd("effect_attacklw4charge", effect_attacklw4charge, Priority::Default)
        .acmd("sound_attacklw4charge", sound_attacklw4charge, Priority::Default)
        .acmd("expression_attacklw4charge", expression_attacklw4charge, Priority::Default)

        .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, attacklw4_start_end)
        .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, attacklw4_hold_end)
        .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4, attacklw4_end)
    .install();
}