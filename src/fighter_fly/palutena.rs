use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;

/*
Stuff from vl
 */

const TURN_STICK_X: f32 = 0.3;
const INIT_SPEED_X_MUL: f32 = 0.5; //idk
const SPEED_X_MUL: f32 = 0.8; //idk
const SPEED_X_MAX_MUL: f32 = 0.8; //idk
const FLY_SPEED_Y: [f32;2] = [2.18,2.12];
const SHOOT_FLY_NEXT_FRAME: i32 = 30; //I guess this is for item shooting?


//Listen im not gonna do FIGHTER_STATUS_KIND_ITEM_SHOOT_FLY too
unsafe extern "C" fn game_jumpaerialf1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_JUMP_FLY_NEXT);
    }
}
unsafe extern "C" fn effect_jumpaerialf1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn sound_jumpaerialf1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_jump02"));
    }
}

pub unsafe extern "C" fn opff(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
    if status == *FIGHTER_STATUS_KIND_WAIT && status_frame <= 1 {
        let has_fly_motion = MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("jump_aerial_f1"));
        let has_fly_param = WorkModule::get_param_int(fighter.module_accessor, hash40("aerial_type"), 0) == *FIGHTER_JUMP_AERIAL_TYPE_FLY;
        let jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let max_jump = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        println!("Motion: {has_fly_motion} Param: {has_fly_param} Jumps: {max_jump}");
    }
}

pub unsafe extern "C" fn fly_get_motion(fighter: &mut L2CFighterCommon, current_jump: i32) -> L2CValue {
    let jump_num = current_jump - 1;
    let jump_string = format!("jump_aerial_f{}",jump_num);
    println!("Motion: jump_aerial_f{jump_num}");
    return L2CValue::Hash40(Hash40::new(jump_string.as_str()));
}

pub unsafe extern "C" fn fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.status_FlySub(); //YOU

    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    let flick_x = fighter.global_table[FLICK_X].get_f32();
    //0xfe?
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    let flick_y = fighter.global_table[FLICK_Y].get_f32();
    ControlModule::reset_trigger(fighter.module_accessor);
    println!("Flick: {flick_x},{flick_y}");

    //NEW//
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let turn_stick = TURN_STICK_X;//WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
    println!("Stick: {stick_x}, Turn: {turn_stick}");
    if (stick_x*lr <= turn_stick && stick_x.abs() >= turn_stick) {
        println!("Do some turning");
        TurnModule::set_turn(
            fighter.module_accessor,
            Hash40::new("landing"), //landing?
            lr,
            false,
            false,
            true
        );
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    //

    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    fighter.sub_air_check_fall_common_pre();
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
    let current_jump = 2.max(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT));

    let speed_y = FLY_SPEED_Y[(current_jump-2) as usize];
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y
    );

    //let mot = fighter.sub_getFlyMotion().get_hash();
    let mot = fly_get_motion(fighter,current_jump).get_hash();
    MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    if StopModule::is_stop(fighter.module_accessor) {
        fly_sub(fighter,false.into());
    }
    //fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fly_uniq as *const () as _));
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fly_sub as *const () as _));

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        FighterControlModuleImpl::reserve_on_attack_button(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(fly_main_loop as *const () as _))
}
pub unsafe extern "C" fn fly_sub(fighter: &mut L2CFighterCommon,param_1: L2CValue) -> L2CValue {
    fighter.sub_fly_uniq(false.into())
    //0.into()
}


pub unsafe extern "C" fn fly_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_Fly_Main(); //you too?
}


pub fn install() {    
    Agent::new("palutena")
        .on_line(Main,opff)
        .acmd("game_jumpaerialf1", game_jumpaerialf1,Priority::Default)
        .acmd("game_jumpaerialf2", game_jumpaerialf1,Priority::Default)
        .acmd("effect_jumpaerialf1", effect_jumpaerialf1,Priority::Default)
        .acmd("effect_jumpaerialf2", effect_jumpaerialf1,Priority::Default)
        .acmd("sound_jumpaerialf1", sound_jumpaerialf1,Priority::Default)
        .acmd("sound_jumpaerialf2", sound_jumpaerialf1,Priority::Default)

        .status(Init, *FIGHTER_STATUS_KIND_FLY, empty_status)
        .status(Main, *FIGHTER_STATUS_KIND_FLY, fly_main)
    .install();
}