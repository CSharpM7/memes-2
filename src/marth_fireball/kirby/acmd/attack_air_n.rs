use crate::imports::imports_acmd::*;
use crate::marth_fireball::imports::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        println!("?");
        /*
        
            FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER: 0x18,
            FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTERSHOT: 0x19,
            FIGHTER_KIRBY_GENERATE_ARTICLE_STARMISSILE: 0x1A,
            FIGHTER_KIRBY_GENERATE_ARTICLE_STONE: 0x1B,
            FIGHTER_KIRBY_GENERATE_ARTICLE_HAT: 0x1C,
            FIGHTER_KIRBY_GENERATE_ARTICLE_ULTRASWORD: 0x1F,
            FIGHTER_KIRBY_GENERATE_ARTICLE_ROSETTATICOMISSILE: 0x21,
            FIGHTER_KIRBY_GENERATE_ARTICLE_ULTRASWORDHAT: 0x20,
            21 crashes
            FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR: 0x22,
            FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER: 0x25,
            FIGHTER_KIRBY_GENERATE_ARTICLE_WINDUMMY: 0x26,
        
        */
        ArticleModule::generate_article(agent.module_accessor, 0x27, false, -1);
    }
}

pub fn install() {   
    Agent::new("kirby")
        .game_acmd("game_attackairn", game_attackairn)
        .install();
}