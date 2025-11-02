use crate::imports::imports_acmd::*;
use crate::koopajr_mech::imports::*;

mod attack;
mod movement;

unsafe extern "C" fn game_specialhilanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_PILOT);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhilanding",game_specialhilanding,Priority::Default);
    agent.acmd("effect_specialhilanding",empty_acmd,Priority::Default);
    agent.acmd("sound_specialhilanding",empty_acmd,Priority::Default);

    movement::install(agent);
    attack::install(agent);
}