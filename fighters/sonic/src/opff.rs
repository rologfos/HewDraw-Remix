use ::common::opff_import::*;
use super::*;
use globals::*;
use ::common::opff::*;
 

// Sonic Spindash Jump Waveland
unsafe fn sonic_spindash_jump_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP {
        if boma.is_cat_flag(Cat1::JumpButton) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
            if situation_kind == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    sonic_spindash_jump_waveland(boma, status_kind, situation_kind, cat[0]);
    //sonic_moveset(boma, situation_kind, status_kind, motion_kind, frame, cat[0], id);
    //sonic_lightspeed_dash(boma, status_kind, motion_kind, situation_kind, cat[0], id);
}

#[utils::opff(FIGHTER_KIND_SONIC )]
pub fn sonic_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		sonic_frame(fighter)
    }
}

pub unsafe fn sonic_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}