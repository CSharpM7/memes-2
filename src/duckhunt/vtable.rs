use crate::imports::imports_status::*;

unsafe fn cheater(module_accessor: &mut BattleObjectModuleAccessor) {
    let status = StatusModule::status_kind(module_accessor);
    if WorkModule::is_flag(module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCELED) {
        let has_clay = false;//ArticleModule::is_exist(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY); //4
        let has_can =  false;//ArticleModule::is_exist(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN); //5
        let has_final = WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) &&
        WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ONE_MORE_FINAL_DEAD_FAILED);
        if ((status != *FIGHTER_STATUS_KIND_SPECIAL_N) && !has_can) {
            if (!has_clay && !has_final) {
                WorkModule::unable_transition_term_forbid(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            }
        }
        if ((status == *FIGHTER_STATUS_KIND_SPECIAL_S || has_clay) == false) {
            WorkModule::unable_transition_term_forbid(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        }
    }
}

/*

#[skyline::hook(offset = 0x9a2090, inline)]
unsafe extern "C" fn duckhunt_per_frame(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[22].x.as_ref() as *mut BattleObjectModuleAccessor;
     */
#[skyline::hook(offset = 0x9a2090)]
pub unsafe extern "C" fn duckhunt_per_frame(vtable: u64, fighter: &mut Fighter) {
	if !app::sv_information::is_ready_go() {
		return
	}

    let module_accessor = (fighter.battle_object).module_accessor;
    //let original_fn = original!()(vtable, fighter);  //this makes it explode
    let status = StatusModule::status_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let is_uniq = true;//color == 0;
    if !is_uniq {return original!()(vtable, fighter);}

    //cheater(&mut *module_accessor);
    println!("HUH");
	
    let has_can = ArticleModule::is_exist(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN); //2
    let has_clay = ArticleModule::is_exist(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY); //1
    let has_reticle = ArticleModule::is_exist(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_RETICLE);
    if has_can {
        //plVar25 = *(long **)(*(long *)((ulong)FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN + 8) + 0x50);
        if WorkModule::is_flag(module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CAN) {
            //do something 0x20000006
            ArticleModule::set_flag(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN, true, *WEAPON_DUCKHUNT_CAN_INSTANCE_WORK_ID_FLAG_FLY);
        }
    }
    let shoot_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    if has_clay {
        if WorkModule::is_flag(module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CLAY) {
            ArticleModule::set_flag(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY, true, *WEAPON_DUCKHUNT_CAN_INSTANCE_WORK_ID_FLAG_FLY);
        }
        let offset_x = WorkModule::get_param_float(module_accessor, hash40("param_special_s"),hash40("shoot_pos_offset_x"));
        let offset_y = WorkModule::get_param_float(module_accessor, hash40("param_special_s"),hash40("shoot_pos_offset_y"));
        let offset = &mut Vector3f{x:offset_x*10.0,y:offset_y*10.0,z:0.0};
        ModelModule::joint_global_position_with_offset(module_accessor, Hash40::new("top"), shoot_pos, offset,false);
    }
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let too_slow = (*battle_object_slow.add(0x8) != 0 && *(battle_object_slow as *const u32) != 0);
    if too_slow {
        //return;
    }

    let some_int = WorkModule::get_int(module_accessor, 0x100000c4);
    if 0 < some_int {
        let mut some_bool = false;
        if StatusModule::status_kind(module_accessor) < 0x36 
        //&& ((1L << ((ulong)uVar10 & 0x3f) & 0x24800000000000U) != 0))
        {
            let some_int_2 = WorkModule::get_int(module_accessor, 0x11000011);
            if (some_int_2 - 1 < 2) {
                some_bool = WorkModule::is_flag(module_accessor, 0x21000021);
            }
            else {
                some_bool = false;
            }
        }
    }

    //
    //Countdown
    */
    let countdown = WorkModule::get_int(module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_SHOOT_CAN_FRAME);
    if -1 < countdown {
	    if !has_clay {
	    	WorkModule::count_down_int(module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_SHOOT_CAN_FRAME,-1);
	    }
	    else {
		if ArticleModule::is_flag(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY,0x20000011) {
			WorkModule::set_int(module_accessor, -1, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_SHOOT_CAN_FRAME);    
		}
	    }
    }
	if  (status - 0x1e0 < 0xb) && ((status - 0x1e0 & 0x1f) & 0x7f1U) != 0)) {
	}
}

pub fn install() {
    skyline::install_hooks!(
        //duckhunt_per_frame
    );
}
