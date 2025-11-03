mod acmd;
mod status;
mod frame;

mod vine;

pub fn install() {   
    let agent = &mut smashline::Agent::new("pfushigisou");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    vine::install();
}