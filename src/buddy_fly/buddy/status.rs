use crate::imports::imports_status::*;
use crate::buddy_fly::imports::*;

pub unsafe extern "C" fn specialhi_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    //Set rate based on param and end frame

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_INVOKE_AIR);
    }
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_main_loop as *const () as _))
}

pub unsafe extern "C" fn specialhi_main_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    fighter.sub_exec_special_start_common_kinetic_setting(Hash40::new("param_special_hi").into());

    let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
    //if status_frame > min charge frame then...
    //check for button off
    if MotionModule::is_end(fighter.module_accessor) {
        //charge_rage is probably (status_frame-min_charge) / (max_charge-min_charge)
        let charge_rate = 1.0;
        WorkModule::set_float(fighter.module_accessor, charge_rate, *FIGHTER_BUDDY_STATUS_SPECIAL_HI_WORK_FLOAT_CHARGE_RATE);
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
        return 0.into();
    }

    if StatusModule::is_changing(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_INVOKE_AIR) {
            if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
            }
        }
    }
    else if 
    StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND &&
        fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        }
        else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR &&
        fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        }
    }
    0.into()
}
pub unsafe extern "C" fn specialhi_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let original_status = smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
        WorkModule::set_float(fighter.module_accessor, LANDING_FRAME_CANCEL, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    
    original_status
}

//unused
unsafe extern "C" fn specialhi_jump_control_energy(fighter: &mut L2CFighterCommon, speed_x: f32, is_falling: bool) {
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    ); 
    let jump_accel_x_mul = if !is_falling {0.5} else {0.4};
    let jump_speed_x_max_mul = if !is_falling {0.4} else {1.0};
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, jump_speed_x_max_mul);
    app::sv_kinetic_energy::mul_x_speed_max(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, jump_accel_x_mul);
    app::sv_kinetic_energy::mul_x_accel_mul(fighter.lua_state_agent);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
}
unsafe extern "C" fn specialhi_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
    //specialhi_jump_control_energy(fighter,0.0,false);
    let speed_y_mul_g = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_ground_jump_speed_y_mul"));
    let speed_y_mul_a = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_air_jump_speed_y_mul"));

    let jump_speed_y_mul = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_INVOKE_AIR) {speed_y_mul_g} else {speed_y_mul_a};
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        jump_speed_y_mul*JUMP_Y_BONUS_MUL
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);

    0.into()
}

pub const STICK_MUL: f32 = 20.0;
pub unsafe extern "C" fn specialhi_jump_set_direction(fighter: &mut smashline::L2CFighterCommon) {
    let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let mut is_reverse = stick_x.signum() != PostureModule::lr(fighter.module_accessor);
    if stick_x.abs() <= 0.25 {
        stick_x = 0.0;
        is_reverse = false;
    }
    let mut angle = -STICK_MUL*stick_x;
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    angle = angle.to_radians();
    WorkModule::set_float(fighter.module_accessor, angle,FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLOAT_JUMP_ANGLE);
}

unsafe extern "C" fn specialhi_jump_apply_angle(fighter: &mut L2CFighterCommon){
    let mut angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLOAT_JUMP_ANGLE);
    sv_kinetic_energy!(
        set_angle,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        angle
    );
}

pub unsafe extern "C" fn specialhi_jump_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    //specialhi_jump_set_direction(fighter);
    //specialhi_jump_apply_angle(fighter);

    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable*FALL_SPEED_MAX_Y
    );

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_BUDDY_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    GroundModule::set_shape_safe_pos(fighter.module_accessor, &Vector2f{x: pos_x, y:pos_y});
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_jump_main_loop as *const () as _))
}

pub unsafe extern "C" fn specialhi_jump_main_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    /*
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    } 
    */
    if MotionModule::is_end(fighter.module_accessor) {
        //ControlModule__get_attack_air_kind
        let attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
        WorkModule::set_int(fighter.module_accessor, attack_air_kind, FIGHTER_BUDDY_STATUS_SPECIAL_HI_INT_ATTACK_AIR_KIND);
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_RESET_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_RESET_CONTROL);
        let speed_x = {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
        };
        //specialhi_jump_control_energy(fighter,speed_x,true);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_RESET_ANGLE) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_RESET_ANGLE);
        sv_kinetic_energy!(
            set_angle,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.0
        );
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_DECIDE_LR) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_DECIDE_LR);
        specialhi_turn_check(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_TURNING) {
        specialhi_turn_check(fighter);
    }

    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if (speed_y < 0.1 && MotionModule::frame(fighter.module_accessor) > 20.0)
        || WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_FALL)
        {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_JUMP_FALL);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_speed_y_stable*FALL_SPEED_MAX_Y
            );
        }
    }

    0.into()
}

pub unsafe extern "C" fn specialhi_jump_exec(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {

    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL {
        specialhi_fall_exec(fighter);
    }
    0.into()
}

unsafe extern "C" fn specialhi_fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}
pub unsafe extern "C" fn specialhi_fall_init(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable*FALL_SPEED_MAX_Y
    );

    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        air_accel_x_mul * ACCEL_X_MAX_MUL_GLIDE
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        air_accel_x_add * ACCEL_X_MAX_MUL_GLIDE
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * SPEED_X_MAX_MUL_GLIDE,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * SPEED_X_MAX_MUL_GLIDE,
        0.0
    );

    0.into()
}
pub unsafe extern "C" fn specialhi_fall_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x: 0.0, y: 3.0});

    let attack_air_kind = WorkModule::get_int(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_INT_ATTACK_AIR_KIND);
    ControlModule::set_attack_air_kind(fighter.module_accessor, attack_air_kind);

    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, lr, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLOAT_TURN_DIR_INIT);
    
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fall"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_fall_main_loop as *const () as _))
}

pub unsafe extern "C" fn specialhi_fall_main_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    /*
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    } 
     */
    if fighter.sub_transition_group_check_air_special().get_bool()
    || fighter.sub_transition_group_check_air_attack().get_bool() 
    || fighter.sub_transition_group_check_air_escape().get_bool() //new
    {
        WorkModule::set_float(fighter.module_accessor, LANDING_FRAME_CANCEL, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::set_float(fighter.module_accessor, LANDING_FRAME_GLIDE, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        return 0.into();
    }
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw4_stick_y"));
    let prev_stick_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
    //Sorry down special
    if prev_stick_y <= squat_stick_y {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_DROP.into(), false.into());
        return 1.into();
    }

    0.into()
}


pub unsafe fn specialhi_turn_check(fighter: &mut smashline::L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let turn_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_x"));

    let is_turning = WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_TURNING);
    if !is_turning {
        if (stick_x*lr <= turn_stick && stick_x.abs() >= turn_stick) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_TURNING);
            WorkModule::set_float(fighter.module_accessor, -lr,FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLOAT_TURN_DIR_INIT);
            WorkModule::set_float(fighter.module_accessor, 0.0,FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLOAT_ROT_Y);
    
            TurnModule::set_turn(
                fighter.module_accessor,
                Hash40::new("item_lift"), //landing?
                lr,
                false,
                false,
                true
            );
            PostureModule::reverse_lr(fighter.module_accessor);
        }
    }
    else {
        if !TurnModule::is_turn(fighter.module_accessor) {
            let target_dir = WorkModule::get_float(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLOAT_TURN_DIR_INIT);
            PostureModule::set_lr(fighter.module_accessor, target_dir);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_TURNING);
            KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
}
pub unsafe extern "C" fn specialhi_fall_exec(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    
    //ANIM//
    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP {
        let stick_magnitude = sv_math::vec2_length(stick_x, stick_y);
        let motion_rate = lerp(FLIGHT_MOTION_SPEED_MIN,FLIGHT_MOTION_SPEED_MAX,stick_magnitude.min(1.0));
        MotionModule::set_rate(fighter.module_accessor, motion_rate);
    }

    //TURN//
	if !StatusModule::is_changing(fighter.module_accessor) {
        specialhi_turn_check(fighter);
    }
    0.into()
}

pub unsafe extern "C" fn specialhi_fall_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x: 0.0, y: 0.0});

    if fighter.global_table[STATUS_KIND].get_i32() != FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_DROP {
        let max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        WorkModule::set_int(fighter.module_accessor, max_jumps, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

unsafe extern "C" fn specialhi_drop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW);
    }
    else if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}
pub unsafe extern "C" fn specialhi_drop_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    WorkModule::set_float(fighter.module_accessor, LANDING_FRAME_DROP, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_drop_main_loop as *const () as _))
}

pub unsafe extern "C" fn specialhi_drop_main_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    } 
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
	if !StatusModule::is_changing(fighter.module_accessor)
	&& StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
	}

    0.into()
}

unsafe extern "C" fn specialhi_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}
pub unsafe extern "C" fn specialhi_landing_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);

    let landing_motion = hash40("special_hi_landing");
    let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if landing_frame != 0.0 {
        let motion_rate = fighter.sub_get_landing_motion_rate(landing_motion.into(), landing_frame.into()).get_f32();
        MotionModule::set_rate(fighter.module_accessor, motion_rate);
    }

    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(landing_motion), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_landing_main_loop as *const () as _))
}

pub unsafe extern "C" fn specialhi_landing_main_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    } 
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
	if !StatusModule::is_changing(fighter.module_accessor)
	&& StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
	}

    0.into()
}


unsafe extern "C" fn fall_from_specialhi_motion(fighter: &mut L2CFighterCommon) {
    let motion_n = hash40("special_hi2_end");
    let motion_f = hash40("fall_special_f");
    let motion_b = hash40("fall_special_b");
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_n), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int64(fighter.module_accessor, motion_n as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
    WorkModule::set_int64(fighter.module_accessor, motion_n as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
}
unsafe extern "C" fn fall_from_specialhi_main(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL) {return;}
    fall_from_specialhi_motion(fighter);
}
unsafe extern "C" fn fall_from_specialhi_main_loop(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL) {return;}

    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall_special"), 0.0, 1.0, false, 0.0, false, false);
        let motion_f = hash40("fall_special_f");
        let motion_b = hash40("fall_special_b");
        WorkModule::set_int64(fighter.module_accessor, motion_f as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
        WorkModule::set_int64(fighter.module_accessor, motion_b as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
    }

    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
        let fall_motions = [hash40("special_hi2_end"),hash40("fall_special"),hash40("fall_special_f"),hash40("fall_special_b")];
        if !fall_motions.contains(&motion) {
            return;
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
        let motion_f = hash40("fall_f");
        let motion_b = hash40("fall_b");
        WorkModule::set_int64(fighter.module_accessor, motion_f as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
        WorkModule::set_int64(fighter.module_accessor, motion_b as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
    }
    else {
        let fall_motions = [hash40("fall"),hash40("fall_f"),hash40("fall_b")];
        if !fall_motions.contains(&motion) {
            return;
        }
        fall_from_specialhi_motion(fighter);
    }
}

unsafe extern "C" fn fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_FallSub(0.into()); //sure...
    fall_from_specialhi_main(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(fall_main_loop as *const () as _))
}
unsafe extern "C" fn fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Fall_Main();
    fall_from_specialhi_main_loop(fighter);
    0.into()
}
unsafe extern "C" fn fall_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_FallAerialSub();
    fall_from_specialhi_main(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(fall_aerial_main_loop as *const () as _))
}
unsafe extern "C" fn fall_aerial_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_FallAerial_Main();
    fall_from_specialhi_main_loop(fighter);
    0.into()
}

unsafe extern "C" fn landing_from_special_hi_main(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL) {return;}
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
}
unsafe extern "C" fn landinglight_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LandingLightSub();
    landing_from_special_hi_main(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_LandingLight_Main as *const () as _))
}
unsafe extern "C" fn landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_uniq_process_init();
    
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    //if motion != hash40("landing_fall_special") {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL) {
        return 0.into();
    }
    let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if landing_frame != 0.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    0.into()
}
unsafe extern "C" fn landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_uniq_landing = WorkModule::is_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);

    fighter.status_LandingSub();
    landing_from_special_hi_main(fighter);
    fighter.status_LandingStiffness();
    fighter.sub_landing_start_check_damage_face();
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let fall_special_motion = hash40("landing_fall_special");
    //if motion != fall_special_motion {
    if !is_uniq_landing {
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Landing_Main as *const () as _));
    }
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
    
    let landing_frame = LANDING_FRAME_CANCEL;//WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    println!("Landing Frame after: {landing_frame}");
    if landing_frame != 0.0 {
        let motion_rate = fighter.sub_get_landing_motion_rate(fall_special_motion.into(), landing_frame.into()).get_f32();
        MotionModule::set_rate(fighter.module_accessor, motion_rate);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(landing_from_special_hi_main_loop as *const () as _))
}
unsafe extern "C" fn landing_from_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common_pre().get_bool();
    }
    else {
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
    }
    fighter.status_Landing_Main()
}

unsafe extern "C" fn landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    0.into()
}

unsafe extern "C" fn landingattackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LandingAttackAirSub();
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_LandingAttackAir_Main as *const () as _))
}
unsafe extern "C" fn landingfallspecial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_landing_fall_special_sub();
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_BUDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_landing_fall_special_main as *const () as _))
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_end);

    //agent.status(Init, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP, specialhi_jump_init);
    agent.status(Main, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP, specialhi_jump_main);   
    agent.status(Exec, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP, specialhi_jump_exec);

    agent.status(Pre, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_FALL, specialhi_fall_pre);
    agent.status(Init, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_FALL, specialhi_fall_init);
    agent.status(Main, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_FALL, specialhi_fall_main);
    agent.status(Exec, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_FALL, specialhi_fall_exec);
    agent.status(End, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_FALL, specialhi_fall_end);

    agent.status(Pre, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_DROP, specialhi_drop_pre);
    agent.status(Main, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_DROP, specialhi_drop_main);
    agent.status(End, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_DROP, specialhi_fall_end);

    agent.status(Pre, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_LANDING, specialhi_landing_pre);
    agent.status(Main, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_LANDING, specialhi_landing_main);
    agent.status(End, FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_LANDING, empty_status);
    
    agent.status(Main, *FIGHTER_STATUS_KIND_FALL, fall_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_FALL_AERIAL, fall_aerial_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_LANDING_LIGHT, landinglight_main);
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING, landing_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_LANDING, landing_main);
    agent.status(End, *FIGHTER_STATUS_KIND_LANDING, landing_end);

    agent.status(Main, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, landingattackair_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landingfallspecial_main);
}