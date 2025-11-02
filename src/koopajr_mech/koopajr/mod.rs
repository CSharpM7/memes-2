mod acmd;
mod agent_init;
mod status;
mod frame;

pub fn install() {   
    let agent = &mut smashline::Agent::new("koopajr");
    acmd::install(agent);
    agent_init::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();
}