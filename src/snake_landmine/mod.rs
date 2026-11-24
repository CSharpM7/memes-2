mod landmine;
mod ground;
mod snake;

pub fn install() {
    snake::install();
    landmine::install();
    ground::install();
}

pub fn install_hook() {
    landmine::install_hook();
    ground::install_hook();
}
