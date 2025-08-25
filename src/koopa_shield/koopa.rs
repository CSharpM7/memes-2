use crate::imports::imports_status::*;
use crate::imports::imports_acmd::*;

const SHIELD_EFFS: [&str ; 5] = [
    "sys_shield",
    "sys_shield_damage1",
    "sys_shield_damage2",
    "sys_shield_damage3",
    "sys_shield_smoke",
];
unsafe extern "C" fn kill_effects(fighter: &mut L2CFighterCommon) {
    for shield_eff in SHIELD_EFFS.iter() {
        //continue;
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new(&shield_eff), true, true);
        sv_module_access::effect(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
}
unsafe extern "C" fn fun_710000a4b0(fighter: &mut L2CFighterCommon){
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let shield_radius = 0.6*WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: shield_radius, y: shield_radius, z: shield_radius});
    //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2dc1210b69));

    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    let mut model_color = (shield_hp / shield_max).clamp(0.05, 1.0);
    if ([*FIGHTER_STATUS_KIND_ESCAPE_F,*FIGHTER_STATUS_KIND_ESCAPE_B].contains(&status_kind)) {
        let max_frame = 15.0;
        let frame = MotionModule::frame(fighter.module_accessor);
        let frame_factor = (frame / max_frame).min(1.0);
        //println!("Factor: {frame_factor} Color: {model_color}");
        model_color = model_color.max(frame_factor);
    }
    ModelModule::set_color_rgb(fighter.module_accessor, model_color, model_color, model_color, MODEL_COLOR_TYPE{_address: 0});
    kill_effects(fighter);
}
unsafe extern "C" fn common_exit(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if !([*FIGHTER_STATUS_KIND_GUARD_ON,*FIGHTER_STATUS_KIND_GUARD,*FIGHTER_STATUS_KIND_GUARD_DAMAGE,
    *FIGHTER_STATUS_KIND_ESCAPE_F,*FIGHTER_STATUS_KIND_ESCAPE_B
    ].contains(&status_kind)) {
        //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2dd241385f));
        ModelModule::set_color_rgb(fighter.module_accessor, 1.0, 1.0, 1.0, MODEL_COLOR_TYPE{_address: 0});
    }
    kill_effects(fighter);
}

unsafe extern "C" fn guardon_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardOn_initStatus_common();
    fun_710000a4b0(fighter);
    //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d9c13b541));
    0.into()
}
unsafe extern "C" fn guardon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_on_common();
    fighter.main_shift(guardon_main_loop)
}
unsafe extern "C" fn guardon_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_status_guard_on_main_air_common().get_bool() {
        return 0.into();
    }
    if !fighter.sub_status_guard_on_main_air_common().get_bool()
    && !fighter.sub_guard_cont().get_bool()
    && !fighter.status_guard_main_common().get_bool() {
        if MotionModule::is_end(fighter.module_accessor)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
        }
    }

    0.into()
}

unsafe extern "C" fn guardon_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardOn_execStatus_common();
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT);
    fun_710000a4b0(fighter);
    //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d9c13b541));
    0.into()
}
unsafe extern "C" fn guardon_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_710000a4b0(fighter);
    fighter.sub_ftStatusUniqProcessGuardOn_execStop_Inner(LUA_SCRIPT_LINE_SYSTEM_POST.into());
    0.into()
}
unsafe extern "C" fn guardon_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardOn_exitStatus_common();
    common_exit(fighter);
    0.into()
}

unsafe extern "C" fn guard_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_initStatus_common();
    0.into()
}
unsafe extern "C" fn guard_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStatus_common();
    fun_710000a4b0(fighter);
    0.into()
}
unsafe extern "C" fn guard_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStop_common();
    fun_710000a4b0(fighter);
    0.into()
}
unsafe extern "C" fn guard_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_exitStatus_common();
    common_exit(fighter);
    0.into()
}

unsafe extern "C" fn guarddamage_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus_Inner();
    0.into()
}
unsafe extern "C" fn guarddamage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter.status_GuardDamage_common(true.into());
    }
    else {
        fighter.status_GuardDamage_common(false.into());
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard"), 0.0, 0.0, false, 0.0, false, false);
    kill_effects(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_GuardDamage_Main as *const () as _))
}
unsafe extern "C" fn guarddamage_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_execStatus_common();
    fun_710000a4b0(fighter);
    0.into()
}
unsafe extern "C" fn guarddamage_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_execStop_common();
    fun_710000a4b0(fighter);
    0.into()
}
unsafe extern "C" fn guarddamage_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_exitStatus_common();
    common_exit(fighter);
    0.into()
}
unsafe extern "C" fn escape_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_710000a4b0(fighter);
    0.into()
}
unsafe extern "C" fn escape_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    common_exit(fighter);
    0.into()
}

unsafe extern "C" fn game_guardon(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("snout"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("tail2"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("snout"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("tail2"), *HIT_STATUS_NORMAL);
    }
}
unsafe extern "C" fn game_guard(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, *ATTACH_ITEM_GROUP_ALL as u8);
    }
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("snout"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("tail2"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
}

pub fn install() {   
    Agent::new("koopa")
        .acmd("game_guardon", game_guardon,Priority::Default)
        .acmd("game_guard", game_guard,Priority::Default)

        .status(Init, *FIGHTER_STATUS_KIND_GUARD_ON, guardon_init)
        .status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, guardon_main)
        .status(Exec, *FIGHTER_STATUS_KIND_GUARD_ON, guardon_exec)
        .status(ExecStop, *FIGHTER_STATUS_KIND_GUARD_ON, guardon_execstop)
        .status(Exit, *FIGHTER_STATUS_KIND_GUARD_ON, guardon_exit)

        .status(Init, *FIGHTER_STATUS_KIND_GUARD, guard_init)
        .status(Exec, *FIGHTER_STATUS_KIND_GUARD, guard_exec)
        .status(ExecStop, *FIGHTER_STATUS_KIND_GUARD, guard_execstop)
        .status(Exit, *FIGHTER_STATUS_KIND_GUARD, guard_exit)

        .status(Init, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_init)
        .status(Main, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_main)
        .status(Exec, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_exec)
        .status(ExecStop, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_execstop)
        .status(Exit, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_exit)

        .status(Exec, *FIGHTER_STATUS_KIND_ESCAPE_F, escape_exec)
        .status(Exec, *FIGHTER_STATUS_KIND_ESCAPE_B, escape_exec)
        .status(Exit, *FIGHTER_STATUS_KIND_ESCAPE_F, escape_exit)
        .status(Exit, *FIGHTER_STATUS_KIND_ESCAPE_B, escape_exit)
    .install();
}