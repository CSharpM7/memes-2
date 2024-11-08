mod acmd;
mod agent;
mod status;

use smashline;

pub fn install() {
    acmd::install();
    status::install();
    agent::install();
    //frame::install();
    //other_fighters::install();

    smashline::add_param_object("edge","attack_air_smash");
}