use crate::imports::imports_acmd::*;
use crate::special_pummel::imports::*;

pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID: i32 = 0x100000BF;
pub const FIGHTER_METAKNIGHT_STATUS_THROW_FLAG_START: i32 = 0x2100000E;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 0, 0, 10, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_METAKNIGHT_STATUS_THROW_FLAG_START);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("gekkouga_kageuchi_warp_end"), Hash40::new("top"), 0, 9.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        //macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("gekkouga_kageuchi_warp_start"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gekkouga_special_s02"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
}

unsafe extern "C" fn game_catchspecialf(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.1);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 36, 104, 0, 60, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn game_catchspecialb(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.1);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 48, 103, 0, 60, 8.0, 0.0, 8.0, -15.0, Some(0.0), Some(8.5), Some(-12.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

/*
STATUS
*/
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = catch_attack_main_inner(fighter);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        let opponent_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        WorkModule::set_int64(fighter.module_accessor, opponent_id as i64, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID);
        
        let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
        StatusModule::change_status_force(opponent, *FIGHTER_STATUS_KIND_FURAFURA_STAND, true);
        
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchAttack_Main as *const () as _));
    }
    
    return to_return;
}

pub unsafe extern "C" fn throw_main_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_CATCH_ATTACK 
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
        fighter.status_Throw_Sub();

        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);

        let capture_id = WorkModule::get_int64(fighter.module_accessor,FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID) as u64;
        if capture_id != OBJECT_ID_NULL as u64{
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            MotionModule::change_motion(capture_boma, Hash40::new("damage_lw_3"),0.0,0.8,false,0.0,false,false); 
        }
        return fighter.sub_shift_status_main(L2CValue::Ptr(throw_sp_main_loop_uniq as *const () as _))
    }
    fighter.status_Throw_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Throw_Main as *const () as _))
}

pub unsafe extern "C" fn throw_sp_main_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("catch_special").hash {
        CameraModule::set_enable_camera(fighter.module_accessor,false,0);
        let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
        if MotionModule::is_end(fighter.module_accessor) ||
        WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_STATUS_THROW_FLAG_START) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_STATUS_THROW_FLAG_START);

            let lr = PostureModule::lr(fighter.module_accessor);
            let throw_F = lr == ControlModule::get_stick_x(fighter.module_accessor).signum()
            || ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.2;
            //let motion = if throw_F {Hash40::new("special_lw_f")} else {Hash40::new("special_lw_b")};
            let motion = if throw_F {Hash40::new("catch_special_f")} else {Hash40::new("catch_special_b")};
            
            let capture_id = WorkModule::get_int64(fighter.module_accessor,FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID) as u64;
            if capture_id != OBJECT_ID_NULL as u64 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let capture_pos = *PostureModule::pos(capture_boma);
                let add_x = if throw_F {-7.0} else {7.0};
                PostureModule::set_pos(fighter.module_accessor, 
                    &Vector3f::new(capture_pos.x + (add_x*2.0), capture_pos.y, capture_pos.z)
                );
            }
            MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
            
            VisibilityModule::set_whole(fighter.module_accessor, true);
        }
        return 0.into();
    }
    else {
        if AttackModule::is_attack(fighter.module_accessor, 0, false) 
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT){
            CameraModule::set_enable_camera(fighter.module_accessor,true,0);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
        fighter.status_Throw_Main()
    }
}
pub unsafe extern "C" fn throw_end_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    CameraModule::set_enable_camera(fighter.module_accessor,true,0);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    fighter.status_end_Throw()
}

pub fn install() {
    smashline::Agent::new("gekkouga")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)

        .acmd("game_catchspecialf", game_catchspecialf,Priority::Default)
        .acmd("game_catchspecialb", game_catchspecialb,Priority::Default)
        
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Main, *FIGHTER_STATUS_KIND_THROW, throw_main_uniq)
        .status(End, *FIGHTER_STATUS_KIND_THROW, throw_end_uniq)
    .install();
}