use crate::imports::imports_status::*;
use crate::koopajr_mech::imports::*;

unsafe extern "C" fn common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_PICOPICOHAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into();
}

pub unsafe extern "C" fn wait_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let original = fighter.status_Wait();
    let flag = WorkModule::is_flag(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_PILOT);
    if !is_pilot(fighter) {return original;}
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait_4"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wait_main_loop as *const () as _))
}

pub unsafe extern "C" fn wait_main_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    return fighter.status_Wait_Main();

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
        fighter.change_status(FIGHTER_STATUS_KIND_HAMMER_WAIT.into(),false.into());
        return 1.into();
    }
    let is_wait = fighter.sub_wait_common_Main().get_bool();
    if !is_wait {
        //fighter.sub_wait_motion(true.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAMON_EXHAUST) {
            fighter.change_status(FIGHTER_STATUS_KIND_METAMON_OUT.into(), false.into());
            return 1.into();
        }
    }

    0.into()
}

pub unsafe extern "C" fn walk_mot_helper(_fighter: &mut L2CFighterCommon, walk_mot: L2CValue, curr_mot: L2CValue) -> L2CValue {
    let curr = curr_mot.get_u64();
    let walk = walk_mot.get_u64();
    if curr == hash40("walk_fast") 
    && walk == hash40("walk_jr_fast") {
        return true.into();
    }
    if curr == hash40("walk_middle") 
    && walk == hash40("walk_jr_middle") {
        return true.into();
    }
    if curr == hash40("walk_slow") 
    && walk == hash40("walk_jr_slow") {
        return true.into();
    }
    false.into()
}
unsafe extern "C" fn walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_pilot(fighter) {
        return fighter.status_Walk();
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("walk_jr_start"), 0.0, 1.0, false, 0.0, false, false);
    return fighter.status_Walk();
    /*
    fighter.status_Walk_Common();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_walk_uniq_check();
    }
    return fighter.status_Walk(); 
    */
}

unsafe extern "C" fn walk_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_pilot(fighter) {
        return fighter.sub_walk_uniq_process_main();
    }
    /* 
    let walk_anim = hash40("walk_jr_fast");
    fighter.sub_walk_uniq_process_main_common(
        walk_anim.into(),
        walk_anim.into(),
        walk_anim.into(),
        false.into()
    );
    return 0.into();*/
    fighter.sub_walk_uniq_process_main_common(
        hash40("walk_jr_fast").into(),
        hash40("walk_jr_fast").into(),
        hash40("walk_jr_slow").into(),
        L2CValue::Ptr(walk_mot_helper as *const () as _)
    );
    0.into()
}

pub unsafe extern "C" fn turn_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let original = fighter.status_Turn();
    if !is_pilot(fighter) {return original;}

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("turn_jr"), 0.0, 1.0, false, 0.0, false, false);
    return original;
}

unsafe extern "C" fn attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = fighter.status_Attack();
    if !is_pilot(fighter) {
        return to_return;
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_jr_11"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_loop as *const () as _))
}
unsafe extern "C" fn attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
    }
	if CancelModule::is_enable_cancel(fighter.module_accessor) 
    {
		if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
		}
	} 
    if fighter.status_Attack_Main().get_bool() { 
        return 1.into();
    }
	0.into()
}

unsafe extern "C" fn attacks3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = fighter.status_AttackS3();
    if !is_pilot(fighter) {
        return to_return;
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_jr_s3_s"), 0.0, 1.0, false, 0.0, false, false);
    return to_return;
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_WAIT, wait_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_WALK, walk_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_WALK, walk_exec);
    agent.status(Main, *FIGHTER_STATUS_KIND_TURN, turn_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK, attack_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK, common_end);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S3, attacks3_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S3, common_end);
}