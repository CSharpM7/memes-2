pub mod status;
mod agent;
pub fn install() {
    status::install();
    agent::install();
}