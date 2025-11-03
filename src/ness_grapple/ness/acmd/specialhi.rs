use crate::imports::imports_acmd::*;
use crate::ness_grapple::imports::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE, false, -1);
        ArticleModule::change_status(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE, 0, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let body_angle_intp = WorkModule::get_param_int(agent.module_accessor, hash40("air_lasso_data"), hash40("body_angle_intp"));
        println!("body_angle_intp: {body_angle_intp}");
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_FLAG_SET_ANGLE);
        macros::ATTACK(agent, 0, 0, Hash40::new("viner2"), 11.0, 50, 80, 0, 50, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("viner3"), 11.0, 50, 80, 0, 50, 4.2, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("viner4"), 11.0, 50, 80, 0, 50, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 3, 0, Hash40::new("viner5"), 11.0, 50, 80, 0, 50, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 4, 0, Hash40::new("viner5"), 13.0, 50, 80, 0, 70, 4.5, 8.8, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE, false, -1);

        //ArticleModule::set_visibility_whole(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE_HEAD, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        //ArticleModule::set_visibility_whole(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        
        //ArticleModule::generate_article(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE, false, -1);
        //ArticleModule::set_visibility_whole(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        //ArticleModule::change_status(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE, WEAPON_NESS_YOYO_HEAD_STATUS_KIND_LASSO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

        //ArticleModule::generate_article(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE_HEAD, false, -1);
        //ArticleModule::set_visibility_whole(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE_HEAD, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        //ArticleModule::change_status(agent.module_accessor, GENERATE_ARTICLE_GRAPPLE_HEAD, WEAPON_NESS_YOYO_HEAD_STATUS_KIND_LASSO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_SET_MAP_COLL_OFFSET);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_FLAG_SET_ANGLE);
        macros::ATTACK(agent, 0, 0, Hash40::new("viner2"), 11.0, 50, 80, 0, 40, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("viner3"), 11.0, 50, 80, 0, 40, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("viner4"), 11.0, 50, 80, 0, 40, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 3, 0, Hash40::new("viner5"), 11.0, 50, 80, 0, 40, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 4, 0, Hash40::new("viner5"), 13.0, 50, 80, 0, 70, 5.5, 8.8, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhistart", game_specialhi, Priority::Default);

    agent.acmd("game_specialairhistart", game_specialairhi, Priority::Default);
}