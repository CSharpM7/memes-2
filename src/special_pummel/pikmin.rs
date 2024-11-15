use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app44FighterPikminLinkEventWeaponPikminConstraint13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminConstraint__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app46FighterPikminLinkEventWeaponPikminChangeMotion13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminChangeMotion__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app46FighterPikminLinkEventWeaponPikminChangeStatus13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app40FighterPikminLinkEventWeaponPikminSyncLR13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app40FighterPikminLinkEventWeaponPikminSyncPos13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminSyncPos__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
    fn new_event_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventE"]
    fn store_event_table(event: *const app::LinkEvent) -> L2CValue;
}
unsafe extern "C" fn link_event_store_l2c_table(fighter: &mut L2CFighterCommon, link_no: L2CValue, event: L2CValue) -> L2CValue {
    let callable: extern "C" fn() -> *mut app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_parents_struct(fighter.module_accessor, link_no.get_i32(), link_event);
    let ret = store_event_table(link_event);
    let deleter: extern "C" fn(*mut app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    ret
}
pub unsafe  fn pikmin_variantion_to_string(variation: i32) -> &'static str {
    /*
    
    WEAPON_PIKMIN_PIKMIN_VARIATION_BLUE: 0x2,
    WEAPON_PIKMIN_PIKMIN_VARIATION_RED: 0x0,
    WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET: 0x4,
    WEAPON_PIKMIN_PIKMIN_VARIATION_WHITE: 0x3,
    WEAPON_PIKMIN_PIKMIN_VARIATION_YELLOW: 0x1,
     */
    return match variation {
        0 => {"red"}
        1 => {"yellow"}
        2 => {"blue"}
        3 => {"white"}
        4 => {"purple"}
        _ => {"?"}
    };
}

pub const FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN: i32 = 0x1100000E;
pub const FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT: i32 = 0x1100000F;
pub const FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT_MAX: i32 = 0x11000010;
pub const FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID: i32 = 0x11000011; //FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT can be used for throws
pub const FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE: i32 = 0x2100000F;
pub const FIGHTER_PIKMIN_STATUS_THROW_FLAG_DISABLE_CLATTER: i32 = 0x21000010;
pub const WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_DISABLE_CLATTER: i32 = 0x21000010;
/*
    FIGHTER_STATUS_CATCH_CUT_WORK_INT_SITUATION: 0x11000005,
    FIGHTER_STATUS_CATCH_DASH_WORK_INT_CATCH_TURN_FRAME: 0x11000005,
    FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT: 0x2100000C,
    FIGHTER_STATUS_CATCH_PULL_FLAG_UNNECESSARY_CLEAR_TRANS: 0x2100000C,
    FIGHTER_STATUS_CATCH_PULL_WORK_INT_MOTION_KIND: 0x11000005,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_IK_LEFT_JOINT_ID: 0x11000007,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_IK_RIGHT_JOINT_ID: 0x11000008,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS: 0x11000006,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND: 0x11000005,

FIGHTER_STATUS_THROW_FLAG_INVINCIBLE: 0x2100000D,
    FIGHTER_STATUS_THROW_FLAG_START_AIR: 0x2100000C,
    FIGHTER_STATUS_THROW_FLAG_STOP: 0x2100000E,
    FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE: 0x1000007,
    FIGHTER_STATUS_THROW_WORK_INT_STOP_FRAME: 0x1100000D,
    FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP: 0x1100000B,
    FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO: 0x1100000C,
    FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT: 0x1100000A,
    
    WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_MOTION_STARTED: 0x21000000,
*/
/*
ACMD
 */
unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 361, 25, 0, 30, 3.5, 0.0, 5.0, 13.5, Some(0.0), Some(5.0), Some(9.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        //AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE);
    }
    frame(agent.lua_state_agent, 9.0); 
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CLATTER);
        //WorkModule::on_flag(agent.module_accessor, FIGHTER_PIKMIN_STATUS_THROW_FLAG_DISABLE_CLATTER);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pikmin_order"), Hash40::new("s_antenna4"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("pikmin_seiretsu"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_special_l01"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn pikmin_catchspecial(agent: &mut L2CAgentBase) {
    let mut variation = 0;
    let mut variation_as_str = "r";
    let mut is_sub = false;
    if macros::is_excute(agent) {
        variation = WorkModule::get_int(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        variation_as_str = pikmin_variantion_to_string(variation);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 340, 0, 10, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        //macros::ATTACK_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 361, 0, 10, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        is_sub = WorkModule::get_int(agent.module_accessor,*WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_MOTION_START_DELAY_FRAME) != 0;
        println!("{variation_as_str} is Locked and loaded: {}",!is_sub);

        WorkModule::on_flag(agent.module_accessor, WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_DISABLE_CLATTER);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    if !is_sub {
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 315, 0, 10, 140, 2.0, 0.0, 0.0, 1.6, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

/*
STATUS
*/
pub unsafe extern "C" fn throw_pikmin(fighter: &mut L2CFighterCommon, p: i32) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let oLRmar = PostureModule::lr(fighter.module_accessor);
    let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
    if pikmin_id == 0 {
        println!("No id");
        return false;
    }

    let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;
    let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
    //p[0] is tossed, any others arent
    if is_link & 1 != 0  {
        let pikmin_boma = sv_battle_object::module_accessor(pikmin_id as u32);
        let variation = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        let variation_as_str = pikmin_variantion_to_string(variation);
        println!("Throw {variation_as_str} Pikmin (#{p})");

        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_FAILURE_GROUND_FOLLOW_FORCE);
        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_FAILURE_WAIT_END);
        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_RETURN_END);

        let mut original_pos = *PostureModule::pos(pikmin_boma);
        let mut pikmin_pos = *PostureModule::pos(pikmin_boma);
        let mut target_pos = *PostureModule::pos(pikmin_boma);
        
        //This shouldnt be in a loop but whatever
        let mut capture_id =  WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID) as u32;
        /*
        if capture_id == OBJECT_ID_NULL {
            capture_id = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
            if capture_id != OBJECT_ID_NULL {
                WorkModule::set_int(fighter.module_accessor,capture_id as i32, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID);
            }
        } */
        if capture_id != OBJECT_ID_NULL {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
            ModelModule::joint_global_position(capture_boma, Hash40::new("hip"), hip_pos, false);
            target_pos = Vector3f{x: hip_pos.x, y: hip_pos.y, z: hip_pos.z};

            let i = (((p-1) as f32) * 6.0) + (oLRmar); 
            pikmin_pos = Vector3f{x: target_pos.x + i, y: pikmin_pos.y, z: pikmin_pos.z};
        }

        let pikmin_lr = (target_pos.x-pikmin_pos.x).signum();
        
        println!("Moved from {} to {}. Face {pikmin_lr} to {}",original_pos.x,pikmin_pos.x,target_pos.x);
        //PostureModule::set_pos(pikmin_boma, &pikmin_pos);
        
        WorkModule::set_float(pikmin_boma, target_pos.x, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_TARGET_X);
        WorkModule::set_float(pikmin_boma, target_pos.y,*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_TARGET_Y);

        let new_status = *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S;
        let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));
        link_event["lr_"].assign(&L2CValue::F32(pikmin_lr));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_ATTACK_STOP as u8}, true);

        let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
        link_event["status_kind_"].assign(&L2CValue::I32(new_status));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);

        //StatusModule::change_status_force(pikmin_boma, new_status, false);
        LinkModule::unlink(fighter.module_accessor, link_node);
        return true;
    }
    else {
        println!("lost pikid");
    }
    return false;
}
pub unsafe extern "C" fn throw_pikmin_all(fighter: &mut L2CFighterCommon) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if hold_pikmin_num == 0 {
        //solimar
        println!("How tf are you Solimar during grab??");
        return false;
    }
    println!("Holding {hold_pikmin_num} pikmin");
    let iVar14 = hold_pikmin_num-1;
    let mut p = 0;
    let mut lead_pikmin_id = OBJECT_ID_NULL;
    for _ in 0..4 {
        if !throw_pikmin(fighter,p) {
            break;
        }
        let bVar3 = p < iVar14;
        println!("{p} < {iVar14} ? ");
        if !bVar3 {break;}
        
        //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        p=p+1;
    }
    return true;
}


pub unsafe extern "C" fn catch_attack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special(fighter) {
        println!("Spummel init?");
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL); 

        WorkModule::set_int(fighter.module_accessor, OBJECT_ID_NULL as i32, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID);

        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT);
        /* 
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if hold_pikmin_num == 0 {
            //solimar
            println!("How tf are you Solimar during grab??");
            return 0.into();
        }
        println!("Holding {hold_pikmin_num} pikmin");
        let iVar14 = hold_pikmin_num-1;
        let mut p = 0;
        let mut lead_pikmin_id = OBJECT_ID_NULL;
        loop {
            if !throw_pikmin(fighter,p) {
                break;
            }
            let bVar3 = p < iVar14;
            println!("{p} < {iVar14} ? ");
            if !bVar3 {break;}
            
            FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
            FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
            p=p+1;
        }
        
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
        //FighterSpecializer_Pikmin::reduce_pikmin_all(olima); //apparently this fucks shit up
        */
        //return 0.into();
    }
    return 0.into();
    //return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
}

pub unsafe extern "C" fn catch_attack_init_variables(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, OBJECT_ID_NULL as i32, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN);
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    
    //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
    //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    println!("Init Holding: {hold_pikmin_num}");
    WorkModule::set_int(fighter.module_accessor, hold_pikmin_num-1, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT);

    if hold_pikmin_num == 0 { return; }  
    let lead_pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;
    if lead_pikmin_id == 0 { return; }

    let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN_THROW;
    let is_link = LinkModule::link(fighter.module_accessor, link_node, lead_pikmin_id as u32);
    if is_link & 1 != 0  {
        let pikmin_boma = sv_battle_object::module_accessor(lead_pikmin_id as u32);
        let capture_id = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
        if capture_id != OBJECT_ID_NULL {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let target_pos = *PostureModule::pos(capture_boma);
            println!("Has target at {}",target_pos.x);
            
            let target_const = if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW 
            {*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT} else {FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID};
            WorkModule::set_int(fighter.module_accessor, capture_id as i32, target_const);
        }
        LinkModule::unlink(fighter.module_accessor, link_node);
    } 
}
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL); 
        println!("SpummelMar"); 

        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM) <= 0 {
            println!("Solimar");
            return fighter.status_CatchAttack();
        }
 
        //catch_attack_init_variables(fighter);
        //let capture_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID) as u32;
        //if capture_id != OBJECT_ID_NULL {
        //    println!("Has opponent");
        //}

        //fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
        //return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchAttack_Main as *const () as _));
        /*
        throw_pikmin_all(fighter);
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima); */

        WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);  
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchAttack_Main as *const () as _));
    }
    
    return fighter.status_CatchAttack();
}

pub unsafe extern "C" fn catch_attack_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {

    return catch_attack_main_default_loop(fighter);
}

pub unsafe extern "C" fn throw_main_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_CATCH_ATTACK 
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        println!("Spummel Throw");
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;

        //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        catch_attack_init_variables(fighter);
        //throw_pikmin_all(fighter);

        WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
        //WorkModule::set_int(fighter.module_accessor, 3, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN);
        //fighter.status_Throw_Sub();

        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);
        return fighter.sub_shift_status_main(L2CValue::Ptr(throw_sp_main_loop_uniq as *const () as _))
    }
    else {
        fighter.status_Throw_Sub();
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Throw_Main as *const () as _))
}

pub unsafe extern "C" fn throw_sp_main_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*
    let capture_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT) as u32;
    if capture_id != OBJECT_ID_NULL {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        let mut clatter = ControlModule::get_clatter_time(capture_boma, 0);
        println!("Clatter: {clatter}");
        if clatter <= 0.0 && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CLATTER) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(),false.into());
            StatusModule::change_status_request(capture_boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, false);
        }
    }
 */
    /* 
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if !fighter.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::count_down_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN, 0) {
        println!("One more");
        throw_pikmin(fighter,0);
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
    }*/

    fighter.status_Throw_Main()
}

pub unsafe extern "C" fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    /* 
    if WorkModule::count_down_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN, 0) {
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    
        FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if hold_pikmin_num == 0 {
            //solimar
            println!("Solimar");
            return 0.into();
        }
        //println!("Holding {hold_pikmin_num} pikmin");

        let p = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT);
        let could_throw = throw_pikmin(fighter,p);
        if could_throw {
            WorkModule::set_int(fighter.module_accessor, 3, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN);
        }
        else {
            FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
        }
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT);
        
        println!("ONe more?");
        throw_pikmin_all(fighter);
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
    }*/ 
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE);
        //WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN);
        println!("CHARGE");
        /*
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        throw_pikmin_all(fighter);
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
*/

        let capture_id = WorkModule::get_int64(fighter.module_accessor,FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID) as u32;
        if capture_id != OBJECT_ID_NULL {
            let capture_boma = sv_battle_object::module_accessor(capture_id);
            StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_CAPTURE_CUT, false);
        }  
        //fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
    }
    return 0.into();
}


pub unsafe extern "C" fn pikmin_special_s_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S)(weapon);

    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        let hold_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_INDEX);
        let variation = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        let variation_as_str = pikmin_variantion_to_string(variation);

        //KineticModule::change_kinetic(weapon.module_accessor,*WEAPON_KINETIC_TYPE_NONE);
        KineticModule::clear_speed_all(weapon.module_accessor);
        //PostureModule::add_pos(weapon.module_accessor, &Vector3f::new(0.0,0.25,0.0));
        let lr = PostureModule::lr(weapon.module_accessor);
        let pos = *PostureModule::pos(weapon.module_accessor);


        let target_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_TARGET_X);
        let target_y = WorkModule::get_float(weapon.module_accessor,*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_TARGET_Y);
        
        let mut direction_full = Vector2f{x:target_x-pos.x, y: (target_y-pos.y)};
        let direction_len = sv_math::vec2_length(direction_full.x,direction_full.y);
        let direction = Vector2f{x:direction_full.x/direction_len,y:direction_full.y/direction_len};
        let speed = 3.0;

        let speed_x = direction.x*speed;
        let speed_y = direction.y*speed;


        println!("{variation_as_str} Pikmin (#{hold_num}) Speed: {speed_x},{speed_y}");
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_x,//*lr,
            speed_y
        );
    }
    
    return original;
}


pub unsafe extern "C" fn pikmin_throw_f_sp(weapon: &mut L2CWeaponCommon,sub: bool) -> L2CValue {
    let hold_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_INDEX);
    let variation = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let variation_as_str = pikmin_variantion_to_string(variation);
    println!("FThrow: {variation_as_str} Pikmin (#{hold_num}): {sub} ");
    WorkModule::set_int(weapon.module_accessor, if sub {1} else {0}, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_MOTION_START_DELAY_FRAME);

    if !sub && false {
        WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    }
    let motion = Hash40::new("catch_special");
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);
    
    if !sub && false {
        WorkModule::set_int(weapon.module_accessor, variation, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    }
    //AttackModule::set_power_up(weapon.module_accessor,0.0);
    WorkModule::set_int64(weapon.module_accessor, motion.hash as i64, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_MOTION_KIND);
    WorkModule::on_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_MOTION_STARTED);
    

    if !sub {
        let capture_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
        if capture_id != OBJECT_ID_NULL {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            MotionModule::change_motion(capture_boma, Hash40::new("damage_lw_3"), 0.0, 1.0, false, 0.0, false, false);
        }
    }

    weapon.fastshift(L2CValue::Ptr(pikmin_throw_f_sp_loop as *const () as _))
}

pub unsafe extern "C" fn pikmin_throw_f_sp_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let sub = WorkModule::get_int(weapon.module_accessor,*WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_MOTION_START_DELAY_FRAME) == 1;
    //if !StatusModule::is_changing(weapon.module_accessor)
    //&& StatusModule::is_situation_changed(weapon.module_accessor) {
        if !weapon.is_grounded() {
            weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_FALL.into(), false.into());
        }
    //}
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW.into(), false.into());
        return 1.into();
    }

    if CatchModule::is_catch(weapon.module_accessor) {
        let capture_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
        if capture_id != OBJECT_ID_NULL {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let mut clatter = ControlModule::get_clatter_time(capture_boma, 0);
            //if !sub {println!("Clatter: {clatter}");}
            if clatter <= 0.0 && !WorkModule::is_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_DISABLE_CLATTER) {
                let new_status = if !sub {WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT} else {WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT_SUB};
                weapon.change_status(new_status.into(),false.into());
                StatusModule::change_status_request(capture_boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, false);
                let owner = get_owner_boma(weapon);
                WorkModule::on_flag(owner, *FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_CATCH_CUT);
            }
            /* 
            else if WorkModule::is_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_DISABLE_CLATTER) {
                let p_frame = WorkModule::get_int(capture_boma,*FIGHTER_INSTANCE_WORK_ID_INT_INVALID_PARALYZE_FRAME);
                println!("P: {p_frame}");
                if p_frame > 1 {
                    WorkModule::set_int(capture_boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_PARALYZE_FRAME);
                }
            }*/
        }
        else {
            println!("HUH");
        }
    }
    else if !sub {
        //println!("Thrown");
    }

    0.into()
}

pub unsafe extern "C" fn pikmin_throw_f_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        return pikmin_throw_f_sp(weapon,false);
    }
    let original = smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F)(weapon);
    return original;
}
pub unsafe extern "C" fn pikmin_throw_f_sub_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        return pikmin_throw_f_sp(weapon,true);
    }
    let original = smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F_SUB)(weapon);
    return original;
}

pub fn install() {
    smashline::Agent::new("pikmin")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        //.status(Init, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_init)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Main, *FIGHTER_STATUS_KIND_THROW, throw_main_uniq)
        //.status(Exec, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_exec)
    .install();
    smashline::Agent::new("pikmin_pikmin")
        .acmd("game_catchspecial", pikmin_catchspecial,Priority::Default)
        .acmd("game_catchspecial_y", pikmin_catchspecial,Priority::Default)
        .acmd("game_catchspecial_b", pikmin_catchspecial,Priority::Default)
        .acmd("game_catchspecial_w", pikmin_catchspecial,Priority::Default)
        .acmd("game_catchspecial_v", pikmin_catchspecial,Priority::Default)

        .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S, pikmin_special_s_main)
        .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F, pikmin_throw_f_main)
        .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F_SUB, pikmin_throw_f_sub_main)
    .install();
}
