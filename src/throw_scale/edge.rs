use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.5, 58, 105, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT) as u32;
        if sv_battle_object::is_active(target) {
            let target_boma = sv_battle_object::module_accessor(target);
            LinkModule::set_model_constraint_flag(target_boma, *CONSTRAINT_FLAG_MTX as u32);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EDGE_STATUS_THROW_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 80, 0, 60, 7.5, 0.0, 11.0, -13.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, -15, 8);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.25);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: -5.0, y: 3.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        if sv_battle_object::is_active(target as u32) {
            let target_boma = sv_battle_object::module_accessor(target as u32);
            LinkModule::off_model_constraint_flag(target_boma, *CONSTRAINT_FLAG_MTX as u32);
        }

        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Throw_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(throw_main_loop_uniq as *const () as _))
}
pub unsafe extern "C" fn throw_main_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        println!("Cut");
        fighter.change_status(FIGHTER_STATUS_KIND_SLIP.into(), true.into());
		//CatchModule::catch_cut(fighter.module_accessor, false,false);
		//capture!(fighter, *MA_MSC_SET_IGNORE_CATCHING, false);
        return 1.into();
    }
    fighter.status_Throw_Main()
}

pub fn install() {   
    Agent::new("edge")
        .acmd("game_throwb", game_throwb,Priority::Default)
        .status(Main, *FIGHTER_STATUS_KIND_THROW, throw_main)
    .install();
}