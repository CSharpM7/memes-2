use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;
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
                    ArticleModule::shoot(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                }
            }
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
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
unsafe extern "C" fn game_catchspecial2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, -1);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CLATTER);
    }
    frame(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        let has_c4 = ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4);
        if has_c4 {
            ArticleModule::change_status(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CLATTER);
    }
}

unsafe extern "C" fn effect_catchspecial2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_catchspecial2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_special_l04"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_snake_rnd_special_l"));
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_special_l05"));
    }
}

unsafe extern "C" fn expression_catchspecial2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
/*
STATUS
*/
pub unsafe extern "C" fn catch_attack_snake(fighter: &mut L2CFighterCommon) -> L2CValue {
    let has_c4 = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4);
    WorkModule::set_flag(fighter.module_accessor, has_c4, FIGHTER_SNAKE_STATUS_CATCH_FLAG_HAS_C4);
    if has_c4 && catch_attack_check_special(fighter) {
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special2")));
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchAttack_Main as *const () as _));
    }
    #[cfg(not(feature = "dev"))]
    return catch_attack_main_inner(fighter);

    #[cfg(feature = "dev")]
    return fighter.status_CatchAttack();
}

pub unsafe extern "C" fn catch_attack_exec_snake(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
pub unsafe extern "C" fn catch_attack_end_snake(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    #[cfg(not(feature = "dev"))]
    return catch_attack_end_inner(fighter);

    #[cfg(feature = "dev")]
    return fighter.status_end_CatchAttack();
}

pub unsafe extern "C" fn catch_cut_snake(fighter: &mut L2CFighterCommon) -> L2CValue {
    let has_c4 = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4);
    if has_c4 {
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighter_obj : *mut Fighter = std::mem::transmute(object);
        if is_constraint_article(fighter_obj, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub fn install() {
    smashline::Agent::new("snake")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        
        .acmd("game_catchspecial2", game_catchspecial2,Priority::Default)
        .acmd("effect_catchspecial2", effect_catchspecial2,Priority::Default)
        .acmd("sound_catchspecial2", sound_catchspecial2,Priority::Default)
        .acmd("expression_catchspecial2", expression_catchspecial2,Priority::Default)

        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_snake)
        .status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_end_snake)
        .status(Init, *FIGHTER_STATUS_KIND_CATCH_CUT, catch_cut_snake)
    .install();
}
