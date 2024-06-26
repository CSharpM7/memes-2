/*
Marth Fireball

Gives Marth Mario's fireball. Uploaded this because a lot of people were confused on the process. If you want to give another character a fireball (don't do what I think you're going to do with ganondorf, it doesn't work), simply rename the Marth stuff to that character. You'll also need a copy of their vl.prc, and mario's vl.prc "param_fireball" entry to the end of your character's. Also note that Kirby Copy Abilities don't receive the extra article

Other fighter's file is for dealing with Isabelle/Villager's pocket. Normally this will cause a crash so instead this just despawns it before they pocket the item.
*/


pub mod vars;
pub mod util;
pub mod imports;

mod marth;
mod kirby;

pub fn install() {
    marth::install();
    kirby::install();
}
