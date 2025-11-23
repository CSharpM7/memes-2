use crate::imports::imports_status::*;
use super::imports::*;

unsafe extern "C" fn status_pre_Guard_common(fighter: &mut L2CFighterCommon,status: i32) -> L2CValue {
    let mut keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;
    let mut keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT;
    let mut keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT;
    let mut fs_succeeds = 0;

    if status == *FIGHTER_STATUS_KIND_GUARD {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_FLOAT;
        fs_succeeds = *FS_SUCCEEDS_KEEP_VISIBILITY;
    }
    else if status == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_FLOAT;
        fs_succeeds = *FS_SUCCEEDS_KEEP_RUMBLE;
    }
    else if status == *FIGHTER_STATUS_KIND_GUARD_OFF {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLOAT;
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true,
        keep_flag,
        keep_int,
        keep_float,
        fs_succeeds
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    0.into()
}
#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardOn)]
unsafe extern "C" fn status_pre_GuardOn(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_pre_Guard_common(fighter,*FIGHTER_STATUS_KIND_GUARD_ON)
}
#[skyline::hook(replace = L2CFighterCommon_status_pre_Guard)]
unsafe extern "C" fn status_pre_Guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_pre_Guard_common(fighter,*FIGHTER_STATUS_KIND_GUARD)
}
#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardDamage)]
unsafe extern "C" fn status_pre_GuardDamage(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_pre_Guard_common(fighter,*FIGHTER_STATUS_KIND_GUARD_DAMAGE)
}
#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardOff)]
unsafe extern "C" fn status_pre_GuardOff(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_pre_Guard_common(fighter,*FIGHTER_STATUS_KIND_GUARD_OFF)
}

unsafe extern "C" fn status_guard_update_kinetics(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND 
    && status_frame > 2 {
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(), FIGHTER_KINETIC_TYPE_FALL.into());
        fighter.sub_set_ground_correct_by_situation(false.into());
        return false.into();
    }
    else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR 
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR 
    && status_frame > 2 {
        let status = StatusModule::status_kind(fighter.module_accessor);
        let next_status = if status == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {FIGHTER_STATUS_KIND_DAMAGE_FALL} else {FIGHTER_STATUS_KIND_FALL};
        fighter.change_status(next_status.into(), false.into());
        ControlModule::clear_command(fighter.module_accessor, true);
        return true.into();
        //fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(), FIGHTER_KINETIC_TYPE_FALL.into());
        //fighter.sub_set_ground_correct_by_situation(false.into());
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_on_main_air_common)]
unsafe extern "C" fn sub_status_guard_on_main_air_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_guard_update_kinetics(fighter)
}
#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common_air)]
unsafe extern "C" fn status_guard_main_common_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_guard_update_kinetics(fighter)
}
#[skyline::hook(replace = L2CFighterCommon_status_guard_damage_main_common_air)]
unsafe extern "C" fn status_guard_damage_main_common_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_guard_update_kinetics(fighter)
}
#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_air)]
unsafe extern "C" fn sub_status_guard_off_main_common_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_guard_update_kinetics(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_escape)]
unsafe extern "C" fn sub_transition_group_check_air_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) 
    {
        let status = StatusModule::status_kind(fighter.module_accessor); //???
        if ![*FIGHTER_STATUS_KIND_GUARD_ON,*FIGHTER_STATUS_KIND_GUARD,*FIGHTER_STATUS_KIND_GUARD_DAMAGE,*FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status) {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
            return true.into();
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOn_Main)]
unsafe extern "C" fn status_guardon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        return original!()(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
        return 0.into();
    }
    original!()(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common)]
unsafe extern "C" fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        return original!()(fighter);
    }
    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD) > 0.0 {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME) <= 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
                return true.into();
            }
        }
    }
    original!()(fighter)
}
#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Main)]
unsafe extern "C" fn status_guardoff_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        return original!()(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    original!()(fighter)
}


#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_Main)]
unsafe extern "C" fn status_guarddamage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        return original!()(fighter);
    }
    if fighter.status_guard_damage_main_common_air().get_bool() {
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
            if fighter.FighterStatusGuard__is_continue_just_shield_count().get_bool() {
                 fighter.status_guard_damage_main_common();
                 return 0.into();
            }
            if StopModule::is_hit(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP);
                if CancelModule::is_enable_cancel(fighter.module_accessor) {
                    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                        StopModule::cancel_hit_stop(fighter.module_accessor);
                        return 0.into();
                    }
                }
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP);
            };
        }
    }
    fighter.status_guard_damage_main_common();
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_damage_main_common)]
unsafe extern "C" fn status_guard_damage_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        return original!()(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME) == 0 {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD) {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
            return 1.into();
        }
    }
    //original!()(fighter)
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_GuardOn,
            status_pre_Guard,
            //status_pre_GuardDamage,
            status_pre_GuardOff,

            sub_status_guard_on_main_air_common,
            status_guard_main_common_air,
            status_guard_damage_main_common_air,
            sub_status_guard_off_main_common_air,
    
            sub_transition_group_check_air_escape,

            status_guardon_main,
            status_guard_main_common,
            status_guardoff_main,

            //status_guarddamage_main,
            //status_guard_damage_main_common
        );
    }
}

pub fn install() {   
    skyline::nro::add_hook(nro_hook);
}