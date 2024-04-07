mod acmd;
mod other_fighters;

use smashline;

pub fn install() {
    acmd::install();
    other_fighters::install();

    /*
    Last arg is if you want to use original code or nah. If set to false, you'll have to reimplement most of the status and acmd stuff (which might not be too hard with 
        smashline::original_status(Main, agent, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR)(agent);
    */
}