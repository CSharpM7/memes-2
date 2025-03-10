use super::imports::*;
use crate::imports::imports_status::*;
use crate::imports::imports_acmd::*;

pub const FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_FROM_CRAWL: i32 = 0x21000012; //True if entered SpecialLw from Squat
pub const FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOCKOUT: i32 = 0x100000C3; //Frames to prevent going into Special Lw again
pub const FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_FROM_CRAWL: i32 = 0x200000E5; //True if entered SpecialLw from Squat

pub const FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_CRAWL: i32 = 0x200000E6; //Preserves if samus is crawling, used to enter squat_n instead of squat

/*
ACMD
 */
pub unsafe extern "C" fn game_crawl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    }
}

pub unsafe extern "C" fn expression_crawl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10, true);
    }
    for i in 1..i32::MAX{
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 30.0);
    }
}

pub unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
}
unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_FROM_CRAWL){
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
pub unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
}

/*
STATUS: SQUAT
 */

 pub unsafe extern "C" fn squat_disable_terms(fighter: &mut L2CFighterCommon) {
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);

    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    //WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);

    if VarModule::get_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN) > 0{
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    }
}

pub unsafe extern "C" fn squat_check_bomb_input(fighter: &mut L2CFighterCommon) -> bool {
    let can_spawn = WorkModule::get_int(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOCKOUT) <= 0;
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW)
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
        ControlModule::clear_command(fighter.module_accessor, false);
        let article = *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB;
        let maxbomb = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"),hash40("bomb_max_req"));
        if (ArticleModule::get_active_num(fighter.module_accessor, article) as i32) < maxbomb 
        && can_spawn 
        {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_FROM_CRAWL);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
            return true;
        }
    }
    return false;
}

pub unsafe extern "C" fn squat_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,true);
}

pub unsafe extern "C" fn squat_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,false);
}

unsafe extern "C" fn squat_main(fighter: &mut L2CFighterCommon, f: bool) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_CRAWL);
    if f {
        fighter.status_SquatF();
        return fighter.main_shift(squat_f_main_loop)
    }
    else{
        fighter.status_SquatB();
        return fighter.main_shift(squat_b_main_loop)
    }
}
unsafe extern "C" fn squat_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatF_Main();
    squat_disable_terms(fighter);
    if squat_check_bomb_input(fighter) {return 1.into();}
    return 0.into();
}
unsafe extern "C" fn squat_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatB_Main();
    return squat_main_loop(fighter);
}
unsafe extern "C" fn squat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    squat_disable_terms(fighter);
    if squat_check_bomb_input(fighter) {return 1.into();}
    return 0.into();
}

pub unsafe extern "C" fn squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&prev)
    {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_CRAWL);
        return fighter.status_SquatWait();
    }
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_CRAWL);

    fighter.status_SquatWait_common(0xc0.into());

    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("squat_n"), 6.0,0.0, 0.0);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);

    fighter.main_shift(squat_wait_main_loop)
}


unsafe extern "C" fn squat_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait_Main();
    squat_main_loop(fighter);

    return 0.into();
}

unsafe extern "C" fn squat_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next = StatusModule::status_kind_next(fighter.module_accessor);
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,*FIGHTER_STATUS_KIND_SQUAT_WAIT,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&next)
    {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_CRAWL);
    }
    return 0.into();
}
/*
STATUS: SPECIAL LW
*/
unsafe extern "C" fn morph_force_crawl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let stick_y_sensitivity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), Hash40::new_raw(0x10d088fec9).hash);
        if stick_y < stick_y_sensitivity {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
            ControlModule::clear_command(fighter.module_accessor, false);
            let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_lw"), true) as i32;
            let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
            let lock_frame = (cancel_frame - status_frame).max(0);
            WorkModule::set_int(fighter.module_accessor, lock_frame as i32, FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOCKOUT);
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            return 1.into();
            
        }
    }
    return 0.into();
}

pub unsafe extern "C" fn bomb_g_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, fighter, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G)(fighter);
    return fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_common_loop as *const () as _));
}

pub unsafe extern "C" fn speciallw_g_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
    smashline::original_status(Main, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW)(fighter);

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_FROM_CRAWL) {
        //Keep speed from crawl
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_FROM_CRAWL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV_CONT);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        let sp_lw_gr_ax_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("sp_lw_gr_ax_mul"));
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            sp_lw_gr_ax_mul
        );

        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let stable_speed_x = sv_kinetic_energy::get_stable_speed_x(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        let sp_lw_gr_vx_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("sp_lw_gr_vx_mul"));

        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let stable_speed_y = sv_kinetic_energy::get_stable_speed_y(fighter.lua_state_agent);
        fighter.clear_lua_stack();

        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            stable_speed_x*sp_lw_gr_vx_mul,
            stable_speed_y
        );

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_FROM_CRAWL);
    return fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_common_loop as *const () as _));
}
pub unsafe fn special_lw_common_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_FROM_CRAWL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    if morph_force_crawl(fighter).get_i32() == 1 {
        return 1.into();
    };
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw") {
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_lw"), true) as i32;
        let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
        if MotionModule::frame(fighter.module_accessor) > 3.0
        || status_frame >= cancel_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
        }
    }

    /* Original */
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
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[0xE].get_f32() > 1.0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A.into(), false.into());
                return 1.into();
            }
        }
    }
    //Check bomb
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
                let max_bomb = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("bomb_max_req")) as u64;
                if ArticleModule::get_active_num(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB) < max_bomb {
                    ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB,false,-1);
                    ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                }
            }
        }
    }

    0.into()
}
/*
OPFF
 */
pub unsafe extern "C" fn samus_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let lockout = WorkModule::get_int(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOCKOUT);
    if lockout > 0 {
        WorkModule::count_down_int(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOCKOUT, 0);
    }
}
/*
AGENT
 */
unsafe extern "C" fn special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lockout = WorkModule::get_int(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOCKOUT);
    if lockout > 0 {
        return false.into();
    }
    true.into()
}

unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(special_lw_callback as *const () as _));   
}

pub fn install() {   
    Agent::new("samus")
        .acmd("game_squatf", game_crawl, Priority::Default)
        .acmd("effect_squatf", acmd_stub, Priority::Default)
        .acmd("sound_squatf", acmd_stub, Priority::Default)
        .acmd("expression_squatf", expression_crawl, Priority::Default)
        
        .acmd("game_squatb", game_crawl, Priority::Default)
        .acmd("effect_squatb", acmd_stub, Priority::Default)
        .acmd("sound_squatb", acmd_stub, Priority::Default)
        .acmd("expression_squatb", expression_crawl, Priority::Default)

        .acmd("game_squatn", game_crawl, Priority::Default)

        .acmd("game_speciallw", game_speciallw, Priority::Default)
        .acmd("effect_speciallw", effect_speciallw, Priority::Default)
        
        .status(Main, *FIGHTER_STATUS_KIND_SQUAT_F, squat_f_main)
        .status(Main, *FIGHTER_STATUS_KIND_SQUAT_B, squat_b_main)
        .status(Main, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_wait_main)

        .status(Exit, *FIGHTER_STATUS_KIND_SQUAT_F, squat_exit)
        .status(Exit, *FIGHTER_STATUS_KIND_SQUAT_B, squat_exit)
        .status(Exit, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_exit)
        
        .status(Main, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, bomb_g_main) 
        .status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, speciallw_g_main)

        .on_line(Main, samus_update)
        .on_start(agent_start)
    .install();
}