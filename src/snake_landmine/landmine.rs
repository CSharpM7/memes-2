use crate::imports::imports_acmd::*;

pub const DEFAULT_GENERATE_ARTICLE_ID: i32 = 0x0;
pub static mut GENERATE_ARTICLE_ID: i32 = 0x0;
pub static mut WEAPON_KIND: i32 = 0x0;
pub const OWNER_NAME: &str = "snake";
pub const ARTICLE_NAME: &str = "landmine";

pub const STATUS_KIND_READY: i32 = 0x0;
pub const STATUS_KIND_HAVED: i32 = 0x4;
pub const STATUS_KIND_EXPLOSION: i32 = 0x1;

pub const STATUS_FLAG_ENABLE_SEARCH: i32 = 0x0;
pub const STATUS_FLAG_FALL: i32 = 0x1;
pub const STATUS_FLOAT_HOLD_FRAME: i32 = 0x0;

pub const ACTIVE_FRAME: f32 = 70.0;//19.0;

/*
Hitbox for claymore comes out 70 frames after generation.
The claymore has a detection hitbox that comes out 1 frame before the actual hitbox.

Charges between	19-20
Claymore generated	61
Animation length	76
Make sure friendly fire is on for searching. Only one can exist at a time.

If the platform the mine is set on disappears (such as on Rainbow Cruise or Delfino Plaza) 
the mine falls down and if it makes contact with anything, even the ground, it explosions

doesnt need a hurtbox

Split between two models? One is purely cosemetic and the other is the actual landmine
*/
const LIFE : i32 = 60*15;

unsafe extern "C" fn game_ready(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, ACTIVE_FRAME);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 0.0, Some(0.0), Some(12.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
}
unsafe extern "C" fn effect_ready(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, ACTIVE_FRAME);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.4, true);
    }
}
unsafe extern "C" fn sound_ready(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, ACTIVE_FRAME);
    if macros::is_excute(agent) {
    }
}

unsafe extern "C" fn game_explosion(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 14.0, 88, 90, 0, 58, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, 0x50000000);
        VisibilityModule::set_whole(agent.module_accessor, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0x50000000);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new("wn_msc_cmd_self_wn_remove"));
    }
}
unsafe extern "C" fn effect_explosion(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sound_explosion(agent: &mut L2CAgentBase) {
    //frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_special_l06"));
    }
}

pub unsafe extern "C" fn haved_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn haved_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let mut has_link = LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if !has_link {
        let link_created = LinkModule::link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,owner_id);
        has_link = link_created & 1 != 0;
    }
    /*
    if has_link {
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SLOW as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        //LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SLOPE as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_VISIBILITY as u8}, true);
        //LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_POS as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_FLIP as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_POWER_UP as u8}, true);
    } */
    LinkModule::remove_model_constraint(weapon.module_accessor, true);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let rot_y = PostureModule::rot_y(owner, 0);
    ModelModule::clear_joint_srt(weapon.module_accessor, Hash40::new("have"));
    ModelModule::clear_joint_srt(weapon.module_accessor, Hash40::new("rot"));
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: rot_y, z: 0.0}, 0);
    let parent_bone = Hash40::new("haver");

    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("have"),parent_bone,
    (*CONSTRAINT_FLAG_MTX 
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    LinkModule::set_constraint_rot_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 0.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(haved_main_loop as *const () as _))
}

unsafe extern "C" fn haved_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = sv_battle_object::module_accessor(owner_id);
    let owner_status = StatusModule::status_kind(owner_boma);
    if owner_status != *FIGHTER_STATUS_KIND_ATTACK_LW4 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub unsafe extern "C" fn ready_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn ready_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Pos
    LinkModule::remove_model_constraint(weapon.module_accessor, true);
    /*
    let mut has_link = LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if !has_link {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }*/
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let owner_pos = *PostureModule::pos(owner);
    PostureModule::set_pos(weapon.module_accessor, &owner_pos);
    let owner_power_mul_status = AttackModule::power_mul_status(owner);
    AttackModule::set_power_mul_status(weapon.module_accessor, owner_power_mul_status);

    //Life
    let life = LIFE;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("ready"), 0.0, 1.0, false, 0.0, false, false);
    
    weapon.fastshift(L2CValue::Ptr(ready_main_status_loop as *const () as _))
}

unsafe extern "C" fn ready_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let mut despawn = false;
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if !sv_battle_object::is_active(owner) {
        despawn = true;
    }
    else {
        let owner = sv_battle_object::module_accessor(owner_id);
        let owner_status = StatusModule::status_kind(owner_boma);
        if [*FIGHTER_STATUS_KIND_ATTACK_LW4_START,*FIGHTER_STATUS_KIND_DEAD].contains(&owner_status) {
            despawn = true;
        }
    }
    if despawn {
        let eff_pos = *PostureModule::pos(fighter.module_accessor);
        EffectModule::req(fighter.module_accessor, Hash40::new("sys_erace_smoke"), 
        &Vector3f{x: eff_pos.x, y: eff_pos.y+0.0, z: eff_pos.z}, &Vector3f{x: 0.0, y: 300.0, z: 0.0}, 
        1.0, 0,-1,false,0) as u32;
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }

    if !WorkModule::is_flag(weapon.module_accessor, STATUS_FLAG_FALL) {
        if !GroundModule::is_touch(weapon.module_accessor,0xff) {
            WorkModule::on_flag(weapon.module_accessor, STATUS_FLAG_FALL);
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

            KineticModule::change_kinetic(fighter.module_accessor, *WEAPON_KINETIC_TYPE_NORMAL);
            sv_kinetic_energy!(
                set_speed,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                set_limit_speed,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                -1.0,
                3.0
            );
            sv_kinetic_energy!(
                set_stable_speed,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                -1.0,
                3.0
            );
            sv_kinetic_energy!(
                set_accel,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                0.0,
                0.2
            );

            KineticModule::enable_energy(fighter.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
            sv_kinetic_energy!(
                set_speed, 
                weapon, 
                WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL, 
                22.0, 
                1.0,  //???
                0.0
            );
            return 0.into();
        }

        let normal_x = GroundModule::get_touch_normal_x(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let angle = normal_x.atan2(normal_y)-90.0;
        PostureModule::set_rot(weapon.module_accessor, &Vector3f {x: 0.0, y: angle, z:0.0}, 0);
    }
    else {
        if GroundModule::is_touch(weapon.module_accessor,0xff) {
            weapon.change_status(STATUS_KIND_EXPLOSION.into(),false.into());
            return 0.into();
        }
    }

    0.into()
}

pub unsafe extern "C" fn explosion_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
pub unsafe extern "C" fn explosion_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explosion"), 0.0, 1.0, false, 0.0, false, false);
    
    weapon.fastshift(L2CValue::Ptr(explosion_main_status_loop as *const () as _))
}

unsafe extern "C" fn explosion_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {
    #[cfg(feature = "dev")] {
        GENERATE_ARTICLE_ID = *WEAPON_SNAKE_ARTICLE_TERM;
        WEAPON_KIND = *WEAPON_KIND_TERM;
    }

    let agent_name = format!("{OWNER_NAME}_{ARTICLE_NAME}");
    Agent::new(agent_name.as_str())
        .acmd("game_ready", game_ready, Priority::Default)
        .acmd("effect_ready", effect_ready, Priority::Default)
        .acmd("sound_ready", sound_ready, Priority::Default)
        
        .acmd("game_explosion", game_explosion, Priority::Default)
        .acmd("effect_explosion", effect_explosion, Priority::Default)
        .acmd("sound_explosion", sound_explosion, Priority::Default)

        .status(Pre, STATUS_KIND_HAVED, haved_pre)
        .status(Main, STATUS_KIND_HAVED, haved_main)
        .status(End, STATUS_KIND_HAVED, empty_status)

        .status(Pre, STATUS_KIND_READY, ready_pre)
        .status(Main, STATUS_KIND_READY, ready_main)
        .status(End, STATUS_KIND_READY, empty_status)
        
        .status(Pre, STATUS_KIND_EXPLOSION, explosion_pre)
        .status(Main, STATUS_KIND_EXPLOSION, explosion_main)
        .status(End, STATUS_KIND_EXPLOSION, empty_status)
    .install();
}

unsafe extern "C" fn onsearch(_vtable: u64, weapon: &mut smash::cpp::root::app::Weapon, log: u64) {
    let boma = weapon.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *mut CollisionLogScuffed;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let status_kind = StatusModule::status_kind(boma);
    let is_searching = WorkModule::on_flag(boma,STATUS_FLAG_ENABLE_SEARCH);
    if status_kind == *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_READY {
        if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            StatusModule::change_status_force(boma, *STATUS_KIND_EXPLOSION, true);
        }
    }
}
pub fn install_hook() {  
    unsafe {
        let clone = smashline::clone_weapon(
            OWNER_NAME, ARTICLE_NAME, *WEAPON_KIND_SHIZUE_CLAYROCKET,true
        );

        GENERATE_ARTICLE_ID = clone.generate_id;
        WEAPON_KIND = clone.weapon_id;
        println!("[smashline_kuribo::kuribo] (HOOK) Clone assigned to {}",GENERATE_ARTICLE_ID);

        //smashline::enable_weapon_module(clone.weapon_id, WeaponModule::Search);
        //smashline::replace_weapon_vtable(clone.weapon_id, WeaponVtableSlot::OnSearch, onsearch as *const () as usize)
    }
}