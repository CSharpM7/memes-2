use crate::imports::imports_agent::*;

pub fn install() {    
    let mut param_ints: Vec<(u64,u64,i32)> = Vec::new();
    param_ints.push((hash40("aerial_type"),0 as u64, *FIGHTER_JUMP_AERIAL_TYPE_FLY));
    param_ints.push((hash40("jump_count_max"),0 as u64, 3));
    for p in param_ints {
        param_config::update_int_2(*FIGHTER_KIND_PALUTENA, vec![-1], p);
    }
}
