mod landmine;
mod snake;

pub fn install() {
    snake::install();
    landmine::install();
}

pub fn install_hook() {
    landmine::install_hook();
}
