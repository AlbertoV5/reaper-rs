//! Provides all functions from `reaper_plugin_functions.h` with the following improvements:
//! - Snake-case function and parameter names
//! - Return values instead of output parameters
//! - No C strings
//! - Panics if function not available (we should make sure on plug-in load that all necessary
//!   functions are available, maybe provide "_available" functions for conditional execution)
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::{null, null_mut};

use c_str_macro::c_str;

use crate::low_level;
use crate::low_level::{audio_hook_register_t, GUID, HWND, IReaperControlSurface, KbdSectionInfo, MediaTrack, midi_Input, midi_Output, ReaProject, TrackEnvelope};
pub use crate::medium_level::control_surface::ControlSurface;
use crate::medium_level::control_surface::DelegatingControlSurface;

mod control_surface;

pub struct Reaper {
    pub low: low_level::Reaper
}

const ZERO_GUID: GUID = GUID {
    Data1: 0,
    Data2: 0,
    Data3: 0,
    Data4: [0; 8],
};

fn with_string_buffer<T>(max_size: u32, fill_buffer: impl FnOnce(*mut c_char, i32) -> T) -> (CString, T) {
    let vec: Vec<u8> = vec![1; max_size as usize];
    let c_string = unsafe { CString::from_vec_unchecked(vec) };
    let raw = c_string.into_raw();
    let result = fill_buffer(raw, max_size as i32);
    let string = unsafe { CString::from_raw(raw) };
    (string, result)
}

impl Reaper {
    pub fn new(low: low_level::Reaper) -> Reaper {
        Reaper { low }
    }

    // TODO Unifiy u32 vs i32
    pub fn enum_projects(&self, idx: i32, projfn_out_optional_sz: u32) -> (*mut ReaProject, Option<CString>) {
        if projfn_out_optional_sz == 0 {
            let project = self.low.EnumProjects.unwrap()(idx, null_mut(), 0);
            (project, None)
        } else {
            let (file_path, project) = with_string_buffer(projfn_out_optional_sz, |buffer, max_size| {
                self.low.EnumProjects.unwrap()(idx, buffer, max_size)
            });

            (project, if file_path.as_bytes().len() == 0 { None } else { Some(file_path) })
        }
    }

    pub fn get_track(&self,
                     proj: *mut ReaProject,
                     trackidx: i32,
    ) -> *mut MediaTrack {
        self.low.GetTrack.unwrap()(proj, trackidx)
    }

    pub fn validate_ptr_2(&self, proj: *mut ReaProject, pointer: *mut c_void, ctypename: &CStr) -> bool {
        self.low.ValidatePtr2.unwrap()(proj, pointer, ctypename.as_ptr())
    }

    pub fn get_set_media_track_info(&self, tr: *mut MediaTrack, parmname: &CStr, set_new_value: *mut c_void) -> *mut c_void {
        self.low.GetSetMediaTrackInfo.unwrap()(tr, parmname.as_ptr(), set_new_value)
    }

    pub fn show_console_msg(&self, msg: &CStr) {
        self.low.ShowConsoleMsg.unwrap()(msg.as_ptr())
    }

    pub fn plugin_register(&self, name: &CStr, infostruct: *mut c_void) -> i32 {
        self.low.plugin_register.unwrap()(name.as_ptr(), infostruct)
    }

    pub fn main_on_command_ex(&self, command: i32, flag: i32, proj: *mut ReaProject) {
        self.low.Main_OnCommandEx.unwrap()(command, flag, proj);
    }

    pub fn csurf_set_surface_mute(&self, trackid: *mut MediaTrack, mute: bool, ignoresurf: *mut IReaperControlSurface) {
        self.low.CSurf_SetSurfaceMute.unwrap()(trackid, mute, ignoresurf);
    }

    pub fn csurf_set_surface_solo(&self, trackid: *mut MediaTrack, solo: bool, ignoresurf: *mut IReaperControlSurface) {
        self.low.CSurf_SetSurfaceSolo.unwrap()(trackid, solo, ignoresurf);
    }

    pub fn gen_guid(&self) -> GUID {
        let mut guid = ZERO_GUID;
        self.low.genGuid.unwrap()(&mut guid as *mut GUID);
        guid
    }

    // Once installed, it stays installed until this module unloaded
    pub fn install_control_surface(&self, control_surface: impl ControlSurface + 'static) {
        let delegating_control_surface = DelegatingControlSurface::new(control_surface);
        self.low.install_control_surface(delegating_control_surface);
    }

    // Must be idempotent
    // Please take care of unregistering once you are done!
    pub fn register_control_surface(&self) {
        self.plugin_register(c_str!("csurf_inst"), self.low.get_cpp_control_surface());
    }

    // Must be idempotent
    pub fn unregister_control_surface(&self) {
        self.plugin_register(c_str!("-csurf_inst"), self.low.get_cpp_control_surface());
    }

    pub fn section_from_unique_id(&self, unique_id: i32) -> *mut KbdSectionInfo {
        self.low.SectionFromUniqueID.unwrap()(unique_id)
    }

    pub fn kbd_on_main_action_ex(&self, cmd: i32, val: i32, valhw: i32, relmode: i32, hwnd: HWND, proj: *mut ReaProject) -> i32 {
        self.low.KBD_OnMainActionEx.unwrap()(cmd, val, valhw, relmode, hwnd, proj)
    }

    pub fn get_main_hwnd(&self) -> HWND {
        self.low.GetMainHwnd.unwrap()()
    }

    pub fn named_command_lookup(&self, command_name: &CStr) -> i32 {
        self.low.NamedCommandLookup.unwrap()(command_name.as_ptr())
    }

    pub fn clear_console(&self) {
        self.low.ClearConsole.unwrap()();
    }

    pub fn count_tracks(&self, proj: *mut ReaProject) -> i32 {
        self.low.CountTracks.unwrap()(proj)
    }

    pub fn insert_track_at_index(&self, idx: i32, want_defaults: bool) {
        self.low.InsertTrackAtIndex.unwrap()(idx, want_defaults);
    }

    pub fn get_midi_input(&self, idx: u32) -> *mut midi_Input {
        self.low.GetMidiInput.unwrap()(idx as i32)
    }

    pub fn get_midi_output(&self, idx: u32) -> *mut midi_Output {
        self.low.GetMidiOutput.unwrap()(idx as i32)
    }

    pub fn get_max_midi_inputs(&self) -> u32 {
        self.low.GetMaxMidiInputs.unwrap()() as u32
    }

    pub fn get_max_midi_outputs(&self) -> u32 {
        self.low.GetMaxMidiOutputs.unwrap()() as u32
    }

    // TODO When not present, does it still return a name? Adjust signature accordingly!
    pub fn get_midi_input_name(&self, dev: u32, nameout_sz: u32) -> (bool, Option<CString>) {
        if nameout_sz == 0 {
            let is_present = self.low.GetMIDIInputName.unwrap()(dev as i32, null_mut(), 0);
            (is_present, None)
        } else {
            let (name, is_present) = with_string_buffer(nameout_sz, |buffer, max_size| {
                self.low.GetMIDIInputName.unwrap()(dev as i32, buffer, max_size)
            });
            (is_present, if name.as_bytes().len() == 0 { None } else { Some(name) })
        }
    }

    pub fn track_fx_add_by_name(&self, track: *mut MediaTrack, fxname: &CStr, rec_fx: bool, instantiate: i32) -> i32 {
        self.low.TrackFX_AddByName.unwrap()(track, fxname.as_ptr(), rec_fx, instantiate)
    }

    pub fn get_midi_output_name(&self, dev: u32, nameout_sz: u32) -> (bool, Option<CString>) {
        if nameout_sz == 0 {
            let is_present = self.low.GetMIDIOutputName.unwrap()(dev as i32, null_mut(), 0);
            (is_present, None)
        } else {
            let (name, is_present) = with_string_buffer(nameout_sz, |buffer, max_size| {
                self.low.GetMIDIOutputName.unwrap()(dev as i32, buffer, max_size)
            });
            (is_present, if name.as_bytes().len() == 0 { None } else { Some(name) })
        }
    }

    pub fn track_fx_get_enabled(&self, track: *mut MediaTrack, fx: i32) -> bool {
        self.low.TrackFX_GetEnabled.unwrap()(track, fx)
    }

    pub fn track_fx_get_fx_name(&self, track: *mut MediaTrack, fx: i32, buf_sz: u32) -> Option<CString> {
        assert!(buf_sz > 0);
        let (name, successful) = with_string_buffer(buf_sz, |buffer, max_size| {
            self.low.TrackFX_GetFXName.unwrap()(track, fx, buffer, max_size)
        });
        if !successful || name.as_bytes().len() == 0 {
            return None;
        }
        Some(name)
    }

    pub fn track_fx_get_instrument(&self, track: *mut MediaTrack) -> i32 {
        self.low.TrackFX_GetInstrument.unwrap()(track)
    }

    pub fn track_fx_set_enabled(&self, track: *mut MediaTrack, fx: i32, enabled: bool) {
        self.low.TrackFX_SetEnabled.unwrap()(track, fx, enabled);
    }

    pub fn track_fx_get_num_params(&self, track: *mut MediaTrack, fx: i32) -> i32 {
        self.low.TrackFX_GetNumParams.unwrap()(track, fx)
    }

    pub fn get_current_project_in_load_save(&self) -> *mut ReaProject {
        self.low.GetCurrentProjectInLoadSave.unwrap()()
    }

    pub fn track_fx_get_param_name(&self, track: *mut MediaTrack, fx: i32, param: i32,
                                   buf_sz: u32) -> Option<CString> {
        assert!(buf_sz > 0);
        let (name, successful) = with_string_buffer(buf_sz, |buffer, max_size| {
            self.low.TrackFX_GetParamName.unwrap()(track, fx, param, buffer, max_size)
        });
        if !successful || name.as_bytes().len() == 0 {
            return None;
        }
        Some(name)
    }

    pub fn track_fx_get_formatted_param_value(&self, track: *mut MediaTrack, fx: i32, param: i32,
                                              buf_sz: u32) -> Option<CString> {
        assert!(buf_sz > 0);
        let (name, successful) = with_string_buffer(buf_sz, |buffer, max_size| {
            self.low.TrackFX_GetFormattedParamValue.unwrap()(track, fx, param, buffer, max_size)
        });
        if !successful || name.as_bytes().len() == 0 {
            return None;
        }
        Some(name)
    }

    pub fn track_fx_format_param_value_normalized(&self, track: *mut MediaTrack, fx: i32, param: i32,
                                                  value: f64, buf_sz: u32) -> Option<CString> {
        assert!(buf_sz > 0);
        let (name, successful) = with_string_buffer(buf_sz, |buffer, max_size| {
            self.low.TrackFX_FormatParamValueNormalized.unwrap()(track, fx, param, value, buffer, max_size)
        });
        if !successful || name.as_bytes().len() == 0 {
            return None;
        }
        Some(name)
    }

    pub fn track_fx_set_param_normalized(&self, track: *mut MediaTrack, fx: i32, param: i32,
                                         value: f64) -> bool {
        self.low.TrackFX_SetParamNormalized.unwrap()(track, fx, param, value)
    }

    pub fn get_last_touched_fx(&self) -> Option<GetLastTouchedFxResult> {
        let mut tracknumber = -1;
        let mut fxnumber = -1;
        let mut paramnumber = -1;
        let is_valid = self.low.GetLastTouchedFX.unwrap()(
            &mut tracknumber as *mut i32,
            &mut fxnumber as *mut i32,
            &mut paramnumber as *mut i32,
        );
        if !is_valid {
            return None;
        }
        Some(GetLastTouchedFxResult {
            tracknumber,
            fxnumber,
            paramnumber,
        })
    }

    pub fn track_fx_get_parameter_step_sizes(&self, track: *mut MediaTrack, fx: i32,
                                             param: i32) -> Option<GetParameterStepSizesResult> {
        let mut step = -1.0;
        let mut small_step = -1.0;
        let mut large_step = -1.0;
        let mut is_toggle = false;
        let successful = self.low.TrackFX_GetParameterStepSizes.unwrap()(
            track,
            fx,
            param,
            &mut step as *mut f64,
            &mut small_step as *mut f64,
            &mut large_step as *mut f64,
            &mut is_toggle as *mut bool,
        );
        if !successful {
            return None;
        }
        GetParameterStepSizesResult {
            step: complain_if_minus_one(step),
            small_step: complain_if_minus_one(small_step),
            large_step: complain_if_minus_one(large_step),
            is_toggle,
        }.into()
    }

    pub fn track_fx_get_param_ex(&self, track: *mut MediaTrack, fx: i32, param: i32) -> GetParamExResult {
        let mut min_val = -1.0;
        let mut max_val = -1.0;
        let mut mid_val = -1.0;
        let value = self.low.TrackFX_GetParamEx.unwrap()(
            track,
            fx,
            param,
            &mut min_val as *mut f64,
            &mut max_val as *mut f64,
            &mut mid_val as *mut f64,
        );
        GetParamExResult {
            value: complain_if_minus_one(value),
            min_val: complain_if_minus_one(min_val),
            mid_val: complain_if_minus_one(mid_val),
            max_val: complain_if_minus_one(max_val),
        }.into()
    }


    pub fn undo_begin_block_2(&self, proj: *mut ReaProject) {
        self.low.Undo_BeginBlock2.unwrap()(proj);
    }

    pub fn undo_end_block_2(&self, proj: *mut ReaProject, descchange: &CStr, extraflags: i32) {
        self.low.Undo_EndBlock2.unwrap()(proj, descchange.as_ptr(), extraflags);
    }

    pub fn undo_can_undo_2(&self, proj: *mut ReaProject) -> Option<&CStr> {
        let ptr = self.low.Undo_CanUndo2.unwrap()(proj);
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr) })
    }

    pub fn undo_can_redo_2(&self, proj: *mut ReaProject) -> Option<&CStr> {
        let ptr = self.low.Undo_CanRedo2.unwrap()(proj);
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr) })
    }

    pub fn undo_do_undo_2(&self, proj: *mut ReaProject) -> i32 {
        self.low.Undo_DoUndo2.unwrap()(proj)
    }

    pub fn undo_do_redo_2(&self, proj: *mut ReaProject) -> i32 {
        self.low.Undo_DoRedo2.unwrap()(proj)
    }

    pub fn mark_project_dirty(&self, proj: *mut ReaProject) {
        self.low.MarkProjectDirty.unwrap()(proj);
    }

    pub fn is_project_dirty(&self, proj: *mut ReaProject) -> i32 {
        self.low.IsProjectDirty.unwrap()(proj)
    }

    pub fn track_list_update_all_external_surfaces(&self) {
        self.low.TrackList_UpdateAllExternalSurfaces.unwrap()();
    }

    pub fn get_app_version(&self) -> &'static CStr {
        let ptr = self.low.GetAppVersion.unwrap()();
        unsafe { CStr::from_ptr(ptr) }
    }

    pub fn get_track_automation_mode(&self, tr: *mut MediaTrack) -> i32 {
        // TODO Use macro instead of unwrap everywhere in order to panic with good message on non-loaded function
        self.low.GetTrackAutomationMode.unwrap()(tr)
    }

    pub fn get_global_automation_override(&self) -> i32 {
        self.low.GetGlobalAutomationOverride.unwrap()()
    }

    pub fn get_track_envelope_by_name(&self, track: *mut MediaTrack, envname: &CStr) -> *mut TrackEnvelope {
        self.low.GetTrackEnvelopeByName.unwrap()(track, envname.as_ptr())
    }

    pub fn get_media_track_info_value(&self, tr: *mut MediaTrack, parmname: &CStr) -> f64 {
        self.low.GetMediaTrackInfo_Value.unwrap()(tr, parmname.as_ptr())
    }

    pub fn track_fx_get_count(&self, track: *mut MediaTrack) -> i32 {
        self.low.TrackFX_GetCount.unwrap()(track)
    }

    pub fn track_fx_get_rec_count(&self, track: *mut MediaTrack) -> i32 {
        self.low.TrackFX_GetRecCount.unwrap()(track)
    }

    pub fn track_fx_get_fx_guid(&self, track: *mut MediaTrack, fx: i32) -> *mut GUID {
        self.low.TrackFX_GetFXGUID.unwrap()(track, fx)
    }

    pub fn track_fx_get_param_normalized(&self, track: *mut MediaTrack, fx: i32, param: i32) -> f64 {
        self.low.TrackFX_GetParamNormalized.unwrap()(track, fx, param)
    }

    pub fn get_master_track(&self, proj: *mut ReaProject) -> *mut MediaTrack {
        self.low.GetMasterTrack.unwrap()(proj)
    }

    pub fn guid_to_string(&self, g: &GUID) -> CString {
        let (guid_string, _) = with_string_buffer(64, |buffer, _| {
            self.low.guidToString.unwrap()(g as *const GUID, buffer)
        });
        guid_string
    }

    pub fn master_get_tempo(&self) -> f64 {
        self.low.Master_GetTempo.unwrap()()
    }

    pub fn set_current_bpm(&self, __proj: *mut ReaProject, bpm: f64, want_undo: bool) {
        self.low.SetCurrentBPM.unwrap()(__proj, bpm, want_undo);
    }

    pub fn master_get_play_rate(&self, project: *mut ReaProject) -> f64 {
        self.low.Master_GetPlayRate.unwrap()(project)
    }

    pub fn csurf_on_play_rate_change(&self, playrate: f64) {
        self.low.CSurf_OnPlayRateChange.unwrap()(playrate);
    }

    pub fn show_message_box(&self, msg: &CStr, title: &CStr, type_: i32) -> i32 {
        self.low.ShowMessageBox.unwrap()(msg.as_ptr(), title.as_ptr(), type_)
    }

    pub fn string_to_guid(&self, str: &CStr) -> Option<GUID> {
        let mut guid = ZERO_GUID;
        self.low.stringToGuid.unwrap()(str.as_ptr(), &mut guid as *mut GUID);
        if guid == ZERO_GUID {
            return None;
        }
        Some(guid)
    }

    pub fn csurf_on_input_monitoring_change_ex(&self, trackid: *mut MediaTrack, monitor: i32, allowgang: bool) -> i32 {
        self.low.CSurf_OnInputMonitorChangeEx.unwrap()(trackid, monitor, allowgang)
    }

    pub fn set_media_track_info_value(&self, tr: *mut MediaTrack, parmname: &CStr, newvalue: f64) -> bool {
        self.low.SetMediaTrackInfo_Value.unwrap()(tr, parmname.as_ptr(), newvalue)
    }

    pub fn stuff_midimessage(&self, mode: i32, msg1: i32, msg2: i32, msg3: i32) {
        self.low.StuffMIDIMessage.unwrap()(mode, msg1, msg2, msg3);
    }

    pub fn db2slider(&self, x: f64) -> f64 {
        self.low.DB2SLIDER.unwrap()(x)
    }

    pub fn slider2db(&self, y: f64) -> f64 {
        self.low.SLIDER2DB.unwrap()(y)
    }

    pub fn get_track_ui_vol_pan(&self, track: *mut MediaTrack) -> Option<(f64, f64)> {
        let mut volume = 0.0;
        let mut pan = 0.0;
        let successful = self.low.GetTrackUIVolPan.unwrap()(track, &mut volume as *mut f64, &mut pan as *mut f64);
        if !successful {
            return None;
        }
        Some((volume, pan))
    }

    pub fn audio_reg_hardware_hook(&self, is_add: bool, reg: *const audio_hook_register_t) -> i32 {
        self.low.Audio_RegHardwareHook.unwrap()(is_add, reg)
    }

    pub fn csurf_set_surface_volume(&self, trackid: *mut MediaTrack, volume: f64, ignoresurf: *mut IReaperControlSurface) {
        self.low.CSurf_SetSurfaceVolume.unwrap()(trackid, volume, ignoresurf);
    }

    pub fn csurf_on_volume_change_ex(&self, trackid: *mut MediaTrack, volume: f64, relative: bool, allow_gang: bool) -> f64 {
        self.low.CSurf_OnVolumeChangeEx.unwrap()(trackid, volume, relative, allow_gang)
    }

    pub fn csurf_set_surface_pan(&self, trackid: *mut MediaTrack, pan: f64, ignoresurf: *mut IReaperControlSurface) {
        self.low.CSurf_SetSurfacePan.unwrap()(trackid, pan, ignoresurf);
    }

    pub fn csurf_on_pan_change_ex(&self, trackid: *mut MediaTrack, pan: f64, relative: bool, allow_gang: bool) -> f64 {
        self.low.CSurf_OnPanChangeEx.unwrap()(trackid, pan, relative, allow_gang)
    }

    pub fn count_selected_tracks_2(&self, proj: *mut ReaProject, wantmaster: bool) -> i32 {
        self.low.CountSelectedTracks2.unwrap()(proj, wantmaster)
    }

    pub fn set_track_selected(&self, track: *mut MediaTrack, selected: bool) {
        self.low.SetTrackSelected.unwrap()(track, selected);
    }

    pub fn get_selected_track_2(&self, proj: *mut ReaProject, seltrackidx: i32, wantmaster: bool) -> *mut MediaTrack {
        self.low.GetSelectedTrack2.unwrap()(proj, seltrackidx, wantmaster)
    }

    pub fn set_only_track_selected(&self, track: *mut MediaTrack) {
        self.low.SetOnlyTrackSelected.unwrap()(track);
    }

    pub fn delete_track(&self, tr: *mut MediaTrack) {
        self.low.DeleteTrack.unwrap()(tr);
    }

    pub fn get_track_num_sends(&self, tr: *mut MediaTrack, category: i32) -> u32 {
        self.low.GetTrackNumSends.unwrap()(tr, category) as u32
    }

    pub fn get_set_track_send_info(&self, tr: *mut MediaTrack, category: i32, sendidx: u32, parmname: &CStr, set_new_value: *mut c_void) -> *mut c_void {
        self.low.GetSetTrackSendInfo.unwrap()(tr, category, sendidx as i32, parmname.as_ptr(), set_new_value)
    }

    pub fn get_track_state_chunk(&self, track: *mut MediaTrack, str_need_big_sz: u32, isundo_optional: bool) -> Option<CString> {
        let (chunk_content, successful) = with_string_buffer(str_need_big_sz, |buffer, max_size| {
            self.low.GetTrackStateChunk.unwrap()(track, buffer, max_size, isundo_optional)
        });
        if !successful {
            return None;
        }
        Some(chunk_content)
    }

    pub fn create_track_send(&self, tr: *mut MediaTrack, desttr_in_optional: *mut MediaTrack) -> u32 {
        self.low.CreateTrackSend.unwrap()(tr, desttr_in_optional) as u32
    }

    pub fn csurf_on_rec_arm_change_ex(&self, trackid: *mut MediaTrack, recarm: i32, allowgang: bool) -> bool {
        self.low.CSurf_OnRecArmChangeEx.unwrap()(trackid, recarm, allowgang)
    }

    pub fn set_track_state_chunk(&self, track: *mut MediaTrack, str: &CStr, isundo_optional: bool) -> bool {
        self.low.SetTrackStateChunk.unwrap()(track, str.as_ptr(), isundo_optional)
    }

    pub fn csurf_on_send_volume_change(&self, trackid: *mut MediaTrack, send_index: u32, volume: f64, relative: bool) -> f64 {
        self.low.CSurf_OnSendVolumeChange.unwrap()(trackid, send_index as i32, volume, relative)
    }

    pub fn csurf_on_send_pan_change(&self, trackid: *mut MediaTrack, send_index: u32, pan: f64, relative: bool) -> f64 {
        self.low.CSurf_OnSendPanChange.unwrap()(trackid, send_index as i32, pan, relative)
    }

    pub fn kbd_get_text_from_cmd(&self, cmd: u32, section: *mut KbdSectionInfo) -> Option<&CStr> {
        let ptr = self.low.kbd_getTextFromCmd.unwrap()(cmd, section);
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr) })
    }

    pub fn get_toggle_command_state_2(&self, section: *mut KbdSectionInfo, command_id: i32) -> i32 {
        self.low.GetToggleCommandState2.unwrap()(section, command_id)
    }

    pub fn reverse_named_command_lookup(&self, command_id: i32) -> Option<&CStr> {
        let ptr = self.low.ReverseNamedCommandLookup.unwrap()(command_id);
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr) })
    }

    // TODO Maybe prefer Result as return data type in such cases
    pub fn get_track_send_ui_vol_pan(&self, track: *mut MediaTrack, send_index: u32) -> Option<(f64, f64)> {
        let mut volume = 0.0;
        let mut pan = 0.0;
        let successful = self.low.GetTrackSendUIVolPan.unwrap()(
            track, send_index as i32, &mut volume as *mut f64, &mut pan as *mut f64);
        if successful {
            Some((volume, pan))
        } else {
            None
        }
    }

    pub fn track_fx_get_preset_index(&self, track: *mut MediaTrack, fx: i32) -> Result<(i32, u32), ()> {
        let mut num_presets: i32 = 0;
        let index = self.low.TrackFX_GetPresetIndex.unwrap()(track, fx, &mut num_presets as *mut i32);
        if index == -1 {
            return Err(());
        }
        return Ok((index, num_presets as u32));
    }

    pub fn track_fx_set_preset_by_index(&self, track: *mut MediaTrack, fx: i32, idx: i32) -> bool {
        self.low.TrackFX_SetPresetByIndex.unwrap()(track, fx, idx)
    }

    pub fn track_fx_navigate_presets(&self, track: *mut MediaTrack, fx: i32, presetmove: i32) -> bool {
        self.low.TrackFX_NavigatePresets.unwrap()(track, fx, presetmove)
    }

    pub fn track_fx_get_preset(&self, track: *mut MediaTrack, fx: i32, presetname_sz: u32) -> (bool, Option<CString>) {
        if presetname_sz == 0 {
            let state_matches_preset = self.low.TrackFX_GetPreset.unwrap()(track, fx, null_mut(), 0);
            (state_matches_preset, None)
        } else {
            let (name, state_matches_preset) = with_string_buffer(presetname_sz, |buffer, max_size| {
                self.low.TrackFX_GetPreset.unwrap()(track, fx, buffer, max_size)
            });
            let name = if name.as_bytes().len() == 0 {
                None
            } else {
                Some(name)
            };
            (state_matches_preset, name)
        }
    }

    // TODO Rename
    // TODO Don't turn to owned string immediately
    pub fn convenient_get_media_track_info_string(&self, tr: *mut MediaTrack, parmname: &CStr) -> CString {
        let info = self.get_set_media_track_info(tr, parmname, null_mut());
        let info = info as *const c_char;
        let c_str = unsafe { CStr::from_ptr(info) };
        c_str.to_owned()
    }

    // TODO Rename or remove
    pub fn convenient_get_media_track_info_i32_value(&self, tr: *mut MediaTrack, parmname: &CStr) -> i32 {
        self.get_set_media_track_info(tr, parmname, null_mut()) as i32
    }

    // TODO Rename or remove
    pub fn convenient_get_media_track_info_i32_ptr(&self, tr: *mut MediaTrack, parmname: &CStr) -> i32 {
        let ptr = self.get_set_media_track_info(tr, parmname, null_mut()) as *mut i32;
        unsafe { *ptr }
    }

    // TODO Rename or remove
    pub fn convenient_get_media_track_info_guid(&self, tr: *mut MediaTrack, parmname: &CStr) -> *mut GUID {
        self.get_set_media_track_info(tr, parmname, null_mut()) as *mut GUID
    }
}

// TODO See what's really an option and what not
pub struct GetParameterStepSizesResult {
    pub step: f64,
    pub small_step: f64,
    pub large_step: f64,
    pub is_toggle: bool,
}

// TODO See what's really an option and what not
pub struct GetParamExResult {
    pub value: f64,
    pub min_val: f64,
    pub mid_val: f64,
    pub max_val: f64,
}

pub struct GetLastTouchedFxResult {
    pub tracknumber: i32,
    pub fxnumber: i32,
    pub paramnumber: i32,
}

// TODO Panic for now, just to detect which situations can actually occur
fn complain_if_minus_one(value: f64) -> f64 {
    if value == -1.0 {
        panic!("Out parameter was not set by REAPER")
    }
    value
}