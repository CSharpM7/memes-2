use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;
use crate::special_pummel::imports::*;

pub const FIGHTER_ROSETTA_STATUS_CATCH_FLAG_STARPIECES: i32 = 0x2100000D;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_ROSETTA_STATUS_CATCH_FLAG_STARPIECES) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 100, 30, 0, 7.0, 0.0, 10.0, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        }
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_wand_light"), Hash40::new("havel"), 0, 7.5, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_wand_stardust"), Hash40::new("havel"), 0, 7.5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

/*
STATUS
*/
pub unsafe extern "C" fn is_tico_down(boma: *mut BattleObjectModuleAccessor) -> bool {
    let status = StatusModule::status_kind(boma);
    return (*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE..*WEAPON_ROSETTA_TICO_STATUS_KIND_DOWN).contains(&status) 
    || is_tico_dead(boma);
}
pub unsafe extern "C" fn is_tico_dead(boma: *mut BattleObjectModuleAccessor) -> bool {
    let status = StatusModule::status_kind(boma);
    return (*WEAPON_ROSETTA_TICO_STATUS_KIND_STANDBY..*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD).contains(&status);
}

pub unsafe extern "C" fn catch_attack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special(fighter) {
    }
    return 0.into();
}

pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = catch_attack_main_inner(fighter);

    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
            let tico_boma = get_article_boma(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO);
            
            let tico_is_down = is_tico_down(tico_boma);
            let tico_is_free = ArticleModule::is_flag(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_FREE);
            let tico_status = StatusModule::status_kind(tico_boma);
            println!("Status: {tico_status} Free: {tico_is_free} Down: {tico_is_down}");
            if tico_is_free {
                println!("Request Return");
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL);
                ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, false, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_FREE);
                ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, true, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
                ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, true, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_CATCH_PARENT);
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_rosetta_special_n03"), true, false, false, false, enSEType(0));
            }
            else if !tico_is_down {
                println!("Request Bits");
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_ROSETTA_STATUS_CATCH_FLAG_STARPIECES);
                let lr = PostureModule::lr(fighter.module_accessor);
                //PostureModule::set_lr(tico_boma, lr);
                ArticleModule::set_float(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, lr, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLOAT_TARGET_LR);
                ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO,*WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_S);
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_rosetta_special_s02"), true, false, false, false, enSEType(0));
            }
            else {
                WorkModule::off_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL);
                return catch_attack_main_default(fighter);
            }
        }
    }
    
    to_return
}

pub unsafe extern "C" fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        HitModule::set_hit_stop_mul(fighter.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
        ShakeModule::stop(fighter.module_accessor);
    }
    0.into()
}

pub unsafe extern "C" fn starpiece_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let to_return = smashline::original_status(Main, weapon, *WEAPON_ROSETTA_STARPIECE_STATUS_KIND_SHOOT)(weapon);
    let rosa_boma = get_founder_boma(weapon);
    let rosa_status = StatusModule::status_kind(rosa_boma);
    
    if [*FIGHTER_STATUS_KIND_CATCH_WAIT,*FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&rosa_status) {
        AttackModule::clear_all(weapon.module_accessor);
    }

    return to_return;
}
pub fn install() {
    smashline::Agent::new("rosetta")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Exec, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_exec)
    .install();
    smashline::Agent::new("rosetta_starpiece")
        .status(Main, *WEAPON_ROSETTA_STARPIECE_STATUS_KIND_SHOOT, starpiece_main)
    .install(); 
}
