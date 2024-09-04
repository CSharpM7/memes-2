mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("samusd");
    acmd::install(agent);
    agent.install();
}