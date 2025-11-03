use smash::{
    app::{self, lua_bind::*, *},
    hash40,
    lib::{lua_const::*, L2CAgent, L2CValue},
    lua2cpp::*,
    phx::*,
};

/*
*/
pub mod vars;
pub mod imports;
mod ness;

pub fn install_hook() {
	unsafe {
        println!("Stored grapple");
		vars::GENERATE_ARTICLE_GRAPPLE += vars::GENERATE_ARTICLE_LAST + smashline::clone_weapon("dolly", *WEAPON_KIND_DOLLY_CAP, 
        "ness", "grapple",false);

        let mut param_ints: Vec<(u64,u64,i32)> = Vec::new();
        let mut param_floats: Vec<(u64,u64,f32)> = Vec::new();
        param_floats.push((hash40("air_lasso_data"), hash40("rewind_frame"), 10.0));
        param_ints.push((hash40("air_lasso_data"), hash40("frame"), 120));
        param_ints.push((hash40("air_lasso_data"), hash40("body_angle_intp"), 30));
        param_ints.push((hash40("air_lasso_data"), hash40("rewind_body_angle_intp"), 5));
        param_ints.push((hash40("air_lasso_data"), hash40("failure_wait_frame"), 5));
        param_ints.push((hash40("air_lasso_data"), hash40("shoot_end_frame"), 40));
        param_ints.push((hash40("air_lasso_data"), hash40("rewind_brake_x"), 0));
        param_ints.push((hash40("air_lasso_data"), hash40("hang_up_accel_rate"), 4));
        param_ints.push((hash40("air_lasso_data"), hash40("hang_down_accel_rate"), 2));
        
        for p in param_ints {
            param_config::update_int_2(*FIGHTER_KIND_NESS, vec![-1], p);
        }
        for p in param_floats {
            param_config::update_float_2(*FIGHTER_KIND_NESS, vec![-1], p);
        }
	}
}

pub fn install() {
    #[cfg(feature = "dev")]{
		unsafe {
			vars::GENERATE_ARTICLE_GRAPPLE = vars::GENERATE_ARTICLE_LAST;
		}
	}
    ness::install();
}
