mod specialhi;
mod lasso;
pub fn install(agent: &mut smashline::Agent) {
    specialhi::install(agent);
    lasso::install(agent);
}