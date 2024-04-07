use crate::imports::imports_acmd::*;
use crate::marth_fireball::imports::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    for i in 0..0x1D { //29
        if macros::is_excute(agent) {
            /*
            
                FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER: 0x18,
                FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTERSHOT: 0x19,
                FIGHTER_KIRBY_GENERATE_ARTICLE_STARMISSILE: 0x1A,
                FIGHTER_KIRBY_GENERATE_ARTICLE_STONE: 0x1B,
                FIGHTER_KIRBY_GENERATE_ARTICLE_HAT: 0x1C,
                //1xD Crashes
                FIGHTER_KIRBY_GENERATE_ARTICLE_ULTRASWORD: 0x1F,marth/specialnkirby
                FIGHTER_KIRBY_GENERATE_ARTICLE_ULTRASWORDHAT: 0x20,
                FIGHTER_KIRBY_GENERATE_ARTICLE_ROSETTATICOMISSILE: 0x21,
                21 crashes
                FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR: 0x22,
                0x23,24 does nothing
                FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER: 0x25,
                FIGHTER_KIRBY_GENERATE_ARTICLE_WINDUMMY: 0x26,
            
            */
            println!("{}",i);
            ArticleModule::generate_article(agent.module_accessor, i, false, -1);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, 0x27, false, -1);
    }
}

pub fn install() {   
    Agent::new("kirby")
        .game_acmd("game_attackairn", game_attackairn)
        .install();
}