use crate::imports::imports_agent::*;

unsafe extern "C" fn print_line<T: ToString>(fighter: &mut L2CFighterCommon,string: T) {
    let status_frame = fighter.global_table[STATUS_FRAME].get_i32();
    if status_frame < 1 {
        println!("{}",string.to_string());
    }
}

unsafe extern "C" fn pikmin_frame(fighter: &mut L2CFighterCommon) {
    //fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].assign(&L2CValue::Ptr(check_air_escape_uniq as *const () as _));
    if ItemModule::is_attach_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BADGE)) {
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("virtualbadge"), &Vector3f::new(0.0, 10.0, 10.0), false, false);
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("virtualbadge"), &Vector3f::new(0.0, 40.0, 45.0), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_NONE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        ModelModule::set_joint_scale(fighter.module_accessor, smash::phx::Hash40::new("virtualbadge"), &Vector3f::new(30.0, 30.0, 30.0));
    }
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF) {

    }
}

pub fn install() {
    let agent = &mut Agent::new("pikmin");
    agent.on_line(Main, pikmin_frame);
    agent.install();
}