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
pub const FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID: i32 = 0x100000CA; //FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT can be used for throws
pub const FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN: i32 = 0x200000CB;
pub const FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE: i32 = 0x2100000F;
pub const FIGHTER_PIKMIN_STATUS_THROW_FLAG_DISABLE_CLATTER: i32 = 0x21000010;
pub const FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_X: i32 = 0x4E;
pub const FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_Y: i32 = 0x4F;

pub const WEAPON_PIKMIN_PIKMIN_MAX_CHARGE_RANGE: f32 = 70.0;
pub const WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_DISABLE_CLATTER: i32 = 0x21000010;
pub const WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_DISABLE_CHARGE_CHECK: i32 = 0x2000000E;
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
    
    FIGHTER_PIKMIN_INSTANCE_ATTACK_AIR_WORK_FLAG_FALL_SPECIAL: 0x200000E7,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_CATCH_CUT: 0x200000E8,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_CHANGE_CATCH_MOTION_RATE: 0x200000EA,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPYCLOAK: 0x200000EE,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR: 0x200000E1,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND: 0x200000E2,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END: 0x200000E3,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_PIKMIN_CATCH_DASH_STATUS: 0x200000EB,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CATCH_MOTION_RATE: 0x4D,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME: 0x4C,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM: 0x100000C5,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0: 0x100000C6,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION: 0x100000C9,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_WING_PIKMIN_END_EFFECT_HANDLE: 0x100000C1,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_WING_PIKMIN_END_FRAME_COUNT: 0x100000C2,
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
    }
    frame(agent.lua_state_agent, 4.0); 
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CLATTER);
        //WorkModule::on_flag(agent.module_accessor, FIGHTER_PIKMIN_STATUS_THROW_FLAG_DISABLE_CLATTER);
    }
    frame(agent.lua_state_agent, 5.0); 
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE);
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
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 0, 10, 60, 2.0, 0.0, 0.0, 1.6, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND,false);
    }
}

/*
STATUS
*/
pub unsafe extern "C" fn change_status_pikmin(fighter: &mut L2CFighterCommon, p: i32, status: i32, force: bool) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let oLRmar = PostureModule::lr(fighter.module_accessor);
    let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
    if pikmin_id == 0 {
        println!("No id");
        return false;
    }

    let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;
    let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
    if is_link & 1 != 0  {
        let pikmin_boma = sv_battle_object::module_accessor(pikmin_id as u32);
        let variation = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        let variation_as_str = pikmin_variantion_to_string(variation);
        println!("Change {variation_as_str} Pikmin (#{p})");

        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_FAILURE_GROUND_FOLLOW_FORCE);
        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_FAILURE_WAIT_END);
        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_RETURN_END);
         
        let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
        link_event["status_kind_"].assign(&L2CValue::I32(status));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        
        if force {StatusModule::change_status_force(pikmin_boma, status, false);}

        LinkModule::unlink(fighter.module_accessor, link_node);
        return true;
    }
    else {
        println!("lost pikid");
    }
    return false;
}

pub unsafe extern "C" fn change_status_pikmin_all(fighter: &mut L2CFighterCommon, status: i32, force: bool) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    println!("Holding {hold_pikmin_num} pikmin");
    if hold_pikmin_num <= 0 {return false;}
    let mut p = 0;
    let mut lead_pikmin_id = OBJECT_ID_NULL;
    for p in 0..hold_pikmin_num {
        if !change_status_pikmin(fighter,p,status, force) {
            break;
        }        
        //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
    }
    return true;
}

pub unsafe extern "C" fn liberate_pikmin_all(fighter: &mut L2CFighterCommon) -> bool {
    let reduce_instead = false;
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    println!("Freeing {hold_pikmin_num} pikmin");
    if hold_pikmin_num <= 0 {return false;}
    let mut p = 0;
    let mut lead_pikmin_id = OBJECT_ID_NULL;
    for p in 0..hold_pikmin_num {
        println!("Pik: {p}");
        let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
        let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;
        let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
        if is_link & 1 != 0  {
            //make em fall?
            LinkModule::unlink(fighter.module_accessor, link_node);
        }
        if reduce_instead {
            FighterSpecializer_Pikmin::reduce_pikmin_all(olima);
        } else {
            FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
        }
    }
    return true;
}

pub unsafe extern "C" fn catch_attack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special(fighter) {
        println!("Spummel init?");
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL); 

        WorkModule::set_int(fighter.module_accessor, OBJECT_ID_NULL as i32, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID);

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
            if !change_status_pikmin(fighter,p) {
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
    WorkModule::set_int(fighter.module_accessor, OBJECT_ID_NULL as i32, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID);
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
            println!("Has target {capture_id} at {}",target_pos.x);
            
            WorkModule::set_int(fighter.module_accessor, capture_id as i32, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID);
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW{
                WorkModule::set_int(fighter.module_accessor, capture_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            }
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
 
        catch_attack_init_variables(fighter);
        //let capture_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID) as u32;
        //if capture_id != OBJECT_ID_NULL {
        //    println!("Has opponent");
        //}

        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
        return fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_loop_uniq as *const () as _));
        /*
        change_status_pikmin_all(fighter);
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
         */
        /* 
        WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);  
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchAttack_Main as *const () as _));
        */
    }
    
    return fighter.status_CatchAttack();
}

pub unsafe extern "C" fn catch_attack_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let capture_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID) as u32;
    let disable_clatter = WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CLATTER);
    if capture_id != OBJECT_ID_NULL {
        let opponent = sv_battle_object::module_accessor(capture_id as u32);
        WorkModule::off_flag(opponent,*FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_JUMP);
    
        let mut clatter = ControlModule::get_clatter_time(opponent, 0);
        //println!("Clatter: {clatter}");
        if disable_clatter {
            clatter = WorkModule::get_float(fighter.module_accessor,FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
            ControlModule::set_clatter_time(opponent, clatter,0);
        }
        else {
            WorkModule::set_float(fighter.module_accessor, clatter, FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
        }

        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN) == 0 {
            let hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
            ModelModule::joint_global_position(opponent, Hash40::new("hip"), hip_pos, false);
            let mut target_x = hip_pos.x;
            let mut target_y = hip_pos.y;
            if target_x == 0.0 && target_y == 0.0 {
                target_x = PostureModule::pos_x(opponent);
                target_y = PostureModule::pos_y(opponent) + 5.0;
            }
            WorkModule::set_float(fighter.module_accessor, target_x, FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_X);
            WorkModule::set_float(fighter.module_accessor, target_y, FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_Y);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CLATTER);
        //WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN);
        //WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN);
        
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        println!("CHARGE: {hold_pikmin_num}");
        change_status_pikmin_all(fighter,*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S,true);
        //FighterSpecializer_Pikmin::liberty_pikmin_all(olima);

        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return 1.into();
    }
    return 0.into();//fighter.status_CatchAttack_Main();
}

pub unsafe extern "C" fn throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        println!("UH");
        return 0.into();}
    return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_THROW)(fighter);
}
pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = MotionModule::frame(fighter.module_accessor);
    let current_rate = MotionModule::rate(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) 
    {
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        //MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, current_frame, true, true, false);
        //MotionModule::set_rate(fighter.module_accessor,current_rate);
        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), current_frame,current_rate, false, 0.0, false, false);
        //
        //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        //liberate_pikmin_all(fighter);
        //let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        //println!("Throw hold {hold_pikmin_num}");

        //change_status_pikmin(fighter,0,*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND);
        //liberate_pikmin_all(fighter);
        //FighterSpecializer_Pikmin::sort_pikmin_no_change_status(olima);

        return fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_catch_main as *const () as _));
    }
    
    fighter.status_Throw()
}

pub unsafe extern "C" fn special_lw_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if !fighter.is_grounded() {
            fighter.sub_set_ground_correct_by_situation(false.into());
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
            fighter.sub_change_motion_by_situation(Hash40::new("special_n").into(), Hash40::new("special_air_n").into(), true.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
    if status_frame == 1 {
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::hold_pikmin(olima, 1);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        change_status_pikmin(fighter,0,*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S,true);
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
        println!("One more! Has {hold_pikmin_num}");
    }


    0.into()
}

pub unsafe extern "C" fn pikmin_special_s_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S)(weapon);

    let owner = get_owner_boma(weapon);
    //if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
    if StatusModule::prev_status_kind(weapon.module_accessor, 0) == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND {
        
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);

        let hold_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_INDEX);
        let variation = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        let variation_as_str = pikmin_variantion_to_string(variation);

        //KineticModule::change_kinetic(weapon.module_accessor,*WEAPON_KINETIC_TYPE_NONE);
        KineticModule::clear_speed_all(weapon.module_accessor);
        //PostureModule::add_pos(weapon.module_accessor, &Vector3f::new(0.0,0.25,0.0));
        let lr = PostureModule::lr(weapon.module_accessor);
        let pos = *PostureModule::pos(weapon.module_accessor);

        //let mut target_x = PostureModule::pos_x(weapon.module_accessor) + PostureModule::lr(weapon.module_accessor);
        //let mut target_y = PostureModule::pos_y(weapon.module_accessor) + 1.0;
        //let target_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_TARGET_X);
        //let target_y = WorkModule::get_float(weapon.module_accessor,*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_TARGET_Y);
        let x_offset = (hold_num-1) as f32 * 1.0;
        let y_offset = (hold_num-1) as f32 * 1.0;
        let target_x = WorkModule::get_float(owner, FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_X)+x_offset;
        let target_y = WorkModule::get_float(owner,FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_Y)+y_offset;
        let target_id = WorkModule::get_int(owner, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID) as u32;
        /*
        if target_id != OBJECT_ID_NULL {
            let target_boma = sv_battle_object::module_accessor(target_id);
            let target_pos = *PostureModule::pos(target_boma);
            target_x = target_pos.x;
            target_y = target_pos.y;
        } */
        //println!("Homing in on {target_id} at {target_x},{target_y}");
        println!("{} > {target_y}",pos.y);
        
        let mut direction_full = Vector2f{x:target_x-pos.x, y: (target_y-pos.y)};
        let direction_len = sv_math::vec2_length(direction_full.x,direction_full.y);
        let direction = Vector2f{x:direction_full.x/direction_len,y:direction_full.y/direction_len};
        let speed = (direction_len*0.1).clamp(1.0, 6.0);

        let speed_x = direction.x*speed;
        let speed_y = (direction.y*speed).max(1.0);


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


pub unsafe extern "C" fn pikmin_catch_cut_pre_inner(weapon: &mut L2CWeaponCommon) -> bool {
    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) 
    && StatusModule::status_kind(owner) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        StatusModule::set_status_kind_interrupt(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_AIR_FOLLOW);
        //weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND.into(),false.into());
        return true;
    }
    return false;
}
pub unsafe extern "C" fn pikmin_catch_cut_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if pikmin_catch_cut_pre_inner(weapon) {return 1.into();}
    let original = smashline::original_status(Pre, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT)(weapon);
    return original;
}
pub unsafe extern "C" fn pikmin_catch_cut_pre_sub(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if pikmin_catch_cut_pre_inner(weapon) {return 1.into();}
    let original = smashline::original_status(Pre, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT_SUB)(weapon);
    return original;
}
pub unsafe extern "C" fn pikmin_cling_remove_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if pikmin_catch_cut_pre_inner(weapon) {return 1.into();}
    let original = smashline::original_status(Pre, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE)(weapon);
    return original;
}
pub unsafe extern "C" fn pikmin_fall_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //TODO: figure out distance between pik and oli
    if pikmin_catch_cut_pre_inner(weapon) {return 1.into();}
    let original = smashline::original_status(Pre, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_FALL)(weapon);
    return original;
}



pub unsafe extern "C" fn pikmin_special_lw_respond_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("sp_lw_respond"), 0.0, 2.0, false, 0.0, false, false);
        return weapon.fastshift(L2CValue::Ptr(pikmin_special_lw_respond_loop as *const () as _)); 
    }
    return smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND)(weapon);
}

pub unsafe extern "C" fn pikmin_special_lw_respond_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn olimar_frame(fighter: &mut L2CFighterCommon)  {
    let charge = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN);
    if charge > 0 {
        println!("Charge: {charge}");
    }
    WorkModule::count_down_int(fighter.module_accessor, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN, 0);
}
pub unsafe extern "C" fn pikmin_frame(weapon: &mut L2CWeaponCommon)  {
    let status = StatusModule::status_kind(weapon.module_accessor);
    let listen_to_charge =
    (*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_WAIT..*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_TURN_WAIT).contains(&status)
    || (*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW..*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_JUMP_AERIAL).contains(&status)
    || (*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING..*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE).contains(&status)
    || *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT == status
    || *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_ATTACK == status
    || *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT_SUB == status
    ;
    if listen_to_charge {
        let owner = get_owner_boma(weapon);
        if WorkModule::get_int(owner, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN) > 0 
        && !WorkModule::is_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_DISABLE_CHARGE_CHECK) {
            WorkModule::on_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_DISABLE_CHARGE_CHECK);

            let pos = *PostureModule::pos(weapon.module_accessor);
            let target_x = WorkModule::get_float(owner, FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_X);
            let target_y = WorkModule::get_float(owner,FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_Y);
        
            let mut direction_full = Vector2f{x:target_x-pos.x, y: (target_y-pos.y)};
            let direction_len = sv_math::vec2_length(direction_full.x,direction_full.y);

            if direction_len < WEAPON_PIKMIN_PIKMIN_MAX_CHARGE_RANGE {
                WorkModule::on_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);
                weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND.into(),false.into());
            }
        }
    }
    else {
        WorkModule::off_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_DISABLE_CHARGE_CHECK);
    }
    //debug(weapon);
}
pub unsafe extern "C" fn debug(weapon: &mut L2CWeaponCommon)  {
    let hold_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_INDEX);
    let variation = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let variation_as_str = pikmin_variantion_to_string(variation);
    if variation == 0 {
        if MotionModule::frame(weapon.module_accessor) < 1.0 {
            let status = StatusModule::status_kind(weapon.module_accessor);
            println!("{variation_as_str} status: {status}");
        }
    }
}

pub fn install() {
    smashline::Agent::new("pikmin")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Init, *FIGHTER_STATUS_KIND_THROW, throw_init)
        .status(Main, *FIGHTER_STATUS_KIND_THROW, special_lw_main)
        .on_line(Main, olimar_frame)
    .install();
    smashline::Agent::new("pikmin_pikmin")
        .acmd("game_catchspecial", pikmin_catchspecial,Priority::Default)
        .acmd("game_catchspecial_y", pikmin_catchspecial,Priority::Default)
        .acmd("game_catchspecial_b", pikmin_catchspecial,Priority::Default)
        .acmd("game_catchspecial_w", pikmin_catchspecial,Priority::Default)
        .acmd("game_catchspecial_v", pikmin_catchspecial,Priority::Default)

        .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S, pikmin_special_s_main)

        .status(Pre, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT, pikmin_catch_cut_pre)
        //.status(Pre, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT_SUB, pikmin_catch_cut_pre_sub)
        //.status(Pre, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE, pikmin_cling_remove_pre)
        //.status(Pre, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_FALL, pikmin_fall_pre)

        .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND, pikmin_special_lw_respond_main)
        .on_line(Main, pikmin_frame)
    .install();
}
