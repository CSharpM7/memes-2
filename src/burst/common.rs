use super::imports::*;
use crate::imports::imports_status::*;
use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if (fighter_kind == *FIGHTER_KIND_EDGE) {
        if MotionModule::frame(fighter.module_accessor) < 2.0 {
            /*
            let meter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_GAUGE);
            let dmg = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_ADD_DAMAGE);
            let add = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CHARGE_FINAL_ADD_GAUGE_COUNTER);
            
            println!("M: {meter} D {dmg} A {add}"); 
            */
            let can_final = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE);
            let charge_final = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE);
            println!("Can: {can_final} Charge {charge_final}"); 
        }
    }
}

pub fn install() {   
    Agent::new("fighter")
        .on_line(Main, update)
        //.status(Main, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_wait_main)
        
    .install();
}