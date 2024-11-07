mod catch_special;

pub fn install(agent: &mut smashline::Agent) {
    catch_special::install(agent);
}
