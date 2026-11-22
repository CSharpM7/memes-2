use crate::imports::imports_acmd::*;

pub const DEFAULT_GENERATE_ARTICLE_ID: i32 = 0x0;
pub static mut GENERATE_ARTICLE_ID: i32 = 0x0;
pub static mut WEAPON_KIND: i32 = 0x0;
pub const OWNER_NAME: &str = "gaogaen";
pub const ARTICLE_NAME: &str = "pickelobject";

pub static mut DEBUG_HAS_LVD: bool = false;

const LIFE : i32 = 90;

pub unsafe extern "C" fn wait_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
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

pub unsafe extern "C" fn wait_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    /*
    let loaded = smashline::load_weapon_lvd(block.weapon_id as u8, "forge.lvd");
    DEBUG_HAS_LVD = loaded;
    println!("[smashline_kuribo::kuribo] LVD: {loaded}");
     */


    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner);
    let lr = PostureModule::lr(owner_boma);
    
    /*
    SNAP TO BONE
    */
    let newPos = Vector3f{
        x: PostureModule::pos_x(owner_boma) + 0.0, 
        y: PostureModule::pos_y(owner_boma) + -10.0, 
        z: GroundModule::get_z(weapon.module_accessor)
    };
    PostureModule::set_pos(weapon.module_accessor, &newPos); 

    0.into()
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

unsafe extern "C" fn wait_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {
    #[cfg(feature = "dev")] {
        GENERATE_ARTICLE_ID = DEFAULT_GENERATE_ARTICLE_ID;
        WEAPON_KIND = *WEAPON_KIND_TERM;
    }

    let agent_name = format!("{OWNER_NAME}_{ARTICLE_NAME}");
    Agent::new(agent_name.as_str())
        .status(Init, 0, wait_init)
        .status(Pre, 0, wait_pre)
        .status(Main, 0, wait_main)
        .status(End, 0, wait_end)
    .install();
}


pub fn install_hook() {  
    unsafe {
        let block = smashline::clone_weapon(
            OWNER_NAME, ARTICLE_NAME, *WEAPON_KIND_MARIO_FIREBALL,false
        );

        GENERATE_ARTICLE_ID = block.generate_id;
        WEAPON_KIND = block.weapon_id;
        println!("[smashline_kuribo::kuribo] (HOOK) Block assigned to {}",GENERATE_ARTICLE_ID);
     
        //let loaded = smashline::load_weapon_lvd(block.weapon_id as u8, "forge.lvd");
        //DEBUG_HAS_LVD = loaded;
        //println!("[smashline_kuribo::kuribo] LVD: {loaded}");
        
        use smashline::{set_weapon_lvd_nodes, Hash40, LvdNodeBinding};
        let col01 = Hash40::new("virtualgrdcol01");
        let col02 = Hash40::new("virtualgrdcol02");

        // facing right: vertex 1 is on col01, vertex 0 on col02
        let facing_right = [LvdNodeBinding::new(1, col01), LvdNodeBinding::new(0, col02)];
        // facing left: the same nodes, vertex indices swapped
        let facing_left = [LvdNodeBinding::new(0, col01), LvdNodeBinding::new(1, col02)];

        // the hydrant tips over, so it re-pins every frame
        set_weapon_lvd_nodes(block.weapon_id, &facing_right, &facing_left, true);
    }
}