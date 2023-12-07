use crate::imports::imports_status::*;
use crate::air_smash::imports::*;

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    VarModule::set_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE, 0);
    let hold_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_frame"), 0);
    VarModule::set_int(fighter.battle_object, fighter::instance::int::SMASH_HOLD_FRAME, hold_frame_max);

    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    let test = WorkModule::get_param_float(fighter.module_accessor, hash40("edge"), hash40("landing_attack_air_frame_f"));
    //println!("Param: {test}");

    let mut fighter_log_attack_kind = 0;
    if motion_kind == hash40("attack_air_n") {
        fighter_log_attack_kind = *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N;
    }
    else if motion_kind == hash40("attack_air_f") {
        fighter_log_attack_kind = *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F;
    }
    else if motion_kind == hash40("attack_air_b") {
        fighter_log_attack_kind = *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B;
    }
    else if motion_kind == hash40("attack_air_hi") {
        fighter_log_attack_kind = *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI;
    }
    else if motion_kind == hash40("attack_air_lw") {
        fighter_log_attack_kind = *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW;
    }
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, fighter_log_attack_kind);

    let flick_x = ControlModule::get_flick_x(fighter.module_accessor);
    let flick_y = ControlModule::get_flick_after_y(fighter.module_accessor);
    let flick_y2 = ControlModule::get_flick_no_reset_y(fighter.module_accessor);
    let smash_x_param = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("special_smash_flick_x"));
    let smash_y_param = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("attack_hi4_flick_y"));
    let smash_x = flick_x <= smash_x_param;// && [*FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F,*FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B].contains(&fighter_log_attack_kind);
    let smash_y = flick_y <= smash_y_param; //&&[*FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI,*FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW].contains(&fighter_log_attack_kind);

    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    let smash_input = !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SMASH) 
    || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0
    || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
    || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0;

    println!("Input: {smash_input} SmashType: {fighter_log_attack_kind}");
    if smash_input 
    || smash_x || smash_y 
    {
        VarModule::set_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE, fighter_log_attack_kind);
        StatusModule::change_status_force(fighter.module_accessor, FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), false.into());

        WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    }
    //fighter.main_shift(attack_air_main_loop)
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_loop as *const () as _))
}

pub unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);

    fighter.status_AttackAir_Main()
}

pub unsafe extern "C" fn attack_s4_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if !(//StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && 
    smash_type>0) {
        return fighter.status_pre_AttackS4Start();
    }
    return attack_s4_shared_pre(fighter)
}
pub unsafe extern "C" fn attack_s4_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if !(//StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && 
    smash_type>0) {
        return fighter.status_pre_AttackS4Hold();
    }
    return attack_s4_shared_pre(fighter)
}
pub unsafe extern "C" fn attack_s4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if !(//StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && 
    smash_type>0) {
        return fighter.status_pre_AttackS4();
    }
    return attack_s4_shared_pre(fighter)
}
pub unsafe extern "C" fn attack_s4_shared_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        //*FIGHTER_KINETIC_TYPE_AIR_STOP,
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT,
        *FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_S4 |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_HAJIKI
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32,
        0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    0.into()
}

pub unsafe extern "C" fn attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if !(//StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && 
    smash_type>0) {
        return fighter.status_AttackS4Start();
    }
    let mut new_motion = "";
    if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F {
        new_motion = "attack_air_f";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B {
        new_motion = "attack_air_f";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI {
        new_motion = "attack_air_hi4";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW {
        new_motion = "attack_air_lw";
    }
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, smash_type);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(new_motion), 0.0, 1.0, false, 0.0, false, false);

    /*
    //KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    
    let brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_brake_x"), 0);
    let brake_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_brake_y"), 0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL); 
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        brake_x,
        brake_y
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
 */
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_smash_start as *const () as _))
}

pub unsafe extern "C" fn attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if !(//StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && 
    smash_type>0) {
        return fighter.status_AttackS4Hold();
    }

    let mut new_motion = "";
    if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F {
        new_motion = "attack_air_f_hold";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B {
        new_motion = "attack_air_f_hold";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI {
        new_motion = "attack_air_hi4_hold";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW {
        new_motion = "attack_air_lw_hold";
    }
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, smash_type);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(new_motion), 0.0, 1.0, false, 0.0, false, false);

    //KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_frame"), 0);
    let keep_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_keep_frame"), 0);
    WorkModule::set_int(fighter.module_accessor, hold_frame,*FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    WorkModule::set_int(fighter.module_accessor, keep_frame,*FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);

    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_smash_hold as *const () as _))
}

pub unsafe extern "C" fn attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if !(//StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && 
    smash_type>0) {
        return fighter.status_AttackS4();
    }
    let mut new_motion = "";
    if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F {
        new_motion = "attack_air_f";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B {
        new_motion = "attack_air_f";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI {
        new_motion = "attack_air_hi4";
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW {
        new_motion = "attack_air_lw";
    }
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, smash_type);
    let resume_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(new_motion), resume_frame, 1.0, false, 0.0, false, false);

    let hold_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_frame"), 0) as f32;
    let hold_frame = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_HOLD_FRAME) as f32;
    //let hold_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME) as f32;
    let hold_ratio = 1.0-(hold_frame/hold_frame_max);
    let attack_mul = lerp(1.0,1.4,hold_ratio);
    println!("Hold: {hold_frame}, Max: {hold_frame_max}, Mul: {attack_mul}");
    AttackModule::set_power_mul_status(fighter.module_accessor, attack_mul);
    //1.0->1.4

    /*
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL); 
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); 
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y
    );
    */

    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_smash as *const () as _))
}

pub unsafe extern "C" fn attack_air_smash_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_smash_main_loop(fighter,0)
}
pub unsafe extern "C" fn attack_air_smash_hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_smash_main_loop(fighter,1)
}
pub unsafe extern "C" fn attack_air_smash(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_smash_main_loop(fighter,2)
}

unsafe fn get_attack_air_info(fighter: &mut L2CFighterCommon) -> (u64,Hash40,smash::lib::LuaConst) {

    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if smash_type == 0 {
        return (0,Hash40::new(""),FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N);
    }

    let landing_lag_param;
    let landing_mot;
    let kind;
    if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F {
        landing_lag_param = hash40("landing_attack_air_frame_f");
        landing_mot = Hash40::new("landing_air_f4");
        kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F;
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B {
        landing_lag_param = hash40("landing_attack_air_frame_f");
        landing_mot = Hash40::new("landing_air_f4");
        kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F;
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI {
        landing_lag_param = hash40("landing_attack_air_frame_hi");
        landing_mot = Hash40::new("landing_air_hi4");
        kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI;
    }
    else if smash_type == *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW {
        landing_lag_param = hash40("landing_attack_air_frame_lw");
        landing_mot = Hash40::new("landing_air_lw4");
        kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW;
    }
    else {
        landing_lag_param = hash40("landing_attack_air_frame_n");
        landing_mot = Hash40::new("landing_air_n4");
        kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N;
    }
    return (landing_lag_param,landing_mot,kind);
}

pub unsafe extern "C" fn attack_air_smash_main_loop(fighter: &mut L2CFighterCommon, phase: i32) -> L2CValue {
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation == *SITUATION_KIND_GROUND {
            //landing lag
            let attack_info = get_attack_air_info(fighter);
            let landing_lag_param = attack_info.0;
            let landing_mot= attack_info.1;
            let kind= attack_info.2;
            if phase < 2 {
                let landing_lag = 12.0;//.max(WorkModule::get_param_float(fighter.module_accessor, landing_lag_param, 0) * 0.5);
                WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            }

            let new_status = if phase < 2 {FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL} else {FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR};
            fighter.change_status(new_status.into(), false.into());
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) && phase == 2{
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) 
    && phase == 2 {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    fighter.sub_air_check_dive();  
    if phase == 0 {
        //if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_HOLD.into(), false.into());
                    return 1.into();
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4.into(), false.into());
                    return 1.into();
                }
            }
        /*}
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4.into(), false.into());
                return 1.into();
            }
        }
        */
    }
    else if phase == 1 {
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4.into(), false.into());
            return 1.into();
        }
        fighter.sub_smash_hold_uniq(true.into());
        let hold = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
        let keep = WorkModule::get_int(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);
        VarModule::set_int(fighter.battle_object, fighter::instance::int::SMASH_HOLD_FRAME, hold);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME) <= 0
        //&& WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME)
        {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4.into(), false.into());
            return 1.into();
        }
    }
    else {     
    }
    0.into()
}


pub unsafe extern "C" fn landing_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
    if smash_type == 0{
        return fighter.status_LandingAttackAir();
    }
    //fighter.status_LandingAttackAir();

    let attack_info = get_attack_air_info(fighter);
    let landing_lag_param = attack_info.0;
    let landing_mot= attack_info.1;
    let kind= attack_info.2;

    fighter.sub_landing_attack_air_init(landing_mot.hash.into(), landing_lag_param.into(), 0.into());
    MotionModule::change_motion(fighter.module_accessor, landing_mot, 0.0, 1.0, false, 0.0, false, false);
    AttackModule::clear_all(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(landing_smash_air as *const () as _))
}
pub unsafe extern "C" fn landing_smash_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LandingAttackAir_Main()
}

#[skyline::hook(replace = L2CFighterCommon_sub_landing_uniq_process_init_main_param)]
pub unsafe extern "C" fn sub_landing_uniq_process_init_main_param(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue) {
    let landing_mot = param_3.get_u64();
    let status_interrupt = fighter.global_table[0x9].get_i32();
    if status_interrupt != param_2.get_i32()
    && status_interrupt != *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT {
        if status_interrupt != param_4.get_i32() {
            if status_interrupt == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
                let smash_type = VarModule::get_int(fighter.battle_object, fighter::instance::int::SMASH_AIR_TYPE);
                if smash_type > 0 {
                    println!("Smash landing lag");

                    let aerial_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
                    let attack_info = get_attack_air_info(fighter);
                    let landing_lag_param = attack_info.0;
                    let landing_mot= attack_info.1;
                    let kind= attack_info.2;
                    
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, kind);

                    //TODO figure out what this does
                    fighter.sub_landing_attack_air_init(landing_mot.hash.into(), landing_lag_param.into(), 0.into());

                    let mut landing_lag = WorkModule::get_param_float(fighter.module_accessor, landing_lag_param, 0);
                    let new_rate = landing_lag/(20.0-0.0);
                    MotionModule::set_rate(fighter.module_accessor,1.0);
                    MotionModule::change_motion(fighter.module_accessor, landing_mot, 0.0, 1.0, false, 0.0, false, false);

                    return;
                }
            }
        }
    }
    return original!()(fighter, param_1, param_2, param_3, param_4);
}


fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_landing_uniq_process_init_main_param
        );
    }
}

pub fn install() {   
    skyline::nro::add_hook(nro_hook);
}