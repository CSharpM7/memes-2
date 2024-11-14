use crate::imports::imports_status::*;

pub unsafe extern "C" fn pik_deny_follow(weapon: &mut L2CWeaponCommon) -> bool {
    let prev_status = weapon.global_table[PREV_STATUS_KIND].get_i32();
    return [
        *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_LANDING,
        *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPIN_LANDING,
    ].contains(&prev_status);
}

pub unsafe extern "C" fn pik_ground_follow_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if pik_deny_follow(weapon) {
        WorkModule::set_int(weapon.module_accessor, 
            *WEAPON_PIKMIN_PIKMIN_OWNER_CONDITION_BORING_INDICATION, 
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_CURRENT
        ); /*
        WorkModule::set_int(weapon.module_accessor, 
            *WEAPON_PIKMIN_PIKMIN_OWNER_CONDITION_HIDE_INDICATION, 
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_FOLLOW
        ); */
        WorkModule::on_flag(weapon.module_accessor, 
            *WEAPON_PIKMIN_PIKMIN_STATUS_FOLLOW_COMMON_WORK_FLAG_IS_PERPLEXED
        );
        return 0.into();
    }
    return smashline::original_status(Init, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW)(weapon);
}
pub unsafe extern "C" fn pik_ground_follow_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if pik_deny_follow(weapon) {
        //StatusModule::set_status_kind_interrupt(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_BORING_WAIT);
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_BORING_WAIT.into(), true.into());
        return 0.into();
    }
    return smashline::original_status(Pre, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW)(weapon);
}

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

pub const THROW_TIMER: i32 = 0x11000007;
pub const THROW_CYCLE: i32 = 0x11000008;
pub const THROW_PIKMIN: i32 = 0x2100000D;
/*
STATUS
*/
pub unsafe extern "C" fn throw_pikmin(fighter: &mut L2CFighterCommon, p: i32) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    //FighterSpecializer_Pikmin::hold_pikmin(olima, 1);
    //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
    /* 
    let curr_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if curr_pikmin_num == 0 {
        println!("No pikmin");
        return false;
    }*/

    let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
    if pikmin_id == 0 {
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
        //FighterSpecializer_Pikmin::reduce_pikmin_all(olima); //apparently this fucks shit up
        println!("No id");
        return false;
    }
    let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;

    let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
    println!("Throw Pikmin #{p}: {pikmin_id} ?") ;
    //p[0] is tossed, any others arent
    if is_link & 1 != 0  {
        println!("Toss em");
        let pikmin_boma = sv_battle_object::module_accessor(pikmin_id as u32);
        let p_situation = //StatusModule::situation_kind(pikmin_boma);
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R) as u64 == hash40("appeal_hi_r") 
        {*SITUATION_KIND_AIR} else {*SITUATION_KIND_GROUND};
        /*
        let mut link_event = FighterPikminLinkEventWeaponPikminConstraint__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_constraint")));
        link_event["owner_joint_id_"].assign(&L2CValue::Hash40(Hash40::new("top")));
        link_event["joint_id_"].assign(&L2CValue::Hash40(Hash40::new("top")));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_ATTACK_STOP as u8}, true);

        let mut link_event = FighterPikminLinkEventWeaponPikminChangeMotion__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_motion")));
        let motion = //MotionModule::motion_kind(fighter.module_accessor);
        link_event["motion_kind_"].assign(&L2CValue::Hash40(Hash40::new("sp_s_thrown_star")));
        link_event["start_frame_"].assign(&L2CValue::F32(0.0));
        link_event["rate_"].assign(&L2CValue::F32(1.0));
        link_event["loop_"].assign(&L2CValue::Bool(false));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        */
        let lr = PostureModule::lr(fighter.module_accessor);
        let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));
        link_event["lr_"].assign(&L2CValue::F32(lr));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_ATTACK_STOP as u8}, true);

        //PostureModule::set_lr(pikmin_boma,lr);
        //PostureModule::update_rot_y_lr(pikmin_boma);

        //WEAPON_PIKMIN_PIKMIN_STATUS_KIND_ATTACK_LW4 doesnt tell the pikmin to follow olimar again
        let new_status = if p_situation == *SITUATION_KIND_GROUND {*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_ATTACK_LW4} else {*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S};
        let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
        link_event["status_kind_"].assign(&L2CValue::I32(new_status));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        
        LinkModule::unlink(fighter.module_accessor, link_node);
        return true;
    }
    else {
        println!("lost pikid");
    }
    return false;
}

pub unsafe extern "C" fn catch_attack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 1, THROW_TIMER);
    WorkModule::set_int(fighter.module_accessor, 0, THROW_CYCLE);
    WorkModule::off_flag(fighter.module_accessor, THROW_PIKMIN);
    
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;

    FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
    FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if hold_pikmin_num == 0 {
        //solimar
            println!("Solimar");
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
    //LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN);
    println!("End loop");

    0.into()
}


pub fn install() {   
    Agent::new("pikmin_pikmin")
        .status(Init, *FIGHTER_STATUS_KIND_APPEAL, catch_attack_init)
        //.status(Exec, *FIGHTER_STATUS_KIND_APPEAL, catch_attack_exec)
        .status(Init, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW, pik_ground_follow_init)
        .status(Pre, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW, pik_ground_follow_pre)
    .install();
}