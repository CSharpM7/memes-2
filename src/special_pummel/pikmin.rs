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

pub const FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN: i32 = 0x11000007;
pub const FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT: i32 = 0x11000008;
pub const FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID: i32 = 0x11000009;
pub const FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE: i32 = 0x2100000D;
/*
ACMD
 */
unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 361, 25, 0, 30, 3.5, 0.0, 5.0, 13.5, Some(0.0), Some(5.0), Some(9.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //WorkModule::on_flag(agent.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
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


/*
STATUS
*/
pub unsafe extern "C" fn throw_pikmin(fighter: &mut L2CFighterCommon, p: i32) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
    if pikmin_id == 0 {
        println!("No id");
        return false;
    }

    let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;
    let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
    println!("Throw Pikmin #{p}: {pikmin_id} ?") ;
    //p[0] is tossed, any others arent
    if is_link & 1 != 0  {
        let pikmin_boma = sv_battle_object::module_accessor(pikmin_id as u32);
        let mut target_pos =*PostureModule::pos(fighter.module_accessor);
        
        //This shouldnt be in a loop but whatever
        let mut capture_id =  WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID) as u32;
        if capture_id == OBJECT_ID_NULL {
            capture_id = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
            if capture_id != OBJECT_ID_NULL {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                target_pos = *PostureModule::pos(capture_boma);
            }
        }

        let pikmin_pos = *PostureModule::pos(pikmin_boma);
        let pikmin_lr = (target_pos.x-pikmin_pos.x).signum();
        
        println!("Toss em: {} to {} = {pikmin_lr}",pikmin_pos.x,target_pos.x);
        let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));
        link_event["lr_"].assign(&L2CValue::F32(pikmin_lr));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_ATTACK_STOP as u8}, true);

        let new_status = *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S;
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
    if catch_attack_check_special(fighter) {
        println!("Spummel init?");
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL); 

        WorkModule::set_int(fighter.module_accessor, OBJECT_ID_NULL as i32, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_TARGET_ID);

        WorkModule::set_int(fighter.module_accessor, 5, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN);
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

pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        println!("SpummelMar");
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Throw_Main as *const () as _));
    }
    
    return fighter.status_CatchAttack();
}

pub unsafe extern "C" fn catch_attack_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {

    return catch_attack_main_default_loop(fighter);
}


pub unsafe extern "C" fn pikmin_special_s_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S)(weapon);

    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        KineticModule::change_kinetic(weapon.module_accessor,*WEAPON_KINETIC_TYPE_NONE);
        KineticModule::clear_speed_all(weapon.module_accessor);
        PostureModule::add_pos(weapon.module_accessor, &Vector3f::new(0.0,2.0,0.0));
        let lr = PostureModule::lr(weapon.module_accessor);
        let speed_x = 1.0;
        let speed_y = 1.0;
        println!("Spummel Pikmin: {lr}");
        /* 
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_x*lr,
            speed_y
        );*/
    }
    
    return original;
}
pub unsafe extern "C" fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        println!("Holding {hold_pikmin_num} pikmin");

        let p = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT);
        let could_throw = throw_pikmin(fighter,p);
        if could_throw {
            WorkModule::set_int(fighter.module_accessor, 5, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNTDOWN);
        }
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_WORK_INT_CHARGE_COUNT);
    }
    return 0.into();
}

pub fn install() {
    smashline::Agent::new("pikmin")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        .status(Init, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_init)
        .status(Exec, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_exec)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        //.status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_main_default)
    .install();
    /* 
    smashline::Agent::new("pikmin_pikmin")
       .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S, pikmin_special_s_main)
    .install();*/
}
