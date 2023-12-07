/*
Marth Fireball

Gives Marth Mario's fireball. Uploaded this because a lot of people were confused on the process. If you want to give another character a fireball (don't do what I think you're going to do with ganondorf, it doesn't work), simply rename the marth stuff to that character. You'll also need a copy of their vl.prc, and mario's vl.prc "param_fireball" entry to the end of your character's.

Other fighter's file is for dealing with Isabelle/Villager's pocket. Normally this will cause a crash so instead this just despawns it before they pocket the item.
*/

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
    smashline::clone_weapon("mario", "fireball", "marth", "fireball",true);
}