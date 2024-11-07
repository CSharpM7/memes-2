mod acmd;
use crate::imports::imports_agent::*;
use crate::special_pummel::imports::*;

pub unsafe extern "C" fn catch_attack_snake(fighter: &mut L2CFighterCommon) -> L2CValue {
    let has_c4 = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4);
    WorkModule::set_flag(fighter.module_accessor, has_c4, FIGHTER_SNAKE_STATUS_CATCH_FLAG_HAS_C4);
    println!("Set C4 variable: {has_c4}");
    if has_c4 {
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_attack")));
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchAttack_Main as *const () as _));
    }
    fighter.status_CatchAttack()
}

pub fn install() {
    let agent = &mut smashline::Agent::new("snake");
    acmd::install(agent);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_snake);
    agent.install();
}