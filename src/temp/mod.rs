use crate::imports::imports_status::*;

pub unsafe extern "C" fn special_s_forward_main(fighter: &mut L2CFighterCommon)  -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_CHECK_CLIFF);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF);
    
    if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            let temp = &Vector2f{x: 0.0, y: 0.0};
            let normal = GroundModule::get_touch_normal_consider_gravity(fighter.module_accessor,*GROUND_TOUCH_FLAG_DOWN as u32);
            let normal_x = normal.x;//pLVar7 , auStack224 + 0x10?
            let normal_y = normal.y;//pLVar9,?
            let len = sv_math::vec2_length(normal_x, normal_y);

            //This is related to slopes, probably just makes sure there is valid ground directly below
            if (len > 1e-05){
                let arctangent = normal_y.atan2(normal_x);
                let mut degree = arctangent.to_degrees().abs();
                
                //Defaults to 41
                let slope_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), Hash40::new_raw(0x11e7fad1ad).hash);

                //Not sure when this is ever false...
                if (slope_threshold < degree) {
                    let lr = PostureModule::lr(fighter.module_accessor);
                    let mut pLVar7 = normal_x*lr; //degree?
                    //This should make the player go into the air if on a steep slope
                    //This should also check to make sure the player is headed down the slope, rather than up it
                    println!("Var7: {pLVar7} D: {degree} / {slope_threshold}");
                    if (0.0 < pLVar7) {
                        fighter.set_situation(SITUATION_KIND_AIR.into());
                        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
                    }
                }
            }
        }
    }

    //Set motion and ground correct
    fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
    
    //Set kinetics
	fighter.clear_lua_stack();
	lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
	let mut speed_x = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    
    let kinetic_type = if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {*FIGHTER_KINETIC_TYPE_MOTION} else {*FIGHTER_KINETIC_TYPE_MOTION_AIR};
    KineticModule::change_kinetic(fighter.module_accessor,kinetic_type);
    
    let speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("speed_x_mul"));
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    
	fighter.clear_lua_stack();
	lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
	fighter.clear_lua_stack();
	lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x_mul);
	app::sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    
    //Magnify Glass related stuff
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    
    //Ground Correct 2
    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, true);
    
    return fighter.sub_shift_status_main(L2CValue::Ptr(special_s_forward_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_s_forward_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        return 0.into();
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END.into(),false.into());
        return 0.into();
    }
    let mut lstack176 = false;
    
    let ground_cliff_stop_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("ground_cliff_stop_frame")) as f32;
    let current_frame = fighter.global_table[0xE].get_f32();
    if (ground_cliff_stop_frame > current_frame) {
        //LAB_71000130a8, go to is FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT
        lstack176 = true;
    }
    else {    
        let is_near_cliff_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("is_near_cliff_threshold"));
        
        //Active for frames 11-32
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_CHECK_CLIFF) {
            //Check if near cliff
            if !WorkModule::is_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF) {
                let scale = PostureModule::scale(fighter.module_accessor);
                if GroundModule::is_ottotto(fighter.module_accessor, scale*is_near_cliff_threshold) {
                    println!("Near cliff");
                    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF);
                }
            }
            //IS_TILT check
            lstack176 = true;
        }
    }
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT) {
        lstack176=false;
        println!("Tilted");
        let air_fix_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("air_fix_frame")) as f32;
        if (air_fix_frame <= current_frame) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
        }
    }
    //Only change motion if checking for cliff?
    if lstack176 {
        //All this is just motion change stuff
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let motion_g = Hash40::new("special_s"); //aLStack192
        let motion_a = Hash40::new("special_air_s"); //LStack208
        
        let frame = MotionModule::frame(fighter.module_accessor);
        let rate = MotionModule::rate(fighter.module_accessor);
        
        //If in air but not air motion, fix it
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND 
        && motion_kind != motion_a.hash {
            println!("Fix motion for air");
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion_a, -1.0, 1.0, 0.0, false, false);
        }
        //If ground...
        else if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_AIR
        && motion_kind != motion_g.hash {
            println!("Fix motion for ground");
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion_g, -1.0, 1.0, 0.0, false, false);
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, frame, true, true, false);
        }
        fighter.sub_set_ground_correct_by_situation(true.into()); //0xd0?
    }
    
    return 0.into();
}

pub fn install() {   
    Agent::new("elight")
        .status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD, special_s_forward_main)
        .install();
}