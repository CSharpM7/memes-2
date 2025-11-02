/*
Use this as a template for cloning articles. 
Wave should be a decent test to see if a character can use cloned articles.
*/
use smash::lib::lua_const::*;
use crate::imports::imports_acmd::*;
use smashline::*;

mod wave;

pub const GENERATE_ARTICLE_LAST: i32 = 0x6;
pub static mut GENERATE_ARTICLE_WAVE: i32 = 0x0;


unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        println!("Req gen");
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, GENERATE_ARTICLE_WAVE, false, -1);
    }
}


pub fn install_hook() {
	unsafe {
		GENERATE_ARTICLE_WAVE += GENERATE_ARTICLE_LAST + smashline::clone_weapon("miiswordsman", *WEAPON_KIND_MIISWORDSMAN_WAVE, 
        "pichu", "wave",false);
	}
}
pub fn install() {
    #[cfg(feature = "dev")]{
		unsafe {
			GENERATE_ARTICLE_WAVE += GENERATE_ARTICLE_LAST;
		}
	}

    Agent::new("pichu")
        .acmd("game_attack11", game_attack11,Priority::Default)
        .install();

	wave::install();
}