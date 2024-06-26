use crate::imports::imports_agent::*;

//Check to make sure a given cannonball is owned by edge, and assume that it is Cappy
pub unsafe extern "C" fn is_cloned_article(object_boma: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *object_boma) == *WEAPON_KIND_MARIO_FIREBALL {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_KIRBY {
            return true;
        }
    }
    return false;
}