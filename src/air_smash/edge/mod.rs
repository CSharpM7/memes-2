mod acmd;
mod agent;
mod status;
mod frame;
mod other_fighters;

use smashline;

pub fn install() {
    acmd::install();
    status::install();
    agent::install();
    //frame::install();
    //other_fighters::install();

    smashline::add_param_object("edge","attack_air_smash");
}