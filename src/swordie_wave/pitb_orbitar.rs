use crate::imports::imports_acmd::*;
use crate::imports::imports_agent::*;

/*
ACMD
*/
unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 20, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
    }
}

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_shield"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
}

/*
STATUS
*/

const STATUS_KIND_FLY : i32 = 0x0;
const MOVE_SPEED : f32 = 0.5;
const LIFE : i32 = 90;

pub unsafe extern "C" fn fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

pub unsafe extern "C" fn fly_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner);
    let lr = PostureModule::lr(owner_boma);
    
    /*
    SNAP TO BONE
    */
    let owner_bone = Hash40::new("haver");
    let mut offset = Vector3f{x:0.0,y:0.0,z:0.0};
    
    let mut owner_bone_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let owner_offset = ModelModule::joint_global_offset_from_top(owner_boma, owner_bone, &mut owner_bone_pos);
    let newPos = Vector3f{
        x: PostureModule::pos_x(owner_boma) + owner_bone_pos.x + (offset.x*lr), 
        y: PostureModule::pos_y(owner_boma) + owner_bone_pos.y + offset.y, 
        z: GroundModule::get_z(weapon.module_accessor)
    };
    PostureModule::set_pos(weapon.module_accessor, &newPos); 

    /*
    SET LR
    */
    PostureModule::set_lr(weapon.module_accessor, lr);
    PostureModule::update_rot_y_lr(weapon.module_accessor);

    /*
    SET SPEED
    */
    let move_speed = MOVE_SPEED;
    let speed_x = move_speed;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        move_speed*lr,
        0.0
    );

    0.into()
}

pub unsafe extern "C" fn fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Life
    let life = LIFE;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    
    weapon.fastshift(L2CValue::Ptr(fly_main_status_loop as *const () as _))
}

unsafe extern "C" fn fly_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    //Die on attack collision
    if !AttackModule::is_attack(weapon.module_accessor, 0, false) 
    || AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT){
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }
    //Die on wall hit
    if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
        let eff_pos = *PostureModule::pos(weapon.module_accessor);
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), 
        &Vector3f{x: eff_pos.x, y: eff_pos.y+0.0, z: eff_pos.z}, &Vector3f{x: 0.0, y: 300.0, z: 0.0}, 
        1.0, 0,-1,false,0) as u32;
        
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn fly_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {
    let agent = &mut smashline::Agent::new("pitb_orbitar");
    agent.acmd("game_fly", game_fly,Priority::Low);
    agent.acmd("effect_fly", effect_fly,Priority::Low);
    agent.acmd("sound_fly", sound_fly,Priority::Low);
    
    agent.status(Pre, STATUS_KIND_FLY, fly_pre);
    agent.status(Init, STATUS_KIND_FLY, fly_init);
    agent.status(Main, STATUS_KIND_FLY, fly_main);
    agent.status(End, STATUS_KIND_FLY, fly_end);

    agent.install();
}