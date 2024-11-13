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
        return 0.into();
    }
    println!("Holding {hold_pikmin_num} pikmin");
    let iVar4 = hold_pikmin_num-1; //???
    let mut p = 0;
    let mut lead_pikmin_id = OBJECT_ID_NULL;
    loop {
        let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;//+p?
        if p == 0 {
            lead_pikmin_id = pikmin_id;
        }
        let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;

        let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
        println!("Throw Pikmin #{p}: {pikmin_id} ?") ;
        //p[0] is tossed, any others arent
        if is_link & 1 != 0 {
            println!("Toss em");
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

            let new_status = *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S;
            let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
            link_event["status_kind_"].assign(&L2CValue::I32(new_status));
            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event["sender_id_"].assign(&L2CValue::U32(object_id));
            link_event_store_l2c_table(fighter, link_node.into(), link_event);
            
            FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
            LinkModule::unlink(fighter.module_accessor, link_node);
        } 
        let bVar3 = p < iVar4;
        println!("{p} < {iVar4} ? ");
        p=p+1;
        if !bVar3 {
            break;
        }
    }
    //LinkModule::unlink(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN);
    println!("End loop");
    //let lead_pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;
    if lead_pikmin_id == OBJECT_ID_NULL || !sv_battle_object::is_active(lead_pikmin_id) {
        println!("no lead");
        return 0.into();
    }
    let lead_pikmin_boma = sv_battle_object::module_accessor(lead_pikmin_id as u32);
    //somethin about motion rate and what not

    0.into()
}

//If pikmin miss, game crashes. If they cling, game stays alive?
pub unsafe extern "C" fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    return catch_attack_main_new(fighter,true);
}

pub fn install() {
    smashline::Agent::new("pikmin")
        .status(Init, *FIGHTER_STATUS_KIND_APPEAL, catch_attack_init)
        //.status(Exec, *FIGHTER_STATUS_KIND_APPEAL, catch_attack_exec)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
    .install();
}
