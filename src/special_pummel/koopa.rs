use crate::imports::imports_acmd::*;
use crate::special_pummel::imports::*;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 361, 100, 30, 0, 7.0, 0.0, 10.0, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_KOOPA_STATUS_BREATH_FLAG_GENE_BREATH);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor,*FIGHTER_KOOPA_STATUS_BREATH_FLAG_GENE_BREATH);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor,*FIGHTER_KOOPA_STATUS_BREATH_FLAG_GENE_BREATH);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("head"), -2, 5, 0, 180, 0, 50, 1, true);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_koopa_special_n02_win03"));
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        SoundModule::stop_status_se(agent.module_accessor);
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 11.0);
    for i in 0..5 {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        wait(agent.lua_state_agent, 10.0);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub unsafe extern "C" fn catch_attack_init_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let interval = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("gene_interval"));
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_GENERATE_COUNT);
    0.into()
}
pub unsafe extern "C" fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        HitModule::set_hit_stop_mul(fighter.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
        ShakeModule::stop(fighter.module_accessor);
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_KOOPA_STATUS_BREATH_FLAG_GENE_BREATH)  {
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_KOOPA_STATUS_BREATH_WORK_FLOAT_GENE_ANGLE);
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_GENERATE_COUNT);
            let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_GENERATE_COUNT);
            if count <= 0 {
                let interval = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("gene_interval"));
                WorkModule::set_int(fighter.module_accessor, interval/2, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_GENERATE_COUNT);
                //ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("koopa_breath"), Hash40::new("head"), -2, 5, 5, 180, 0, 50, 0.9, true);
            }
        }
    }
    0.into()
}

pub fn install() {
    smashline::Agent::new("koopa")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        .status(Init, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_init_uniq)
        .status(Exec, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_exec)
    .install();
}
