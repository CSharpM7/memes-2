use smash::lib::lua_const::*;
use super::imports::imports_acmd::*;
use smashline::*;

mod pitb_orbitar;

pub static mut FIGHTER_PITB_GENERATE_ARTICLE_ORBITAR: i32 = 0xE;


unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_PITB_GENERATE_ARTICLE_ORBITAR, false, -1);
    }
}


pub fn install_hook() {
	unsafe {
		FIGHTER_PITB_GENERATE_ARTICLE_ORBITAR = *FIGHTER_PITB_ARTICLE_TERM + smashline::clone_weapon("miiswordsman", *WEAPON_KIND_MIISWORDSMAN_WAVE, "pitb", "orbitar",false);
	}
}
pub fn install() {
    #[cfg(feature = "dev")]{
		unsafe {
			FIGHTER_PITB_GENERATE_ARTICLE_ORBITAR = *FIGHTER_PITB_ARTICLE_TERM;
		}
	}

    Agent::new("pitb")
        .acmd("game_attack11", game_attack11,Priority::Default)
        .install();

	pitb_orbitar::install();
}