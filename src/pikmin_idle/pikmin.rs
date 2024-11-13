use crate::imports::imports_status::*;

pub unsafe extern "C" fn pik_deny_follow(weapon: &mut L2CWeaponCommon) -> bool {
    let prev_status = weapon.global_table[PREV_STATUS_KIND].get_i32();
    return [
        *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_LANDING,
        *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPIN_LANDING,
    ].contains(&prev_status);
}

pub unsafe extern "C" fn pik_ground_follow_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if pik_deny_follow(weapon) {
        WorkModule::set_int(weapon.module_accessor, 
            *WEAPON_PIKMIN_PIKMIN_OWNER_CONDITION_BORING_INDICATION, 
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_CURRENT
        ); /*
        WorkModule::set_int(weapon.module_accessor, 
            *WEAPON_PIKMIN_PIKMIN_OWNER_CONDITION_HIDE_INDICATION, 
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_FOLLOW
        ); */
        WorkModule::on_flag(weapon.module_accessor, 
            *WEAPON_PIKMIN_PIKMIN_STATUS_FOLLOW_COMMON_WORK_FLAG_IS_PERPLEXED
        );
        return 0.into();
    }
    return smashline::original_status(Init, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW)(weapon);
}
pub unsafe extern "C" fn pik_ground_follow_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if pik_deny_follow(weapon) {
        //StatusModule::set_status_kind_interrupt(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_BORING_WAIT);
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_BORING_WAIT.into(), true.into());
        return 0.into();
    }
    return smashline::original_status(Pre, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW)(weapon);
}
pub fn install() {   
    Agent::new("pikmin_pikmin")
        .status(Init, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW, pik_ground_follow_init)
        .status(Pre, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW, pik_ground_follow_pre)
    .install();
}