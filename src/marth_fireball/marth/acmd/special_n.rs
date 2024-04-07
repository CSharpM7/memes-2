use crate::imports::imports_acmd::*;
use crate::marth_fireball::imports::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        println!("Fireball?");
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_MARTH_GENERATE_ARTICLE, false, -1);
    }
}

pub fn install() {   
    Agent::new("marth")
        .game_acmd("game_specialnstart", game_specialnstart)
        .game_acmd("game_specialairnstart", game_specialnstart)
        .install();
}