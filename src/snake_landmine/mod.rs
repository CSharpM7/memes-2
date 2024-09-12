use smash::lib::lua_const::*;
use super::imports::imports_acmd::*;
use smashline::*;

pub static mut FIGHTER_SNAKE_GENERATE_ARTICLE_LANDMINE: i32 = 0xE;


unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        println!("Fireball?");
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_SNAKE_GENERATE_ARTICLE_LANDMINE, false, -1);
    }
}


pub fn install() {
    unsafe {
        FIGHTER_SNAKE_GENERATE_ARTICLE_LANDMINE += smashline::clone_weapon("miiswordsman", *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_WAVE, "snake", "fireball",false);
    }
    Agent::new("snake")
        .acmd("game_attacklw3", game_attacklw3,Priority::Default)
        .install();
}