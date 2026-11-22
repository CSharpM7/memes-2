use crate::imports::imports_acmd::*;

pub const DEFAULT_GENERATE_ARTICLE_ID: i32 = 0x0;
pub static mut GENERATE_ARTICLE_ID: i32 = 0x0;
pub static mut WEAPON_KIND: i32 = 0x0;
pub const OWNER_NAME: &str = "snake";
pub const ARTICLE_NAME: &str = "landmine";


const LIFE : i32 = 90;

unsafe extern "C" fn game_ready(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 0.0, Some(0.0), Some(12.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
}
unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 96, 100, 70, 0, 6.0, 0.0, 0.5, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.9, 89, 10, 0, 70, 6.0, 0.0, 0.5, 0.0, None, None, None, 0.3, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

pub unsafe extern "C" fn wait_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Life
    let life = LIFE;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
    
    weapon.fastshift(L2CValue::Ptr(wait_main_status_loop as *const () as _))
}

unsafe extern "C" fn wait_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
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

pub fn install() {
    #[cfg(feature = "dev")] {
        GENERATE_ARTICLE_ID = *WEAPON_SNAKE_ARTICLE_TERM;
        WEAPON_KIND = *WEAPON_KIND_TERM;
    }

    let agent_name = format!("{OWNER_NAME}_{ARTICLE_NAME}");
    Agent::new(agent_name.as_str())
        .acmd("game_ready", game_ready, Priority::Default)
        .acmd("game_fly", game_fly, Priority::Default)

        .status(Main, 10, wait_main)
    .install();
}

unsafe extern "C" fn landmine_onsearch(_vtable: u64, weapon: &mut smash::cpp::root::app::Weapon, log: u64) {
    let boma = weapon.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *mut CollisionLogScuffed;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_READY {
        if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            StatusModule::change_status_force(boma, *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_FLY, true);
        }
    }
}
pub fn install_hook() {  
    unsafe {
        let block = smashline::clone_weapon(
            OWNER_NAME, ARTICLE_NAME, *WEAPON_KIND_SHIZUE_CLAYROCKET,true
        );

        GENERATE_ARTICLE_ID = block.generate_id;
        WEAPON_KIND = block.weapon_id;
        println!("[smashline_kuribo::kuribo] (HOOK) Block assigned to {}",GENERATE_ARTICLE_ID);

        smashline::enable_weapon_module(block.weapon_id, WeaponModule::Search);
        smashline::replace_weapon_vtable(block.weapon_id, WeaponVtableSlot::OnSearch, landmine_onsearch as *const () as usize)
    }
}