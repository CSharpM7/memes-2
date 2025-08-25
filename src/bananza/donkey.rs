use crate::imports::imports_acmd::*;

// HDR
unsafe fn try_pickup_item(module_accessor: &mut BattleObjectModuleAccessor, range: f32, bone: Option<Hash40>, offset: Option<&Vector2f>) {// -> Option<&mut BattleObjectModuleAccessor> {
    use smash_rs::app::ItemManager;

    // item manager singleton instance
    let item_manager = ItemManager::instance().unwrap();

    // if you already have an item, return that item instead
    if ItemModule::is_have_item(module_accessor, 0) {
        let have_id = ItemModule::get_have_item_id(module_accessor, 0);
        let item = item_manager.find_active_item_from_id(have_id as u32) as *mut smash::app::Item;
        let item_module_accessor = smash::app::lua_bind::Item::item_module_accessor(item) as *mut ItemModuleAccessor;
        let item_boma = &mut (*item_module_accessor).battle_object_module_accessor;
        return; //Some(item_boma);
    }
    
    // get the global position of the bone, defaulting to "top"
    let fighter_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let bone_hash = bone.unwrap_or(Hash40::new("top"));
    ModelModule::joint_global_position(module_accessor, bone_hash, fighter_pos, false);
    // zero out the z axis
    fighter_pos.z = 0.0;
    match offset {
        Some(offset) => {
            fighter_pos.x += offset.x * PostureModule::lr(module_accessor);
            fighter_pos.y += offset.y;
        },
        None => {}
    }
    
    let total = item_manager.get_num_of_active_item_all();
    for id in 0..total {
        // pointer to the item
        let item_ptr = item_manager.get_active_item(id as u64);
        if item_ptr.is_null() {
            continue;
        }

        // if this item is close to us, grab it
        let item = item_ptr as *mut smash::app::Item;
        let item_module_accessor = smash::app::lua_bind::Item::item_module_accessor(item) as *mut ItemModuleAccessor;
        let item_boma = &mut (*item_module_accessor).battle_object_module_accessor;
        let item_pos = PostureModule::pos(item_boma);

        if ((*item_pos).x - (*fighter_pos).x).abs() < range
            && ((*item_pos).y - (*fighter_pos).y).abs() < range {
            ItemModule::have_item_instance(module_accessor, item, 0, false, false, false, false);
            return; //Some(item_boma);
        }
    }
    return;// None;
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_RIPSTICK), 0, 0, false, false);
        ItemModule::pickup_item(agent.module_accessor, ItemSize{_address: *ITEM_SIZE_HEAVY as u8},
            *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL,
            QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8},
            ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
    }
}
unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AreaModule::enable_area(agent.module_accessor, *FIGHTER_AREA_GROUP_ITEM_PICKUP, true, -1);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        println!("Please pick it up??");
        /*
        if ItemModule::get_pickable_item_size(agent.module_accessor) == *ITEM_SIZE_KIND_LARGE as u64 {
            ItemModule::pickup_item(agent.module_accessor, ItemSize{ _address: *ITEM_SIZE_HEAVY as u8 }, 0, *ITEM_TRAIT_FLAG_EQUIP, 
            QuickItemTreatType{ _address: 0 as u8 }, ItemPickupSearchMode{ _address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8 });
        } 
        */
        try_pickup_item(&mut *agent.module_accessor,16.0, Some(Hash40::new("top")), Some(&Vector2f{x: 10.0, y: 0.0}));
        if ItemModule::is_have_item(agent.module_accessor, 0) {
            let sound = SoundModule::play_se(agent.module_accessor, Hash40::new("se_item_item_get"), true, false, false, false, app::enSEType(0));
        }
    }
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw_start").into(), Hash40::new("special_air_lw").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    if fighter.is_grounded() {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = MotionModule::motion_kind(fighter.module_accessor); 
    let is_air_start = motion == hash40("special_air_lw");
    let is_air_end = motion == hash40("special_lw_end");
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        if fighter.is_grounded() {
            let pos = *PostureModule::pos(fighter.module_accessor);
            EffectModule::req(
                fighter.module_accessor,
                Hash40::new("sys_merikomi"),
                &Vector3f{x:pos.x+8.0,y:pos.y+0.0,z:pos.z},
                &Vector3f{x:0.0,y:0.0,z:0.0},
                1.1,
                0,
                -1,
                false,
                0
            );            
            let sound = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_pitin_move_02"), true, false, false, false, app::enSEType(0));
            fighter.change_status_by_situation(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_FALL.into(), false.into());
            return 1.into();
        }
        else if is_air_start {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_end"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), false.into());
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() 
        || fighter.sub_transition_group_check_air_landing().get_bool() //?
        {
            return 1.into();
        }
    }
    
    let is_grounded = fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND;
    
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if is_grounded {
            if is_air_start && is_grounded {
                WorkModule::set_float(fighter.module_accessor, 16.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            else if is_air_end {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 1.into();
            }
        }
        else if !is_air_start && !is_air_end {
            fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_LANDING.into(), false.into());
            return 1.into();
        }
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_start").into(), Hash40::new("special_air_lw").into(), true.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status_by_situation(FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WAIT.into(), FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_FALL.into(), false.into());
        }
        else {
            fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}


unsafe extern "C" fn heavy_get_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let to_return = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP)(fighter);
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            println!("Ground");
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, frame, true, true, false);
        }
        else {
            println!("Air");
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_end"), 0.0, 1.0, false, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            //KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::clear_speed_all(fighter.module_accessor);
            JostleModule::set_status(fighter.module_accessor, false);
            fighter.sub_set_ground_correct_by_situation(true.into());
            fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(heavy_get_substatus3 as *const () as _));
        }
    }
    return to_return;
}
unsafe extern "C" fn heavy_get_substatus3(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        fighter.sub_change_motion_by_situation(Hash40::new("item_heavy_get").into(), Hash40::new("special_lw_end").into(), true.into());
    }
	0.into()
}

pub fn install() {   
    Agent::new("donkey")
        .acmd("game_speciallwstart", game_speciallwstart,Priority::Default)
        
        .acmd("game_specialairlw", game_specialairlw,Priority::Default)
        .acmd("effect_specialairlw", empty_acmd,Priority::Default)
        .acmd("sound_specialairlw", empty_acmd,Priority::Default)
        .acmd("game_specialairlw", game_specialairlw,Priority::Default)
        
        .acmd("expression_speciallwend", empty_acmd,Priority::Default)

        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main)
        .status(Main, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, heavy_get_main)
    .install();
}