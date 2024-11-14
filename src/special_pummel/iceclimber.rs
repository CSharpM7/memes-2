use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;

pub const FIGHTER_POPO_STATUS_THROW_FLAG_STALL: i32 = 0x2100000E;
pub const FIGHTER_POPO_STATUS_THROW_FLAG_HAS_NANA: i32 = 0x2100000F;
pub const FIGHTER_POPO_STATUS_THROW_WORK_INT_STATE: i32 = 0x11000004;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 340, 0, 10, 70, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 16, 9);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    //FT_MOTION_RATE_RANGE(agent,10.0,18.0,20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 366, 0, 10,50, 5.0, 0.0, 6.5, 10.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_HAMMER);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_POPO_STATUS_THROW_FLAG_STALL);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {        
        let damage = 2.0;
        let angle: u64 = 45;
        let kbg = 80;
        let bkb = 35;
        AttackModule::set_ice_frame_mul(agent.module_accessor, 0, 0.75, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, angle, kbg, 0, bkb, 5.0, 0.0, 6.5, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0.0, 8.0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("havel"), 1.0, 8.0, 0.0, 0, 0, -90, 0.37, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("popo_attack_arc_b"), Hash40::new("popo_attack_arc_b"), Hash40::new("top"), 0.5, 8, 1, 163.714, 220.144, -140.09, 1.35, true, *EF_FLIP_YZ);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_ice"),false,false);
        macros::EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("havel"), 0.0, 6.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::AFTER_IMAGE_OFF(agent, 0);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
        let vc = if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_POPO_INSTANCE_WORK_ID_FLAG_MAIN_FIGHTER_NANA)
        {Hash40::new("vc_popo_attack01")} else {Hash40::new("vc_nana_attack01")};
        macros::PLAY_SE(agent, vc);
    }
    wait(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_popo_swing_l"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        smash::app::sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(agent.lua_state_agent);

        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_catchspecialnana(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,16.0,10.0);
    frame(agent.lua_state_agent, 16.0);
    FT_MOTION_RATE_RANGE(agent,16.0,51.0,1.0);
    frame(agent.lua_state_agent, 51.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        println!("DUDE");
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 60, 0, 35, 5.0, 0.0, 6.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    }
}

unsafe extern "C" fn effect_catchspecialnana(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("popo_blizzerd_shoot"), Hash40::new("top"), 0, 5, 7, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_catchspecialnana(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        let vc = if true || !WorkModule::is_flag(agent.module_accessor, *FIGHTER_POPO_INSTANCE_WORK_ID_FLAG_MAIN_FIGHTER_NANA)
        {Hash40::new("vc_popo_attack04")} else {Hash40::new("vc_nana_attack04")};
        macros::PLAY_SE(agent, vc);
        macros::PLAY_SE(agent, Hash40::new("se_popo_special_l01"));
    }
}

unsafe extern "C" fn expression_catchspecialnana(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 4);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 40, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

/*
STATUS
*/
pub unsafe fn is_nana_near(fighter: &mut L2CFighterCommon) -> bool {
    let mut bVar2 = true;

    fighter.clear_lua_stack();
    lua_args!(fighter, Hash40::new_raw(0x253ce36631));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    bVar2 = fighter.pop_lua_stack(1).get_bool();
    println!("A: Partner dead");
    if !bVar2 {return false;}
    
    fighter.clear_lua_stack();
    lua_args!(fighter, Hash40::new_raw(0x2ac2788592));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    bVar2 = fighter.pop_lua_stack(1).get_bool();
    println!("B: Partner unreachable?");
    if !bVar2 {return false;}

    let dist = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("couple_distance"))*1.5;
    fighter.clear_lua_stack();
    lua_args!(fighter, Hash40::new_raw(0x290fb81a9f), dist);
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    bVar2 = fighter.pop_lua_stack(1).get_bool();
    println!("C: Partner too far?");
    if !bVar2 {return false;}

    println!("D: Partner Unlinked?");
    if !LinkModule::is_link(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER) {return false;}

    return true;
}
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_nana = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUB_FIGHTER);
    println!("Catch attack: {is_nana}");
    if !is_nana && catch_attack_check_special(fighter) {

        println!("Popo Spummel");
        if !is_nana_near(fighter) {return catch_attack_main_default(fighter);}
        println!("SUCCESS");
        
        //NANA VARS
        let partner_id = LinkModule::get_node_object_id(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER) as u32;
        let partner_boma = sv_battle_object::module_accessor(partner_id);
        let lr = PostureModule::lr(fighter.module_accessor);
        let pos = *PostureModule::pos(fighter.module_accessor);
        PostureModule::set_pos(partner_boma, &Vector3f{x:pos.x+(3.0*-lr),y:pos.y,z:pos.z});
        PostureModule::set_lr(partner_boma, lr);
        PostureModule::update_rot_y_lr(partner_boma);
        
        let partner_next_status = FIGHTER_STATUS_KIND_THROW;

        //DEFAULT POPO STUFF
        WorkModule::on_flag(partner_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL); 
        
        //DEFAULT NANA STUFF
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new_raw(0x254341e6cf),*FIGHTER_POPO_LINK_NO_PARTNER,Hash40::new_raw(0x254341e6cf),*partner_next_status);
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new_raw(0xf38368121), Hash40::new_raw(0x254341e6cf),
        *FIGHTER_POPO_LINK_NO_PARTNER,Hash40::new_raw(0x22fe7e65ef),false);
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);

        LinkModule::set_attribute(fighter.module_accessor, 
            *FIGHTER_POPO_LINK_NO_PARTNER, app::LinkAttribute{_address:*LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        LinkModule::set_attribute(fighter.module_accessor, 
            *FIGHTER_POPO_LINK_NO_PARTNER, app::LinkAttribute{_address:*LINK_ATTRIBUTE_REFERENCE_PARENT_ATTACK_STOP as u8}, true);

        //CUSTOM NANA STUFF
        StatusModule::change_status_force(partner_boma, *partner_next_status, false);

        //CUSTOM POPO STUFF
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchAttack_Main as *const () as _));
    }
    return catch_attack_main_default(fighter);
}

pub unsafe extern "C" fn throw_main_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_nana = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUB_FIGHTER);
    if (StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_CATCH_ATTACK || is_nana) 
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        println!("Spummel Throw Popo: {is_nana}");
        WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
        fighter.status_Throw_Sub();

        let motion = if !is_nana {Hash40::new("catch_special")} else {Hash40::new("catch_special_nana")};
        let frame = if !is_nana {0.0} else {0.0};
        let rate = if !is_nana {1.0} else {2.0};
        MotionModule::change_motion(fighter.module_accessor,motion, frame, rate, false, 0.0, false, false);

        if !is_nana {
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_POPO_STATUS_THROW_WORK_INT_STATE);
            return fighter.sub_shift_status_main(L2CValue::Ptr(throw_main_loop_uniq as *const () as _));
        }
    }
    else {
        fighter.status_Throw_Sub();
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Throw_Main as *const () as _))
}

unsafe extern "C" fn throw_main_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut state = WorkModule::get_int(fighter.module_accessor, FIGHTER_POPO_STATUS_THROW_WORK_INT_STATE);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_POPO_STATUS_THROW_FLAG_STALL) 
    && state < 2 {
        if state == 0 {
            state = 1;
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_POPO_STATUS_THROW_WORK_INT_STATE);
        }
        let has_nana = is_nana_near(fighter);
        if has_nana {
            let rate = (20.0 / (18.0 - 10.0));
            MotionModule::set_rate(fighter.module_accessor, rate);
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_POPO_STATUS_THROW_FLAG_STALL);
            WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_POPO_STATUS_THROW_WORK_INT_STATE);
        }
    }
    else if state == 1 {
        WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_POPO_STATUS_THROW_WORK_INT_STATE);
    }
    fighter.status_Throw_Main()
} 

unsafe extern "C" fn bliz_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    let owner_status = StatusModule::status_kind(owner);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL)
    && owner_status == *FIGHTER_STATUS_KIND_THROW {
        AttackModule::set_power_mul(weapon.module_accessor, 0.1);
        AttackModule::set_reaction_mul(weapon.module_accessor, 0.1);
    } 
    0.into()
}

pub fn install_helper(agent: &mut smashline::Agent, article: i32) {
    if article < *WEAPON_KIND_POPO_CONDOR {
        agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq);
        agent.status(Main, *FIGHTER_STATUS_KIND_THROW, throw_main_uniq);
    
        agent.acmd("game_catchspecial", game_catchspecial,Priority::Default);
        agent.acmd("effect_catchspecial", effect_catchspecial,Priority::Default);
        agent.acmd("sound_catchspecial", sound_catchspecial,Priority::Default);
        agent.acmd("expression_catchspecial", expression_catchspecial,Priority::Default);
    }
    else if article == *WEAPON_KIND_POPO_BLIZZARD {
        //agent.status(Init, *WEAPON_POPO_BLIZZARD_STATUS_KIND_DUMMY, bliz_init);
    }

    agent.install();
}
pub fn install() {   
    let popo = &mut Agent::new("popo");
    let nana = &mut Agent::new("nana");
    let p_blizzard = &mut Agent::new("popo_blizzard");
    let n_blizzard = &mut Agent::new("nana_blizzard");
    install_helper(popo,*FIGHTER_KIND_POPO);
    install_helper(nana,*FIGHTER_KIND_NANA);
    install_helper(p_blizzard,*WEAPON_KIND_POPO_BLIZZARD);
    install_helper(n_blizzard,*WEAPON_KIND_POPO_BLIZZARD);
}
