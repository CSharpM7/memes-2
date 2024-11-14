use crate::imports::imports_status::*;
use crate::imports::imports_acmd::*;

pub const FIGHTER_POPO_STATUS_KIND_SPECIAL_S1: i32 = 0x1EF;
pub const FIGHTER_POPO_STATUS_SPECIAL_S_PARTNER_FLAG_FLY: i32 = 0x21000015;
pub const FLY_SPEED: f32 = 3.5;

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_PHASE_END);
    }
}
unsafe extern "C" fn effect_specials1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
unsafe extern "C" fn sound_specials1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_popo_rnd_attack"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
}

unsafe extern "C" fn game_specialairs2nana(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 16.0, 45, 60, 0, 60, 5.0, 0.0, 0.0, -0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
}
unsafe extern "C" fn effect_specialairs2nana(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.9, true);
    }
}
unsafe extern "C" fn sound_specialairs2nana(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if app::sv_math::rand(hash40("fighter"), 3) == 0 {
            macros::PLAY_FLY_VOICE(agent, Hash40::new("seq_nana_rnd_futtobi01"), Hash40::new("seq_nana_rnd_futtobi02"));
        }
        else {
            PLAY_VC(agent,Hash40::new("vc_nana_damage02"),0.4);
        }
    }
}


unsafe extern "C" fn special_s1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("S1 popo");
    let motion_g = Hash40::new("special_s1");
    let motion_a = Hash40::new("special_air_s1");
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {motion_g} else {motion_a};
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_FALL.into());
    fighter.sub_set_ground_correct_by_situation(false.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s1_main_loop as *const () as _))
}

unsafe extern "C" fn special_s1n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("S2 nana init");
    if LinkModule::is_link(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER) {
        if LinkModule::is_model_constraint(fighter.module_accessor) {
            println!("HUH");
            
        }
        LinkModule::set_model_constraint_pos_ort(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER, Hash40::new("rot"), Hash40::new("haver"), 
        (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION) as u32, true);
        LinkModule::set_attribute(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER, LinkAttribute{_address:*LINK_ATTRIBUTE_REFERENCE_PARENT_SLOW as u8}, false);
        LinkModule::set_attribute(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER, LinkAttribute{_address:*LINK_ATTRIBUTE_ADJUST_PARENT_SHAPE as u8}, false);
    }
    0.into()
}
unsafe extern "C" fn special_s1n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("S2 nana");
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_nana"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_FALL.into());
    fighter.sub_set_ground_correct_by_situation(false.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s1_main_loop as *const () as _))
}

unsafe extern "C" fn special_s1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Popo
    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
            else if fighter.sub_air_check_stop_ceil().get_bool() {
                return 1.into();
            }
        }

        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_PHASE_END) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_PHASE_END);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_COUPLE) {
                //Enable flag for nana
                if LinkModule::is_link(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER) {
                    println!("nana check");
                    let nana_id = LinkModule::get_node_object_id(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER) as u32;
                    let nana_boma = sv_battle_object::module_accessor(nana_id);
                    if StatusModule::status_kind(nana_boma) == *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER {
                        StatusModule::change_status_force(nana_boma, FIGHTER_POPO_STATUS_KIND_SPECIAL_S1, false);
                    }
                }
            }
        }
    }
    //Nana
    else {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_NONE as u32, true, false);
    }

    return 0.into()
}

unsafe extern "C" fn special_s1n_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("Exit");
    if LinkModule::is_model_constraint(fighter.module_accessor) {
        LinkModule::remove_model_constraint(fighter.module_accessor, true);
    }
    smashline::original_status(Exit, fighter, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_PARTNER)(fighter)
}
unsafe extern "C" fn special_s2fly_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let popo_id = LinkModule::get_parent_object_id(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER) as u32;
    let popo_boma = sv_battle_object::module_accessor(popo_id);

    let mut popo_pos = VECTOR_ZERO;
    let mut nana_pos = VECTOR_ZERO;
    let lr = PostureModule::lr(popo_boma);
    PostureModule::set_lr(fighter.module_accessor, lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    let popo_offset = ModelModule::joint_global_offset_from_top(popo_boma, Hash40::new("haver"), &mut popo_pos);  
    let nana_offset = ModelModule::joint_global_offset_from_top(fighter.module_accessor, Hash40::new("rot"), &mut nana_pos);      
    let offset = Vector3f{x:-1.0*lr,y:1.0,z:0.0};
    let newPos = Vector3f{
        x: PostureModule::pos_x(popo_boma) + popo_pos.x - nana_pos.x + (offset.x), 
        y: PostureModule::pos_y(popo_boma) + popo_pos.y + offset.y, 
        z: PostureModule::pos_z(popo_boma) + popo_pos.z- nana_pos.z + offset.z
    };
    PostureModule::set_pos(fighter.module_accessor, &newPos);

    let rot_y = PostureModule::rot_y(fighter.module_accessor, 0);
    let rot = Vector3f{x: 90.0, y: rot_y, z: 0.0};
    PostureModule::set_rot(
        fighter.module_accessor,
        &rot,
        0
    );
    
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        FLY_SPEED*lr,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0125,
        0.0
    );

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        0.25,
        0.0,
        0.0,
        0.0
    );;
    let air_accel_y_mul = 0.5;
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y * air_accel_y_mul
    );
    
    0.into()
}
unsafe extern "C" fn special_s2fly_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        //*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_s2fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let max_jump = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT, max_jump);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_nana"), 0.0, 1.0, false, 0.0, false, false);

    //I hate nana
    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_specialairs2nana"), -1);
    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialairs2nana"), -1);
    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_specialairs2nana"), -1);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2fly_main_loop as *const () as _))
}

unsafe extern "C" fn special_s2fly_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }
    }
    let stable_speed = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("air_speed_x_stable"), 0);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let frame = fighter.global_table[STATUS_FRAME].get_i32();
    if frame >= 5 {
        let t = speed_x / FLY_SPEED;
        let mul = lerp(0.5,1.0,t);
        AttackModule::set_power_mul(fighter.module_accessor, mul);
    }
    if frame > 300 
    || speed_x <= stable_speed {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_DOWN.into(), FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
        return 0.into();
    }
    if MotionModule::frame(fighter.module_accessor) > 16.0 {
        MotionModule::set_frame(fighter.module_accessor, 0.0, false);
    }

    if StatusModule::is_situation_changed(fighter.module_accessor) 
    && !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), false.into());
            return 1.into();
        }
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    {
        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR.into(), false.into());
        return 1.into();
    }

    0.into()
}
unsafe extern "C" fn special_s2fly_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rot_y = PostureModule::rot_y(fighter.module_accessor, 0);
    let rot = Vector3f{x: 0.0, y: rot_y, z: 0.0};
    PostureModule::set_rot(
        fighter.module_accessor,
        &rot,
        0
    );
    0.into()
}

pub fn install_helper(agent: &mut smashline::Agent) {
    agent.acmd("game_specials1",game_specials1);
    agent.acmd("game_specialairs1",game_specials1);
    agent.acmd("effect_specials1",effect_specials1);
    agent.acmd("effect_specialairs1",effect_specials1);
    agent.acmd("sound_specials1",sound_specials1);
    agent.acmd("sound_specialairs1",sound_specials1);

    agent.acmd("game_specials2_nana",empty_acmd);
    agent.acmd("effect_specials2_nana",empty_acmd);
    agent.acmd("sound_specials2_nana",empty_acmd);

    agent.acmd("game_specialairs2_nana",game_specialairs2nana);
    agent.acmd("game_specialairs2",game_specialairs2nana);
    agent.acmd("effect_specialairs2",effect_specialairs2nana);
    agent.acmd("effect_specialairs2_nana",effect_specialairs2nana);
    agent.acmd("sound_specialairs2",sound_specialairs2nana);
    agent.acmd("sound_specialairs2_nana",sound_specialairs2nana);
    
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s1_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, empty_status);
    agent.status(Init, *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER, special_s1n_init);
    agent.status(Main, *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER, special_s1n_main);
    agent.status(Exit, *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER, special_s1n_exit);
    
    agent.status(Init, FIGHTER_POPO_STATUS_KIND_SPECIAL_S1, special_s2fly_init);
    agent.status(Pre, FIGHTER_POPO_STATUS_KIND_SPECIAL_S1, special_s2fly_pre);
    agent.status(Main, FIGHTER_POPO_STATUS_KIND_SPECIAL_S1, special_s2fly_main);
    agent.status(End, FIGHTER_POPO_STATUS_KIND_SPECIAL_S1, empty_status);
    agent.status(Exit, FIGHTER_POPO_STATUS_KIND_SPECIAL_S1, special_s2fly_exit);
    
}


pub fn install() {   
    let popo = &mut Agent::new("popo");
    let nana = &mut Agent::new("nana");
    install_helper(popo);
    install_helper(nana);
    popo.install();
    nana.install();
}