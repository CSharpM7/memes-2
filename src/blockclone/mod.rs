mod block;
mod fighter;

pub fn install() {
    fighter::install();
    block::install();
}

pub fn install_hook() {
    block::install_hook();
}
