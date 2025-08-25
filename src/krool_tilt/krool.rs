use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        if GroundModule::is_ottotto(agent.module_accessor, 6.5) {
            println!("OTTO!");
            let lr = PostureModule::lr(agent.module_accessor);
            KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.5*lr, y: 0.0, z: 0.0});
            PostureModule::add_pos(agent.module_accessor, &Vector3f{x: 6.0*lr, y: 0.0, z: 0.0});
            StatusModule::set_situation_kind(agent.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
            StatusModule::change_status_request(agent.module_accessor, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
        }
        else {
            println!("notto");
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 13.0, 361, 85, 0, 40, 4.5, 2.5, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
            frame(agent.lua_state_agent, 14.0);
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 50, 0, 80, 5.0, 0.0, 3.5, 16.5, Some(0.0), Some(3.5), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 50, 0, 30, 5.0, 0.0, 3.5, 16.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 85, 25, 0, 100, 10.0, 0.0, 5.5, 13.0, Some(0.0), Some(5.5), Some(23.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            }
            wait(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                AttackModule::clear_all(agent.module_accessor);
            }
        }
    }
}


unsafe extern "C" fn attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_lw3_main_loop as *const () as _))
}

unsafe extern "C" fn attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && AttackModule::is_attack(fighter.module_accessor, 0, false) { 
        if GroundModule::is_ottotto(fighter.module_accessor, 6.5) {
            println!("OTTO!");
            let lr = PostureModule::lr(fighter.module_accessor);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.5*lr, y: 0.0, z: 0.0});
            PostureModule::add_pos(fighter.module_accessor, &Vector3f{x: 6.0*lr, y: 0.0, z: 0.0});
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
            //StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("run_fall_l"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                -0.1,
                0.0,
                0.0,
                0.0
            ); 
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_speed_y_stable
            );

            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
                lr*0.1,
                0.0,
                0.0,
                0.0,
                0.0
            );
            return 0.into();
        }
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("run_fall_l") {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() 
            || fighter.sub_transition_group_check_air_landing().get_bool() //?
            {
                return 1.into();
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        return 0.into();
    }
    fighter.status_AttackLw3_Main()
}

pub fn install() {   
    Agent::new("krool")
        //.acmd("game_attacklw3", game_attacklw3,Priority::Default)
        .status(Main,*FIGHTER_STATUS_KIND_ATTACK_LW3,attack_lw3_main)
    .install();
}