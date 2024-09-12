use super::imports::*;
use crate::imports::imports_status::*;
use crate::imports::imports_acmd::*;

pub const FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CRAWL: i32 = 0x21000011;
/*
FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ARTICLE_MOTION_RATE_SYNC: 0x200000E5,
FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_FINAL_EXEC: 0x200000E0,
FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET: 0x200000E4,
FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ST_INIT: 0x200000E1,
 */
pub const FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_CRAWL: i32 = 0x200000E6;

pub unsafe extern "C" fn  game_crawl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    }
}

pub unsafe extern "C" fn  effect_crawl(fighter: &mut L2CAgentBase) {
    //frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("special_lw").hash{
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub unsafe extern "C" fn  sound_crawl(fighter: &mut L2CAgentBase) {
    /*
    if macros::is_excute(fighter) {
        let prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        if ![
            *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW,*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW
        ].contains(&prev){
            macros::PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
        }
    } */
}

pub unsafe extern "C" fn  expression_crawl(fighter: &mut L2CAgentBase) {
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

pub unsafe extern "C" fn  game_speciallw(agent: &mut L2CAgentBase) {
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
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CRAWL);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
}

pub unsafe extern "C" fn  effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        //macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe extern "C" fn  squat_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,true);
}

pub unsafe extern "C" fn  squat_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,false);
}

unsafe extern "C" fn squat_main(fighter: &mut L2CFighterCommon, f: bool) -> L2CValue {
    println!("Crawl: {f}");
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
    check_bomb_input(fighter);
    return 0.into();
}
unsafe extern "C" fn squat_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatB_Main();
    return squat_main_loop(fighter);
}
unsafe extern "C" fn squat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    squat_disable_terms(fighter);
    check_bomb_input(fighter);
    return 0.into();
}

pub unsafe extern "C" fn  squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&prev)
    {
        println!("Squat main: no crawl");
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_CRAWL);
        //smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SQUAT_WAIT)(fighter);
        return fighter.status_SquatWait();
    }
    else {
        println!("Resume crawl");
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
    let next = StatusModule::status_kind_next(fighter.module_accessor);//fighter.global_table[0xB].get_i32();
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,*FIGHTER_STATUS_KIND_SQUAT_WAIT,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&next)
    {
        println!("Exit crawl");
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_CRAWL);
    }
    return 0.into();
}

pub unsafe extern "C" fn  bomb_g_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("BombG");
    return morph_force_crawl(fighter);
}

pub unsafe extern "C" fn  speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_lw = smashline::original_status(Main, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW)(fighter);
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,*FIGHTER_STATUS_KIND_SQUAT_WAIT,
    //*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&prev)
    {
    }

}
pub unsafe extern "C" fn  speciallw_a_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("SpecialLwA");
    morph_force_crawl(fighter);
    return 0.into();
}
pub unsafe extern "C" fn  speciallw_g_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("SpecialLwG");
    morph_force_crawl(fighter);
    return smashline::original_status(Exec, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW)(fighter);
}

unsafe extern "C" fn morph_force_crawl(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let crawlcheck = WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CRAWL)
    || (frame >= 30.0 && frame < 31.0);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CRAWL) {
        println!("Force flag");
    }
    if crawlcheck {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CRAWL);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let stick_y_sensitivity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), Hash40::new_raw(0x10d088fec9).hash);
        if stick_y < stick_y_sensitivity {
            println!("Enter crawl for special lw");
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
            //VarModule::set_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN, samus::BOMB_COOLDOWN_MAX - (frame as i32));
            //VarModule::on_flag(fighter.battle_object, samus::instance::flag::SPECIAL_LW_CRAWL);
            ControlModule::clear_command(fighter.module_accessor, false);
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
        }
    }
    return 0.into();
}

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

pub unsafe extern "C" fn check_bomb_input(fighter: &mut L2CFighterCommon) {
    let can_spawn = true;//VarModule::get_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN) <= 0;
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW)
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
       // let cooldown = VarModule::get_int(fighter.battle_object, samus::instance::int::BOMB_COOLDOWN);
        let cooldown = 0;
        println!("Cooldown: {cooldown}");
        ControlModule::clear_command(fighter.module_accessor, false);
        let article = *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB;
        let maxbomb = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"),hash40("bomb_max_req"));
        if (ArticleModule::get_active_num(fighter.module_accessor, article) as i32) < maxbomb 
        && can_spawn 
        {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        }
    }
}

pub fn install() {   
    Agent::new("samus")
        .acmd("game_squatf", game_crawl, Priority::Default)
        .acmd("effect_squatf", effect_crawl, Priority::Default)
        .acmd("sound_squatf", sound_crawl, Priority::Default)
        .acmd("expression_squatf", expression_crawl, Priority::Default)
        
        .acmd("game_squatb", game_crawl, Priority::Default)
        .acmd("effect_squatb", effect_crawl, Priority::Default)
        .acmd("sound_squatb", sound_crawl, Priority::Default)
        .acmd("expression_squatb", expression_crawl, Priority::Default)

        .acmd("game_squatn", game_crawl, Priority::Default)

        //.status(Main, *FIGHTER_STATUS_KIND_SQUAT_F, squat_f_main)
        //.status(Main, *FIGHTER_STATUS_KIND_SQUAT_B, squat_b_main)
        .status(Main, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_wait_main)

        .status(Exit, *FIGHTER_STATUS_KIND_SQUAT_F, squat_exit)
        .status(Exit, *FIGHTER_STATUS_KIND_SQUAT_B, squat_exit)
        .status(Exit, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_exit)

        .status(Exec, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, bomb_g_exec)
        .status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, speciallw_g_exec)
        //.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, speciallw_a_exec)

        //.acmd("game_speciallw", game_speciallw, Priority::Default)
        //.acmd("effect_speciallw", effect_speciallw, Priority::Default)
    .install();
}