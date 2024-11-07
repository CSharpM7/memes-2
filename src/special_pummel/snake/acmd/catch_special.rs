use crate::imports::imports_acmd::*;
use crate::special_pummel::imports::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Snake21is_constraint_articleERNS_7FighterEiNS_22ArticleOperationTargetE"]
    pub fn is_constraint_article(
        arg1: *mut smash::app::Fighter,
        arg2: i32,
        arg3: smash::app::ArticleOperationTarget,
    ) -> bool;
}

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    let object = sv_system::battle_object(agent.lua_state_agent);
    let fighter : *mut Fighter = std::mem::transmute(object);

    if macros::is_excute(agent) {
        //if !WorkModule::is_flag(agent.module_accessor, FIGHTER_SNAKE_STATUS_CATCH_FLAG_HAS_C4) 
        {
            frame(agent.lua_state_agent, 5.0);
            {
                ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, false, -1);
            }
            frame(agent.lua_state_agent, 12.0);
            if macros::is_excute(agent) {
                if is_constraint_article(fighter, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
                    ArticleModule::change_status(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_ESTABLISH_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                }
            }
            frame(agent.lua_state_agent, 13.0);
            if macros::is_excute(agent) {
                if is_constraint_article(fighter, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
                    println!("Stick");
                    ArticleModule::shoot(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                }
            }
        }
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_snake_catchspecial"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_catchspecial", game_catchspecial,Priority::Default);
    agent.acmd("effect_catchspecial", effect_catchspecial,Priority::Default);
    agent.acmd("sound_catchspecial", sound_catchspecial,Priority::Default);
    agent.acmd("expression_catchspecial", expression_catchspecial,Priority::Default);
}