
#[no_mangle]
pub fn smashline_install() {
    install();
}

pub fn install() {
    crate::ike_air::install();
}