use crate::imports::imports_acmd::*;

pub const DEFAULT_GENERATE_ARTICLE_ID: i32 = 0x0;
pub static mut GENERATE_ARTICLE_ID: i32 = 0x0;
pub static mut WEAPON_KIND: i32 = 0x0;
pub const OWNER_NAME: &str = "snake";
pub const ARTICLE_NAME: &str = "ground";

unsafe extern "C" fn effect_attacklw4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    loop {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}

pub unsafe extern "C" fn wait_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let mut has_link = LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if !has_link {
        let link_created = LinkModule::link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,owner_id);
        has_link = link_created & 1 != 0;
    }
    if has_link {
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SLOW as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SLOPE as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_VISIBILITY as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_POS as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_FLIP as u8}, true);
    }

    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("attack_lw4"), 0.0, 1.0, false, 0.0, false, false);
    
    weapon.fastshift(L2CValue::Ptr(wait_main_status_loop as *const () as _))
}

unsafe extern "C" fn wait_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let owner_status = StatusModule::status_kind(owner_boma);
    let owner_motion = MotionModule::motion_kind(weapon.module_accessor);
    let current_motion = MotionModule::motion_kind(weapon.module_accessor);
    if current_motion != owner_motion {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new_raw(owner_motion), 0.0, 1.0, false, 0.0, false, false);
    }
    let owner_frame = MotionModule::frame(owner_boma);
    MotionModule::set_frame(weapon.module_accessor,owner_frame);
    if MotionModule::is_end(weapon.module_accessor) && owner_status == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    let normal_x = GroundModule::get_touch_normal_x(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let normal_y = GroundModule::get_touch_normal_y(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let angle = normal_x.atan2(normal_y)-90.0;
    PostureModule::set_rot(weapon.module_accessor, &Vector3f {x: 0.0, y: angle, z:0.0}, 0);

    0.into()
}

pub fn install() {
    #[cfg(feature = "dev")] {
        GENERATE_ARTICLE_ID = *WEAPON_SNAKE_ARTICLE_TERM+1;
        WEAPON_KIND = *WEAPON_KIND_TERM+1;
    }

    let agent_name = format!("{OWNER_NAME}_{ARTICLE_NAME}");
    Agent::new(agent_name.as_str())
        .acmd("effect_attacklw4charge", effect_attacklw4charge, Priority::Default)
        
        .status(Main, 0, wait_main)
    .install();
}

pub fn install_hook() {  
    unsafe {
        let clone = smashline::clone_weapon(
            OWNER_NAME, ARTICLE_NAME, *WEAPON_KIND_SHIZUE_TRAFFICSIGN,false
        );

        GENERATE_ARTICLE_ID = clone.generate_id;
        WEAPON_KIND = clone.weapon_id;
        println!("[smashline_kuribo::kuribo] (HOOK) Clone assigned to {}",GENERATE_ARTICLE_ID);
    }
}