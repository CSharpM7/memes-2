use crate::imports::imports_agent::*;

pub unsafe extern "C" fn check_select(boma: *mut BattleObjectModuleAccessor, status: i32) {
}

pub unsafe extern "C" fn ramram_update(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    let status = StatusModule::status_kind(boma);
    let bound = [*WEAPON_TANTAN_PUNCH1_STATUS_KIND_BOUND,*WEAPON_TANTAN_PUNCH1_STATUS_KIND_DAMAGE].contains(&status);
    if AttackModule::is_infliction(boma,*COLLISION_KIND_MASK_REFLECTOR){
        WorkModule::on_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_HIT_SHIELD);
    }
}

pub fn install() {
    Agent::new("tantan_punch1")
        .on_line(Main, ramram_update)
        .install();
}