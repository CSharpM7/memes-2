mod duckhunt;
mod vtable;

pub fn install() {
    #[cfg(feature = "devhook")]{
        vtable::install();
        return;
    }


    duckhunt::install();
}
