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
        WorkModule::is_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_FLAG_ENABLE_CUT);
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

pub unsafe extern "C" fn catch_attack_snake(fighter: &mut L2CFighterCommon) -> L2CValue {
    let has_c4 = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4);
    WorkModule::set_flag(fighter.module_accessor, has_c4, FIGHTER_SNAKE_STATUS_CATCH_FLAG_HAS_C4);
    println!("Set C4 variable: {has_c4}");
    if has_c4 {
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_attack")));
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchAttack_Main as *const () as _));
    }
    fighter.status_CatchAttack()
}

pub fn install() {
    smashline::Agent::new("snake")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_snake)
    .install();
}
