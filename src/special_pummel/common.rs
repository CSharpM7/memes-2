use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;

pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL: i32 = 0x20000116;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL: i32 = 0x20000117;
pub const FIGHTER_INSTANCE_CATCH_ATTACK_COUNT : i32 = 0x100000ED;

pub const FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT: i32 = 0x2100000B;
pub const FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP: i32 = 0x1000007;

pub const PUMMEL_PENALTY_COUNT_MIN: i32 = 0; //0 removes penalty (b). 99 makes it always max penalty (a)
pub const PUMMEL_JAB_PENALTY_COUNT_MIN: i32 = 0; 
pub const PUMMEL_MAX_PENALTY_FACTOR: f32 = 0.75;

        
pub unsafe extern "C" fn lerp_pummel_power(fighter: &mut L2CFighterCommon,a: f32, b: f32) -> f32 {
    let pummels = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_ATTACK_COUNT);
    let penalty_count = if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CATCH_ATTACK 
    {PUMMEL_PENALTY_COUNT_MIN} else {PUMMEL_JAB_PENALTY_COUNT_MIN};

    if penalty_count >= 99 {
        return a;
    }
    else if penalty_count == 0 {
        return b;
    }
    else {
        let t = ((pummels as f32) / (penalty_count as f32)).min(1.0);
        return lerp(a,b,t);
    }
}
// AGENT
pub unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let status_kind_next = StatusModule::status_kind_next(fighter.module_accessor);
    let is_damaged = is_damage_status_next(fighter.module_accessor);
    
    if (
        ![*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind_next)
        && ![*FIGHTER_STATUS_KIND_THROW,*FIGHTER_STATUS_KIND_THROW_KIRBY].contains(&status_kind_next)
        && (status_kind != *FIGHTER_STATUS_KIND_CATCH_ATTACK && status_kind_next != *FIGHTER_STATUS_KIND_CATCH_ATTACK)
    )
    || is_damaged
    {
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL);
        
        if !(*FIGHTER_STATUS_KIND_CATCH_WAIT..*FIGHTER_STATUS_KIND_CAPTURE_JUMP).contains(&status_kind_next) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL);
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_CATCH_ATTACK_COUNT);
        }
    }
    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

// STATUS HELPERS
pub unsafe fn catch_attack_check_special_input(fighter: &mut L2CFighterCommon) -> bool {
    let special_input = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW);
    let can_special = !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL);
    println!("Can special: {can_special}");
    return special_input && can_special;
}

pub unsafe fn catch_attack_check_special_anim_boma(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let has_anim = MotionModule::is_anim_resource(module_accessor, Hash40::new("catch_special"));
    println!("Special pummel has anim: {has_anim}");
    return has_anim;
}

pub unsafe fn catch_attack_check_special_anim(fighter: &mut L2CFighterCommon) -> bool {
    return catch_attack_check_special_anim_boma(fighter.module_accessor);
}

pub unsafe fn catch_attack_check_special(fighter: &mut L2CFighterCommon) -> bool {
    return catch_attack_check_special_input(fighter) && catch_attack_check_special_anim(fighter);
}

// STATUS
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchAttack)]
unsafe fn status_CatchAttack(fighter: &mut L2CFighterCommon) -> L2CValue {
    return catch_attack_main_inner(fighter);
}
pub unsafe extern "C" fn catch_attack_main_new(fighter: &mut L2CFighterCommon, call_original_first: bool) -> L2CValue {
    if call_original_first {
        let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
        if catch_attack_check_special_input(fighter) {
            return catch_attack_main_inner(fighter);
        }
        return original;
    }
    else {
        if catch_attack_check_special_input(fighter) {
            return catch_attack_main_inner(fighter);
        }
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
    }
}
pub unsafe extern "C" fn catch_attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
    if catch_attack_check_special_input(fighter) {
        ControlModule::clear_command(fighter.module_accessor, false);

        if catch_attack_check_special_anim(fighter) {
            catch_special_main(fighter);

            fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
            return fighter.sub_shift_status_main(L2CValue::Ptr(catch_special_main_loop as *const () as _))
        }
        else {
            let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
            let mut next_status = FIGHTER_STATUS_KIND_WAIT;

            if [*FIGHTER_KIND_KIRBY,*FIGHTER_KIND_PFUSHIGISOU,*FIGHTER_KIND_WOLF]
            .contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_SPECIAL_N;}

            else if [*FIGHTER_KIND_GAMEWATCH,*FIGHTER_KIND_ROBOT,*FIGHTER_KIND_JACK]
            .contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_SPECIAL_S;}

            //else if [*FIGHTER_KIND_REFLET,*FIGHTER_KIND_DEMON]
            //.contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_SPECIAL_LW;}
            
            else if [*FIGHTER_KIND_PIKACHU,*FIGHTER_KIND_PICHU].contains(&fighter_kind) {next_status = FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK;}
            else if [*FIGHTER_KIND_POPO,*FIGHTER_KIND_NANA,].contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_ATTACK_LW3;}
            else if *FIGHTER_KIND_PIKMIN == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_S3;}
            else if [*FIGHTER_KIND_MURABITO,*FIGHTER_KIND_SHIZUE].contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_ATTACK_S3;}
            else if *FIGHTER_KIND_PACMAN == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_DASH;}
            else if *FIGHTER_KIND_RYU == fighter_kind {next_status = FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1;}
            else if *FIGHTER_KIND_KEN == fighter_kind {next_status = FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1;}
            else if *FIGHTER_KIND_CLOUD == fighter_kind {
                next_status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL) 
                {FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3} else {FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3}; //Keep it nerfed I guess
            }
            else if *FIGHTER_KIND_DOLLY == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_HI3;}
            else if *FIGHTER_KIND_PICKEL == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_LW3;}
            else if *FIGHTER_KIND_DEMON == fighter_kind {next_status = FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH;}
            else {
                let attack100_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack100_type"), 0);
                let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
                if attack100_type != *FIGHTER_ATTACK100_TYPE_NONE {
                    next_status = FIGHTER_STATUS_KIND_ATTACK_100;
                }
                else {
                    next_status = FIGHTER_STATUS_KIND_ATTACK;
                }
            }            
            println!("New status: {}",*next_status);
            fighter.change_status(next_status.into(), false.into());
            return 1.into()
        }
    }
    catch_attack_main_default(fighter)
}

pub unsafe extern "C" fn catch_attack_main_default(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_ATTACK_COUNT);
    fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_attack")));
    return fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_main_default_loop as *const () as _));
}
pub unsafe extern "C" fn catch_attack_main_default_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_CatchAttack_Main();
}
pub unsafe extern "C" fn catch_special_main(fighter: &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL); 

    let opponent_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
    if opponent_id != OBJECT_ID_NULL {
        let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
        let opponent_damage = DamageModule::damage(opponent, 0);
        let mut clatter = ControlModule::get_clatter_time(opponent, 0);
        let clatter_factor = lerp_pummel_power(fighter,PUMMEL_MAX_PENALTY_FACTOR,1.0);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,Hash40::new("catch_special"),true);
        let bonus_t = (opponent_damage/75.0).min(1.0);
        let clatter_bonus = lerp(0.0,cancel_frame,bonus_t);
        println!("New clatter: {clatter}*{clatter_factor} + {clatter_bonus}.");
        ControlModule::set_clatter_time(opponent, (clatter*clatter_factor)+clatter_bonus,0);
    }
}

pub unsafe extern "C" fn catch_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let opponent_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
    if opponent_id != OBJECT_ID_NULL {
        let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
        WorkModule::off_flag(opponent,*FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_JUMP);

        let mut clatter = ControlModule::get_clatter_time(opponent, 0);
        let disable_clatter = WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
        //println!("Clatter: {clatter} ({disable_clatter})");
        if disable_clatter {
            //clatter = WorkModule::get_float(fighter.module_accessor,FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
            if clatter <= 1.0 {
                ControlModule::set_clatter_time(opponent, 1.0,0);
            }
            //ControlModule::set_clatter_time(opponent, clatter,0);
        }
        else {
            WorkModule::set_float(fighter.module_accessor, clatter, FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
            if clatter <= 1.0 {
                ControlModule::end_clatter(opponent, 0);
                //println!("Reset");
            }
        }
    }
    return fighter.status_CatchAttack_Main();
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_attack_mtrans_post_process)]
unsafe extern "C" fn attack_mtrans_pre_process(fighter: &mut L2CFighterCommon) {
    let original = original!()(fighter);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
        let new_motion = match (attack_combo_max) {
            3 => "attack_13",
            2 => "attack_12",
            _ => "attack_11"
        };
        let power = lerp_pummel_power(fighter,1.25,1.5);
        AttackModule::set_power_mul_status(fighter.module_accessor, power);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new(new_motion), 0.0, 1.0, false, 0.0, false, false);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Attack_Main)]
unsafe extern "C" fn attack_main(fighter: &mut L2CFighterCommon) {
    attack_main_inner(fighter);
}

pub unsafe extern "C" fn attack_main_dev(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Attack();
    return fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_inner as *const () as _));
}

pub unsafe extern "C" fn attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        if AttackModule::is_attack(fighter.module_accessor, 0, false)  {
            for i in 0..6 {
                if AttackModule::is_attack(fighter.module_accessor, i, false)  {
                    (*AttackModule::attack_data(fighter.module_accessor, i, false)).stop_frame = 0.5;
                }
                else {break;}
            }
        }
    }
    return fighter.status_Attack_Main_button(CONTROL_PAD_BUTTON_ATTACK.into(),L2CValue::Ptr(L2CFighterCommon_change_status_jump_mini_attack as *const () as _));
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Attack100_Main_uniq_func)]
unsafe extern "C" fn attack_100_main(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    original!()(fighter,param_1); 

    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        if step == *FIGHTER_STATUS_ATTACK_100_STEP_LOOP {
            let pummels = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_ATTACK_COUNT);
            let power_t = ((pummels as f32) / (PUMMEL_JAB_PENALTY_COUNT_MIN as f32)).min(1.0);
            let power = lerp_pummel_power(fighter,1.5,1.75);
            AttackModule::set_power_mul_status(fighter.module_accessor, power);

            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_LOOPED) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_100_end"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT);
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_100_STEP_END, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_UNIQ, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
            }
        }
        else if step == *FIGHTER_STATUS_ATTACK_100_STEP_END {
            AttackModule::set_power_mul_status(fighter.module_accessor, 0.625);
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        #[cfg(not(feature = "dev"))]{
            println!("Install hooks");
            skyline::install_hooks!(
                status_CatchAttack,
                    
                attack_mtrans_pre_process, //HDR
                attack_main,
                attack_100_main,
            );
        }
        #[cfg(feature = "dev")]
        println!("Dev is in nro hook?");
    }
}

pub fn install() {
    #[cfg(not(feature = "dev"))]{
    skyline::nro::add_hook(nro_hook);
    }

    #[cfg(feature = "devhook")]
    return;

    let common = &mut Agent::new("fighter");
    common.on_start(agent_start);
    //common.status(Main, *FIGHTER_STATUS_KIND_ATTACK, attack_main_dev);
    
    common.install();
}

pub fn uninstall() {
}
