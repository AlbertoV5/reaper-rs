# ! [ allow ( non_upper_case_globals ) ] # ! [ allow ( non_camel_case_types ) ] # ! [ allow ( non_snake_case ) ]use super::{bindings::root, ReaperPluginContext};
use c_str_macro::c_str;
#[doc = r" This is the low-level API access point to all REAPER functions. In order to use it, you first"]
#[doc = r" must obtain an instance of this struct by invoking [`load`](struct.Reaper.html#method.load)."]
#[doc = r""]
#[doc = r" Please note that it's possible that functions are *not available*. This can be the case if"]
#[doc = r" the user runs your plug-in in an older version of REAPER which doesn't have that function yet."]
#[doc = r" The availability of a function can be checked by inspecting the respective function pointer"]
#[doc = r" option in the [`pointers`](struct.Reaper.html#structfield.pointers) field. The actual methods"]
#[doc = r" in [`Reaper`](struct.Reaper.html) are just convenience methods which unwrap the function"]
#[doc = r" pointers and panic if they are not available."]
#[derive(Default)]
pub struct Reaper {
    pub pointers: ReaperFunctionPointers,
}
impl Reaper {
    #[doc = r" Loads all available REAPER functions plug-in context and returns a `Reaper` instance"]
    #[doc = r" which allows you to call these functions."]
    pub fn load(context: &ReaperPluginContext) -> Reaper {
        let get_func = &context.function_provider;
        let pointers = unsafe {
            ReaperFunctionPointers {
                __mergesort: std::mem::transmute(get_func(c_str!(stringify!(__mergesort)))),
                AddCustomizableMenu: std::mem::transmute(get_func(c_str!(stringify!(
                    AddCustomizableMenu
                )))),
                AddExtensionsMainMenu: std::mem::transmute(get_func(c_str!(stringify!(
                    AddExtensionsMainMenu
                )))),
                AddMediaItemToTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    AddMediaItemToTrack
                )))),
                AddProjectMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    AddProjectMarker
                )))),
                AddProjectMarker2: std::mem::transmute(get_func(c_str!(stringify!(
                    AddProjectMarker2
                )))),
                AddRemoveReaScript: std::mem::transmute(get_func(c_str!(stringify!(
                    AddRemoveReaScript
                )))),
                AddTakeToMediaItem: std::mem::transmute(get_func(c_str!(stringify!(
                    AddTakeToMediaItem
                )))),
                AddTempoTimeSigMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    AddTempoTimeSigMarker
                )))),
                adjustZoom: std::mem::transmute(get_func(c_str!(stringify!(adjustZoom)))),
                AnyTrackSolo: std::mem::transmute(get_func(c_str!(stringify!(AnyTrackSolo)))),
                APIExists: std::mem::transmute(get_func(c_str!(stringify!(APIExists)))),
                APITest: std::mem::transmute(get_func(c_str!(stringify!(APITest)))),
                ApplyNudge: std::mem::transmute(get_func(c_str!(stringify!(ApplyNudge)))),
                ArmCommand: std::mem::transmute(get_func(c_str!(stringify!(ArmCommand)))),
                Audio_Init: std::mem::transmute(get_func(c_str!(stringify!(Audio_Init)))),
                Audio_IsPreBuffer: std::mem::transmute(get_func(c_str!(stringify!(
                    Audio_IsPreBuffer
                )))),
                Audio_IsRunning: std::mem::transmute(get_func(c_str!(stringify!(Audio_IsRunning)))),
                Audio_Quit: std::mem::transmute(get_func(c_str!(stringify!(Audio_Quit)))),
                Audio_RegHardwareHook: std::mem::transmute(get_func(c_str!(stringify!(
                    Audio_RegHardwareHook
                )))),
                AudioAccessorStateChanged: std::mem::transmute(get_func(c_str!(stringify!(
                    AudioAccessorStateChanged
                )))),
                AudioAccessorUpdate: std::mem::transmute(get_func(c_str!(stringify!(
                    AudioAccessorUpdate
                )))),
                AudioAccessorValidateState: std::mem::transmute(get_func(c_str!(stringify!(
                    AudioAccessorValidateState
                )))),
                BypassFxAllTracks: std::mem::transmute(get_func(c_str!(stringify!(
                    BypassFxAllTracks
                )))),
                CalculatePeaks: std::mem::transmute(get_func(c_str!(stringify!(CalculatePeaks)))),
                CalculatePeaksFloatSrcPtr: std::mem::transmute(get_func(c_str!(stringify!(
                    CalculatePeaksFloatSrcPtr
                )))),
                ClearAllRecArmed: std::mem::transmute(get_func(c_str!(stringify!(
                    ClearAllRecArmed
                )))),
                ClearConsole: std::mem::transmute(get_func(c_str!(stringify!(ClearConsole)))),
                ClearPeakCache: std::mem::transmute(get_func(c_str!(stringify!(ClearPeakCache)))),
                ColorFromNative: std::mem::transmute(get_func(c_str!(stringify!(ColorFromNative)))),
                ColorToNative: std::mem::transmute(get_func(c_str!(stringify!(ColorToNative)))),
                CountActionShortcuts: std::mem::transmute(get_func(c_str!(stringify!(
                    CountActionShortcuts
                )))),
                CountAutomationItems: std::mem::transmute(get_func(c_str!(stringify!(
                    CountAutomationItems
                )))),
                CountEnvelopePoints: std::mem::transmute(get_func(c_str!(stringify!(
                    CountEnvelopePoints
                )))),
                CountEnvelopePointsEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CountEnvelopePointsEx
                )))),
                CountMediaItems: std::mem::transmute(get_func(c_str!(stringify!(CountMediaItems)))),
                CountProjectMarkers: std::mem::transmute(get_func(c_str!(stringify!(
                    CountProjectMarkers
                )))),
                CountSelectedMediaItems: std::mem::transmute(get_func(c_str!(stringify!(
                    CountSelectedMediaItems
                )))),
                CountSelectedTracks: std::mem::transmute(get_func(c_str!(stringify!(
                    CountSelectedTracks
                )))),
                CountSelectedTracks2: std::mem::transmute(get_func(c_str!(stringify!(
                    CountSelectedTracks2
                )))),
                CountTakeEnvelopes: std::mem::transmute(get_func(c_str!(stringify!(
                    CountTakeEnvelopes
                )))),
                CountTakes: std::mem::transmute(get_func(c_str!(stringify!(CountTakes)))),
                CountTCPFXParms: std::mem::transmute(get_func(c_str!(stringify!(CountTCPFXParms)))),
                CountTempoTimeSigMarkers: std::mem::transmute(get_func(c_str!(stringify!(
                    CountTempoTimeSigMarkers
                )))),
                CountTrackEnvelopes: std::mem::transmute(get_func(c_str!(stringify!(
                    CountTrackEnvelopes
                )))),
                CountTrackMediaItems: std::mem::transmute(get_func(c_str!(stringify!(
                    CountTrackMediaItems
                )))),
                CountTracks: std::mem::transmute(get_func(c_str!(stringify!(CountTracks)))),
                CreateLocalOscHandler: std::mem::transmute(get_func(c_str!(stringify!(
                    CreateLocalOscHandler
                )))),
                CreateMIDIInput: std::mem::transmute(get_func(c_str!(stringify!(CreateMIDIInput)))),
                CreateMIDIOutput: std::mem::transmute(get_func(c_str!(stringify!(
                    CreateMIDIOutput
                )))),
                CreateNewMIDIItemInProj: std::mem::transmute(get_func(c_str!(stringify!(
                    CreateNewMIDIItemInProj
                )))),
                CreateTakeAudioAccessor: std::mem::transmute(get_func(c_str!(stringify!(
                    CreateTakeAudioAccessor
                )))),
                CreateTrackAudioAccessor: std::mem::transmute(get_func(c_str!(stringify!(
                    CreateTrackAudioAccessor
                )))),
                CreateTrackSend: std::mem::transmute(get_func(c_str!(stringify!(CreateTrackSend)))),
                CSurf_FlushUndo: std::mem::transmute(get_func(c_str!(stringify!(CSurf_FlushUndo)))),
                CSurf_GetTouchState: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_GetTouchState
                )))),
                CSurf_GoEnd: std::mem::transmute(get_func(c_str!(stringify!(CSurf_GoEnd)))),
                CSurf_GoStart: std::mem::transmute(get_func(c_str!(stringify!(CSurf_GoStart)))),
                CSurf_NumTracks: std::mem::transmute(get_func(c_str!(stringify!(CSurf_NumTracks)))),
                CSurf_OnArrow: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnArrow)))),
                CSurf_OnFwd: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnFwd)))),
                CSurf_OnFXChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnFXChange
                )))),
                CSurf_OnInputMonitorChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnInputMonitorChange
                )))),
                CSurf_OnInputMonitorChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnInputMonitorChangeEx
                )))),
                CSurf_OnMuteChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnMuteChange
                )))),
                CSurf_OnMuteChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnMuteChangeEx
                )))),
                CSurf_OnOscControlMessage: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnOscControlMessage
                )))),
                CSurf_OnPanChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnPanChange
                )))),
                CSurf_OnPanChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnPanChangeEx
                )))),
                CSurf_OnPause: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnPause)))),
                CSurf_OnPlay: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnPlay)))),
                CSurf_OnPlayRateChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnPlayRateChange
                )))),
                CSurf_OnRecArmChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnRecArmChange
                )))),
                CSurf_OnRecArmChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnRecArmChangeEx
                )))),
                CSurf_OnRecord: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnRecord)))),
                CSurf_OnRecvPanChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnRecvPanChange
                )))),
                CSurf_OnRecvVolumeChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnRecvVolumeChange
                )))),
                CSurf_OnRew: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnRew)))),
                CSurf_OnRewFwd: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnRewFwd)))),
                CSurf_OnScroll: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnScroll)))),
                CSurf_OnSelectedChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnSelectedChange
                )))),
                CSurf_OnSendPanChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnSendPanChange
                )))),
                CSurf_OnSendVolumeChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnSendVolumeChange
                )))),
                CSurf_OnSoloChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnSoloChange
                )))),
                CSurf_OnSoloChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnSoloChangeEx
                )))),
                CSurf_OnStop: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnStop)))),
                CSurf_OnTempoChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnTempoChange
                )))),
                CSurf_OnTrackSelection: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnTrackSelection
                )))),
                CSurf_OnVolumeChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnVolumeChange
                )))),
                CSurf_OnVolumeChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnVolumeChangeEx
                )))),
                CSurf_OnWidthChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnWidthChange
                )))),
                CSurf_OnWidthChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnWidthChangeEx
                )))),
                CSurf_OnZoom: std::mem::transmute(get_func(c_str!(stringify!(CSurf_OnZoom)))),
                CSurf_ResetAllCachedVolPanStates: std::mem::transmute(get_func(c_str!(
                    stringify!(CSurf_ResetAllCachedVolPanStates)
                ))),
                CSurf_ScrubAmt: std::mem::transmute(get_func(c_str!(stringify!(CSurf_ScrubAmt)))),
                CSurf_SetAutoMode: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetAutoMode
                )))),
                CSurf_SetPlayState: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetPlayState
                )))),
                CSurf_SetRepeatState: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetRepeatState
                )))),
                CSurf_SetSurfaceMute: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfaceMute
                )))),
                CSurf_SetSurfacePan: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfacePan
                )))),
                CSurf_SetSurfaceRecArm: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfaceRecArm
                )))),
                CSurf_SetSurfaceSelected: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfaceSelected
                )))),
                CSurf_SetSurfaceSolo: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfaceSolo
                )))),
                CSurf_SetSurfaceVolume: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfaceVolume
                )))),
                CSurf_SetTrackListChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetTrackListChange
                )))),
                CSurf_TrackFromID: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_TrackFromID
                )))),
                CSurf_TrackToID: std::mem::transmute(get_func(c_str!(stringify!(CSurf_TrackToID)))),
                DB2SLIDER: std::mem::transmute(get_func(c_str!(stringify!(DB2SLIDER)))),
                DeleteActionShortcut: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteActionShortcut
                )))),
                DeleteEnvelopePointEx: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteEnvelopePointEx
                )))),
                DeleteEnvelopePointRange: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteEnvelopePointRange
                )))),
                DeleteEnvelopePointRangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteEnvelopePointRangeEx
                )))),
                DeleteExtState: std::mem::transmute(get_func(c_str!(stringify!(DeleteExtState)))),
                DeleteProjectMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteProjectMarker
                )))),
                DeleteProjectMarkerByIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteProjectMarkerByIndex
                )))),
                DeleteTakeStretchMarkers: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteTakeStretchMarkers
                )))),
                DeleteTempoTimeSigMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteTempoTimeSigMarker
                )))),
                DeleteTrack: std::mem::transmute(get_func(c_str!(stringify!(DeleteTrack)))),
                DeleteTrackMediaItem: std::mem::transmute(get_func(c_str!(stringify!(
                    DeleteTrackMediaItem
                )))),
                DestroyAudioAccessor: std::mem::transmute(get_func(c_str!(stringify!(
                    DestroyAudioAccessor
                )))),
                DestroyLocalOscHandler: std::mem::transmute(get_func(c_str!(stringify!(
                    DestroyLocalOscHandler
                )))),
                DoActionShortcutDialog: std::mem::transmute(get_func(c_str!(stringify!(
                    DoActionShortcutDialog
                )))),
                Dock_UpdateDockID: std::mem::transmute(get_func(c_str!(stringify!(
                    Dock_UpdateDockID
                )))),
                DockGetPosition: std::mem::transmute(get_func(c_str!(stringify!(DockGetPosition)))),
                DockIsChildOfDock: std::mem::transmute(get_func(c_str!(stringify!(
                    DockIsChildOfDock
                )))),
                DockWindowActivate: std::mem::transmute(get_func(c_str!(stringify!(
                    DockWindowActivate
                )))),
                DockWindowAdd: std::mem::transmute(get_func(c_str!(stringify!(DockWindowAdd)))),
                DockWindowAddEx: std::mem::transmute(get_func(c_str!(stringify!(DockWindowAddEx)))),
                DockWindowRefresh: std::mem::transmute(get_func(c_str!(stringify!(
                    DockWindowRefresh
                )))),
                DockWindowRefreshForHWND: std::mem::transmute(get_func(c_str!(stringify!(
                    DockWindowRefreshForHWND
                )))),
                DockWindowRemove: std::mem::transmute(get_func(c_str!(stringify!(
                    DockWindowRemove
                )))),
                DuplicateCustomizableMenu: std::mem::transmute(get_func(c_str!(stringify!(
                    DuplicateCustomizableMenu
                )))),
                EditTempoTimeSigMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    EditTempoTimeSigMarker
                )))),
                EnsureNotCompletelyOffscreen: std::mem::transmute(get_func(c_str!(stringify!(
                    EnsureNotCompletelyOffscreen
                )))),
                EnumerateFiles: std::mem::transmute(get_func(c_str!(stringify!(EnumerateFiles)))),
                EnumerateSubdirectories: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumerateSubdirectories
                )))),
                EnumPitchShiftModes: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumPitchShiftModes
                )))),
                EnumPitchShiftSubModes: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumPitchShiftSubModes
                )))),
                EnumProjectMarkers: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumProjectMarkers
                )))),
                EnumProjectMarkers2: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumProjectMarkers2
                )))),
                EnumProjectMarkers3: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumProjectMarkers3
                )))),
                EnumProjects: std::mem::transmute(get_func(c_str!(stringify!(EnumProjects)))),
                EnumProjExtState: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumProjExtState
                )))),
                EnumRegionRenderMatrix: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumRegionRenderMatrix
                )))),
                EnumTrackMIDIProgramNames: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumTrackMIDIProgramNames
                )))),
                EnumTrackMIDIProgramNamesEx: std::mem::transmute(get_func(c_str!(stringify!(
                    EnumTrackMIDIProgramNamesEx
                )))),
                Envelope_Evaluate: std::mem::transmute(get_func(c_str!(stringify!(
                    Envelope_Evaluate
                )))),
                Envelope_FormatValue: std::mem::transmute(get_func(c_str!(stringify!(
                    Envelope_FormatValue
                )))),
                Envelope_GetParentTake: std::mem::transmute(get_func(c_str!(stringify!(
                    Envelope_GetParentTake
                )))),
                Envelope_GetParentTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    Envelope_GetParentTrack
                )))),
                Envelope_SortPoints: std::mem::transmute(get_func(c_str!(stringify!(
                    Envelope_SortPoints
                )))),
                Envelope_SortPointsEx: std::mem::transmute(get_func(c_str!(stringify!(
                    Envelope_SortPointsEx
                )))),
                ExecProcess: std::mem::transmute(get_func(c_str!(stringify!(ExecProcess)))),
                file_exists: std::mem::transmute(get_func(c_str!(stringify!(file_exists)))),
                FindTempoTimeSigMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    FindTempoTimeSigMarker
                )))),
                format_timestr: std::mem::transmute(get_func(c_str!(stringify!(format_timestr)))),
                format_timestr_len: std::mem::transmute(get_func(c_str!(stringify!(
                    format_timestr_len
                )))),
                format_timestr_pos: std::mem::transmute(get_func(c_str!(stringify!(
                    format_timestr_pos
                )))),
                FreeHeapPtr: std::mem::transmute(get_func(c_str!(stringify!(FreeHeapPtr)))),
                genGuid: std::mem::transmute(get_func(c_str!(stringify!(genGuid)))),
                get_config_var: std::mem::transmute(get_func(c_str!(stringify!(get_config_var)))),
                get_config_var_string: std::mem::transmute(get_func(c_str!(stringify!(
                    get_config_var_string
                )))),
                get_ini_file: std::mem::transmute(get_func(c_str!(stringify!(get_ini_file)))),
                get_midi_config_var: std::mem::transmute(get_func(c_str!(stringify!(
                    get_midi_config_var
                )))),
                GetActionShortcutDesc: std::mem::transmute(get_func(c_str!(stringify!(
                    GetActionShortcutDesc
                )))),
                GetActiveTake: std::mem::transmute(get_func(c_str!(stringify!(GetActiveTake)))),
                GetAllProjectPlayStates: std::mem::transmute(get_func(c_str!(stringify!(
                    GetAllProjectPlayStates
                )))),
                GetAppVersion: std::mem::transmute(get_func(c_str!(stringify!(GetAppVersion)))),
                GetArmedCommand: std::mem::transmute(get_func(c_str!(stringify!(GetArmedCommand)))),
                GetAudioAccessorEndTime: std::mem::transmute(get_func(c_str!(stringify!(
                    GetAudioAccessorEndTime
                )))),
                GetAudioAccessorHash: std::mem::transmute(get_func(c_str!(stringify!(
                    GetAudioAccessorHash
                )))),
                GetAudioAccessorSamples: std::mem::transmute(get_func(c_str!(stringify!(
                    GetAudioAccessorSamples
                )))),
                GetAudioAccessorStartTime: std::mem::transmute(get_func(c_str!(stringify!(
                    GetAudioAccessorStartTime
                )))),
                GetAudioDeviceInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetAudioDeviceInfo
                )))),
                GetColorTheme: std::mem::transmute(get_func(c_str!(stringify!(GetColorTheme)))),
                GetColorThemeStruct: std::mem::transmute(get_func(c_str!(stringify!(
                    GetColorThemeStruct
                )))),
                GetConfigWantsDock: std::mem::transmute(get_func(c_str!(stringify!(
                    GetConfigWantsDock
                )))),
                GetContextMenu: std::mem::transmute(get_func(c_str!(stringify!(GetContextMenu)))),
                GetCurrentProjectInLoadSave: std::mem::transmute(get_func(c_str!(stringify!(
                    GetCurrentProjectInLoadSave
                )))),
                GetCursorContext: std::mem::transmute(get_func(c_str!(stringify!(
                    GetCursorContext
                )))),
                GetCursorContext2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetCursorContext2
                )))),
                GetCursorPosition: std::mem::transmute(get_func(c_str!(stringify!(
                    GetCursorPosition
                )))),
                GetCursorPositionEx: std::mem::transmute(get_func(c_str!(stringify!(
                    GetCursorPositionEx
                )))),
                GetDisplayedMediaItemColor: std::mem::transmute(get_func(c_str!(stringify!(
                    GetDisplayedMediaItemColor
                )))),
                GetDisplayedMediaItemColor2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetDisplayedMediaItemColor2
                )))),
                GetEnvelopeInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    GetEnvelopeInfo_Value
                )))),
                GetEnvelopeName: std::mem::transmute(get_func(c_str!(stringify!(GetEnvelopeName)))),
                GetEnvelopePoint: std::mem::transmute(get_func(c_str!(stringify!(
                    GetEnvelopePoint
                )))),
                GetEnvelopePointByTime: std::mem::transmute(get_func(c_str!(stringify!(
                    GetEnvelopePointByTime
                )))),
                GetEnvelopePointByTimeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    GetEnvelopePointByTimeEx
                )))),
                GetEnvelopePointEx: std::mem::transmute(get_func(c_str!(stringify!(
                    GetEnvelopePointEx
                )))),
                GetEnvelopeScalingMode: std::mem::transmute(get_func(c_str!(stringify!(
                    GetEnvelopeScalingMode
                )))),
                GetEnvelopeStateChunk: std::mem::transmute(get_func(c_str!(stringify!(
                    GetEnvelopeStateChunk
                )))),
                GetExePath: std::mem::transmute(get_func(c_str!(stringify!(GetExePath)))),
                GetExtState: std::mem::transmute(get_func(c_str!(stringify!(GetExtState)))),
                GetFocusedFX: std::mem::transmute(get_func(c_str!(stringify!(GetFocusedFX)))),
                GetFreeDiskSpaceForRecordPath: std::mem::transmute(get_func(c_str!(stringify!(
                    GetFreeDiskSpaceForRecordPath
                )))),
                GetFXEnvelope: std::mem::transmute(get_func(c_str!(stringify!(GetFXEnvelope)))),
                GetGlobalAutomationOverride: std::mem::transmute(get_func(c_str!(stringify!(
                    GetGlobalAutomationOverride
                )))),
                GetHZoomLevel: std::mem::transmute(get_func(c_str!(stringify!(GetHZoomLevel)))),
                GetIconThemePointer: std::mem::transmute(get_func(c_str!(stringify!(
                    GetIconThemePointer
                )))),
                GetIconThemePointerForDPI: std::mem::transmute(get_func(c_str!(stringify!(
                    GetIconThemePointerForDPI
                )))),
                GetIconThemeStruct: std::mem::transmute(get_func(c_str!(stringify!(
                    GetIconThemeStruct
                )))),
                GetInputChannelName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetInputChannelName
                )))),
                GetInputOutputLatency: std::mem::transmute(get_func(c_str!(stringify!(
                    GetInputOutputLatency
                )))),
                GetItemEditingTime2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetItemEditingTime2
                )))),
                GetItemFromPoint: std::mem::transmute(get_func(c_str!(stringify!(
                    GetItemFromPoint
                )))),
                GetItemProjectContext: std::mem::transmute(get_func(c_str!(stringify!(
                    GetItemProjectContext
                )))),
                GetItemStateChunk: std::mem::transmute(get_func(c_str!(stringify!(
                    GetItemStateChunk
                )))),
                GetLastColorThemeFile: std::mem::transmute(get_func(c_str!(stringify!(
                    GetLastColorThemeFile
                )))),
                GetLastMarkerAndCurRegion: std::mem::transmute(get_func(c_str!(stringify!(
                    GetLastMarkerAndCurRegion
                )))),
                GetLastTouchedFX: std::mem::transmute(get_func(c_str!(stringify!(
                    GetLastTouchedFX
                )))),
                GetLastTouchedTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    GetLastTouchedTrack
                )))),
                GetMainHwnd: std::mem::transmute(get_func(c_str!(stringify!(GetMainHwnd)))),
                GetMasterMuteSoloFlags: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMasterMuteSoloFlags
                )))),
                GetMasterTrack: std::mem::transmute(get_func(c_str!(stringify!(GetMasterTrack)))),
                GetMasterTrackVisibility: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMasterTrackVisibility
                )))),
                GetMaxMidiInputs: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMaxMidiInputs
                )))),
                GetMaxMidiOutputs: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMaxMidiOutputs
                )))),
                GetMediaItem: std::mem::transmute(get_func(c_str!(stringify!(GetMediaItem)))),
                GetMediaItem_Track: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItem_Track
                )))),
                GetMediaItemInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemInfo_Value
                )))),
                GetMediaItemNumTakes: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemNumTakes
                )))),
                GetMediaItemTake: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemTake
                )))),
                GetMediaItemTake_Item: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemTake_Item
                )))),
                GetMediaItemTake_Peaks: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemTake_Peaks
                )))),
                GetMediaItemTake_Source: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemTake_Source
                )))),
                GetMediaItemTake_Track: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemTake_Track
                )))),
                GetMediaItemTakeByGUID: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemTakeByGUID
                )))),
                GetMediaItemTakeInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemTakeInfo_Value
                )))),
                GetMediaItemTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaItemTrack
                )))),
                GetMediaSourceFileName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaSourceFileName
                )))),
                GetMediaSourceLength: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaSourceLength
                )))),
                GetMediaSourceNumChannels: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaSourceNumChannels
                )))),
                GetMediaSourceParent: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaSourceParent
                )))),
                GetMediaSourceSampleRate: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaSourceSampleRate
                )))),
                GetMediaSourceType: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaSourceType
                )))),
                GetMediaTrackInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaTrackInfo_Value
                )))),
                GetMIDIInputName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMIDIInputName
                )))),
                GetMIDIOutputName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMIDIOutputName
                )))),
                GetMixerScroll: std::mem::transmute(get_func(c_str!(stringify!(GetMixerScroll)))),
                GetMouseModifier: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMouseModifier
                )))),
                GetMousePosition: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMousePosition
                )))),
                GetNumAudioInputs: std::mem::transmute(get_func(c_str!(stringify!(
                    GetNumAudioInputs
                )))),
                GetNumAudioOutputs: std::mem::transmute(get_func(c_str!(stringify!(
                    GetNumAudioOutputs
                )))),
                GetNumMIDIInputs: std::mem::transmute(get_func(c_str!(stringify!(
                    GetNumMIDIInputs
                )))),
                GetNumMIDIOutputs: std::mem::transmute(get_func(c_str!(stringify!(
                    GetNumMIDIOutputs
                )))),
                GetNumTracks: std::mem::transmute(get_func(c_str!(stringify!(GetNumTracks)))),
                GetOS: std::mem::transmute(get_func(c_str!(stringify!(GetOS)))),
                GetOutputChannelName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetOutputChannelName
                )))),
                GetOutputLatency: std::mem::transmute(get_func(c_str!(stringify!(
                    GetOutputLatency
                )))),
                GetParentTrack: std::mem::transmute(get_func(c_str!(stringify!(GetParentTrack)))),
                GetPeakFileName: std::mem::transmute(get_func(c_str!(stringify!(GetPeakFileName)))),
                GetPeakFileNameEx: std::mem::transmute(get_func(c_str!(stringify!(
                    GetPeakFileNameEx
                )))),
                GetPeakFileNameEx2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetPeakFileNameEx2
                )))),
                GetPeaksBitmap: std::mem::transmute(get_func(c_str!(stringify!(GetPeaksBitmap)))),
                GetPlayPosition: std::mem::transmute(get_func(c_str!(stringify!(GetPlayPosition)))),
                GetPlayPosition2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetPlayPosition2
                )))),
                GetPlayPosition2Ex: std::mem::transmute(get_func(c_str!(stringify!(
                    GetPlayPosition2Ex
                )))),
                GetPlayPositionEx: std::mem::transmute(get_func(c_str!(stringify!(
                    GetPlayPositionEx
                )))),
                GetPlayState: std::mem::transmute(get_func(c_str!(stringify!(GetPlayState)))),
                GetPlayStateEx: std::mem::transmute(get_func(c_str!(stringify!(GetPlayStateEx)))),
                GetPreferredDiskReadMode: std::mem::transmute(get_func(c_str!(stringify!(
                    GetPreferredDiskReadMode
                )))),
                GetPreferredDiskReadModePeak: std::mem::transmute(get_func(c_str!(stringify!(
                    GetPreferredDiskReadModePeak
                )))),
                GetPreferredDiskWriteMode: std::mem::transmute(get_func(c_str!(stringify!(
                    GetPreferredDiskWriteMode
                )))),
                GetProjectLength: std::mem::transmute(get_func(c_str!(stringify!(
                    GetProjectLength
                )))),
                GetProjectName: std::mem::transmute(get_func(c_str!(stringify!(GetProjectName)))),
                GetProjectPath: std::mem::transmute(get_func(c_str!(stringify!(GetProjectPath)))),
                GetProjectPathEx: std::mem::transmute(get_func(c_str!(stringify!(
                    GetProjectPathEx
                )))),
                GetProjectStateChangeCount: std::mem::transmute(get_func(c_str!(stringify!(
                    GetProjectStateChangeCount
                )))),
                GetProjectTimeOffset: std::mem::transmute(get_func(c_str!(stringify!(
                    GetProjectTimeOffset
                )))),
                GetProjectTimeSignature: std::mem::transmute(get_func(c_str!(stringify!(
                    GetProjectTimeSignature
                )))),
                GetProjectTimeSignature2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetProjectTimeSignature2
                )))),
                GetProjExtState: std::mem::transmute(get_func(c_str!(stringify!(GetProjExtState)))),
                GetResourcePath: std::mem::transmute(get_func(c_str!(stringify!(GetResourcePath)))),
                GetSelectedEnvelope: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSelectedEnvelope
                )))),
                GetSelectedMediaItem: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSelectedMediaItem
                )))),
                GetSelectedTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSelectedTrack
                )))),
                GetSelectedTrack2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSelectedTrack2
                )))),
                GetSelectedTrackEnvelope: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSelectedTrackEnvelope
                )))),
                GetSet_ArrangeView2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSet_ArrangeView2
                )))),
                GetSet_LoopTimeRange: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSet_LoopTimeRange
                )))),
                GetSet_LoopTimeRange2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSet_LoopTimeRange2
                )))),
                GetSetAutomationItemInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetAutomationItemInfo
                )))),
                GetSetAutomationItemInfo_String: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetAutomationItemInfo_String
                )))),
                GetSetEnvelopeInfo_String: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetEnvelopeInfo_String
                )))),
                GetSetEnvelopeState: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetEnvelopeState
                )))),
                GetSetEnvelopeState2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetEnvelopeState2
                )))),
                GetSetItemState: std::mem::transmute(get_func(c_str!(stringify!(GetSetItemState)))),
                GetSetItemState2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetItemState2
                )))),
                GetSetMediaItemInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetMediaItemInfo
                )))),
                GetSetMediaItemInfo_String: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetMediaItemInfo_String
                )))),
                GetSetMediaItemTakeInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetMediaItemTakeInfo
                )))),
                GetSetMediaItemTakeInfo_String: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetMediaItemTakeInfo_String
                )))),
                GetSetMediaTrackInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetMediaTrackInfo
                )))),
                GetSetMediaTrackInfo_String: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetMediaTrackInfo_String
                )))),
                GetSetObjectState: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetObjectState
                )))),
                GetSetObjectState2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetObjectState2
                )))),
                GetSetProjectAuthor: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetProjectAuthor
                )))),
                GetSetProjectGrid: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetProjectGrid
                )))),
                GetSetProjectInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetProjectInfo
                )))),
                GetSetProjectInfo_String: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetProjectInfo_String
                )))),
                GetSetProjectNotes: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetProjectNotes
                )))),
                GetSetRepeat: std::mem::transmute(get_func(c_str!(stringify!(GetSetRepeat)))),
                GetSetRepeatEx: std::mem::transmute(get_func(c_str!(stringify!(GetSetRepeatEx)))),
                GetSetTrackGroupMembership: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetTrackGroupMembership
                )))),
                GetSetTrackGroupMembershipHigh: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetTrackGroupMembershipHigh
                )))),
                GetSetTrackMIDISupportFile: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetTrackMIDISupportFile
                )))),
                GetSetTrackSendInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetTrackSendInfo
                )))),
                GetSetTrackSendInfo_String: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetTrackSendInfo_String
                )))),
                GetSetTrackState: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetTrackState
                )))),
                GetSetTrackState2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetTrackState2
                )))),
                GetSubProjectFromSource: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSubProjectFromSource
                )))),
                GetTake: std::mem::transmute(get_func(c_str!(stringify!(GetTake)))),
                GetTakeEnvelope: std::mem::transmute(get_func(c_str!(stringify!(GetTakeEnvelope)))),
                GetTakeEnvelopeByName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTakeEnvelopeByName
                )))),
                GetTakeName: std::mem::transmute(get_func(c_str!(stringify!(GetTakeName)))),
                GetTakeNumStretchMarkers: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTakeNumStretchMarkers
                )))),
                GetTakeStretchMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTakeStretchMarker
                )))),
                GetTakeStretchMarkerSlope: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTakeStretchMarkerSlope
                )))),
                GetTCPFXParm: std::mem::transmute(get_func(c_str!(stringify!(GetTCPFXParm)))),
                GetTempoMatchPlayRate: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTempoMatchPlayRate
                )))),
                GetTempoTimeSigMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTempoTimeSigMarker
                )))),
                GetToggleCommandState: std::mem::transmute(get_func(c_str!(stringify!(
                    GetToggleCommandState
                )))),
                GetToggleCommandState2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetToggleCommandState2
                )))),
                GetToggleCommandStateEx: std::mem::transmute(get_func(c_str!(stringify!(
                    GetToggleCommandStateEx
                )))),
                GetToggleCommandStateThroughHooks: std::mem::transmute(get_func(c_str!(
                    stringify!(GetToggleCommandStateThroughHooks)
                ))),
                GetTooltipWindow: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTooltipWindow
                )))),
                GetTrack: std::mem::transmute(get_func(c_str!(stringify!(GetTrack)))),
                GetTrackAutomationMode: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackAutomationMode
                )))),
                GetTrackColor: std::mem::transmute(get_func(c_str!(stringify!(GetTrackColor)))),
                GetTrackDepth: std::mem::transmute(get_func(c_str!(stringify!(GetTrackDepth)))),
                GetTrackEnvelope: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackEnvelope
                )))),
                GetTrackEnvelopeByChunkName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackEnvelopeByChunkName
                )))),
                GetTrackEnvelopeByName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackEnvelopeByName
                )))),
                GetTrackFromPoint: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackFromPoint
                )))),
                GetTrackGUID: std::mem::transmute(get_func(c_str!(stringify!(GetTrackGUID)))),
                GetTrackInfo: std::mem::transmute(get_func(c_str!(stringify!(GetTrackInfo)))),
                GetTrackMediaItem: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackMediaItem
                )))),
                GetTrackMIDILyrics: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackMIDILyrics
                )))),
                GetTrackMIDINoteName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackMIDINoteName
                )))),
                GetTrackMIDINoteNameEx: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackMIDINoteNameEx
                )))),
                GetTrackMIDINoteRange: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackMIDINoteRange
                )))),
                GetTrackName: std::mem::transmute(get_func(c_str!(stringify!(GetTrackName)))),
                GetTrackNumMediaItems: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackNumMediaItems
                )))),
                GetTrackNumSends: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackNumSends
                )))),
                GetTrackReceiveName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackReceiveName
                )))),
                GetTrackReceiveUIMute: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackReceiveUIMute
                )))),
                GetTrackReceiveUIVolPan: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackReceiveUIVolPan
                )))),
                GetTrackSendInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackSendInfo_Value
                )))),
                GetTrackSendName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackSendName
                )))),
                GetTrackSendUIMute: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackSendUIMute
                )))),
                GetTrackSendUIVolPan: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackSendUIVolPan
                )))),
                GetTrackState: std::mem::transmute(get_func(c_str!(stringify!(GetTrackState)))),
                GetTrackStateChunk: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackStateChunk
                )))),
                GetTrackUIMute: std::mem::transmute(get_func(c_str!(stringify!(GetTrackUIMute)))),
                GetTrackUIPan: std::mem::transmute(get_func(c_str!(stringify!(GetTrackUIPan)))),
                GetTrackUIVolPan: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackUIVolPan
                )))),
                GetUnderrunTime: std::mem::transmute(get_func(c_str!(stringify!(GetUnderrunTime)))),
                GetUserFileNameForRead: std::mem::transmute(get_func(c_str!(stringify!(
                    GetUserFileNameForRead
                )))),
                GetUserInputs: std::mem::transmute(get_func(c_str!(stringify!(GetUserInputs)))),
                GoToMarker: std::mem::transmute(get_func(c_str!(stringify!(GoToMarker)))),
                GoToRegion: std::mem::transmute(get_func(c_str!(stringify!(GoToRegion)))),
                GR_SelectColor: std::mem::transmute(get_func(c_str!(stringify!(GR_SelectColor)))),
                GSC_mainwnd: std::mem::transmute(get_func(c_str!(stringify!(GSC_mainwnd)))),
                guidToString: std::mem::transmute(get_func(c_str!(stringify!(guidToString)))),
                HasExtState: std::mem::transmute(get_func(c_str!(stringify!(HasExtState)))),
                HasTrackMIDIPrograms: std::mem::transmute(get_func(c_str!(stringify!(
                    HasTrackMIDIPrograms
                )))),
                HasTrackMIDIProgramsEx: std::mem::transmute(get_func(c_str!(stringify!(
                    HasTrackMIDIProgramsEx
                )))),
                Help_Set: std::mem::transmute(get_func(c_str!(stringify!(Help_Set)))),
                HiresPeaksFromSource: std::mem::transmute(get_func(c_str!(stringify!(
                    HiresPeaksFromSource
                )))),
                image_resolve_fn: std::mem::transmute(get_func(c_str!(stringify!(
                    image_resolve_fn
                )))),
                InsertAutomationItem: std::mem::transmute(get_func(c_str!(stringify!(
                    InsertAutomationItem
                )))),
                InsertEnvelopePoint: std::mem::transmute(get_func(c_str!(stringify!(
                    InsertEnvelopePoint
                )))),
                InsertEnvelopePointEx: std::mem::transmute(get_func(c_str!(stringify!(
                    InsertEnvelopePointEx
                )))),
                InsertMedia: std::mem::transmute(get_func(c_str!(stringify!(InsertMedia)))),
                InsertMediaSection: std::mem::transmute(get_func(c_str!(stringify!(
                    InsertMediaSection
                )))),
                InsertTrackAtIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    InsertTrackAtIndex
                )))),
                IsInRealTimeAudio: std::mem::transmute(get_func(c_str!(stringify!(
                    IsInRealTimeAudio
                )))),
                IsItemTakeActiveForPlayback: std::mem::transmute(get_func(c_str!(stringify!(
                    IsItemTakeActiveForPlayback
                )))),
                IsMediaExtension: std::mem::transmute(get_func(c_str!(stringify!(
                    IsMediaExtension
                )))),
                IsMediaItemSelected: std::mem::transmute(get_func(c_str!(stringify!(
                    IsMediaItemSelected
                )))),
                IsProjectDirty: std::mem::transmute(get_func(c_str!(stringify!(IsProjectDirty)))),
                IsREAPER: std::mem::transmute(get_func(c_str!(stringify!(IsREAPER)))),
                IsTrackSelected: std::mem::transmute(get_func(c_str!(stringify!(IsTrackSelected)))),
                IsTrackVisible: std::mem::transmute(get_func(c_str!(stringify!(IsTrackVisible)))),
                joystick_create: std::mem::transmute(get_func(c_str!(stringify!(joystick_create)))),
                joystick_destroy: std::mem::transmute(get_func(c_str!(stringify!(
                    joystick_destroy
                )))),
                joystick_enum: std::mem::transmute(get_func(c_str!(stringify!(joystick_enum)))),
                joystick_getaxis: std::mem::transmute(get_func(c_str!(stringify!(
                    joystick_getaxis
                )))),
                joystick_getbuttonmask: std::mem::transmute(get_func(c_str!(stringify!(
                    joystick_getbuttonmask
                )))),
                joystick_getinfo: std::mem::transmute(get_func(c_str!(stringify!(
                    joystick_getinfo
                )))),
                joystick_getpov: std::mem::transmute(get_func(c_str!(stringify!(joystick_getpov)))),
                joystick_update: std::mem::transmute(get_func(c_str!(stringify!(joystick_update)))),
                kbd_enumerateActions: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_enumerateActions
                )))),
                kbd_formatKeyName: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_formatKeyName
                )))),
                kbd_getCommandName: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_getCommandName
                )))),
                kbd_getTextFromCmd: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_getTextFromCmd
                )))),
                KBD_OnMainActionEx: std::mem::transmute(get_func(c_str!(stringify!(
                    KBD_OnMainActionEx
                )))),
                kbd_OnMidiEvent: std::mem::transmute(get_func(c_str!(stringify!(kbd_OnMidiEvent)))),
                kbd_OnMidiList: std::mem::transmute(get_func(c_str!(stringify!(kbd_OnMidiList)))),
                kbd_ProcessActionsMenu: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_ProcessActionsMenu
                )))),
                kbd_processMidiEventActionEx: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_processMidiEventActionEx
                )))),
                kbd_reprocessMenu: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_reprocessMenu
                )))),
                kbd_RunCommandThroughHooks: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_RunCommandThroughHooks
                )))),
                kbd_translateAccelerator: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_translateAccelerator
                )))),
                kbd_translateMouse: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_translateMouse
                )))),
                LICE__Destroy: std::mem::transmute(get_func(c_str!(stringify!(LICE__Destroy)))),
                LICE__DestroyFont: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE__DestroyFont
                )))),
                LICE__DrawText: std::mem::transmute(get_func(c_str!(stringify!(LICE__DrawText)))),
                LICE__GetBits: std::mem::transmute(get_func(c_str!(stringify!(LICE__GetBits)))),
                LICE__GetDC: std::mem::transmute(get_func(c_str!(stringify!(LICE__GetDC)))),
                LICE__GetHeight: std::mem::transmute(get_func(c_str!(stringify!(LICE__GetHeight)))),
                LICE__GetRowSpan: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE__GetRowSpan
                )))),
                LICE__GetWidth: std::mem::transmute(get_func(c_str!(stringify!(LICE__GetWidth)))),
                LICE__IsFlipped: std::mem::transmute(get_func(c_str!(stringify!(LICE__IsFlipped)))),
                LICE__resize: std::mem::transmute(get_func(c_str!(stringify!(LICE__resize)))),
                LICE__SetBkColor: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE__SetBkColor
                )))),
                LICE__SetFromHFont: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE__SetFromHFont
                )))),
                LICE__SetTextColor: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE__SetTextColor
                )))),
                LICE__SetTextCombineMode: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE__SetTextCombineMode
                )))),
                LICE_Arc: std::mem::transmute(get_func(c_str!(stringify!(LICE_Arc)))),
                LICE_Blit: std::mem::transmute(get_func(c_str!(stringify!(LICE_Blit)))),
                LICE_Blur: std::mem::transmute(get_func(c_str!(stringify!(LICE_Blur)))),
                LICE_BorderedRect: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_BorderedRect
                )))),
                LICE_Circle: std::mem::transmute(get_func(c_str!(stringify!(LICE_Circle)))),
                LICE_Clear: std::mem::transmute(get_func(c_str!(stringify!(LICE_Clear)))),
                LICE_ClearRect: std::mem::transmute(get_func(c_str!(stringify!(LICE_ClearRect)))),
                LICE_ClipLine: std::mem::transmute(get_func(c_str!(stringify!(LICE_ClipLine)))),
                LICE_Copy: std::mem::transmute(get_func(c_str!(stringify!(LICE_Copy)))),
                LICE_CreateBitmap: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_CreateBitmap
                )))),
                LICE_CreateFont: std::mem::transmute(get_func(c_str!(stringify!(LICE_CreateFont)))),
                LICE_DrawCBezier: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_DrawCBezier
                )))),
                LICE_DrawChar: std::mem::transmute(get_func(c_str!(stringify!(LICE_DrawChar)))),
                LICE_DrawGlyph: std::mem::transmute(get_func(c_str!(stringify!(LICE_DrawGlyph)))),
                LICE_DrawRect: std::mem::transmute(get_func(c_str!(stringify!(LICE_DrawRect)))),
                LICE_DrawText: std::mem::transmute(get_func(c_str!(stringify!(LICE_DrawText)))),
                LICE_FillCBezier: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_FillCBezier
                )))),
                LICE_FillCircle: std::mem::transmute(get_func(c_str!(stringify!(LICE_FillCircle)))),
                LICE_FillConvexPolygon: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_FillConvexPolygon
                )))),
                LICE_FillRect: std::mem::transmute(get_func(c_str!(stringify!(LICE_FillRect)))),
                LICE_FillTrapezoid: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_FillTrapezoid
                )))),
                LICE_FillTriangle: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_FillTriangle
                )))),
                LICE_GetPixel: std::mem::transmute(get_func(c_str!(stringify!(LICE_GetPixel)))),
                LICE_GradRect: std::mem::transmute(get_func(c_str!(stringify!(LICE_GradRect)))),
                LICE_Line: std::mem::transmute(get_func(c_str!(stringify!(LICE_Line)))),
                LICE_LineInt: std::mem::transmute(get_func(c_str!(stringify!(LICE_LineInt)))),
                LICE_LoadPNG: std::mem::transmute(get_func(c_str!(stringify!(LICE_LoadPNG)))),
                LICE_LoadPNGFromResource: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_LoadPNGFromResource
                )))),
                LICE_MeasureText: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_MeasureText
                )))),
                LICE_MultiplyAddRect: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_MultiplyAddRect
                )))),
                LICE_PutPixel: std::mem::transmute(get_func(c_str!(stringify!(LICE_PutPixel)))),
                LICE_RotatedBlit: std::mem::transmute(get_func(c_str!(stringify!(
                    LICE_RotatedBlit
                )))),
                LICE_RoundRect: std::mem::transmute(get_func(c_str!(stringify!(LICE_RoundRect)))),
                LICE_ScaledBlit: std::mem::transmute(get_func(c_str!(stringify!(LICE_ScaledBlit)))),
                LICE_SimpleFill: std::mem::transmute(get_func(c_str!(stringify!(LICE_SimpleFill)))),
                Loop_OnArrow: std::mem::transmute(get_func(c_str!(stringify!(Loop_OnArrow)))),
                Main_OnCommand: std::mem::transmute(get_func(c_str!(stringify!(Main_OnCommand)))),
                Main_OnCommandEx: std::mem::transmute(get_func(c_str!(stringify!(
                    Main_OnCommandEx
                )))),
                Main_openProject: std::mem::transmute(get_func(c_str!(stringify!(
                    Main_openProject
                )))),
                Main_SaveProject: std::mem::transmute(get_func(c_str!(stringify!(
                    Main_SaveProject
                )))),
                Main_UpdateLoopInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    Main_UpdateLoopInfo
                )))),
                MarkProjectDirty: std::mem::transmute(get_func(c_str!(stringify!(
                    MarkProjectDirty
                )))),
                MarkTrackItemsDirty: std::mem::transmute(get_func(c_str!(stringify!(
                    MarkTrackItemsDirty
                )))),
                Master_GetPlayRate: std::mem::transmute(get_func(c_str!(stringify!(
                    Master_GetPlayRate
                )))),
                Master_GetPlayRateAtTime: std::mem::transmute(get_func(c_str!(stringify!(
                    Master_GetPlayRateAtTime
                )))),
                Master_GetTempo: std::mem::transmute(get_func(c_str!(stringify!(Master_GetTempo)))),
                Master_NormalizePlayRate: std::mem::transmute(get_func(c_str!(stringify!(
                    Master_NormalizePlayRate
                )))),
                Master_NormalizeTempo: std::mem::transmute(get_func(c_str!(stringify!(
                    Master_NormalizeTempo
                )))),
                MB: std::mem::transmute(get_func(c_str!(stringify!(MB)))),
                MediaItemDescendsFromTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    MediaItemDescendsFromTrack
                )))),
                MIDI_CountEvts: std::mem::transmute(get_func(c_str!(stringify!(MIDI_CountEvts)))),
                MIDI_DeleteCC: std::mem::transmute(get_func(c_str!(stringify!(MIDI_DeleteCC)))),
                MIDI_DeleteEvt: std::mem::transmute(get_func(c_str!(stringify!(MIDI_DeleteEvt)))),
                MIDI_DeleteNote: std::mem::transmute(get_func(c_str!(stringify!(MIDI_DeleteNote)))),
                MIDI_DeleteTextSysexEvt: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_DeleteTextSysexEvt
                )))),
                MIDI_DisableSort: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_DisableSort
                )))),
                MIDI_EnumSelCC: std::mem::transmute(get_func(c_str!(stringify!(MIDI_EnumSelCC)))),
                MIDI_EnumSelEvts: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_EnumSelEvts
                )))),
                MIDI_EnumSelNotes: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_EnumSelNotes
                )))),
                MIDI_EnumSelTextSysexEvts: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_EnumSelTextSysexEvts
                )))),
                MIDI_eventlist_Create: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_eventlist_Create
                )))),
                MIDI_eventlist_Destroy: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_eventlist_Destroy
                )))),
                MIDI_GetAllEvts: std::mem::transmute(get_func(c_str!(stringify!(MIDI_GetAllEvts)))),
                MIDI_GetCC: std::mem::transmute(get_func(c_str!(stringify!(MIDI_GetCC)))),
                MIDI_GetCCShape: std::mem::transmute(get_func(c_str!(stringify!(MIDI_GetCCShape)))),
                MIDI_GetEvt: std::mem::transmute(get_func(c_str!(stringify!(MIDI_GetEvt)))),
                MIDI_GetGrid: std::mem::transmute(get_func(c_str!(stringify!(MIDI_GetGrid)))),
                MIDI_GetHash: std::mem::transmute(get_func(c_str!(stringify!(MIDI_GetHash)))),
                MIDI_GetNote: std::mem::transmute(get_func(c_str!(stringify!(MIDI_GetNote)))),
                MIDI_GetPPQPos_EndOfMeasure: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_GetPPQPos_EndOfMeasure
                )))),
                MIDI_GetPPQPos_StartOfMeasure: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_GetPPQPos_StartOfMeasure
                )))),
                MIDI_GetPPQPosFromProjQN: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_GetPPQPosFromProjQN
                )))),
                MIDI_GetPPQPosFromProjTime: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_GetPPQPosFromProjTime
                )))),
                MIDI_GetProjQNFromPPQPos: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_GetProjQNFromPPQPos
                )))),
                MIDI_GetProjTimeFromPPQPos: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_GetProjTimeFromPPQPos
                )))),
                MIDI_GetScale: std::mem::transmute(get_func(c_str!(stringify!(MIDI_GetScale)))),
                MIDI_GetTextSysexEvt: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_GetTextSysexEvt
                )))),
                MIDI_GetTrackHash: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_GetTrackHash
                )))),
                MIDI_InsertCC: std::mem::transmute(get_func(c_str!(stringify!(MIDI_InsertCC)))),
                MIDI_InsertEvt: std::mem::transmute(get_func(c_str!(stringify!(MIDI_InsertEvt)))),
                MIDI_InsertNote: std::mem::transmute(get_func(c_str!(stringify!(MIDI_InsertNote)))),
                MIDI_InsertTextSysexEvt: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_InsertTextSysexEvt
                )))),
                midi_reinit: std::mem::transmute(get_func(c_str!(stringify!(midi_reinit)))),
                MIDI_SelectAll: std::mem::transmute(get_func(c_str!(stringify!(MIDI_SelectAll)))),
                MIDI_SetAllEvts: std::mem::transmute(get_func(c_str!(stringify!(MIDI_SetAllEvts)))),
                MIDI_SetCC: std::mem::transmute(get_func(c_str!(stringify!(MIDI_SetCC)))),
                MIDI_SetCCShape: std::mem::transmute(get_func(c_str!(stringify!(MIDI_SetCCShape)))),
                MIDI_SetEvt: std::mem::transmute(get_func(c_str!(stringify!(MIDI_SetEvt)))),
                MIDI_SetItemExtents: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_SetItemExtents
                )))),
                MIDI_SetNote: std::mem::transmute(get_func(c_str!(stringify!(MIDI_SetNote)))),
                MIDI_SetTextSysexEvt: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDI_SetTextSysexEvt
                )))),
                MIDI_Sort: std::mem::transmute(get_func(c_str!(stringify!(MIDI_Sort)))),
                MIDIEditor_GetActive: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDIEditor_GetActive
                )))),
                MIDIEditor_GetMode: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDIEditor_GetMode
                )))),
                MIDIEditor_GetSetting_int: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDIEditor_GetSetting_int
                )))),
                MIDIEditor_GetSetting_str: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDIEditor_GetSetting_str
                )))),
                MIDIEditor_GetTake: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDIEditor_GetTake
                )))),
                MIDIEditor_LastFocused_OnCommand: std::mem::transmute(get_func(c_str!(
                    stringify!(MIDIEditor_LastFocused_OnCommand)
                ))),
                MIDIEditor_OnCommand: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDIEditor_OnCommand
                )))),
                MIDIEditor_SetSetting_int: std::mem::transmute(get_func(c_str!(stringify!(
                    MIDIEditor_SetSetting_int
                )))),
                mkpanstr: std::mem::transmute(get_func(c_str!(stringify!(mkpanstr)))),
                mkvolpanstr: std::mem::transmute(get_func(c_str!(stringify!(mkvolpanstr)))),
                mkvolstr: std::mem::transmute(get_func(c_str!(stringify!(mkvolstr)))),
                MoveEditCursor: std::mem::transmute(get_func(c_str!(stringify!(MoveEditCursor)))),
                MoveMediaItemToTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    MoveMediaItemToTrack
                )))),
                MuteAllTracks: std::mem::transmute(get_func(c_str!(stringify!(MuteAllTracks)))),
                my_getViewport: std::mem::transmute(get_func(c_str!(stringify!(my_getViewport)))),
                NamedCommandLookup: std::mem::transmute(get_func(c_str!(stringify!(
                    NamedCommandLookup
                )))),
                OnPauseButton: std::mem::transmute(get_func(c_str!(stringify!(OnPauseButton)))),
                OnPauseButtonEx: std::mem::transmute(get_func(c_str!(stringify!(OnPauseButtonEx)))),
                OnPlayButton: std::mem::transmute(get_func(c_str!(stringify!(OnPlayButton)))),
                OnPlayButtonEx: std::mem::transmute(get_func(c_str!(stringify!(OnPlayButtonEx)))),
                OnStopButton: std::mem::transmute(get_func(c_str!(stringify!(OnStopButton)))),
                OnStopButtonEx: std::mem::transmute(get_func(c_str!(stringify!(OnStopButtonEx)))),
                OpenColorThemeFile: std::mem::transmute(get_func(c_str!(stringify!(
                    OpenColorThemeFile
                )))),
                OpenMediaExplorer: std::mem::transmute(get_func(c_str!(stringify!(
                    OpenMediaExplorer
                )))),
                OscLocalMessageToHost: std::mem::transmute(get_func(c_str!(stringify!(
                    OscLocalMessageToHost
                )))),
                parse_timestr: std::mem::transmute(get_func(c_str!(stringify!(parse_timestr)))),
                parse_timestr_len: std::mem::transmute(get_func(c_str!(stringify!(
                    parse_timestr_len
                )))),
                parse_timestr_pos: std::mem::transmute(get_func(c_str!(stringify!(
                    parse_timestr_pos
                )))),
                parsepanstr: std::mem::transmute(get_func(c_str!(stringify!(parsepanstr)))),
                PCM_Sink_Create: std::mem::transmute(get_func(c_str!(stringify!(PCM_Sink_Create)))),
                PCM_Sink_CreateEx: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Sink_CreateEx
                )))),
                PCM_Sink_CreateMIDIFile: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Sink_CreateMIDIFile
                )))),
                PCM_Sink_CreateMIDIFileEx: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Sink_CreateMIDIFileEx
                )))),
                PCM_Sink_Enum: std::mem::transmute(get_func(c_str!(stringify!(PCM_Sink_Enum)))),
                PCM_Sink_GetExtension: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Sink_GetExtension
                )))),
                PCM_Sink_ShowConfig: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Sink_ShowConfig
                )))),
                PCM_Source_CreateFromFile: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Source_CreateFromFile
                )))),
                PCM_Source_CreateFromFileEx: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Source_CreateFromFileEx
                )))),
                PCM_Source_CreateFromSimple: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Source_CreateFromSimple
                )))),
                PCM_Source_CreateFromType: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Source_CreateFromType
                )))),
                PCM_Source_Destroy: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Source_Destroy
                )))),
                PCM_Source_GetPeaks: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Source_GetPeaks
                )))),
                PCM_Source_GetSectionInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    PCM_Source_GetSectionInfo
                )))),
                PeakBuild_Create: std::mem::transmute(get_func(c_str!(stringify!(
                    PeakBuild_Create
                )))),
                PeakBuild_CreateEx: std::mem::transmute(get_func(c_str!(stringify!(
                    PeakBuild_CreateEx
                )))),
                PeakGet_Create: std::mem::transmute(get_func(c_str!(stringify!(PeakGet_Create)))),
                PitchShiftSubModeMenu: std::mem::transmute(get_func(c_str!(stringify!(
                    PitchShiftSubModeMenu
                )))),
                PlayPreview: std::mem::transmute(get_func(c_str!(stringify!(PlayPreview)))),
                PlayPreviewEx: std::mem::transmute(get_func(c_str!(stringify!(PlayPreviewEx)))),
                PlayTrackPreview: std::mem::transmute(get_func(c_str!(stringify!(
                    PlayTrackPreview
                )))),
                PlayTrackPreview2: std::mem::transmute(get_func(c_str!(stringify!(
                    PlayTrackPreview2
                )))),
                PlayTrackPreview2Ex: std::mem::transmute(get_func(c_str!(stringify!(
                    PlayTrackPreview2Ex
                )))),
                plugin_getapi: std::mem::transmute(get_func(c_str!(stringify!(plugin_getapi)))),
                plugin_getFilterList: std::mem::transmute(get_func(c_str!(stringify!(
                    plugin_getFilterList
                )))),
                plugin_getImportableProjectFilterList: std::mem::transmute(get_func(c_str!(
                    stringify!(plugin_getImportableProjectFilterList)
                ))),
                plugin_register: std::mem::transmute(get_func(c_str!(stringify!(plugin_register)))),
                PluginWantsAlwaysRunFx: std::mem::transmute(get_func(c_str!(stringify!(
                    PluginWantsAlwaysRunFx
                )))),
                PreventUIRefresh: std::mem::transmute(get_func(c_str!(stringify!(
                    PreventUIRefresh
                )))),
                projectconfig_var_addr: std::mem::transmute(get_func(c_str!(stringify!(
                    projectconfig_var_addr
                )))),
                projectconfig_var_getoffs: std::mem::transmute(get_func(c_str!(stringify!(
                    projectconfig_var_getoffs
                )))),
                realloc_cmd_ptr: std::mem::transmute(get_func(c_str!(stringify!(realloc_cmd_ptr)))),
                ReaperGetPitchShiftAPI: std::mem::transmute(get_func(c_str!(stringify!(
                    ReaperGetPitchShiftAPI
                )))),
                ReaScriptError: std::mem::transmute(get_func(c_str!(stringify!(ReaScriptError)))),
                RecursiveCreateDirectory: std::mem::transmute(get_func(c_str!(stringify!(
                    RecursiveCreateDirectory
                )))),
                reduce_open_files: std::mem::transmute(get_func(c_str!(stringify!(
                    reduce_open_files
                )))),
                RefreshToolbar: std::mem::transmute(get_func(c_str!(stringify!(RefreshToolbar)))),
                RefreshToolbar2: std::mem::transmute(get_func(c_str!(stringify!(RefreshToolbar2)))),
                relative_fn: std::mem::transmute(get_func(c_str!(stringify!(relative_fn)))),
                RemoveTrackSend: std::mem::transmute(get_func(c_str!(stringify!(RemoveTrackSend)))),
                RenderFileSection: std::mem::transmute(get_func(c_str!(stringify!(
                    RenderFileSection
                )))),
                ReorderSelectedTracks: std::mem::transmute(get_func(c_str!(stringify!(
                    ReorderSelectedTracks
                )))),
                Resample_EnumModes: std::mem::transmute(get_func(c_str!(stringify!(
                    Resample_EnumModes
                )))),
                Resampler_Create: std::mem::transmute(get_func(c_str!(stringify!(
                    Resampler_Create
                )))),
                resolve_fn: std::mem::transmute(get_func(c_str!(stringify!(resolve_fn)))),
                resolve_fn2: std::mem::transmute(get_func(c_str!(stringify!(resolve_fn2)))),
                ReverseNamedCommandLookup: std::mem::transmute(get_func(c_str!(stringify!(
                    ReverseNamedCommandLookup
                )))),
                ScaleFromEnvelopeMode: std::mem::transmute(get_func(c_str!(stringify!(
                    ScaleFromEnvelopeMode
                )))),
                ScaleToEnvelopeMode: std::mem::transmute(get_func(c_str!(stringify!(
                    ScaleToEnvelopeMode
                )))),
                screenset_register: std::mem::transmute(get_func(c_str!(stringify!(
                    screenset_register
                )))),
                screenset_registerNew: std::mem::transmute(get_func(c_str!(stringify!(
                    screenset_registerNew
                )))),
                screenset_unregister: std::mem::transmute(get_func(c_str!(stringify!(
                    screenset_unregister
                )))),
                screenset_unregisterByParam: std::mem::transmute(get_func(c_str!(stringify!(
                    screenset_unregisterByParam
                )))),
                screenset_updateLastFocus: std::mem::transmute(get_func(c_str!(stringify!(
                    screenset_updateLastFocus
                )))),
                SectionFromUniqueID: std::mem::transmute(get_func(c_str!(stringify!(
                    SectionFromUniqueID
                )))),
                SelectAllMediaItems: std::mem::transmute(get_func(c_str!(stringify!(
                    SelectAllMediaItems
                )))),
                SelectProjectInstance: std::mem::transmute(get_func(c_str!(stringify!(
                    SelectProjectInstance
                )))),
                SendLocalOscMessage: std::mem::transmute(get_func(c_str!(stringify!(
                    SendLocalOscMessage
                )))),
                SetActiveTake: std::mem::transmute(get_func(c_str!(stringify!(SetActiveTake)))),
                SetAutomationMode: std::mem::transmute(get_func(c_str!(stringify!(
                    SetAutomationMode
                )))),
                SetCurrentBPM: std::mem::transmute(get_func(c_str!(stringify!(SetCurrentBPM)))),
                SetCursorContext: std::mem::transmute(get_func(c_str!(stringify!(
                    SetCursorContext
                )))),
                SetEditCurPos: std::mem::transmute(get_func(c_str!(stringify!(SetEditCurPos)))),
                SetEditCurPos2: std::mem::transmute(get_func(c_str!(stringify!(SetEditCurPos2)))),
                SetEnvelopePoint: std::mem::transmute(get_func(c_str!(stringify!(
                    SetEnvelopePoint
                )))),
                SetEnvelopePointEx: std::mem::transmute(get_func(c_str!(stringify!(
                    SetEnvelopePointEx
                )))),
                SetEnvelopeStateChunk: std::mem::transmute(get_func(c_str!(stringify!(
                    SetEnvelopeStateChunk
                )))),
                SetExtState: std::mem::transmute(get_func(c_str!(stringify!(SetExtState)))),
                SetGlobalAutomationOverride: std::mem::transmute(get_func(c_str!(stringify!(
                    SetGlobalAutomationOverride
                )))),
                SetItemStateChunk: std::mem::transmute(get_func(c_str!(stringify!(
                    SetItemStateChunk
                )))),
                SetMasterTrackVisibility: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMasterTrackVisibility
                )))),
                SetMediaItemInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMediaItemInfo_Value
                )))),
                SetMediaItemLength: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMediaItemLength
                )))),
                SetMediaItemPosition: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMediaItemPosition
                )))),
                SetMediaItemSelected: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMediaItemSelected
                )))),
                SetMediaItemTake_Source: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMediaItemTake_Source
                )))),
                SetMediaItemTakeInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMediaItemTakeInfo_Value
                )))),
                SetMediaTrackInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMediaTrackInfo_Value
                )))),
                SetMIDIEditorGrid: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMIDIEditorGrid
                )))),
                SetMixerScroll: std::mem::transmute(get_func(c_str!(stringify!(SetMixerScroll)))),
                SetMouseModifier: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMouseModifier
                )))),
                SetOnlyTrackSelected: std::mem::transmute(get_func(c_str!(stringify!(
                    SetOnlyTrackSelected
                )))),
                SetProjectGrid: std::mem::transmute(get_func(c_str!(stringify!(SetProjectGrid)))),
                SetProjectMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    SetProjectMarker
                )))),
                SetProjectMarker2: std::mem::transmute(get_func(c_str!(stringify!(
                    SetProjectMarker2
                )))),
                SetProjectMarker3: std::mem::transmute(get_func(c_str!(stringify!(
                    SetProjectMarker3
                )))),
                SetProjectMarker4: std::mem::transmute(get_func(c_str!(stringify!(
                    SetProjectMarker4
                )))),
                SetProjectMarkerByIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    SetProjectMarkerByIndex
                )))),
                SetProjectMarkerByIndex2: std::mem::transmute(get_func(c_str!(stringify!(
                    SetProjectMarkerByIndex2
                )))),
                SetProjExtState: std::mem::transmute(get_func(c_str!(stringify!(SetProjExtState)))),
                SetRegionRenderMatrix: std::mem::transmute(get_func(c_str!(stringify!(
                    SetRegionRenderMatrix
                )))),
                SetRenderLastError: std::mem::transmute(get_func(c_str!(stringify!(
                    SetRenderLastError
                )))),
                SetTakeStretchMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTakeStretchMarker
                )))),
                SetTakeStretchMarkerSlope: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTakeStretchMarkerSlope
                )))),
                SetTempoTimeSigMarker: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTempoTimeSigMarker
                )))),
                SetToggleCommandState: std::mem::transmute(get_func(c_str!(stringify!(
                    SetToggleCommandState
                )))),
                SetTrackAutomationMode: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackAutomationMode
                )))),
                SetTrackColor: std::mem::transmute(get_func(c_str!(stringify!(SetTrackColor)))),
                SetTrackMIDILyrics: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackMIDILyrics
                )))),
                SetTrackMIDINoteName: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackMIDINoteName
                )))),
                SetTrackMIDINoteNameEx: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackMIDINoteNameEx
                )))),
                SetTrackSelected: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackSelected
                )))),
                SetTrackSendInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackSendInfo_Value
                )))),
                SetTrackSendUIPan: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackSendUIPan
                )))),
                SetTrackSendUIVol: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackSendUIVol
                )))),
                SetTrackStateChunk: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackStateChunk
                )))),
                ShowActionList: std::mem::transmute(get_func(c_str!(stringify!(ShowActionList)))),
                ShowConsoleMsg: std::mem::transmute(get_func(c_str!(stringify!(ShowConsoleMsg)))),
                ShowMessageBox: std::mem::transmute(get_func(c_str!(stringify!(ShowMessageBox)))),
                ShowPopupMenu: std::mem::transmute(get_func(c_str!(stringify!(ShowPopupMenu)))),
                SLIDER2DB: std::mem::transmute(get_func(c_str!(stringify!(SLIDER2DB)))),
                SnapToGrid: std::mem::transmute(get_func(c_str!(stringify!(SnapToGrid)))),
                SoloAllTracks: std::mem::transmute(get_func(c_str!(stringify!(SoloAllTracks)))),
                Splash_GetWnd: std::mem::transmute(get_func(c_str!(stringify!(Splash_GetWnd)))),
                SplitMediaItem: std::mem::transmute(get_func(c_str!(stringify!(SplitMediaItem)))),
                StopPreview: std::mem::transmute(get_func(c_str!(stringify!(StopPreview)))),
                StopTrackPreview: std::mem::transmute(get_func(c_str!(stringify!(
                    StopTrackPreview
                )))),
                StopTrackPreview2: std::mem::transmute(get_func(c_str!(stringify!(
                    StopTrackPreview2
                )))),
                stringToGuid: std::mem::transmute(get_func(c_str!(stringify!(stringToGuid)))),
                StuffMIDIMessage: std::mem::transmute(get_func(c_str!(stringify!(
                    StuffMIDIMessage
                )))),
                TakeFX_AddByName: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_AddByName
                )))),
                TakeFX_CopyToTake: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_CopyToTake
                )))),
                TakeFX_CopyToTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_CopyToTrack
                )))),
                TakeFX_Delete: std::mem::transmute(get_func(c_str!(stringify!(TakeFX_Delete)))),
                TakeFX_EndParamEdit: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_EndParamEdit
                )))),
                TakeFX_FormatParamValue: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_FormatParamValue
                )))),
                TakeFX_FormatParamValueNormalized: std::mem::transmute(get_func(c_str!(
                    stringify!(TakeFX_FormatParamValueNormalized)
                ))),
                TakeFX_GetChainVisible: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetChainVisible
                )))),
                TakeFX_GetCount: std::mem::transmute(get_func(c_str!(stringify!(TakeFX_GetCount)))),
                TakeFX_GetEnabled: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetEnabled
                )))),
                TakeFX_GetEnvelope: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetEnvelope
                )))),
                TakeFX_GetFloatingWindow: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetFloatingWindow
                )))),
                TakeFX_GetFormattedParamValue: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetFormattedParamValue
                )))),
                TakeFX_GetFXGUID: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetFXGUID
                )))),
                TakeFX_GetFXName: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetFXName
                )))),
                TakeFX_GetIOSize: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetIOSize
                )))),
                TakeFX_GetNamedConfigParm: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetNamedConfigParm
                )))),
                TakeFX_GetNumParams: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetNumParams
                )))),
                TakeFX_GetOffline: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetOffline
                )))),
                TakeFX_GetOpen: std::mem::transmute(get_func(c_str!(stringify!(TakeFX_GetOpen)))),
                TakeFX_GetParam: std::mem::transmute(get_func(c_str!(stringify!(TakeFX_GetParam)))),
                TakeFX_GetParameterStepSizes: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetParameterStepSizes
                )))),
                TakeFX_GetParamEx: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetParamEx
                )))),
                TakeFX_GetParamName: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetParamName
                )))),
                TakeFX_GetParamNormalized: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetParamNormalized
                )))),
                TakeFX_GetPinMappings: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetPinMappings
                )))),
                TakeFX_GetPreset: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetPreset
                )))),
                TakeFX_GetPresetIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetPresetIndex
                )))),
                TakeFX_GetUserPresetFilename: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_GetUserPresetFilename
                )))),
                TakeFX_NavigatePresets: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_NavigatePresets
                )))),
                TakeFX_SetEnabled: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_SetEnabled
                )))),
                TakeFX_SetNamedConfigParm: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_SetNamedConfigParm
                )))),
                TakeFX_SetOffline: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_SetOffline
                )))),
                TakeFX_SetOpen: std::mem::transmute(get_func(c_str!(stringify!(TakeFX_SetOpen)))),
                TakeFX_SetParam: std::mem::transmute(get_func(c_str!(stringify!(TakeFX_SetParam)))),
                TakeFX_SetParamNormalized: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_SetParamNormalized
                )))),
                TakeFX_SetPinMappings: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_SetPinMappings
                )))),
                TakeFX_SetPreset: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_SetPreset
                )))),
                TakeFX_SetPresetByIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    TakeFX_SetPresetByIndex
                )))),
                TakeFX_Show: std::mem::transmute(get_func(c_str!(stringify!(TakeFX_Show)))),
                TakeIsMIDI: std::mem::transmute(get_func(c_str!(stringify!(TakeIsMIDI)))),
                ThemeLayout_GetLayout: std::mem::transmute(get_func(c_str!(stringify!(
                    ThemeLayout_GetLayout
                )))),
                ThemeLayout_GetParameter: std::mem::transmute(get_func(c_str!(stringify!(
                    ThemeLayout_GetParameter
                )))),
                ThemeLayout_RefreshAll: std::mem::transmute(get_func(c_str!(stringify!(
                    ThemeLayout_RefreshAll
                )))),
                ThemeLayout_SetLayout: std::mem::transmute(get_func(c_str!(stringify!(
                    ThemeLayout_SetLayout
                )))),
                ThemeLayout_SetParameter: std::mem::transmute(get_func(c_str!(stringify!(
                    ThemeLayout_SetParameter
                )))),
                time_precise: std::mem::transmute(get_func(c_str!(stringify!(time_precise)))),
                TimeMap2_beatsToTime: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap2_beatsToTime
                )))),
                TimeMap2_GetDividedBpmAtTime: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap2_GetDividedBpmAtTime
                )))),
                TimeMap2_GetNextChangeTime: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap2_GetNextChangeTime
                )))),
                TimeMap2_QNToTime: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap2_QNToTime
                )))),
                TimeMap2_timeToBeats: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap2_timeToBeats
                )))),
                TimeMap2_timeToQN: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap2_timeToQN
                )))),
                TimeMap_curFrameRate: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_curFrameRate
                )))),
                TimeMap_GetDividedBpmAtTime: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_GetDividedBpmAtTime
                )))),
                TimeMap_GetMeasureInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_GetMeasureInfo
                )))),
                TimeMap_GetMetronomePattern: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_GetMetronomePattern
                )))),
                TimeMap_GetTimeSigAtTime: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_GetTimeSigAtTime
                )))),
                TimeMap_QNToMeasures: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_QNToMeasures
                )))),
                TimeMap_QNToTime: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_QNToTime
                )))),
                TimeMap_QNToTime_abs: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_QNToTime_abs
                )))),
                TimeMap_timeToQN: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_timeToQN
                )))),
                TimeMap_timeToQN_abs: std::mem::transmute(get_func(c_str!(stringify!(
                    TimeMap_timeToQN_abs
                )))),
                ToggleTrackSendUIMute: std::mem::transmute(get_func(c_str!(stringify!(
                    ToggleTrackSendUIMute
                )))),
                Track_GetPeakHoldDB: std::mem::transmute(get_func(c_str!(stringify!(
                    Track_GetPeakHoldDB
                )))),
                Track_GetPeakInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    Track_GetPeakInfo
                )))),
                TrackCtl_SetToolTip: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackCtl_SetToolTip
                )))),
                TrackFX_AddByName: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_AddByName
                )))),
                TrackFX_CopyToTake: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_CopyToTake
                )))),
                TrackFX_CopyToTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_CopyToTrack
                )))),
                TrackFX_Delete: std::mem::transmute(get_func(c_str!(stringify!(TrackFX_Delete)))),
                TrackFX_EndParamEdit: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_EndParamEdit
                )))),
                TrackFX_FormatParamValue: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_FormatParamValue
                )))),
                TrackFX_FormatParamValueNormalized: std::mem::transmute(get_func(c_str!(
                    stringify!(TrackFX_FormatParamValueNormalized)
                ))),
                TrackFX_GetByName: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetByName
                )))),
                TrackFX_GetChainVisible: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetChainVisible
                )))),
                TrackFX_GetCount: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetCount
                )))),
                TrackFX_GetEnabled: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetEnabled
                )))),
                TrackFX_GetEQ: std::mem::transmute(get_func(c_str!(stringify!(TrackFX_GetEQ)))),
                TrackFX_GetEQBandEnabled: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetEQBandEnabled
                )))),
                TrackFX_GetEQParam: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetEQParam
                )))),
                TrackFX_GetFloatingWindow: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetFloatingWindow
                )))),
                TrackFX_GetFormattedParamValue: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetFormattedParamValue
                )))),
                TrackFX_GetFXGUID: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetFXGUID
                )))),
                TrackFX_GetFXName: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetFXName
                )))),
                TrackFX_GetInstrument: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetInstrument
                )))),
                TrackFX_GetIOSize: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetIOSize
                )))),
                TrackFX_GetNamedConfigParm: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetNamedConfigParm
                )))),
                TrackFX_GetNumParams: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetNumParams
                )))),
                TrackFX_GetOffline: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetOffline
                )))),
                TrackFX_GetOpen: std::mem::transmute(get_func(c_str!(stringify!(TrackFX_GetOpen)))),
                TrackFX_GetParam: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParam
                )))),
                TrackFX_GetParameterStepSizes: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParameterStepSizes
                )))),
                TrackFX_GetParamEx: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParamEx
                )))),
                TrackFX_GetParamName: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParamName
                )))),
                TrackFX_GetParamNormalized: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParamNormalized
                )))),
                TrackFX_GetPinMappings: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetPinMappings
                )))),
                TrackFX_GetPreset: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetPreset
                )))),
                TrackFX_GetPresetIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetPresetIndex
                )))),
                TrackFX_GetRecChainVisible: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetRecChainVisible
                )))),
                TrackFX_GetRecCount: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetRecCount
                )))),
                TrackFX_GetUserPresetFilename: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetUserPresetFilename
                )))),
                TrackFX_NavigatePresets: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_NavigatePresets
                )))),
                TrackFX_SetEnabled: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetEnabled
                )))),
                TrackFX_SetEQBandEnabled: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetEQBandEnabled
                )))),
                TrackFX_SetEQParam: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetEQParam
                )))),
                TrackFX_SetNamedConfigParm: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetNamedConfigParm
                )))),
                TrackFX_SetOffline: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetOffline
                )))),
                TrackFX_SetOpen: std::mem::transmute(get_func(c_str!(stringify!(TrackFX_SetOpen)))),
                TrackFX_SetParam: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetParam
                )))),
                TrackFX_SetParamNormalized: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetParamNormalized
                )))),
                TrackFX_SetPinMappings: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetPinMappings
                )))),
                TrackFX_SetPreset: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetPreset
                )))),
                TrackFX_SetPresetByIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetPresetByIndex
                )))),
                TrackFX_Show: std::mem::transmute(get_func(c_str!(stringify!(TrackFX_Show)))),
                TrackList_AdjustWindows: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackList_AdjustWindows
                )))),
                TrackList_UpdateAllExternalSurfaces: std::mem::transmute(get_func(c_str!(
                    stringify!(TrackList_UpdateAllExternalSurfaces)
                ))),
                Undo_BeginBlock: std::mem::transmute(get_func(c_str!(stringify!(Undo_BeginBlock)))),
                Undo_BeginBlock2: std::mem::transmute(get_func(c_str!(stringify!(
                    Undo_BeginBlock2
                )))),
                Undo_CanRedo2: std::mem::transmute(get_func(c_str!(stringify!(Undo_CanRedo2)))),
                Undo_CanUndo2: std::mem::transmute(get_func(c_str!(stringify!(Undo_CanUndo2)))),
                Undo_DoRedo2: std::mem::transmute(get_func(c_str!(stringify!(Undo_DoRedo2)))),
                Undo_DoUndo2: std::mem::transmute(get_func(c_str!(stringify!(Undo_DoUndo2)))),
                Undo_EndBlock: std::mem::transmute(get_func(c_str!(stringify!(Undo_EndBlock)))),
                Undo_EndBlock2: std::mem::transmute(get_func(c_str!(stringify!(Undo_EndBlock2)))),
                Undo_OnStateChange: std::mem::transmute(get_func(c_str!(stringify!(
                    Undo_OnStateChange
                )))),
                Undo_OnStateChange2: std::mem::transmute(get_func(c_str!(stringify!(
                    Undo_OnStateChange2
                )))),
                Undo_OnStateChange_Item: std::mem::transmute(get_func(c_str!(stringify!(
                    Undo_OnStateChange_Item
                )))),
                Undo_OnStateChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    Undo_OnStateChangeEx
                )))),
                Undo_OnStateChangeEx2: std::mem::transmute(get_func(c_str!(stringify!(
                    Undo_OnStateChangeEx2
                )))),
                update_disk_counters: std::mem::transmute(get_func(c_str!(stringify!(
                    update_disk_counters
                )))),
                UpdateArrange: std::mem::transmute(get_func(c_str!(stringify!(UpdateArrange)))),
                UpdateItemInProject: std::mem::transmute(get_func(c_str!(stringify!(
                    UpdateItemInProject
                )))),
                UpdateTimeline: std::mem::transmute(get_func(c_str!(stringify!(UpdateTimeline)))),
                ValidatePtr: std::mem::transmute(get_func(c_str!(stringify!(ValidatePtr)))),
                ValidatePtr2: std::mem::transmute(get_func(c_str!(stringify!(ValidatePtr2)))),
                ViewPrefs: std::mem::transmute(get_func(c_str!(stringify!(ViewPrefs)))),
                WDL_VirtualWnd_ScaledBlitBG: std::mem::transmute(get_func(c_str!(stringify!(
                    WDL_VirtualWnd_ScaledBlitBG
                )))),
                GetMidiInput: std::mem::transmute(get_func(c_str!(stringify!(GetMidiInput)))),
                GetMidiOutput: std::mem::transmute(get_func(c_str!(stringify!(GetMidiOutput)))),
            }
        };
        Reaper { pointers }
    }
    pub unsafe fn __mergesort(
        &self,
        base: *mut ::std::os::raw::c_void,
        nmemb: usize,
        size: usize,
        cmpfunc: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        tmpspace: *mut ::std::os::raw::c_void,
    ) {
        match self.pointers.__mergesort {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(__mergesort)
            )),
            Some(f) => f(base, nmemb, size, cmpfunc, tmpspace),
        }
    }
    pub unsafe fn AddCustomizableMenu(
        &self,
        menuidstr: *const ::std::os::raw::c_char,
        menuname: *const ::std::os::raw::c_char,
        kbdsecname: *const ::std::os::raw::c_char,
        addtomainmenu: bool,
    ) -> bool {
        match self.pointers.AddCustomizableMenu {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AddCustomizableMenu)
            )),
            Some(f) => f(menuidstr, menuname, kbdsecname, addtomainmenu),
        }
    }
    pub fn AddExtensionsMainMenu(&self) -> bool {
        match self.pointers.AddExtensionsMainMenu {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AddExtensionsMainMenu)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn AddMediaItemToTrack(&self, tr: *mut root::MediaTrack) -> *mut root::MediaItem {
        match self.pointers.AddMediaItemToTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AddMediaItemToTrack)
            )),
            Some(f) => f(tr),
        }
    }
    pub unsafe fn AddProjectMarker(
        &self,
        proj: *mut root::ReaProject,
        isrgn: bool,
        pos: f64,
        rgnend: f64,
        name: *const ::std::os::raw::c_char,
        wantidx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.AddProjectMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AddProjectMarker)
            )),
            Some(f) => f(proj, isrgn, pos, rgnend, name, wantidx),
        }
    }
    pub unsafe fn AddProjectMarker2(
        &self,
        proj: *mut root::ReaProject,
        isrgn: bool,
        pos: f64,
        rgnend: f64,
        name: *const ::std::os::raw::c_char,
        wantidx: ::std::os::raw::c_int,
        color: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.AddProjectMarker2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AddProjectMarker2)
            )),
            Some(f) => f(proj, isrgn, pos, rgnend, name, wantidx, color),
        }
    }
    pub unsafe fn AddRemoveReaScript(
        &self,
        add: bool,
        sectionID: ::std::os::raw::c_int,
        scriptfn: *const ::std::os::raw::c_char,
        commit: bool,
    ) -> ::std::os::raw::c_int {
        match self.pointers.AddRemoveReaScript {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AddRemoveReaScript)
            )),
            Some(f) => f(add, sectionID, scriptfn, commit),
        }
    }
    pub unsafe fn AddTakeToMediaItem(
        &self,
        item: *mut root::MediaItem,
    ) -> *mut root::MediaItem_Take {
        match self.pointers.AddTakeToMediaItem {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AddTakeToMediaItem)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn AddTempoTimeSigMarker(
        &self,
        proj: *mut root::ReaProject,
        timepos: f64,
        bpm: f64,
        timesig_num: ::std::os::raw::c_int,
        timesig_denom: ::std::os::raw::c_int,
        lineartempochange: bool,
    ) -> bool {
        match self.pointers.AddTempoTimeSigMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AddTempoTimeSigMarker)
            )),
            Some(f) => f(
                proj,
                timepos,
                bpm,
                timesig_num,
                timesig_denom,
                lineartempochange,
            ),
        }
    }
    pub fn adjustZoom(
        &self,
        amt: f64,
        forceset: ::std::os::raw::c_int,
        doupd: bool,
        centermode: ::std::os::raw::c_int,
    ) {
        match self.pointers.adjustZoom {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(adjustZoom)
            )),
            Some(f) => f(amt, forceset, doupd, centermode),
        }
    }
    pub unsafe fn AnyTrackSolo(&self, proj: *mut root::ReaProject) -> bool {
        match self.pointers.AnyTrackSolo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AnyTrackSolo)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn APIExists(&self, function_name: *const ::std::os::raw::c_char) -> bool {
        match self.pointers.APIExists {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(APIExists)
            )),
            Some(f) => f(function_name),
        }
    }
    pub fn APITest(&self) {
        match self.pointers.APITest {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(APITest)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn ApplyNudge(
        &self,
        project: *mut root::ReaProject,
        nudgeflag: ::std::os::raw::c_int,
        nudgewhat: ::std::os::raw::c_int,
        nudgeunits: ::std::os::raw::c_int,
        value: f64,
        reverse: bool,
        copies: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.ApplyNudge {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ApplyNudge)
            )),
            Some(f) => f(
                project, nudgeflag, nudgewhat, nudgeunits, value, reverse, copies,
            ),
        }
    }
    pub unsafe fn ArmCommand(
        &self,
        cmd: ::std::os::raw::c_int,
        sectionname: *const ::std::os::raw::c_char,
    ) {
        match self.pointers.ArmCommand {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ArmCommand)
            )),
            Some(f) => f(cmd, sectionname),
        }
    }
    pub fn Audio_Init(&self) {
        match self.pointers.Audio_Init {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Audio_Init)
            )),
            Some(f) => f(),
        }
    }
    pub fn Audio_IsPreBuffer(&self) -> ::std::os::raw::c_int {
        match self.pointers.Audio_IsPreBuffer {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Audio_IsPreBuffer)
            )),
            Some(f) => f(),
        }
    }
    pub fn Audio_IsRunning(&self) -> ::std::os::raw::c_int {
        match self.pointers.Audio_IsRunning {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Audio_IsRunning)
            )),
            Some(f) => f(),
        }
    }
    pub fn Audio_Quit(&self) {
        match self.pointers.Audio_Quit {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Audio_Quit)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn Audio_RegHardwareHook(
        &self,
        isAdd: bool,
        reg: *mut root::audio_hook_register_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.Audio_RegHardwareHook {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Audio_RegHardwareHook)
            )),
            Some(f) => f(isAdd, reg),
        }
    }
    pub unsafe fn AudioAccessorStateChanged(
        &self,
        accessor: *mut root::reaper_functions::AudioAccessor,
    ) -> bool {
        match self.pointers.AudioAccessorStateChanged {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AudioAccessorStateChanged)
            )),
            Some(f) => f(accessor),
        }
    }
    pub unsafe fn AudioAccessorUpdate(&self, accessor: *mut root::reaper_functions::AudioAccessor) {
        match self.pointers.AudioAccessorUpdate {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AudioAccessorUpdate)
            )),
            Some(f) => f(accessor),
        }
    }
    pub unsafe fn AudioAccessorValidateState(
        &self,
        accessor: *mut root::reaper_functions::AudioAccessor,
    ) -> bool {
        match self.pointers.AudioAccessorValidateState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(AudioAccessorValidateState)
            )),
            Some(f) => f(accessor),
        }
    }
    pub fn BypassFxAllTracks(&self, bypass: ::std::os::raw::c_int) {
        match self.pointers.BypassFxAllTracks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(BypassFxAllTracks)
            )),
            Some(f) => f(bypass),
        }
    }
    pub unsafe fn CalculatePeaks(
        &self,
        srcBlock: *mut root::PCM_source_transfer_t,
        pksBlock: *mut root::PCM_source_peaktransfer_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CalculatePeaks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CalculatePeaks)
            )),
            Some(f) => f(srcBlock, pksBlock),
        }
    }
    pub unsafe fn CalculatePeaksFloatSrcPtr(
        &self,
        srcBlock: *mut root::PCM_source_transfer_t,
        pksBlock: *mut root::PCM_source_peaktransfer_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CalculatePeaksFloatSrcPtr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CalculatePeaksFloatSrcPtr)
            )),
            Some(f) => f(srcBlock, pksBlock),
        }
    }
    pub fn ClearAllRecArmed(&self) {
        match self.pointers.ClearAllRecArmed {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ClearAllRecArmed)
            )),
            Some(f) => f(),
        }
    }
    pub fn ClearConsole(&self) {
        match self.pointers.ClearConsole {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ClearConsole)
            )),
            Some(f) => f(),
        }
    }
    pub fn ClearPeakCache(&self) {
        match self.pointers.ClearPeakCache {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ClearPeakCache)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn ColorFromNative(
        &self,
        col: ::std::os::raw::c_int,
        rOut: *mut ::std::os::raw::c_int,
        gOut: *mut ::std::os::raw::c_int,
        bOut: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.ColorFromNative {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ColorFromNative)
            )),
            Some(f) => f(col, rOut, gOut, bOut),
        }
    }
    pub fn ColorToNative(
        &self,
        r: ::std::os::raw::c_int,
        g: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.ColorToNative {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ColorToNative)
            )),
            Some(f) => f(r, g, b),
        }
    }
    pub unsafe fn CountActionShortcuts(
        &self,
        section: *mut root::KbdSectionInfo,
        cmdID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountActionShortcuts {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountActionShortcuts)
            )),
            Some(f) => f(section, cmdID),
        }
    }
    pub unsafe fn CountAutomationItems(
        &self,
        env: *mut root::TrackEnvelope,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountAutomationItems {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountAutomationItems)
            )),
            Some(f) => f(env),
        }
    }
    pub unsafe fn CountEnvelopePoints(
        &self,
        envelope: *mut root::TrackEnvelope,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountEnvelopePoints {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountEnvelopePoints)
            )),
            Some(f) => f(envelope),
        }
    }
    pub unsafe fn CountEnvelopePointsEx(
        &self,
        envelope: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountEnvelopePointsEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountEnvelopePointsEx)
            )),
            Some(f) => f(envelope, autoitem_idx),
        }
    }
    pub unsafe fn CountMediaItems(&self, proj: *mut root::ReaProject) -> ::std::os::raw::c_int {
        match self.pointers.CountMediaItems {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountMediaItems)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn CountProjectMarkers(
        &self,
        proj: *mut root::ReaProject,
        num_markersOut: *mut ::std::os::raw::c_int,
        num_regionsOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountProjectMarkers {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountProjectMarkers)
            )),
            Some(f) => f(proj, num_markersOut, num_regionsOut),
        }
    }
    pub unsafe fn CountSelectedMediaItems(
        &self,
        proj: *mut root::ReaProject,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountSelectedMediaItems {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountSelectedMediaItems)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn CountSelectedTracks(&self, proj: *mut root::ReaProject) -> ::std::os::raw::c_int {
        match self.pointers.CountSelectedTracks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountSelectedTracks)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn CountSelectedTracks2(
        &self,
        proj: *mut root::ReaProject,
        wantmaster: bool,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountSelectedTracks2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountSelectedTracks2)
            )),
            Some(f) => f(proj, wantmaster),
        }
    }
    pub unsafe fn CountTakeEnvelopes(
        &self,
        take: *mut root::MediaItem_Take,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountTakeEnvelopes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountTakeEnvelopes)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn CountTakes(&self, item: *mut root::MediaItem) -> ::std::os::raw::c_int {
        match self.pointers.CountTakes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountTakes)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn CountTCPFXParms(
        &self,
        project: *mut root::ReaProject,
        track: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountTCPFXParms {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountTCPFXParms)
            )),
            Some(f) => f(project, track),
        }
    }
    pub unsafe fn CountTempoTimeSigMarkers(
        &self,
        proj: *mut root::ReaProject,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountTempoTimeSigMarkers {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountTempoTimeSigMarkers)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn CountTrackEnvelopes(
        &self,
        track: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountTrackEnvelopes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountTrackEnvelopes)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn CountTrackMediaItems(
        &self,
        track: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CountTrackMediaItems {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountTrackMediaItems)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn CountTracks(&self, proj: *mut root::ReaProject) -> ::std::os::raw::c_int {
        match self.pointers.CountTracks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CountTracks)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn CreateLocalOscHandler(
        &self,
        obj: *mut ::std::os::raw::c_void,
        callback: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.CreateLocalOscHandler {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CreateLocalOscHandler)
            )),
            Some(f) => f(obj, callback),
        }
    }
    pub fn CreateMIDIInput(&self, dev: ::std::os::raw::c_int) -> *mut root::midi_Input {
        match self.pointers.CreateMIDIInput {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CreateMIDIInput)
            )),
            Some(f) => f(dev),
        }
    }
    pub unsafe fn CreateMIDIOutput(
        &self,
        dev: ::std::os::raw::c_int,
        streamMode: bool,
        msoffset100: *mut ::std::os::raw::c_int,
    ) -> *mut root::midi_Output {
        match self.pointers.CreateMIDIOutput {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CreateMIDIOutput)
            )),
            Some(f) => f(dev, streamMode, msoffset100),
        }
    }
    pub unsafe fn CreateNewMIDIItemInProj(
        &self,
        track: *mut root::MediaTrack,
        starttime: f64,
        endtime: f64,
        qnInOptional: *const bool,
    ) -> *mut root::MediaItem {
        match self.pointers.CreateNewMIDIItemInProj {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CreateNewMIDIItemInProj)
            )),
            Some(f) => f(track, starttime, endtime, qnInOptional),
        }
    }
    pub unsafe fn CreateTakeAudioAccessor(
        &self,
        take: *mut root::MediaItem_Take,
    ) -> *mut root::reaper_functions::AudioAccessor {
        match self.pointers.CreateTakeAudioAccessor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CreateTakeAudioAccessor)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn CreateTrackAudioAccessor(
        &self,
        track: *mut root::MediaTrack,
    ) -> *mut root::reaper_functions::AudioAccessor {
        match self.pointers.CreateTrackAudioAccessor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CreateTrackAudioAccessor)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn CreateTrackSend(
        &self,
        tr: *mut root::MediaTrack,
        desttrInOptional: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CreateTrackSend {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CreateTrackSend)
            )),
            Some(f) => f(tr, desttrInOptional),
        }
    }
    pub fn CSurf_FlushUndo(&self, force: bool) {
        match self.pointers.CSurf_FlushUndo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_FlushUndo)
            )),
            Some(f) => f(force),
        }
    }
    pub unsafe fn CSurf_GetTouchState(
        &self,
        trackid: *mut root::MediaTrack,
        isPan: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.CSurf_GetTouchState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_GetTouchState)
            )),
            Some(f) => f(trackid, isPan),
        }
    }
    pub fn CSurf_GoEnd(&self) {
        match self.pointers.CSurf_GoEnd {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_GoEnd)
            )),
            Some(f) => f(),
        }
    }
    pub fn CSurf_GoStart(&self) {
        match self.pointers.CSurf_GoStart {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_GoStart)
            )),
            Some(f) => f(),
        }
    }
    pub fn CSurf_NumTracks(&self, mcpView: bool) -> ::std::os::raw::c_int {
        match self.pointers.CSurf_NumTracks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_NumTracks)
            )),
            Some(f) => f(mcpView),
        }
    }
    pub fn CSurf_OnArrow(&self, whichdir: ::std::os::raw::c_int, wantzoom: bool) {
        match self.pointers.CSurf_OnArrow {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnArrow)
            )),
            Some(f) => f(whichdir, wantzoom),
        }
    }
    pub fn CSurf_OnFwd(&self, seekplay: ::std::os::raw::c_int) {
        match self.pointers.CSurf_OnFwd {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnFwd)
            )),
            Some(f) => f(seekplay),
        }
    }
    pub unsafe fn CSurf_OnFXChange(
        &self,
        trackid: *mut root::MediaTrack,
        en: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.CSurf_OnFXChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnFXChange)
            )),
            Some(f) => f(trackid, en),
        }
    }
    pub unsafe fn CSurf_OnInputMonitorChange(
        &self,
        trackid: *mut root::MediaTrack,
        monitor: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CSurf_OnInputMonitorChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnInputMonitorChange)
            )),
            Some(f) => f(trackid, monitor),
        }
    }
    pub unsafe fn CSurf_OnInputMonitorChangeEx(
        &self,
        trackid: *mut root::MediaTrack,
        monitor: ::std::os::raw::c_int,
        allowgang: bool,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CSurf_OnInputMonitorChangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnInputMonitorChangeEx)
            )),
            Some(f) => f(trackid, monitor, allowgang),
        }
    }
    pub unsafe fn CSurf_OnMuteChange(
        &self,
        trackid: *mut root::MediaTrack,
        mute: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.CSurf_OnMuteChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnMuteChange)
            )),
            Some(f) => f(trackid, mute),
        }
    }
    pub unsafe fn CSurf_OnMuteChangeEx(
        &self,
        trackid: *mut root::MediaTrack,
        mute: ::std::os::raw::c_int,
        allowgang: bool,
    ) -> bool {
        match self.pointers.CSurf_OnMuteChangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnMuteChangeEx)
            )),
            Some(f) => f(trackid, mute, allowgang),
        }
    }
    pub unsafe fn CSurf_OnOscControlMessage(
        &self,
        msg: *const ::std::os::raw::c_char,
        arg: *const f32,
    ) {
        match self.pointers.CSurf_OnOscControlMessage {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnOscControlMessage)
            )),
            Some(f) => f(msg, arg),
        }
    }
    pub unsafe fn CSurf_OnPanChange(
        &self,
        trackid: *mut root::MediaTrack,
        pan: f64,
        relative: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnPanChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnPanChange)
            )),
            Some(f) => f(trackid, pan, relative),
        }
    }
    pub unsafe fn CSurf_OnPanChangeEx(
        &self,
        trackid: *mut root::MediaTrack,
        pan: f64,
        relative: bool,
        allowGang: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnPanChangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnPanChangeEx)
            )),
            Some(f) => f(trackid, pan, relative, allowGang),
        }
    }
    pub fn CSurf_OnPause(&self) {
        match self.pointers.CSurf_OnPause {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnPause)
            )),
            Some(f) => f(),
        }
    }
    pub fn CSurf_OnPlay(&self) {
        match self.pointers.CSurf_OnPlay {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnPlay)
            )),
            Some(f) => f(),
        }
    }
    pub fn CSurf_OnPlayRateChange(&self, playrate: f64) {
        match self.pointers.CSurf_OnPlayRateChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnPlayRateChange)
            )),
            Some(f) => f(playrate),
        }
    }
    pub unsafe fn CSurf_OnRecArmChange(
        &self,
        trackid: *mut root::MediaTrack,
        recarm: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.CSurf_OnRecArmChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnRecArmChange)
            )),
            Some(f) => f(trackid, recarm),
        }
    }
    pub unsafe fn CSurf_OnRecArmChangeEx(
        &self,
        trackid: *mut root::MediaTrack,
        recarm: ::std::os::raw::c_int,
        allowgang: bool,
    ) -> bool {
        match self.pointers.CSurf_OnRecArmChangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnRecArmChangeEx)
            )),
            Some(f) => f(trackid, recarm, allowgang),
        }
    }
    pub fn CSurf_OnRecord(&self) {
        match self.pointers.CSurf_OnRecord {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnRecord)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn CSurf_OnRecvPanChange(
        &self,
        trackid: *mut root::MediaTrack,
        recv_index: ::std::os::raw::c_int,
        pan: f64,
        relative: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnRecvPanChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnRecvPanChange)
            )),
            Some(f) => f(trackid, recv_index, pan, relative),
        }
    }
    pub unsafe fn CSurf_OnRecvVolumeChange(
        &self,
        trackid: *mut root::MediaTrack,
        recv_index: ::std::os::raw::c_int,
        volume: f64,
        relative: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnRecvVolumeChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnRecvVolumeChange)
            )),
            Some(f) => f(trackid, recv_index, volume, relative),
        }
    }
    pub fn CSurf_OnRew(&self, seekplay: ::std::os::raw::c_int) {
        match self.pointers.CSurf_OnRew {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnRew)
            )),
            Some(f) => f(seekplay),
        }
    }
    pub fn CSurf_OnRewFwd(&self, seekplay: ::std::os::raw::c_int, dir: ::std::os::raw::c_int) {
        match self.pointers.CSurf_OnRewFwd {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnRewFwd)
            )),
            Some(f) => f(seekplay, dir),
        }
    }
    pub fn CSurf_OnScroll(&self, xdir: ::std::os::raw::c_int, ydir: ::std::os::raw::c_int) {
        match self.pointers.CSurf_OnScroll {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnScroll)
            )),
            Some(f) => f(xdir, ydir),
        }
    }
    pub unsafe fn CSurf_OnSelectedChange(
        &self,
        trackid: *mut root::MediaTrack,
        selected: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.CSurf_OnSelectedChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnSelectedChange)
            )),
            Some(f) => f(trackid, selected),
        }
    }
    pub unsafe fn CSurf_OnSendPanChange(
        &self,
        trackid: *mut root::MediaTrack,
        send_index: ::std::os::raw::c_int,
        pan: f64,
        relative: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnSendPanChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnSendPanChange)
            )),
            Some(f) => f(trackid, send_index, pan, relative),
        }
    }
    pub unsafe fn CSurf_OnSendVolumeChange(
        &self,
        trackid: *mut root::MediaTrack,
        send_index: ::std::os::raw::c_int,
        volume: f64,
        relative: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnSendVolumeChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnSendVolumeChange)
            )),
            Some(f) => f(trackid, send_index, volume, relative),
        }
    }
    pub unsafe fn CSurf_OnSoloChange(
        &self,
        trackid: *mut root::MediaTrack,
        solo: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.CSurf_OnSoloChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnSoloChange)
            )),
            Some(f) => f(trackid, solo),
        }
    }
    pub unsafe fn CSurf_OnSoloChangeEx(
        &self,
        trackid: *mut root::MediaTrack,
        solo: ::std::os::raw::c_int,
        allowgang: bool,
    ) -> bool {
        match self.pointers.CSurf_OnSoloChangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnSoloChangeEx)
            )),
            Some(f) => f(trackid, solo, allowgang),
        }
    }
    pub fn CSurf_OnStop(&self) {
        match self.pointers.CSurf_OnStop {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnStop)
            )),
            Some(f) => f(),
        }
    }
    pub fn CSurf_OnTempoChange(&self, bpm: f64) {
        match self.pointers.CSurf_OnTempoChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnTempoChange)
            )),
            Some(f) => f(bpm),
        }
    }
    pub unsafe fn CSurf_OnTrackSelection(&self, trackid: *mut root::MediaTrack) {
        match self.pointers.CSurf_OnTrackSelection {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnTrackSelection)
            )),
            Some(f) => f(trackid),
        }
    }
    pub unsafe fn CSurf_OnVolumeChange(
        &self,
        trackid: *mut root::MediaTrack,
        volume: f64,
        relative: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnVolumeChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnVolumeChange)
            )),
            Some(f) => f(trackid, volume, relative),
        }
    }
    pub unsafe fn CSurf_OnVolumeChangeEx(
        &self,
        trackid: *mut root::MediaTrack,
        volume: f64,
        relative: bool,
        allowGang: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnVolumeChangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnVolumeChangeEx)
            )),
            Some(f) => f(trackid, volume, relative, allowGang),
        }
    }
    pub unsafe fn CSurf_OnWidthChange(
        &self,
        trackid: *mut root::MediaTrack,
        width: f64,
        relative: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnWidthChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnWidthChange)
            )),
            Some(f) => f(trackid, width, relative),
        }
    }
    pub unsafe fn CSurf_OnWidthChangeEx(
        &self,
        trackid: *mut root::MediaTrack,
        width: f64,
        relative: bool,
        allowGang: bool,
    ) -> f64 {
        match self.pointers.CSurf_OnWidthChangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnWidthChangeEx)
            )),
            Some(f) => f(trackid, width, relative, allowGang),
        }
    }
    pub fn CSurf_OnZoom(&self, xdir: ::std::os::raw::c_int, ydir: ::std::os::raw::c_int) {
        match self.pointers.CSurf_OnZoom {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_OnZoom)
            )),
            Some(f) => f(xdir, ydir),
        }
    }
    pub fn CSurf_ResetAllCachedVolPanStates(&self) {
        match self.pointers.CSurf_ResetAllCachedVolPanStates {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_ResetAllCachedVolPanStates)
            )),
            Some(f) => f(),
        }
    }
    pub fn CSurf_ScrubAmt(&self, amt: f64) {
        match self.pointers.CSurf_ScrubAmt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_ScrubAmt)
            )),
            Some(f) => f(amt),
        }
    }
    pub unsafe fn CSurf_SetAutoMode(
        &self,
        mode: ::std::os::raw::c_int,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetAutoMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetAutoMode)
            )),
            Some(f) => f(mode, ignoresurf),
        }
    }
    pub unsafe fn CSurf_SetPlayState(
        &self,
        play: bool,
        pause: bool,
        rec: bool,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetPlayState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetPlayState)
            )),
            Some(f) => f(play, pause, rec, ignoresurf),
        }
    }
    pub unsafe fn CSurf_SetRepeatState(
        &self,
        rep: bool,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetRepeatState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetRepeatState)
            )),
            Some(f) => f(rep, ignoresurf),
        }
    }
    pub unsafe fn CSurf_SetSurfaceMute(
        &self,
        trackid: *mut root::MediaTrack,
        mute: bool,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetSurfaceMute {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetSurfaceMute)
            )),
            Some(f) => f(trackid, mute, ignoresurf),
        }
    }
    pub unsafe fn CSurf_SetSurfacePan(
        &self,
        trackid: *mut root::MediaTrack,
        pan: f64,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetSurfacePan {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetSurfacePan)
            )),
            Some(f) => f(trackid, pan, ignoresurf),
        }
    }
    pub unsafe fn CSurf_SetSurfaceRecArm(
        &self,
        trackid: *mut root::MediaTrack,
        recarm: bool,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetSurfaceRecArm {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetSurfaceRecArm)
            )),
            Some(f) => f(trackid, recarm, ignoresurf),
        }
    }
    pub unsafe fn CSurf_SetSurfaceSelected(
        &self,
        trackid: *mut root::MediaTrack,
        selected: bool,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetSurfaceSelected {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetSurfaceSelected)
            )),
            Some(f) => f(trackid, selected, ignoresurf),
        }
    }
    pub unsafe fn CSurf_SetSurfaceSolo(
        &self,
        trackid: *mut root::MediaTrack,
        solo: bool,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetSurfaceSolo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetSurfaceSolo)
            )),
            Some(f) => f(trackid, solo, ignoresurf),
        }
    }
    pub unsafe fn CSurf_SetSurfaceVolume(
        &self,
        trackid: *mut root::MediaTrack,
        volume: f64,
        ignoresurf: *mut root::IReaperControlSurface,
    ) {
        match self.pointers.CSurf_SetSurfaceVolume {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetSurfaceVolume)
            )),
            Some(f) => f(trackid, volume, ignoresurf),
        }
    }
    pub fn CSurf_SetTrackListChange(&self) {
        match self.pointers.CSurf_SetTrackListChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_SetTrackListChange)
            )),
            Some(f) => f(),
        }
    }
    pub fn CSurf_TrackFromID(
        &self,
        idx: ::std::os::raw::c_int,
        mcpView: bool,
    ) -> *mut root::MediaTrack {
        match self.pointers.CSurf_TrackFromID {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_TrackFromID)
            )),
            Some(f) => f(idx, mcpView),
        }
    }
    pub unsafe fn CSurf_TrackToID(
        &self,
        track: *mut root::MediaTrack,
        mcpView: bool,
    ) -> ::std::os::raw::c_int {
        match self.pointers.CSurf_TrackToID {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(CSurf_TrackToID)
            )),
            Some(f) => f(track, mcpView),
        }
    }
    pub fn DB2SLIDER(&self, x: f64) -> f64 {
        match self.pointers.DB2SLIDER {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DB2SLIDER)
            )),
            Some(f) => f(x),
        }
    }
    pub unsafe fn DeleteActionShortcut(
        &self,
        section: *mut root::KbdSectionInfo,
        cmdID: ::std::os::raw::c_int,
        shortcutidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.DeleteActionShortcut {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteActionShortcut)
            )),
            Some(f) => f(section, cmdID, shortcutidx),
        }
    }
    pub unsafe fn DeleteEnvelopePointEx(
        &self,
        envelope: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
        ptidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.DeleteEnvelopePointEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteEnvelopePointEx)
            )),
            Some(f) => f(envelope, autoitem_idx, ptidx),
        }
    }
    pub unsafe fn DeleteEnvelopePointRange(
        &self,
        envelope: *mut root::TrackEnvelope,
        time_start: f64,
        time_end: f64,
    ) -> bool {
        match self.pointers.DeleteEnvelopePointRange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteEnvelopePointRange)
            )),
            Some(f) => f(envelope, time_start, time_end),
        }
    }
    pub unsafe fn DeleteEnvelopePointRangeEx(
        &self,
        envelope: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
        time_start: f64,
        time_end: f64,
    ) -> bool {
        match self.pointers.DeleteEnvelopePointRangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteEnvelopePointRangeEx)
            )),
            Some(f) => f(envelope, autoitem_idx, time_start, time_end),
        }
    }
    pub unsafe fn DeleteExtState(
        &self,
        section: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_char,
        persist: bool,
    ) {
        match self.pointers.DeleteExtState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteExtState)
            )),
            Some(f) => f(section, key, persist),
        }
    }
    pub unsafe fn DeleteProjectMarker(
        &self,
        proj: *mut root::ReaProject,
        markrgnindexnumber: ::std::os::raw::c_int,
        isrgn: bool,
    ) -> bool {
        match self.pointers.DeleteProjectMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteProjectMarker)
            )),
            Some(f) => f(proj, markrgnindexnumber, isrgn),
        }
    }
    pub unsafe fn DeleteProjectMarkerByIndex(
        &self,
        proj: *mut root::ReaProject,
        markrgnidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.DeleteProjectMarkerByIndex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteProjectMarkerByIndex)
            )),
            Some(f) => f(proj, markrgnidx),
        }
    }
    pub unsafe fn DeleteTakeStretchMarkers(
        &self,
        take: *mut root::MediaItem_Take,
        idx: ::std::os::raw::c_int,
        countInOptional: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.DeleteTakeStretchMarkers {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteTakeStretchMarkers)
            )),
            Some(f) => f(take, idx, countInOptional),
        }
    }
    pub unsafe fn DeleteTempoTimeSigMarker(
        &self,
        project: *mut root::ReaProject,
        markerindex: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.DeleteTempoTimeSigMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteTempoTimeSigMarker)
            )),
            Some(f) => f(project, markerindex),
        }
    }
    pub unsafe fn DeleteTrack(&self, tr: *mut root::MediaTrack) {
        match self.pointers.DeleteTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteTrack)
            )),
            Some(f) => f(tr),
        }
    }
    pub unsafe fn DeleteTrackMediaItem(
        &self,
        tr: *mut root::MediaTrack,
        it: *mut root::MediaItem,
    ) -> bool {
        match self.pointers.DeleteTrackMediaItem {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DeleteTrackMediaItem)
            )),
            Some(f) => f(tr, it),
        }
    }
    pub unsafe fn DestroyAudioAccessor(
        &self,
        accessor: *mut root::reaper_functions::AudioAccessor,
    ) {
        match self.pointers.DestroyAudioAccessor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DestroyAudioAccessor)
            )),
            Some(f) => f(accessor),
        }
    }
    pub unsafe fn DestroyLocalOscHandler(&self, local_osc_handler: *mut ::std::os::raw::c_void) {
        match self.pointers.DestroyLocalOscHandler {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DestroyLocalOscHandler)
            )),
            Some(f) => f(local_osc_handler),
        }
    }
    pub unsafe fn DoActionShortcutDialog(
        &self,
        hwnd: root::HWND,
        section: *mut root::KbdSectionInfo,
        cmdID: ::std::os::raw::c_int,
        shortcutidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.DoActionShortcutDialog {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DoActionShortcutDialog)
            )),
            Some(f) => f(hwnd, section, cmdID, shortcutidx),
        }
    }
    pub unsafe fn Dock_UpdateDockID(
        &self,
        ident_str: *const ::std::os::raw::c_char,
        whichDock: ::std::os::raw::c_int,
    ) {
        match self.pointers.Dock_UpdateDockID {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Dock_UpdateDockID)
            )),
            Some(f) => f(ident_str, whichDock),
        }
    }
    pub fn DockGetPosition(&self, whichDock: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        match self.pointers.DockGetPosition {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DockGetPosition)
            )),
            Some(f) => f(whichDock),
        }
    }
    pub unsafe fn DockIsChildOfDock(
        &self,
        hwnd: root::HWND,
        isFloatingDockerOut: *mut bool,
    ) -> ::std::os::raw::c_int {
        match self.pointers.DockIsChildOfDock {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DockIsChildOfDock)
            )),
            Some(f) => f(hwnd, isFloatingDockerOut),
        }
    }
    pub fn DockWindowActivate(&self, hwnd: root::HWND) {
        match self.pointers.DockWindowActivate {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DockWindowActivate)
            )),
            Some(f) => f(hwnd),
        }
    }
    pub unsafe fn DockWindowAdd(
        &self,
        hwnd: root::HWND,
        name: *const ::std::os::raw::c_char,
        pos: ::std::os::raw::c_int,
        allowShow: bool,
    ) {
        match self.pointers.DockWindowAdd {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DockWindowAdd)
            )),
            Some(f) => f(hwnd, name, pos, allowShow),
        }
    }
    pub unsafe fn DockWindowAddEx(
        &self,
        hwnd: root::HWND,
        name: *const ::std::os::raw::c_char,
        identstr: *const ::std::os::raw::c_char,
        allowShow: bool,
    ) {
        match self.pointers.DockWindowAddEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DockWindowAddEx)
            )),
            Some(f) => f(hwnd, name, identstr, allowShow),
        }
    }
    pub fn DockWindowRefresh(&self) {
        match self.pointers.DockWindowRefresh {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DockWindowRefresh)
            )),
            Some(f) => f(),
        }
    }
    pub fn DockWindowRefreshForHWND(&self, hwnd: root::HWND) {
        match self.pointers.DockWindowRefreshForHWND {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DockWindowRefreshForHWND)
            )),
            Some(f) => f(hwnd),
        }
    }
    pub fn DockWindowRemove(&self, hwnd: root::HWND) {
        match self.pointers.DockWindowRemove {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DockWindowRemove)
            )),
            Some(f) => f(hwnd),
        }
    }
    pub unsafe fn DuplicateCustomizableMenu(
        &self,
        srcmenu: *mut ::std::os::raw::c_void,
        destmenu: *mut ::std::os::raw::c_void,
    ) -> bool {
        match self.pointers.DuplicateCustomizableMenu {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(DuplicateCustomizableMenu)
            )),
            Some(f) => f(srcmenu, destmenu),
        }
    }
    pub unsafe fn EditTempoTimeSigMarker(
        &self,
        project: *mut root::ReaProject,
        markerindex: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.EditTempoTimeSigMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EditTempoTimeSigMarker)
            )),
            Some(f) => f(project, markerindex),
        }
    }
    pub unsafe fn EnsureNotCompletelyOffscreen(&self, rInOut: *mut root::RECT) {
        match self.pointers.EnsureNotCompletelyOffscreen {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnsureNotCompletelyOffscreen)
            )),
            Some(f) => f(rInOut),
        }
    }
    pub unsafe fn EnumerateFiles(
        &self,
        path: *const ::std::os::raw::c_char,
        fileindex: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.EnumerateFiles {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumerateFiles)
            )),
            Some(f) => f(path, fileindex),
        }
    }
    pub unsafe fn EnumerateSubdirectories(
        &self,
        path: *const ::std::os::raw::c_char,
        subdirindex: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.EnumerateSubdirectories {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumerateSubdirectories)
            )),
            Some(f) => f(path, subdirindex),
        }
    }
    pub unsafe fn EnumPitchShiftModes(
        &self,
        mode: ::std::os::raw::c_int,
        strOut: *mut *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.EnumPitchShiftModes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumPitchShiftModes)
            )),
            Some(f) => f(mode, strOut),
        }
    }
    pub fn EnumPitchShiftSubModes(
        &self,
        mode: ::std::os::raw::c_int,
        submode: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.EnumPitchShiftSubModes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumPitchShiftSubModes)
            )),
            Some(f) => f(mode, submode),
        }
    }
    pub unsafe fn EnumProjectMarkers(
        &self,
        idx: ::std::os::raw::c_int,
        isrgnOut: *mut bool,
        posOut: *mut f64,
        rgnendOut: *mut f64,
        nameOut: *mut *const ::std::os::raw::c_char,
        markrgnindexnumberOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.EnumProjectMarkers {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumProjectMarkers)
            )),
            Some(f) => f(
                idx,
                isrgnOut,
                posOut,
                rgnendOut,
                nameOut,
                markrgnindexnumberOut,
            ),
        }
    }
    pub unsafe fn EnumProjectMarkers2(
        &self,
        proj: *mut root::ReaProject,
        idx: ::std::os::raw::c_int,
        isrgnOut: *mut bool,
        posOut: *mut f64,
        rgnendOut: *mut f64,
        nameOut: *mut *const ::std::os::raw::c_char,
        markrgnindexnumberOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.EnumProjectMarkers2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumProjectMarkers2)
            )),
            Some(f) => f(
                proj,
                idx,
                isrgnOut,
                posOut,
                rgnendOut,
                nameOut,
                markrgnindexnumberOut,
            ),
        }
    }
    pub unsafe fn EnumProjectMarkers3(
        &self,
        proj: *mut root::ReaProject,
        idx: ::std::os::raw::c_int,
        isrgnOut: *mut bool,
        posOut: *mut f64,
        rgnendOut: *mut f64,
        nameOut: *mut *const ::std::os::raw::c_char,
        markrgnindexnumberOut: *mut ::std::os::raw::c_int,
        colorOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.EnumProjectMarkers3 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumProjectMarkers3)
            )),
            Some(f) => f(
                proj,
                idx,
                isrgnOut,
                posOut,
                rgnendOut,
                nameOut,
                markrgnindexnumberOut,
                colorOut,
            ),
        }
    }
    pub unsafe fn EnumProjects(
        &self,
        idx: ::std::os::raw::c_int,
        projfnOutOptional: *mut ::std::os::raw::c_char,
        projfnOutOptional_sz: ::std::os::raw::c_int,
    ) -> *mut root::ReaProject {
        match self.pointers.EnumProjects {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumProjects)
            )),
            Some(f) => f(idx, projfnOutOptional, projfnOutOptional_sz),
        }
    }
    pub unsafe fn EnumProjExtState(
        &self,
        proj: *mut root::ReaProject,
        extname: *const ::std::os::raw::c_char,
        idx: ::std::os::raw::c_int,
        keyOutOptional: *mut ::std::os::raw::c_char,
        keyOutOptional_sz: ::std::os::raw::c_int,
        valOutOptional: *mut ::std::os::raw::c_char,
        valOutOptional_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.EnumProjExtState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumProjExtState)
            )),
            Some(f) => f(
                proj,
                extname,
                idx,
                keyOutOptional,
                keyOutOptional_sz,
                valOutOptional,
                valOutOptional_sz,
            ),
        }
    }
    pub unsafe fn EnumRegionRenderMatrix(
        &self,
        proj: *mut root::ReaProject,
        regionindex: ::std::os::raw::c_int,
        rendertrack: ::std::os::raw::c_int,
    ) -> *mut root::MediaTrack {
        match self.pointers.EnumRegionRenderMatrix {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumRegionRenderMatrix)
            )),
            Some(f) => f(proj, regionindex, rendertrack),
        }
    }
    pub unsafe fn EnumTrackMIDIProgramNames(
        &self,
        track: ::std::os::raw::c_int,
        programNumber: ::std::os::raw::c_int,
        programName: *mut ::std::os::raw::c_char,
        programName_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.EnumTrackMIDIProgramNames {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumTrackMIDIProgramNames)
            )),
            Some(f) => f(track, programNumber, programName, programName_sz),
        }
    }
    pub unsafe fn EnumTrackMIDIProgramNamesEx(
        &self,
        proj: *mut root::ReaProject,
        track: *mut root::MediaTrack,
        programNumber: ::std::os::raw::c_int,
        programName: *mut ::std::os::raw::c_char,
        programName_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.EnumTrackMIDIProgramNamesEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(EnumTrackMIDIProgramNamesEx)
            )),
            Some(f) => f(proj, track, programNumber, programName, programName_sz),
        }
    }
    pub unsafe fn Envelope_Evaluate(
        &self,
        envelope: *mut root::TrackEnvelope,
        time: f64,
        samplerate: f64,
        samplesRequested: ::std::os::raw::c_int,
        valueOutOptional: *mut f64,
        dVdSOutOptional: *mut f64,
        ddVdSOutOptional: *mut f64,
        dddVdSOutOptional: *mut f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.Envelope_Evaluate {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Envelope_Evaluate)
            )),
            Some(f) => f(
                envelope,
                time,
                samplerate,
                samplesRequested,
                valueOutOptional,
                dVdSOutOptional,
                ddVdSOutOptional,
                dddVdSOutOptional,
            ),
        }
    }
    pub unsafe fn Envelope_FormatValue(
        &self,
        env: *mut root::TrackEnvelope,
        value: f64,
        bufOut: *mut ::std::os::raw::c_char,
        bufOut_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.Envelope_FormatValue {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Envelope_FormatValue)
            )),
            Some(f) => f(env, value, bufOut, bufOut_sz),
        }
    }
    pub unsafe fn Envelope_GetParentTake(
        &self,
        env: *mut root::TrackEnvelope,
        indexOutOptional: *mut ::std::os::raw::c_int,
        index2OutOptional: *mut ::std::os::raw::c_int,
    ) -> *mut root::MediaItem_Take {
        match self.pointers.Envelope_GetParentTake {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Envelope_GetParentTake)
            )),
            Some(f) => f(env, indexOutOptional, index2OutOptional),
        }
    }
    pub unsafe fn Envelope_GetParentTrack(
        &self,
        env: *mut root::TrackEnvelope,
        indexOutOptional: *mut ::std::os::raw::c_int,
        index2OutOptional: *mut ::std::os::raw::c_int,
    ) -> *mut root::MediaTrack {
        match self.pointers.Envelope_GetParentTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Envelope_GetParentTrack)
            )),
            Some(f) => f(env, indexOutOptional, index2OutOptional),
        }
    }
    pub unsafe fn Envelope_SortPoints(&self, envelope: *mut root::TrackEnvelope) -> bool {
        match self.pointers.Envelope_SortPoints {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Envelope_SortPoints)
            )),
            Some(f) => f(envelope),
        }
    }
    pub unsafe fn Envelope_SortPointsEx(
        &self,
        envelope: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.Envelope_SortPointsEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Envelope_SortPointsEx)
            )),
            Some(f) => f(envelope, autoitem_idx),
        }
    }
    pub unsafe fn ExecProcess(
        &self,
        cmdline: *const ::std::os::raw::c_char,
        timeoutmsec: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.ExecProcess {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ExecProcess)
            )),
            Some(f) => f(cmdline, timeoutmsec),
        }
    }
    pub unsafe fn file_exists(&self, path: *const ::std::os::raw::c_char) -> bool {
        match self.pointers.file_exists {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(file_exists)
            )),
            Some(f) => f(path),
        }
    }
    pub unsafe fn FindTempoTimeSigMarker(
        &self,
        project: *mut root::ReaProject,
        time: f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.FindTempoTimeSigMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(FindTempoTimeSigMarker)
            )),
            Some(f) => f(project, time),
        }
    }
    pub unsafe fn format_timestr(
        &self,
        tpos: f64,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.format_timestr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(format_timestr)
            )),
            Some(f) => f(tpos, buf, buf_sz),
        }
    }
    pub unsafe fn format_timestr_len(
        &self,
        tpos: f64,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
        offset: f64,
        modeoverride: ::std::os::raw::c_int,
    ) {
        match self.pointers.format_timestr_len {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(format_timestr_len)
            )),
            Some(f) => f(tpos, buf, buf_sz, offset, modeoverride),
        }
    }
    pub unsafe fn format_timestr_pos(
        &self,
        tpos: f64,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
        modeoverride: ::std::os::raw::c_int,
    ) {
        match self.pointers.format_timestr_pos {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(format_timestr_pos)
            )),
            Some(f) => f(tpos, buf, buf_sz, modeoverride),
        }
    }
    pub unsafe fn FreeHeapPtr(&self, ptr: *mut ::std::os::raw::c_void) {
        match self.pointers.FreeHeapPtr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(FreeHeapPtr)
            )),
            Some(f) => f(ptr),
        }
    }
    pub unsafe fn genGuid(&self, g: *mut root::GUID) {
        match self.pointers.genGuid {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(genGuid)
            )),
            Some(f) => f(g),
        }
    }
    pub unsafe fn get_config_var(
        &self,
        name: *const ::std::os::raw::c_char,
        szOut: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.get_config_var {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(get_config_var)
            )),
            Some(f) => f(name, szOut),
        }
    }
    pub unsafe fn get_config_var_string(
        &self,
        name: *const ::std::os::raw::c_char,
        bufOut: *mut ::std::os::raw::c_char,
        bufOut_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.get_config_var_string {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(get_config_var_string)
            )),
            Some(f) => f(name, bufOut, bufOut_sz),
        }
    }
    pub fn get_ini_file(&self) -> *const ::std::os::raw::c_char {
        match self.pointers.get_ini_file {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(get_ini_file)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn get_midi_config_var(
        &self,
        name: *const ::std::os::raw::c_char,
        szOut: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.get_midi_config_var {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(get_midi_config_var)
            )),
            Some(f) => f(name, szOut),
        }
    }
    pub unsafe fn GetActionShortcutDesc(
        &self,
        section: *mut root::KbdSectionInfo,
        cmdID: ::std::os::raw::c_int,
        shortcutidx: ::std::os::raw::c_int,
        desc: *mut ::std::os::raw::c_char,
        desclen: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetActionShortcutDesc {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetActionShortcutDesc)
            )),
            Some(f) => f(section, cmdID, shortcutidx, desc, desclen),
        }
    }
    pub unsafe fn GetActiveTake(&self, item: *mut root::MediaItem) -> *mut root::MediaItem_Take {
        match self.pointers.GetActiveTake {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetActiveTake)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn GetAllProjectPlayStates(
        &self,
        ignoreProject: *mut root::ReaProject,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetAllProjectPlayStates {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetAllProjectPlayStates)
            )),
            Some(f) => f(ignoreProject),
        }
    }
    pub fn GetAppVersion(&self) -> *const ::std::os::raw::c_char {
        match self.pointers.GetAppVersion {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetAppVersion)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetArmedCommand(
        &self,
        secOut: *mut ::std::os::raw::c_char,
        secOut_sz: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetArmedCommand {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetArmedCommand)
            )),
            Some(f) => f(secOut, secOut_sz),
        }
    }
    pub unsafe fn GetAudioAccessorEndTime(
        &self,
        accessor: *mut root::reaper_functions::AudioAccessor,
    ) -> f64 {
        match self.pointers.GetAudioAccessorEndTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetAudioAccessorEndTime)
            )),
            Some(f) => f(accessor),
        }
    }
    pub unsafe fn GetAudioAccessorHash(
        &self,
        accessor: *mut root::reaper_functions::AudioAccessor,
        hashNeed128: *mut ::std::os::raw::c_char,
    ) {
        match self.pointers.GetAudioAccessorHash {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetAudioAccessorHash)
            )),
            Some(f) => f(accessor, hashNeed128),
        }
    }
    pub unsafe fn GetAudioAccessorSamples(
        &self,
        accessor: *mut root::reaper_functions::AudioAccessor,
        samplerate: ::std::os::raw::c_int,
        numchannels: ::std::os::raw::c_int,
        starttime_sec: f64,
        numsamplesperchannel: ::std::os::raw::c_int,
        samplebuffer: *mut f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetAudioAccessorSamples {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetAudioAccessorSamples)
            )),
            Some(f) => f(
                accessor,
                samplerate,
                numchannels,
                starttime_sec,
                numsamplesperchannel,
                samplebuffer,
            ),
        }
    }
    pub unsafe fn GetAudioAccessorStartTime(
        &self,
        accessor: *mut root::reaper_functions::AudioAccessor,
    ) -> f64 {
        match self.pointers.GetAudioAccessorStartTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetAudioAccessorStartTime)
            )),
            Some(f) => f(accessor),
        }
    }
    pub unsafe fn GetAudioDeviceInfo(
        &self,
        attribute: *const ::std::os::raw::c_char,
        desc: *mut ::std::os::raw::c_char,
        desc_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetAudioDeviceInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetAudioDeviceInfo)
            )),
            Some(f) => f(attribute, desc, desc_sz),
        }
    }
    pub fn GetColorTheme(
        &self,
        idx: ::std::os::raw::c_int,
        defval: ::std::os::raw::c_int,
    ) -> root::INT_PTR {
        match self.pointers.GetColorTheme {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetColorTheme)
            )),
            Some(f) => f(idx, defval),
        }
    }
    pub unsafe fn GetColorThemeStruct(
        &self,
        szOut: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetColorThemeStruct {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetColorThemeStruct)
            )),
            Some(f) => f(szOut),
        }
    }
    pub unsafe fn GetConfigWantsDock(
        &self,
        ident_str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetConfigWantsDock {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetConfigWantsDock)
            )),
            Some(f) => f(ident_str),
        }
    }
    pub fn GetContextMenu(&self, idx: ::std::os::raw::c_int) -> root::HMENU {
        match self.pointers.GetContextMenu {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetContextMenu)
            )),
            Some(f) => f(idx),
        }
    }
    pub fn GetCurrentProjectInLoadSave(&self) -> *mut root::ReaProject {
        match self.pointers.GetCurrentProjectInLoadSave {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetCurrentProjectInLoadSave)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetCursorContext(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetCursorContext {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetCursorContext)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetCursorContext2(&self, want_last_valid: bool) -> ::std::os::raw::c_int {
        match self.pointers.GetCursorContext2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetCursorContext2)
            )),
            Some(f) => f(want_last_valid),
        }
    }
    pub fn GetCursorPosition(&self) -> f64 {
        match self.pointers.GetCursorPosition {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetCursorPosition)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetCursorPositionEx(&self, proj: *mut root::ReaProject) -> f64 {
        match self.pointers.GetCursorPositionEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetCursorPositionEx)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn GetDisplayedMediaItemColor(
        &self,
        item: *mut root::MediaItem,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetDisplayedMediaItemColor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetDisplayedMediaItemColor)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn GetDisplayedMediaItemColor2(
        &self,
        item: *mut root::MediaItem,
        take: *mut root::MediaItem_Take,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetDisplayedMediaItemColor2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetDisplayedMediaItemColor2)
            )),
            Some(f) => f(item, take),
        }
    }
    pub unsafe fn GetEnvelopeInfo_Value(
        &self,
        tr: *mut root::TrackEnvelope,
        parmname: *const ::std::os::raw::c_char,
    ) -> f64 {
        match self.pointers.GetEnvelopeInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetEnvelopeInfo_Value)
            )),
            Some(f) => f(tr, parmname),
        }
    }
    pub unsafe fn GetEnvelopeName(
        &self,
        env: *mut root::TrackEnvelope,
        bufOut: *mut ::std::os::raw::c_char,
        bufOut_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetEnvelopeName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetEnvelopeName)
            )),
            Some(f) => f(env, bufOut, bufOut_sz),
        }
    }
    pub unsafe fn GetEnvelopePoint(
        &self,
        envelope: *mut root::TrackEnvelope,
        ptidx: ::std::os::raw::c_int,
        timeOutOptional: *mut f64,
        valueOutOptional: *mut f64,
        shapeOutOptional: *mut ::std::os::raw::c_int,
        tensionOutOptional: *mut f64,
        selectedOutOptional: *mut bool,
    ) -> bool {
        match self.pointers.GetEnvelopePoint {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetEnvelopePoint)
            )),
            Some(f) => f(
                envelope,
                ptidx,
                timeOutOptional,
                valueOutOptional,
                shapeOutOptional,
                tensionOutOptional,
                selectedOutOptional,
            ),
        }
    }
    pub unsafe fn GetEnvelopePointByTime(
        &self,
        envelope: *mut root::TrackEnvelope,
        time: f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetEnvelopePointByTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetEnvelopePointByTime)
            )),
            Some(f) => f(envelope, time),
        }
    }
    pub unsafe fn GetEnvelopePointByTimeEx(
        &self,
        envelope: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
        time: f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetEnvelopePointByTimeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetEnvelopePointByTimeEx)
            )),
            Some(f) => f(envelope, autoitem_idx, time),
        }
    }
    pub unsafe fn GetEnvelopePointEx(
        &self,
        envelope: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
        ptidx: ::std::os::raw::c_int,
        timeOutOptional: *mut f64,
        valueOutOptional: *mut f64,
        shapeOutOptional: *mut ::std::os::raw::c_int,
        tensionOutOptional: *mut f64,
        selectedOutOptional: *mut bool,
    ) -> bool {
        match self.pointers.GetEnvelopePointEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetEnvelopePointEx)
            )),
            Some(f) => f(
                envelope,
                autoitem_idx,
                ptidx,
                timeOutOptional,
                valueOutOptional,
                shapeOutOptional,
                tensionOutOptional,
                selectedOutOptional,
            ),
        }
    }
    pub unsafe fn GetEnvelopeScalingMode(
        &self,
        env: *mut root::TrackEnvelope,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetEnvelopeScalingMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetEnvelopeScalingMode)
            )),
            Some(f) => f(env),
        }
    }
    pub unsafe fn GetEnvelopeStateChunk(
        &self,
        env: *mut root::TrackEnvelope,
        strNeedBig: *mut ::std::os::raw::c_char,
        strNeedBig_sz: ::std::os::raw::c_int,
        isundoOptional: bool,
    ) -> bool {
        match self.pointers.GetEnvelopeStateChunk {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetEnvelopeStateChunk)
            )),
            Some(f) => f(env, strNeedBig, strNeedBig_sz, isundoOptional),
        }
    }
    pub fn GetExePath(&self) -> *const ::std::os::raw::c_char {
        match self.pointers.GetExePath {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetExePath)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetExtState(
        &self,
        section: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetExtState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetExtState)
            )),
            Some(f) => f(section, key),
        }
    }
    pub unsafe fn GetFocusedFX(
        &self,
        tracknumberOut: *mut ::std::os::raw::c_int,
        itemnumberOut: *mut ::std::os::raw::c_int,
        fxnumberOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetFocusedFX {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetFocusedFX)
            )),
            Some(f) => f(tracknumberOut, itemnumberOut, fxnumberOut),
        }
    }
    pub unsafe fn GetFreeDiskSpaceForRecordPath(
        &self,
        proj: *mut root::ReaProject,
        pathidx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetFreeDiskSpaceForRecordPath {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetFreeDiskSpaceForRecordPath)
            )),
            Some(f) => f(proj, pathidx),
        }
    }
    pub unsafe fn GetFXEnvelope(
        &self,
        track: *mut root::MediaTrack,
        fxindex: ::std::os::raw::c_int,
        parameterindex: ::std::os::raw::c_int,
        create: bool,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.GetFXEnvelope {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetFXEnvelope)
            )),
            Some(f) => f(track, fxindex, parameterindex, create),
        }
    }
    pub fn GetGlobalAutomationOverride(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetGlobalAutomationOverride {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetGlobalAutomationOverride)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetHZoomLevel(&self) -> f64 {
        match self.pointers.GetHZoomLevel {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetHZoomLevel)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetIconThemePointer(
        &self,
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetIconThemePointer {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetIconThemePointer)
            )),
            Some(f) => f(name),
        }
    }
    pub unsafe fn GetIconThemePointerForDPI(
        &self,
        name: *const ::std::os::raw::c_char,
        dpisc: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetIconThemePointerForDPI {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetIconThemePointerForDPI)
            )),
            Some(f) => f(name, dpisc),
        }
    }
    pub unsafe fn GetIconThemeStruct(
        &self,
        szOut: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetIconThemeStruct {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetIconThemeStruct)
            )),
            Some(f) => f(szOut),
        }
    }
    pub fn GetInputChannelName(
        &self,
        channelIndex: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetInputChannelName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetInputChannelName)
            )),
            Some(f) => f(channelIndex),
        }
    }
    pub unsafe fn GetInputOutputLatency(
        &self,
        inputlatencyOut: *mut ::std::os::raw::c_int,
        outputLatencyOut: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.GetInputOutputLatency {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetInputOutputLatency)
            )),
            Some(f) => f(inputlatencyOut, outputLatencyOut),
        }
    }
    pub unsafe fn GetItemEditingTime2(
        &self,
        which_itemOut: *mut *mut root::PCM_source,
        flagsOut: *mut ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.GetItemEditingTime2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetItemEditingTime2)
            )),
            Some(f) => f(which_itemOut, flagsOut),
        }
    }
    pub unsafe fn GetItemFromPoint(
        &self,
        screen_x: ::std::os::raw::c_int,
        screen_y: ::std::os::raw::c_int,
        allow_locked: bool,
        takeOutOptional: *mut *mut root::MediaItem_Take,
    ) -> *mut root::MediaItem {
        match self.pointers.GetItemFromPoint {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetItemFromPoint)
            )),
            Some(f) => f(screen_x, screen_y, allow_locked, takeOutOptional),
        }
    }
    pub unsafe fn GetItemProjectContext(
        &self,
        item: *mut root::MediaItem,
    ) -> *mut root::ReaProject {
        match self.pointers.GetItemProjectContext {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetItemProjectContext)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn GetItemStateChunk(
        &self,
        item: *mut root::MediaItem,
        strNeedBig: *mut ::std::os::raw::c_char,
        strNeedBig_sz: ::std::os::raw::c_int,
        isundoOptional: bool,
    ) -> bool {
        match self.pointers.GetItemStateChunk {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetItemStateChunk)
            )),
            Some(f) => f(item, strNeedBig, strNeedBig_sz, isundoOptional),
        }
    }
    pub fn GetLastColorThemeFile(&self) -> *const ::std::os::raw::c_char {
        match self.pointers.GetLastColorThemeFile {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetLastColorThemeFile)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetLastMarkerAndCurRegion(
        &self,
        proj: *mut root::ReaProject,
        time: f64,
        markeridxOut: *mut ::std::os::raw::c_int,
        regionidxOut: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.GetLastMarkerAndCurRegion {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetLastMarkerAndCurRegion)
            )),
            Some(f) => f(proj, time, markeridxOut, regionidxOut),
        }
    }
    pub unsafe fn GetLastTouchedFX(
        &self,
        tracknumberOut: *mut ::std::os::raw::c_int,
        fxnumberOut: *mut ::std::os::raw::c_int,
        paramnumberOut: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetLastTouchedFX {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetLastTouchedFX)
            )),
            Some(f) => f(tracknumberOut, fxnumberOut, paramnumberOut),
        }
    }
    pub fn GetLastTouchedTrack(&self) -> *mut root::MediaTrack {
        match self.pointers.GetLastTouchedTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetLastTouchedTrack)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetMainHwnd(&self) -> root::HWND {
        match self.pointers.GetMainHwnd {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMainHwnd)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetMasterMuteSoloFlags(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetMasterMuteSoloFlags {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMasterMuteSoloFlags)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetMasterTrack(&self, proj: *mut root::ReaProject) -> *mut root::MediaTrack {
        match self.pointers.GetMasterTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMasterTrack)
            )),
            Some(f) => f(proj),
        }
    }
    pub fn GetMasterTrackVisibility(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetMasterTrackVisibility {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMasterTrackVisibility)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetMaxMidiInputs(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetMaxMidiInputs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMaxMidiInputs)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetMaxMidiOutputs(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetMaxMidiOutputs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMaxMidiOutputs)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetMediaItem(
        &self,
        proj: *mut root::ReaProject,
        itemidx: ::std::os::raw::c_int,
    ) -> *mut root::MediaItem {
        match self.pointers.GetMediaItem {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItem)
            )),
            Some(f) => f(proj, itemidx),
        }
    }
    pub unsafe fn GetMediaItem_Track(&self, item: *mut root::MediaItem) -> *mut root::MediaTrack {
        match self.pointers.GetMediaItem_Track {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItem_Track)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn GetMediaItemInfo_Value(
        &self,
        item: *mut root::MediaItem,
        parmname: *const ::std::os::raw::c_char,
    ) -> f64 {
        match self.pointers.GetMediaItemInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemInfo_Value)
            )),
            Some(f) => f(item, parmname),
        }
    }
    pub unsafe fn GetMediaItemNumTakes(&self, item: *mut root::MediaItem) -> ::std::os::raw::c_int {
        match self.pointers.GetMediaItemNumTakes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemNumTakes)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn GetMediaItemTake(
        &self,
        item: *mut root::MediaItem,
        tk: ::std::os::raw::c_int,
    ) -> *mut root::MediaItem_Take {
        match self.pointers.GetMediaItemTake {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemTake)
            )),
            Some(f) => f(item, tk),
        }
    }
    pub unsafe fn GetMediaItemTake_Item(
        &self,
        take: *mut root::MediaItem_Take,
    ) -> *mut root::MediaItem {
        match self.pointers.GetMediaItemTake_Item {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemTake_Item)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn GetMediaItemTake_Peaks(
        &self,
        take: *mut root::MediaItem_Take,
        peakrate: f64,
        starttime: f64,
        numchannels: ::std::os::raw::c_int,
        numsamplesperchannel: ::std::os::raw::c_int,
        want_extra_type: ::std::os::raw::c_int,
        buf: *mut f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetMediaItemTake_Peaks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemTake_Peaks)
            )),
            Some(f) => f(
                take,
                peakrate,
                starttime,
                numchannels,
                numsamplesperchannel,
                want_extra_type,
                buf,
            ),
        }
    }
    pub unsafe fn GetMediaItemTake_Source(
        &self,
        take: *mut root::MediaItem_Take,
    ) -> *mut root::PCM_source {
        match self.pointers.GetMediaItemTake_Source {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemTake_Source)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn GetMediaItemTake_Track(
        &self,
        take: *mut root::MediaItem_Take,
    ) -> *mut root::MediaTrack {
        match self.pointers.GetMediaItemTake_Track {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemTake_Track)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn GetMediaItemTakeByGUID(
        &self,
        project: *mut root::ReaProject,
        guid: *const root::GUID,
    ) -> *mut root::MediaItem_Take {
        match self.pointers.GetMediaItemTakeByGUID {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemTakeByGUID)
            )),
            Some(f) => f(project, guid),
        }
    }
    pub unsafe fn GetMediaItemTakeInfo_Value(
        &self,
        take: *mut root::MediaItem_Take,
        parmname: *const ::std::os::raw::c_char,
    ) -> f64 {
        match self.pointers.GetMediaItemTakeInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemTakeInfo_Value)
            )),
            Some(f) => f(take, parmname),
        }
    }
    pub unsafe fn GetMediaItemTrack(&self, item: *mut root::MediaItem) -> *mut root::MediaTrack {
        match self.pointers.GetMediaItemTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaItemTrack)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn GetMediaSourceFileName(
        &self,
        source: *mut root::PCM_source,
        filenamebuf: *mut ::std::os::raw::c_char,
        filenamebuf_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetMediaSourceFileName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaSourceFileName)
            )),
            Some(f) => f(source, filenamebuf, filenamebuf_sz),
        }
    }
    pub unsafe fn GetMediaSourceLength(
        &self,
        source: *mut root::PCM_source,
        lengthIsQNOut: *mut bool,
    ) -> f64 {
        match self.pointers.GetMediaSourceLength {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaSourceLength)
            )),
            Some(f) => f(source, lengthIsQNOut),
        }
    }
    pub unsafe fn GetMediaSourceNumChannels(
        &self,
        source: *mut root::PCM_source,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetMediaSourceNumChannels {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaSourceNumChannels)
            )),
            Some(f) => f(source),
        }
    }
    pub unsafe fn GetMediaSourceParent(&self, src: *mut root::PCM_source) -> *mut root::PCM_source {
        match self.pointers.GetMediaSourceParent {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaSourceParent)
            )),
            Some(f) => f(src),
        }
    }
    pub unsafe fn GetMediaSourceSampleRate(
        &self,
        source: *mut root::PCM_source,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetMediaSourceSampleRate {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaSourceSampleRate)
            )),
            Some(f) => f(source),
        }
    }
    pub unsafe fn GetMediaSourceType(
        &self,
        source: *mut root::PCM_source,
        typebuf: *mut ::std::os::raw::c_char,
        typebuf_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetMediaSourceType {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaSourceType)
            )),
            Some(f) => f(source, typebuf, typebuf_sz),
        }
    }
    pub unsafe fn GetMediaTrackInfo_Value(
        &self,
        tr: *mut root::MediaTrack,
        parmname: *const ::std::os::raw::c_char,
    ) -> f64 {
        match self.pointers.GetMediaTrackInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMediaTrackInfo_Value)
            )),
            Some(f) => f(tr, parmname),
        }
    }
    pub unsafe fn GetMIDIInputName(
        &self,
        dev: ::std::os::raw::c_int,
        nameout: *mut ::std::os::raw::c_char,
        nameout_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetMIDIInputName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMIDIInputName)
            )),
            Some(f) => f(dev, nameout, nameout_sz),
        }
    }
    pub unsafe fn GetMIDIOutputName(
        &self,
        dev: ::std::os::raw::c_int,
        nameout: *mut ::std::os::raw::c_char,
        nameout_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetMIDIOutputName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMIDIOutputName)
            )),
            Some(f) => f(dev, nameout, nameout_sz),
        }
    }
    pub fn GetMixerScroll(&self) -> *mut root::MediaTrack {
        match self.pointers.GetMixerScroll {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMixerScroll)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetMouseModifier(
        &self,
        context: *const ::std::os::raw::c_char,
        modifier_flag: ::std::os::raw::c_int,
        action: *mut ::std::os::raw::c_char,
        action_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetMouseModifier {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMouseModifier)
            )),
            Some(f) => f(context, modifier_flag, action, action_sz),
        }
    }
    pub unsafe fn GetMousePosition(
        &self,
        xOut: *mut ::std::os::raw::c_int,
        yOut: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.GetMousePosition {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMousePosition)
            )),
            Some(f) => f(xOut, yOut),
        }
    }
    pub fn GetNumAudioInputs(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetNumAudioInputs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetNumAudioInputs)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetNumAudioOutputs(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetNumAudioOutputs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetNumAudioOutputs)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetNumMIDIInputs(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetNumMIDIInputs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetNumMIDIInputs)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetNumMIDIOutputs(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetNumMIDIOutputs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetNumMIDIOutputs)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetNumTracks(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetNumTracks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetNumTracks)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetOS(&self) -> *const ::std::os::raw::c_char {
        match self.pointers.GetOS {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetOS)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetOutputChannelName(
        &self,
        channelIndex: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetOutputChannelName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetOutputChannelName)
            )),
            Some(f) => f(channelIndex),
        }
    }
    pub fn GetOutputLatency(&self) -> f64 {
        match self.pointers.GetOutputLatency {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetOutputLatency)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetParentTrack(&self, track: *mut root::MediaTrack) -> *mut root::MediaTrack {
        match self.pointers.GetParentTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetParentTrack)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn GetPeakFileName(
        &self,
        fn_: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetPeakFileName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPeakFileName)
            )),
            Some(f) => f(fn_, buf, buf_sz),
        }
    }
    pub unsafe fn GetPeakFileNameEx(
        &self,
        fn_: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
        forWrite: bool,
    ) {
        match self.pointers.GetPeakFileNameEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPeakFileNameEx)
            )),
            Some(f) => f(fn_, buf, buf_sz, forWrite),
        }
    }
    pub unsafe fn GetPeakFileNameEx2(
        &self,
        fn_: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
        forWrite: bool,
        peaksfileextension: *const ::std::os::raw::c_char,
    ) {
        match self.pointers.GetPeakFileNameEx2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPeakFileNameEx2)
            )),
            Some(f) => f(fn_, buf, buf_sz, forWrite, peaksfileextension),
        }
    }
    pub unsafe fn GetPeaksBitmap(
        &self,
        pks: *mut root::PCM_source_peaktransfer_t,
        maxamp: f64,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        bmp: *mut root::reaper_functions::LICE_IBitmap,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetPeaksBitmap {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPeaksBitmap)
            )),
            Some(f) => f(pks, maxamp, w, h, bmp),
        }
    }
    pub fn GetPlayPosition(&self) -> f64 {
        match self.pointers.GetPlayPosition {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPlayPosition)
            )),
            Some(f) => f(),
        }
    }
    pub fn GetPlayPosition2(&self) -> f64 {
        match self.pointers.GetPlayPosition2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPlayPosition2)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetPlayPosition2Ex(&self, proj: *mut root::ReaProject) -> f64 {
        match self.pointers.GetPlayPosition2Ex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPlayPosition2Ex)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn GetPlayPositionEx(&self, proj: *mut root::ReaProject) -> f64 {
        match self.pointers.GetPlayPositionEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPlayPositionEx)
            )),
            Some(f) => f(proj),
        }
    }
    pub fn GetPlayState(&self) -> ::std::os::raw::c_int {
        match self.pointers.GetPlayState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPlayState)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetPlayStateEx(&self, proj: *mut root::ReaProject) -> ::std::os::raw::c_int {
        match self.pointers.GetPlayStateEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPlayStateEx)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn GetPreferredDiskReadMode(
        &self,
        mode: *mut ::std::os::raw::c_int,
        nb: *mut ::std::os::raw::c_int,
        bs: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.GetPreferredDiskReadMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPreferredDiskReadMode)
            )),
            Some(f) => f(mode, nb, bs),
        }
    }
    pub unsafe fn GetPreferredDiskReadModePeak(
        &self,
        mode: *mut ::std::os::raw::c_int,
        nb: *mut ::std::os::raw::c_int,
        bs: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.GetPreferredDiskReadModePeak {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPreferredDiskReadModePeak)
            )),
            Some(f) => f(mode, nb, bs),
        }
    }
    pub unsafe fn GetPreferredDiskWriteMode(
        &self,
        mode: *mut ::std::os::raw::c_int,
        nb: *mut ::std::os::raw::c_int,
        bs: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.GetPreferredDiskWriteMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetPreferredDiskWriteMode)
            )),
            Some(f) => f(mode, nb, bs),
        }
    }
    pub unsafe fn GetProjectLength(&self, proj: *mut root::ReaProject) -> f64 {
        match self.pointers.GetProjectLength {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjectLength)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn GetProjectName(
        &self,
        proj: *mut root::ReaProject,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetProjectName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjectName)
            )),
            Some(f) => f(proj, buf, buf_sz),
        }
    }
    pub unsafe fn GetProjectPath(
        &self,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetProjectPath {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjectPath)
            )),
            Some(f) => f(buf, buf_sz),
        }
    }
    pub unsafe fn GetProjectPathEx(
        &self,
        proj: *mut root::ReaProject,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetProjectPathEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjectPathEx)
            )),
            Some(f) => f(proj, buf, buf_sz),
        }
    }
    pub unsafe fn GetProjectStateChangeCount(
        &self,
        proj: *mut root::ReaProject,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetProjectStateChangeCount {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjectStateChangeCount)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn GetProjectTimeOffset(&self, proj: *mut root::ReaProject, rndframe: bool) -> f64 {
        match self.pointers.GetProjectTimeOffset {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjectTimeOffset)
            )),
            Some(f) => f(proj, rndframe),
        }
    }
    pub unsafe fn GetProjectTimeSignature(&self, bpmOut: *mut f64, bpiOut: *mut f64) {
        match self.pointers.GetProjectTimeSignature {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjectTimeSignature)
            )),
            Some(f) => f(bpmOut, bpiOut),
        }
    }
    pub unsafe fn GetProjectTimeSignature2(
        &self,
        proj: *mut root::ReaProject,
        bpmOut: *mut f64,
        bpiOut: *mut f64,
    ) {
        match self.pointers.GetProjectTimeSignature2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjectTimeSignature2)
            )),
            Some(f) => f(proj, bpmOut, bpiOut),
        }
    }
    pub unsafe fn GetProjExtState(
        &self,
        proj: *mut root::ReaProject,
        extname: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_char,
        valOutNeedBig: *mut ::std::os::raw::c_char,
        valOutNeedBig_sz: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetProjExtState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetProjExtState)
            )),
            Some(f) => f(proj, extname, key, valOutNeedBig, valOutNeedBig_sz),
        }
    }
    pub fn GetResourcePath(&self) -> *const ::std::os::raw::c_char {
        match self.pointers.GetResourcePath {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetResourcePath)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetSelectedEnvelope(
        &self,
        proj: *mut root::ReaProject,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.GetSelectedEnvelope {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSelectedEnvelope)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn GetSelectedMediaItem(
        &self,
        proj: *mut root::ReaProject,
        selitem: ::std::os::raw::c_int,
    ) -> *mut root::MediaItem {
        match self.pointers.GetSelectedMediaItem {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSelectedMediaItem)
            )),
            Some(f) => f(proj, selitem),
        }
    }
    pub unsafe fn GetSelectedTrack(
        &self,
        proj: *mut root::ReaProject,
        seltrackidx: ::std::os::raw::c_int,
    ) -> *mut root::MediaTrack {
        match self.pointers.GetSelectedTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSelectedTrack)
            )),
            Some(f) => f(proj, seltrackidx),
        }
    }
    pub unsafe fn GetSelectedTrack2(
        &self,
        proj: *mut root::ReaProject,
        seltrackidx: ::std::os::raw::c_int,
        wantmaster: bool,
    ) -> *mut root::MediaTrack {
        match self.pointers.GetSelectedTrack2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSelectedTrack2)
            )),
            Some(f) => f(proj, seltrackidx, wantmaster),
        }
    }
    pub unsafe fn GetSelectedTrackEnvelope(
        &self,
        proj: *mut root::ReaProject,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.GetSelectedTrackEnvelope {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSelectedTrackEnvelope)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn GetSet_ArrangeView2(
        &self,
        proj: *mut root::ReaProject,
        isSet: bool,
        screen_x_start: ::std::os::raw::c_int,
        screen_x_end: ::std::os::raw::c_int,
        start_timeOut: *mut f64,
        end_timeOut: *mut f64,
    ) {
        match self.pointers.GetSet_ArrangeView2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSet_ArrangeView2)
            )),
            Some(f) => f(
                proj,
                isSet,
                screen_x_start,
                screen_x_end,
                start_timeOut,
                end_timeOut,
            ),
        }
    }
    pub unsafe fn GetSet_LoopTimeRange(
        &self,
        isSet: bool,
        isLoop: bool,
        startOut: *mut f64,
        endOut: *mut f64,
        allowautoseek: bool,
    ) {
        match self.pointers.GetSet_LoopTimeRange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSet_LoopTimeRange)
            )),
            Some(f) => f(isSet, isLoop, startOut, endOut, allowautoseek),
        }
    }
    pub unsafe fn GetSet_LoopTimeRange2(
        &self,
        proj: *mut root::ReaProject,
        isSet: bool,
        isLoop: bool,
        startOut: *mut f64,
        endOut: *mut f64,
        allowautoseek: bool,
    ) {
        match self.pointers.GetSet_LoopTimeRange2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSet_LoopTimeRange2)
            )),
            Some(f) => f(proj, isSet, isLoop, startOut, endOut, allowautoseek),
        }
    }
    pub unsafe fn GetSetAutomationItemInfo(
        &self,
        env: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
        desc: *const ::std::os::raw::c_char,
        value: f64,
        is_set: bool,
    ) -> f64 {
        match self.pointers.GetSetAutomationItemInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetAutomationItemInfo)
            )),
            Some(f) => f(env, autoitem_idx, desc, value, is_set),
        }
    }
    pub unsafe fn GetSetAutomationItemInfo_String(
        &self,
        env: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
        desc: *const ::std::os::raw::c_char,
        valuestrNeedBig: *mut ::std::os::raw::c_char,
        is_set: bool,
    ) -> bool {
        match self.pointers.GetSetAutomationItemInfo_String {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetAutomationItemInfo_String)
            )),
            Some(f) => f(env, autoitem_idx, desc, valuestrNeedBig, is_set),
        }
    }
    pub unsafe fn GetSetEnvelopeInfo_String(
        &self,
        env: *mut root::TrackEnvelope,
        parmname: *const ::std::os::raw::c_char,
        stringNeedBig: *mut ::std::os::raw::c_char,
        setNewValue: bool,
    ) -> bool {
        match self.pointers.GetSetEnvelopeInfo_String {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetEnvelopeInfo_String)
            )),
            Some(f) => f(env, parmname, stringNeedBig, setNewValue),
        }
    }
    pub unsafe fn GetSetEnvelopeState(
        &self,
        env: *mut root::TrackEnvelope,
        str: *mut ::std::os::raw::c_char,
        str_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetSetEnvelopeState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetEnvelopeState)
            )),
            Some(f) => f(env, str, str_sz),
        }
    }
    pub unsafe fn GetSetEnvelopeState2(
        &self,
        env: *mut root::TrackEnvelope,
        str: *mut ::std::os::raw::c_char,
        str_sz: ::std::os::raw::c_int,
        isundo: bool,
    ) -> bool {
        match self.pointers.GetSetEnvelopeState2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetEnvelopeState2)
            )),
            Some(f) => f(env, str, str_sz, isundo),
        }
    }
    pub unsafe fn GetSetItemState(
        &self,
        item: *mut root::MediaItem,
        str: *mut ::std::os::raw::c_char,
        str_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetSetItemState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetItemState)
            )),
            Some(f) => f(item, str, str_sz),
        }
    }
    pub unsafe fn GetSetItemState2(
        &self,
        item: *mut root::MediaItem,
        str: *mut ::std::os::raw::c_char,
        str_sz: ::std::os::raw::c_int,
        isundo: bool,
    ) -> bool {
        match self.pointers.GetSetItemState2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetItemState2)
            )),
            Some(f) => f(item, str, str_sz, isundo),
        }
    }
    pub unsafe fn GetSetMediaItemInfo(
        &self,
        item: *mut root::MediaItem,
        parmname: *const ::std::os::raw::c_char,
        setNewValue: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetSetMediaItemInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetMediaItemInfo)
            )),
            Some(f) => f(item, parmname, setNewValue),
        }
    }
    pub unsafe fn GetSetMediaItemInfo_String(
        &self,
        item: *mut root::MediaItem,
        parmname: *const ::std::os::raw::c_char,
        stringNeedBig: *mut ::std::os::raw::c_char,
        setNewValue: bool,
    ) -> bool {
        match self.pointers.GetSetMediaItemInfo_String {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetMediaItemInfo_String)
            )),
            Some(f) => f(item, parmname, stringNeedBig, setNewValue),
        }
    }
    pub unsafe fn GetSetMediaItemTakeInfo(
        &self,
        tk: *mut root::MediaItem_Take,
        parmname: *const ::std::os::raw::c_char,
        setNewValue: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetSetMediaItemTakeInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetMediaItemTakeInfo)
            )),
            Some(f) => f(tk, parmname, setNewValue),
        }
    }
    pub unsafe fn GetSetMediaItemTakeInfo_String(
        &self,
        tk: *mut root::MediaItem_Take,
        parmname: *const ::std::os::raw::c_char,
        stringNeedBig: *mut ::std::os::raw::c_char,
        setNewValue: bool,
    ) -> bool {
        match self.pointers.GetSetMediaItemTakeInfo_String {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetMediaItemTakeInfo_String)
            )),
            Some(f) => f(tk, parmname, stringNeedBig, setNewValue),
        }
    }
    pub unsafe fn GetSetMediaTrackInfo(
        &self,
        tr: *mut root::MediaTrack,
        parmname: *const ::std::os::raw::c_char,
        setNewValue: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetSetMediaTrackInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetMediaTrackInfo)
            )),
            Some(f) => f(tr, parmname, setNewValue),
        }
    }
    pub unsafe fn GetSetMediaTrackInfo_String(
        &self,
        tr: *mut root::MediaTrack,
        parmname: *const ::std::os::raw::c_char,
        stringNeedBig: *mut ::std::os::raw::c_char,
        setNewValue: bool,
    ) -> bool {
        match self.pointers.GetSetMediaTrackInfo_String {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetMediaTrackInfo_String)
            )),
            Some(f) => f(tr, parmname, stringNeedBig, setNewValue),
        }
    }
    pub unsafe fn GetSetObjectState(
        &self,
        obj: *mut ::std::os::raw::c_void,
        str: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char {
        match self.pointers.GetSetObjectState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetObjectState)
            )),
            Some(f) => f(obj, str),
        }
    }
    pub unsafe fn GetSetObjectState2(
        &self,
        obj: *mut ::std::os::raw::c_void,
        str: *const ::std::os::raw::c_char,
        isundo: bool,
    ) -> *mut ::std::os::raw::c_char {
        match self.pointers.GetSetObjectState2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetObjectState2)
            )),
            Some(f) => f(obj, str, isundo),
        }
    }
    pub unsafe fn GetSetProjectAuthor(
        &self,
        proj: *mut root::ReaProject,
        set: bool,
        author: *mut ::std::os::raw::c_char,
        author_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetSetProjectAuthor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetProjectAuthor)
            )),
            Some(f) => f(proj, set, author, author_sz),
        }
    }
    pub unsafe fn GetSetProjectGrid(
        &self,
        project: *mut root::ReaProject,
        set: bool,
        divisionInOutOptional: *mut f64,
        swingmodeInOutOptional: *mut ::std::os::raw::c_int,
        swingamtInOutOptional: *mut f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetSetProjectGrid {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetProjectGrid)
            )),
            Some(f) => f(
                project,
                set,
                divisionInOutOptional,
                swingmodeInOutOptional,
                swingamtInOutOptional,
            ),
        }
    }
    pub unsafe fn GetSetProjectInfo(
        &self,
        project: *mut root::ReaProject,
        desc: *const ::std::os::raw::c_char,
        value: f64,
        is_set: bool,
    ) -> f64 {
        match self.pointers.GetSetProjectInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetProjectInfo)
            )),
            Some(f) => f(project, desc, value, is_set),
        }
    }
    pub unsafe fn GetSetProjectInfo_String(
        &self,
        project: *mut root::ReaProject,
        desc: *const ::std::os::raw::c_char,
        valuestrNeedBig: *mut ::std::os::raw::c_char,
        is_set: bool,
    ) -> bool {
        match self.pointers.GetSetProjectInfo_String {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetProjectInfo_String)
            )),
            Some(f) => f(project, desc, valuestrNeedBig, is_set),
        }
    }
    pub unsafe fn GetSetProjectNotes(
        &self,
        proj: *mut root::ReaProject,
        set: bool,
        notesNeedBig: *mut ::std::os::raw::c_char,
        notesNeedBig_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.GetSetProjectNotes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetProjectNotes)
            )),
            Some(f) => f(proj, set, notesNeedBig, notesNeedBig_sz),
        }
    }
    pub fn GetSetRepeat(&self, val: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        match self.pointers.GetSetRepeat {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetRepeat)
            )),
            Some(f) => f(val),
        }
    }
    pub unsafe fn GetSetRepeatEx(
        &self,
        proj: *mut root::ReaProject,
        val: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetSetRepeatEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetRepeatEx)
            )),
            Some(f) => f(proj, val),
        }
    }
    pub unsafe fn GetSetTrackGroupMembership(
        &self,
        tr: *mut root::MediaTrack,
        groupname: *const ::std::os::raw::c_char,
        setmask: ::std::os::raw::c_uint,
        setvalue: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint {
        match self.pointers.GetSetTrackGroupMembership {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetTrackGroupMembership)
            )),
            Some(f) => f(tr, groupname, setmask, setvalue),
        }
    }
    pub unsafe fn GetSetTrackGroupMembershipHigh(
        &self,
        tr: *mut root::MediaTrack,
        groupname: *const ::std::os::raw::c_char,
        setmask: ::std::os::raw::c_uint,
        setvalue: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint {
        match self.pointers.GetSetTrackGroupMembershipHigh {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetTrackGroupMembershipHigh)
            )),
            Some(f) => f(tr, groupname, setmask, setvalue),
        }
    }
    pub unsafe fn GetSetTrackMIDISupportFile(
        &self,
        proj: *mut root::ReaProject,
        track: *mut root::MediaTrack,
        which: ::std::os::raw::c_int,
        filename: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetSetTrackMIDISupportFile {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetTrackMIDISupportFile)
            )),
            Some(f) => f(proj, track, which, filename),
        }
    }
    pub unsafe fn GetSetTrackSendInfo(
        &self,
        tr: *mut root::MediaTrack,
        category: ::std::os::raw::c_int,
        sendidx: ::std::os::raw::c_int,
        parmname: *const ::std::os::raw::c_char,
        setNewValue: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.GetSetTrackSendInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetTrackSendInfo)
            )),
            Some(f) => f(tr, category, sendidx, parmname, setNewValue),
        }
    }
    pub unsafe fn GetSetTrackSendInfo_String(
        &self,
        tr: *mut root::MediaTrack,
        category: ::std::os::raw::c_int,
        sendidx: ::std::os::raw::c_int,
        parmname: *const ::std::os::raw::c_char,
        stringNeedBig: *mut ::std::os::raw::c_char,
        setNewValue: bool,
    ) -> bool {
        match self.pointers.GetSetTrackSendInfo_String {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetTrackSendInfo_String)
            )),
            Some(f) => f(tr, category, sendidx, parmname, stringNeedBig, setNewValue),
        }
    }
    pub unsafe fn GetSetTrackState(
        &self,
        track: *mut root::MediaTrack,
        str: *mut ::std::os::raw::c_char,
        str_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetSetTrackState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetTrackState)
            )),
            Some(f) => f(track, str, str_sz),
        }
    }
    pub unsafe fn GetSetTrackState2(
        &self,
        track: *mut root::MediaTrack,
        str: *mut ::std::os::raw::c_char,
        str_sz: ::std::os::raw::c_int,
        isundo: bool,
    ) -> bool {
        match self.pointers.GetSetTrackState2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSetTrackState2)
            )),
            Some(f) => f(track, str, str_sz, isundo),
        }
    }
    pub unsafe fn GetSubProjectFromSource(
        &self,
        src: *mut root::PCM_source,
    ) -> *mut root::ReaProject {
        match self.pointers.GetSubProjectFromSource {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetSubProjectFromSource)
            )),
            Some(f) => f(src),
        }
    }
    pub unsafe fn GetTake(
        &self,
        item: *mut root::MediaItem,
        takeidx: ::std::os::raw::c_int,
    ) -> *mut root::MediaItem_Take {
        match self.pointers.GetTake {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTake)
            )),
            Some(f) => f(item, takeidx),
        }
    }
    pub unsafe fn GetTakeEnvelope(
        &self,
        take: *mut root::MediaItem_Take,
        envidx: ::std::os::raw::c_int,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.GetTakeEnvelope {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTakeEnvelope)
            )),
            Some(f) => f(take, envidx),
        }
    }
    pub unsafe fn GetTakeEnvelopeByName(
        &self,
        take: *mut root::MediaItem_Take,
        envname: *const ::std::os::raw::c_char,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.GetTakeEnvelopeByName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTakeEnvelopeByName)
            )),
            Some(f) => f(take, envname),
        }
    }
    pub unsafe fn GetTakeName(
        &self,
        take: *mut root::MediaItem_Take,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetTakeName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTakeName)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn GetTakeNumStretchMarkers(
        &self,
        take: *mut root::MediaItem_Take,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetTakeNumStretchMarkers {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTakeNumStretchMarkers)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn GetTakeStretchMarker(
        &self,
        take: *mut root::MediaItem_Take,
        idx: ::std::os::raw::c_int,
        posOut: *mut f64,
        srcposOutOptional: *mut f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetTakeStretchMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTakeStretchMarker)
            )),
            Some(f) => f(take, idx, posOut, srcposOutOptional),
        }
    }
    pub unsafe fn GetTakeStretchMarkerSlope(
        &self,
        take: *mut root::MediaItem_Take,
        idx: ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.GetTakeStretchMarkerSlope {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTakeStretchMarkerSlope)
            )),
            Some(f) => f(take, idx),
        }
    }
    pub unsafe fn GetTCPFXParm(
        &self,
        project: *mut root::ReaProject,
        track: *mut root::MediaTrack,
        index: ::std::os::raw::c_int,
        fxindexOut: *mut ::std::os::raw::c_int,
        parmidxOut: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetTCPFXParm {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTCPFXParm)
            )),
            Some(f) => f(project, track, index, fxindexOut, parmidxOut),
        }
    }
    pub unsafe fn GetTempoMatchPlayRate(
        &self,
        source: *mut root::PCM_source,
        srcscale: f64,
        position: f64,
        mult: f64,
        rateOut: *mut f64,
        targetlenOut: *mut f64,
    ) -> bool {
        match self.pointers.GetTempoMatchPlayRate {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTempoMatchPlayRate)
            )),
            Some(f) => f(source, srcscale, position, mult, rateOut, targetlenOut),
        }
    }
    pub unsafe fn GetTempoTimeSigMarker(
        &self,
        proj: *mut root::ReaProject,
        ptidx: ::std::os::raw::c_int,
        timeposOut: *mut f64,
        measureposOut: *mut ::std::os::raw::c_int,
        beatposOut: *mut f64,
        bpmOut: *mut f64,
        timesig_numOut: *mut ::std::os::raw::c_int,
        timesig_denomOut: *mut ::std::os::raw::c_int,
        lineartempoOut: *mut bool,
    ) -> bool {
        match self.pointers.GetTempoTimeSigMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTempoTimeSigMarker)
            )),
            Some(f) => f(
                proj,
                ptidx,
                timeposOut,
                measureposOut,
                beatposOut,
                bpmOut,
                timesig_numOut,
                timesig_denomOut,
                lineartempoOut,
            ),
        }
    }
    pub fn GetToggleCommandState(
        &self,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetToggleCommandState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetToggleCommandState)
            )),
            Some(f) => f(command_id),
        }
    }
    pub unsafe fn GetToggleCommandState2(
        &self,
        section: *mut root::KbdSectionInfo,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetToggleCommandState2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetToggleCommandState2)
            )),
            Some(f) => f(section, command_id),
        }
    }
    pub fn GetToggleCommandStateEx(
        &self,
        section_id: ::std::os::raw::c_int,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetToggleCommandStateEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetToggleCommandStateEx)
            )),
            Some(f) => f(section_id, command_id),
        }
    }
    pub unsafe fn GetToggleCommandStateThroughHooks(
        &self,
        section: *mut root::KbdSectionInfo,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetToggleCommandStateThroughHooks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetToggleCommandStateThroughHooks)
            )),
            Some(f) => f(section, command_id),
        }
    }
    pub fn GetTooltipWindow(&self) -> root::HWND {
        match self.pointers.GetTooltipWindow {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTooltipWindow)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn GetTrack(
        &self,
        proj: *mut root::ReaProject,
        trackidx: ::std::os::raw::c_int,
    ) -> *mut root::MediaTrack {
        match self.pointers.GetTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrack)
            )),
            Some(f) => f(proj, trackidx),
        }
    }
    pub unsafe fn GetTrackAutomationMode(
        &self,
        tr: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetTrackAutomationMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackAutomationMode)
            )),
            Some(f) => f(tr),
        }
    }
    pub unsafe fn GetTrackColor(&self, track: *mut root::MediaTrack) -> ::std::os::raw::c_int {
        match self.pointers.GetTrackColor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackColor)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn GetTrackDepth(&self, track: *mut root::MediaTrack) -> ::std::os::raw::c_int {
        match self.pointers.GetTrackDepth {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackDepth)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn GetTrackEnvelope(
        &self,
        track: *mut root::MediaTrack,
        envidx: ::std::os::raw::c_int,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.GetTrackEnvelope {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackEnvelope)
            )),
            Some(f) => f(track, envidx),
        }
    }
    pub unsafe fn GetTrackEnvelopeByChunkName(
        &self,
        tr: *mut root::MediaTrack,
        cfgchunkname: *const ::std::os::raw::c_char,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.GetTrackEnvelopeByChunkName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackEnvelopeByChunkName)
            )),
            Some(f) => f(tr, cfgchunkname),
        }
    }
    pub unsafe fn GetTrackEnvelopeByName(
        &self,
        track: *mut root::MediaTrack,
        envname: *const ::std::os::raw::c_char,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.GetTrackEnvelopeByName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackEnvelopeByName)
            )),
            Some(f) => f(track, envname),
        }
    }
    pub unsafe fn GetTrackFromPoint(
        &self,
        screen_x: ::std::os::raw::c_int,
        screen_y: ::std::os::raw::c_int,
        infoOutOptional: *mut ::std::os::raw::c_int,
    ) -> *mut root::MediaTrack {
        match self.pointers.GetTrackFromPoint {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackFromPoint)
            )),
            Some(f) => f(screen_x, screen_y, infoOutOptional),
        }
    }
    pub unsafe fn GetTrackGUID(&self, tr: *mut root::MediaTrack) -> *mut root::GUID {
        match self.pointers.GetTrackGUID {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackGUID)
            )),
            Some(f) => f(tr),
        }
    }
    pub unsafe fn GetTrackInfo(
        &self,
        track: root::INT_PTR,
        flags: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetTrackInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackInfo)
            )),
            Some(f) => f(track, flags),
        }
    }
    pub unsafe fn GetTrackMediaItem(
        &self,
        tr: *mut root::MediaTrack,
        itemidx: ::std::os::raw::c_int,
    ) -> *mut root::MediaItem {
        match self.pointers.GetTrackMediaItem {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackMediaItem)
            )),
            Some(f) => f(tr, itemidx),
        }
    }
    pub unsafe fn GetTrackMIDILyrics(
        &self,
        track: *mut root::MediaTrack,
        flag: ::std::os::raw::c_int,
        bufWantNeedBig: *mut ::std::os::raw::c_char,
        bufWantNeedBig_sz: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetTrackMIDILyrics {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackMIDILyrics)
            )),
            Some(f) => f(track, flag, bufWantNeedBig, bufWantNeedBig_sz),
        }
    }
    pub fn GetTrackMIDINoteName(
        &self,
        track: ::std::os::raw::c_int,
        pitch: ::std::os::raw::c_int,
        chan: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetTrackMIDINoteName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackMIDINoteName)
            )),
            Some(f) => f(track, pitch, chan),
        }
    }
    pub unsafe fn GetTrackMIDINoteNameEx(
        &self,
        proj: *mut root::ReaProject,
        track: *mut root::MediaTrack,
        pitch: ::std::os::raw::c_int,
        chan: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetTrackMIDINoteNameEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackMIDINoteNameEx)
            )),
            Some(f) => f(proj, track, pitch, chan),
        }
    }
    pub unsafe fn GetTrackMIDINoteRange(
        &self,
        proj: *mut root::ReaProject,
        track: *mut root::MediaTrack,
        note_loOut: *mut ::std::os::raw::c_int,
        note_hiOut: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.GetTrackMIDINoteRange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackMIDINoteRange)
            )),
            Some(f) => f(proj, track, note_loOut, note_hiOut),
        }
    }
    pub unsafe fn GetTrackName(
        &self,
        track: *mut root::MediaTrack,
        bufOut: *mut ::std::os::raw::c_char,
        bufOut_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetTrackName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackName)
            )),
            Some(f) => f(track, bufOut, bufOut_sz),
        }
    }
    pub unsafe fn GetTrackNumMediaItems(&self, tr: *mut root::MediaTrack) -> ::std::os::raw::c_int {
        match self.pointers.GetTrackNumMediaItems {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackNumMediaItems)
            )),
            Some(f) => f(tr),
        }
    }
    pub unsafe fn GetTrackNumSends(
        &self,
        tr: *mut root::MediaTrack,
        category: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GetTrackNumSends {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackNumSends)
            )),
            Some(f) => f(tr, category),
        }
    }
    pub unsafe fn GetTrackReceiveName(
        &self,
        track: *mut root::MediaTrack,
        recv_index: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetTrackReceiveName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackReceiveName)
            )),
            Some(f) => f(track, recv_index, buf, buf_sz),
        }
    }
    pub unsafe fn GetTrackReceiveUIMute(
        &self,
        track: *mut root::MediaTrack,
        recv_index: ::std::os::raw::c_int,
        muteOut: *mut bool,
    ) -> bool {
        match self.pointers.GetTrackReceiveUIMute {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackReceiveUIMute)
            )),
            Some(f) => f(track, recv_index, muteOut),
        }
    }
    pub unsafe fn GetTrackReceiveUIVolPan(
        &self,
        track: *mut root::MediaTrack,
        recv_index: ::std::os::raw::c_int,
        volumeOut: *mut f64,
        panOut: *mut f64,
    ) -> bool {
        match self.pointers.GetTrackReceiveUIVolPan {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackReceiveUIVolPan)
            )),
            Some(f) => f(track, recv_index, volumeOut, panOut),
        }
    }
    pub unsafe fn GetTrackSendInfo_Value(
        &self,
        tr: *mut root::MediaTrack,
        category: ::std::os::raw::c_int,
        sendidx: ::std::os::raw::c_int,
        parmname: *const ::std::os::raw::c_char,
    ) -> f64 {
        match self.pointers.GetTrackSendInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackSendInfo_Value)
            )),
            Some(f) => f(tr, category, sendidx, parmname),
        }
    }
    pub unsafe fn GetTrackSendName(
        &self,
        track: *mut root::MediaTrack,
        send_index: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetTrackSendName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackSendName)
            )),
            Some(f) => f(track, send_index, buf, buf_sz),
        }
    }
    pub unsafe fn GetTrackSendUIMute(
        &self,
        track: *mut root::MediaTrack,
        send_index: ::std::os::raw::c_int,
        muteOut: *mut bool,
    ) -> bool {
        match self.pointers.GetTrackSendUIMute {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackSendUIMute)
            )),
            Some(f) => f(track, send_index, muteOut),
        }
    }
    pub unsafe fn GetTrackSendUIVolPan(
        &self,
        track: *mut root::MediaTrack,
        send_index: ::std::os::raw::c_int,
        volumeOut: *mut f64,
        panOut: *mut f64,
    ) -> bool {
        match self.pointers.GetTrackSendUIVolPan {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackSendUIVolPan)
            )),
            Some(f) => f(track, send_index, volumeOut, panOut),
        }
    }
    pub unsafe fn GetTrackState(
        &self,
        track: *mut root::MediaTrack,
        flagsOut: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.GetTrackState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackState)
            )),
            Some(f) => f(track, flagsOut),
        }
    }
    pub unsafe fn GetTrackStateChunk(
        &self,
        track: *mut root::MediaTrack,
        strNeedBig: *mut ::std::os::raw::c_char,
        strNeedBig_sz: ::std::os::raw::c_int,
        isundoOptional: bool,
    ) -> bool {
        match self.pointers.GetTrackStateChunk {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackStateChunk)
            )),
            Some(f) => f(track, strNeedBig, strNeedBig_sz, isundoOptional),
        }
    }
    pub unsafe fn GetTrackUIMute(&self, track: *mut root::MediaTrack, muteOut: *mut bool) -> bool {
        match self.pointers.GetTrackUIMute {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackUIMute)
            )),
            Some(f) => f(track, muteOut),
        }
    }
    pub unsafe fn GetTrackUIPan(
        &self,
        track: *mut root::MediaTrack,
        pan1Out: *mut f64,
        pan2Out: *mut f64,
        panmodeOut: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetTrackUIPan {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackUIPan)
            )),
            Some(f) => f(track, pan1Out, pan2Out, panmodeOut),
        }
    }
    pub unsafe fn GetTrackUIVolPan(
        &self,
        track: *mut root::MediaTrack,
        volumeOut: *mut f64,
        panOut: *mut f64,
    ) -> bool {
        match self.pointers.GetTrackUIVolPan {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetTrackUIVolPan)
            )),
            Some(f) => f(track, volumeOut, panOut),
        }
    }
    pub unsafe fn GetUnderrunTime(
        &self,
        audio_xrunOutOptional: *mut ::std::os::raw::c_uint,
        media_xrunOutOptional: *mut ::std::os::raw::c_uint,
        curtimeOutOptional: *mut ::std::os::raw::c_uint,
    ) {
        match self.pointers.GetUnderrunTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetUnderrunTime)
            )),
            Some(f) => f(
                audio_xrunOutOptional,
                media_xrunOutOptional,
                curtimeOutOptional,
            ),
        }
    }
    pub unsafe fn GetUserFileNameForRead(
        &self,
        filenameNeed4096: *mut ::std::os::raw::c_char,
        title: *const ::std::os::raw::c_char,
        defext: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.GetUserFileNameForRead {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetUserFileNameForRead)
            )),
            Some(f) => f(filenameNeed4096, title, defext),
        }
    }
    pub unsafe fn GetUserInputs(
        &self,
        title: *const ::std::os::raw::c_char,
        num_inputs: ::std::os::raw::c_int,
        captions_csv: *const ::std::os::raw::c_char,
        retvals_csv: *mut ::std::os::raw::c_char,
        retvals_csv_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.GetUserInputs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetUserInputs)
            )),
            Some(f) => f(title, num_inputs, captions_csv, retvals_csv, retvals_csv_sz),
        }
    }
    pub unsafe fn GoToMarker(
        &self,
        proj: *mut root::ReaProject,
        marker_index: ::std::os::raw::c_int,
        use_timeline_order: bool,
    ) {
        match self.pointers.GoToMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GoToMarker)
            )),
            Some(f) => f(proj, marker_index, use_timeline_order),
        }
    }
    pub unsafe fn GoToRegion(
        &self,
        proj: *mut root::ReaProject,
        region_index: ::std::os::raw::c_int,
        use_timeline_order: bool,
    ) {
        match self.pointers.GoToRegion {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GoToRegion)
            )),
            Some(f) => f(proj, region_index, use_timeline_order),
        }
    }
    pub unsafe fn GR_SelectColor(
        &self,
        hwnd: root::HWND,
        colorOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.GR_SelectColor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GR_SelectColor)
            )),
            Some(f) => f(hwnd, colorOut),
        }
    }
    pub fn GSC_mainwnd(&self, t: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        match self.pointers.GSC_mainwnd {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GSC_mainwnd)
            )),
            Some(f) => f(t),
        }
    }
    pub unsafe fn guidToString(
        &self,
        g: *const root::GUID,
        destNeed64: *mut ::std::os::raw::c_char,
    ) {
        match self.pointers.guidToString {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(guidToString)
            )),
            Some(f) => f(g, destNeed64),
        }
    }
    pub unsafe fn HasExtState(
        &self,
        section: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.HasExtState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(HasExtState)
            )),
            Some(f) => f(section, key),
        }
    }
    pub fn HasTrackMIDIPrograms(
        &self,
        track: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.HasTrackMIDIPrograms {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(HasTrackMIDIPrograms)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn HasTrackMIDIProgramsEx(
        &self,
        proj: *mut root::ReaProject,
        track: *mut root::MediaTrack,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.HasTrackMIDIProgramsEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(HasTrackMIDIProgramsEx)
            )),
            Some(f) => f(proj, track),
        }
    }
    pub unsafe fn Help_Set(
        &self,
        helpstring: *const ::std::os::raw::c_char,
        is_temporary_help: bool,
    ) {
        match self.pointers.Help_Set {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Help_Set)
            )),
            Some(f) => f(helpstring, is_temporary_help),
        }
    }
    pub unsafe fn HiresPeaksFromSource(
        &self,
        src: *mut root::PCM_source,
        block: *mut root::PCM_source_peaktransfer_t,
    ) {
        match self.pointers.HiresPeaksFromSource {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(HiresPeaksFromSource)
            )),
            Some(f) => f(src, block),
        }
    }
    pub unsafe fn image_resolve_fn(
        &self,
        in_: *const ::std::os::raw::c_char,
        out: *mut ::std::os::raw::c_char,
        out_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.image_resolve_fn {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(image_resolve_fn)
            )),
            Some(f) => f(in_, out, out_sz),
        }
    }
    pub unsafe fn InsertAutomationItem(
        &self,
        env: *mut root::TrackEnvelope,
        pool_id: ::std::os::raw::c_int,
        position: f64,
        length: f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.InsertAutomationItem {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(InsertAutomationItem)
            )),
            Some(f) => f(env, pool_id, position, length),
        }
    }
    pub unsafe fn InsertEnvelopePoint(
        &self,
        envelope: *mut root::TrackEnvelope,
        time: f64,
        value: f64,
        shape: ::std::os::raw::c_int,
        tension: f64,
        selected: bool,
        noSortInOptional: *mut bool,
    ) -> bool {
        match self.pointers.InsertEnvelopePoint {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(InsertEnvelopePoint)
            )),
            Some(f) => f(
                envelope,
                time,
                value,
                shape,
                tension,
                selected,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn InsertEnvelopePointEx(
        &self,
        envelope: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
        time: f64,
        value: f64,
        shape: ::std::os::raw::c_int,
        tension: f64,
        selected: bool,
        noSortInOptional: *mut bool,
    ) -> bool {
        match self.pointers.InsertEnvelopePointEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(InsertEnvelopePointEx)
            )),
            Some(f) => f(
                envelope,
                autoitem_idx,
                time,
                value,
                shape,
                tension,
                selected,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn InsertMedia(
        &self,
        file: *const ::std::os::raw::c_char,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.InsertMedia {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(InsertMedia)
            )),
            Some(f) => f(file, mode),
        }
    }
    pub unsafe fn InsertMediaSection(
        &self,
        file: *const ::std::os::raw::c_char,
        mode: ::std::os::raw::c_int,
        startpct: f64,
        endpct: f64,
        pitchshift: f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.InsertMediaSection {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(InsertMediaSection)
            )),
            Some(f) => f(file, mode, startpct, endpct, pitchshift),
        }
    }
    pub fn InsertTrackAtIndex(&self, idx: ::std::os::raw::c_int, wantDefaults: bool) {
        match self.pointers.InsertTrackAtIndex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(InsertTrackAtIndex)
            )),
            Some(f) => f(idx, wantDefaults),
        }
    }
    pub fn IsInRealTimeAudio(&self) -> ::std::os::raw::c_int {
        match self.pointers.IsInRealTimeAudio {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(IsInRealTimeAudio)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn IsItemTakeActiveForPlayback(
        &self,
        item: *mut root::MediaItem,
        take: *mut root::MediaItem_Take,
    ) -> bool {
        match self.pointers.IsItemTakeActiveForPlayback {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(IsItemTakeActiveForPlayback)
            )),
            Some(f) => f(item, take),
        }
    }
    pub unsafe fn IsMediaExtension(
        &self,
        ext: *const ::std::os::raw::c_char,
        wantOthers: bool,
    ) -> bool {
        match self.pointers.IsMediaExtension {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(IsMediaExtension)
            )),
            Some(f) => f(ext, wantOthers),
        }
    }
    pub unsafe fn IsMediaItemSelected(&self, item: *mut root::MediaItem) -> bool {
        match self.pointers.IsMediaItemSelected {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(IsMediaItemSelected)
            )),
            Some(f) => f(item),
        }
    }
    pub unsafe fn IsProjectDirty(&self, proj: *mut root::ReaProject) -> ::std::os::raw::c_int {
        match self.pointers.IsProjectDirty {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(IsProjectDirty)
            )),
            Some(f) => f(proj),
        }
    }
    pub fn IsREAPER(&self) -> bool {
        match self.pointers.IsREAPER {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(IsREAPER)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn IsTrackSelected(&self, track: *mut root::MediaTrack) -> bool {
        match self.pointers.IsTrackSelected {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(IsTrackSelected)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn IsTrackVisible(&self, track: *mut root::MediaTrack, mixer: bool) -> bool {
        match self.pointers.IsTrackVisible {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(IsTrackVisible)
            )),
            Some(f) => f(track, mixer),
        }
    }
    pub unsafe fn joystick_create(
        &self,
        guid: *const root::GUID,
    ) -> *mut root::reaper_functions::joystick_device {
        match self.pointers.joystick_create {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(joystick_create)
            )),
            Some(f) => f(guid),
        }
    }
    pub unsafe fn joystick_destroy(&self, device: *mut root::reaper_functions::joystick_device) {
        match self.pointers.joystick_destroy {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(joystick_destroy)
            )),
            Some(f) => f(device),
        }
    }
    pub unsafe fn joystick_enum(
        &self,
        index: ::std::os::raw::c_int,
        namestrOutOptional: *mut *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.joystick_enum {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(joystick_enum)
            )),
            Some(f) => f(index, namestrOutOptional),
        }
    }
    pub unsafe fn joystick_getaxis(
        &self,
        dev: *mut root::reaper_functions::joystick_device,
        axis: ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.joystick_getaxis {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(joystick_getaxis)
            )),
            Some(f) => f(dev, axis),
        }
    }
    pub unsafe fn joystick_getbuttonmask(
        &self,
        dev: *mut root::reaper_functions::joystick_device,
    ) -> ::std::os::raw::c_uint {
        match self.pointers.joystick_getbuttonmask {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(joystick_getbuttonmask)
            )),
            Some(f) => f(dev),
        }
    }
    pub unsafe fn joystick_getinfo(
        &self,
        dev: *mut root::reaper_functions::joystick_device,
        axesOutOptional: *mut ::std::os::raw::c_int,
        povsOutOptional: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.joystick_getinfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(joystick_getinfo)
            )),
            Some(f) => f(dev, axesOutOptional, povsOutOptional),
        }
    }
    pub unsafe fn joystick_getpov(
        &self,
        dev: *mut root::reaper_functions::joystick_device,
        pov: ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.joystick_getpov {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(joystick_getpov)
            )),
            Some(f) => f(dev, pov),
        }
    }
    pub unsafe fn joystick_update(
        &self,
        dev: *mut root::reaper_functions::joystick_device,
    ) -> bool {
        match self.pointers.joystick_update {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(joystick_update)
            )),
            Some(f) => f(dev),
        }
    }
    pub unsafe fn kbd_enumerateActions(
        &self,
        section: *mut root::KbdSectionInfo,
        idx: ::std::os::raw::c_int,
        nameOut: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        match self.pointers.kbd_enumerateActions {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_enumerateActions)
            )),
            Some(f) => f(section, idx, nameOut),
        }
    }
    pub unsafe fn kbd_formatKeyName(&self, ac: *mut root::ACCEL, s: *mut ::std::os::raw::c_char) {
        match self.pointers.kbd_formatKeyName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_formatKeyName)
            )),
            Some(f) => f(ac, s),
        }
    }
    pub unsafe fn kbd_getCommandName(
        &self,
        cmd: ::std::os::raw::c_int,
        s: *mut ::std::os::raw::c_char,
        section: *mut root::KbdSectionInfo,
    ) {
        match self.pointers.kbd_getCommandName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_getCommandName)
            )),
            Some(f) => f(cmd, s, section),
        }
    }
    pub unsafe fn kbd_getTextFromCmd(
        &self,
        cmd: root::DWORD,
        section: *mut root::KbdSectionInfo,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.kbd_getTextFromCmd {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_getTextFromCmd)
            )),
            Some(f) => f(cmd, section),
        }
    }
    pub unsafe fn KBD_OnMainActionEx(
        &self,
        cmd: ::std::os::raw::c_int,
        val: ::std::os::raw::c_int,
        valhw: ::std::os::raw::c_int,
        relmode: ::std::os::raw::c_int,
        hwnd: root::HWND,
        proj: *mut root::ReaProject,
    ) -> ::std::os::raw::c_int {
        match self.pointers.KBD_OnMainActionEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(KBD_OnMainActionEx)
            )),
            Some(f) => f(cmd, val, valhw, relmode, hwnd, proj),
        }
    }
    pub unsafe fn kbd_OnMidiEvent(
        &self,
        evt: *mut root::MIDI_event_t,
        dev_index: ::std::os::raw::c_int,
    ) {
        match self.pointers.kbd_OnMidiEvent {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_OnMidiEvent)
            )),
            Some(f) => f(evt, dev_index),
        }
    }
    pub unsafe fn kbd_OnMidiList(
        &self,
        list: *mut root::MIDI_eventlist,
        dev_index: ::std::os::raw::c_int,
    ) {
        match self.pointers.kbd_OnMidiList {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_OnMidiList)
            )),
            Some(f) => f(list, dev_index),
        }
    }
    pub unsafe fn kbd_ProcessActionsMenu(
        &self,
        menu: root::HMENU,
        section: *mut root::KbdSectionInfo,
    ) {
        match self.pointers.kbd_ProcessActionsMenu {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_ProcessActionsMenu)
            )),
            Some(f) => f(menu, section),
        }
    }
    pub unsafe fn kbd_processMidiEventActionEx(
        &self,
        evt: *mut root::MIDI_event_t,
        section: *mut root::KbdSectionInfo,
        hwndCtx: root::HWND,
    ) -> bool {
        match self.pointers.kbd_processMidiEventActionEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_processMidiEventActionEx)
            )),
            Some(f) => f(evt, section, hwndCtx),
        }
    }
    pub unsafe fn kbd_reprocessMenu(&self, menu: root::HMENU, section: *mut root::KbdSectionInfo) {
        match self.pointers.kbd_reprocessMenu {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_reprocessMenu)
            )),
            Some(f) => f(menu, section),
        }
    }
    pub unsafe fn kbd_RunCommandThroughHooks(
        &self,
        section: *mut root::KbdSectionInfo,
        actionCommandID: *mut ::std::os::raw::c_int,
        val: *mut ::std::os::raw::c_int,
        valhw: *mut ::std::os::raw::c_int,
        relmode: *mut ::std::os::raw::c_int,
        hwnd: root::HWND,
    ) -> bool {
        match self.pointers.kbd_RunCommandThroughHooks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_RunCommandThroughHooks)
            )),
            Some(f) => f(section, actionCommandID, val, valhw, relmode, hwnd),
        }
    }
    pub unsafe fn kbd_translateAccelerator(
        &self,
        hwnd: root::HWND,
        msg: *mut root::MSG,
        section: *mut root::KbdSectionInfo,
    ) -> ::std::os::raw::c_int {
        match self.pointers.kbd_translateAccelerator {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_translateAccelerator)
            )),
            Some(f) => f(hwnd, msg, section),
        }
    }
    pub unsafe fn kbd_translateMouse(
        &self,
        winmsg: *mut ::std::os::raw::c_void,
        midimsg: *mut ::std::os::raw::c_uchar,
    ) -> bool {
        match self.pointers.kbd_translateMouse {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(kbd_translateMouse)
            )),
            Some(f) => f(winmsg, midimsg),
        }
    }
    pub unsafe fn LICE__Destroy(&self, bm: *mut root::reaper_functions::LICE_IBitmap) {
        match self.pointers.LICE__Destroy {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__Destroy)
            )),
            Some(f) => f(bm),
        }
    }
    pub unsafe fn LICE__DestroyFont(&self, font: *mut root::reaper_functions::LICE_IFont) {
        match self.pointers.LICE__DestroyFont {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__DestroyFont)
            )),
            Some(f) => f(font),
        }
    }
    pub unsafe fn LICE__DrawText(
        &self,
        font: *mut root::reaper_functions::LICE_IFont,
        bm: *mut root::reaper_functions::LICE_IBitmap,
        str: *const ::std::os::raw::c_char,
        strcnt: ::std::os::raw::c_int,
        rect: *mut root::RECT,
        dtFlags: root::UINT,
    ) -> ::std::os::raw::c_int {
        match self.pointers.LICE__DrawText {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__DrawText)
            )),
            Some(f) => f(font, bm, str, strcnt, rect, dtFlags),
        }
    }
    pub unsafe fn LICE__GetBits(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.LICE__GetBits {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__GetBits)
            )),
            Some(f) => f(bm),
        }
    }
    pub unsafe fn LICE__GetDC(&self, bm: *mut root::reaper_functions::LICE_IBitmap) -> root::HDC {
        match self.pointers.LICE__GetDC {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__GetDC)
            )),
            Some(f) => f(bm),
        }
    }
    pub unsafe fn LICE__GetHeight(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
    ) -> ::std::os::raw::c_int {
        match self.pointers.LICE__GetHeight {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__GetHeight)
            )),
            Some(f) => f(bm),
        }
    }
    pub unsafe fn LICE__GetRowSpan(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
    ) -> ::std::os::raw::c_int {
        match self.pointers.LICE__GetRowSpan {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__GetRowSpan)
            )),
            Some(f) => f(bm),
        }
    }
    pub unsafe fn LICE__GetWidth(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
    ) -> ::std::os::raw::c_int {
        match self.pointers.LICE__GetWidth {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__GetWidth)
            )),
            Some(f) => f(bm),
        }
    }
    pub unsafe fn LICE__IsFlipped(&self, bm: *mut root::reaper_functions::LICE_IBitmap) -> bool {
        match self.pointers.LICE__IsFlipped {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__IsFlipped)
            )),
            Some(f) => f(bm),
        }
    }
    pub unsafe fn LICE__resize(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.LICE__resize {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__resize)
            )),
            Some(f) => f(bm, w, h),
        }
    }
    pub unsafe fn LICE__SetBkColor(
        &self,
        font: *mut root::reaper_functions::LICE_IFont,
        color: root::reaper_functions::LICE_pixel,
    ) -> root::reaper_functions::LICE_pixel {
        match self.pointers.LICE__SetBkColor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__SetBkColor)
            )),
            Some(f) => f(font, color),
        }
    }
    pub unsafe fn LICE__SetFromHFont(
        &self,
        font: *mut root::reaper_functions::LICE_IFont,
        hfont: root::HFONT,
        flags: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE__SetFromHFont {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__SetFromHFont)
            )),
            Some(f) => f(font, hfont, flags),
        }
    }
    pub unsafe fn LICE__SetTextColor(
        &self,
        font: *mut root::reaper_functions::LICE_IFont,
        color: root::reaper_functions::LICE_pixel,
    ) -> root::reaper_functions::LICE_pixel {
        match self.pointers.LICE__SetTextColor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__SetTextColor)
            )),
            Some(f) => f(font, color),
        }
    }
    pub unsafe fn LICE__SetTextCombineMode(
        &self,
        ifont: *mut root::reaper_functions::LICE_IFont,
        mode: ::std::os::raw::c_int,
        alpha: f32,
    ) {
        match self.pointers.LICE__SetTextCombineMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE__SetTextCombineMode)
            )),
            Some(f) => f(ifont, mode, alpha),
        }
    }
    pub unsafe fn LICE_Arc(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        cx: f32,
        cy: f32,
        r: f32,
        minAngle: f32,
        maxAngle: f32,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        aa: bool,
    ) {
        match self.pointers.LICE_Arc {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_Arc)
            )),
            Some(f) => f(dest, cx, cy, r, minAngle, maxAngle, color, alpha, mode, aa),
        }
    }
    pub unsafe fn LICE_Blit(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        src: *mut root::reaper_functions::LICE_IBitmap,
        dstx: ::std::os::raw::c_int,
        dsty: ::std::os::raw::c_int,
        srcx: ::std::os::raw::c_int,
        srcy: ::std::os::raw::c_int,
        srcw: ::std::os::raw::c_int,
        srch: ::std::os::raw::c_int,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_Blit {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_Blit)
            )),
            Some(f) => f(dest, src, dstx, dsty, srcx, srcy, srcw, srch, alpha, mode),
        }
    }
    pub unsafe fn LICE_Blur(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        src: *mut root::reaper_functions::LICE_IBitmap,
        dstx: ::std::os::raw::c_int,
        dsty: ::std::os::raw::c_int,
        srcx: ::std::os::raw::c_int,
        srcy: ::std::os::raw::c_int,
        srcw: ::std::os::raw::c_int,
        srch: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_Blur {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_Blur)
            )),
            Some(f) => f(dest, src, dstx, dsty, srcx, srcy, srcw, srch),
        }
    }
    pub unsafe fn LICE_BorderedRect(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        bgcolor: root::reaper_functions::LICE_pixel,
        fgcolor: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_BorderedRect {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_BorderedRect)
            )),
            Some(f) => f(dest, x, y, w, h, bgcolor, fgcolor, alpha, mode),
        }
    }
    pub unsafe fn LICE_Circle(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        cx: f32,
        cy: f32,
        r: f32,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        aa: bool,
    ) {
        match self.pointers.LICE_Circle {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_Circle)
            )),
            Some(f) => f(dest, cx, cy, r, color, alpha, mode, aa),
        }
    }
    pub unsafe fn LICE_Clear(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        color: root::reaper_functions::LICE_pixel,
    ) {
        match self.pointers.LICE_Clear {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_Clear)
            )),
            Some(f) => f(dest, color),
        }
    }
    pub unsafe fn LICE_ClearRect(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        mask: root::reaper_functions::LICE_pixel,
        orbits: root::reaper_functions::LICE_pixel,
    ) {
        match self.pointers.LICE_ClearRect {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_ClearRect)
            )),
            Some(f) => f(dest, x, y, w, h, mask, orbits),
        }
    }
    pub unsafe fn LICE_ClipLine(
        &self,
        pX1Out: *mut ::std::os::raw::c_int,
        pY1Out: *mut ::std::os::raw::c_int,
        pX2Out: *mut ::std::os::raw::c_int,
        pY2Out: *mut ::std::os::raw::c_int,
        xLo: ::std::os::raw::c_int,
        yLo: ::std::os::raw::c_int,
        xHi: ::std::os::raw::c_int,
        yHi: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.LICE_ClipLine {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_ClipLine)
            )),
            Some(f) => f(pX1Out, pY1Out, pX2Out, pY2Out, xLo, yLo, xHi, yHi),
        }
    }
    pub unsafe fn LICE_Copy(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        src: *mut root::reaper_functions::LICE_IBitmap,
    ) {
        match self.pointers.LICE_Copy {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_Copy)
            )),
            Some(f) => f(dest, src),
        }
    }
    pub fn LICE_CreateBitmap(
        &self,
        mode: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    ) -> *mut root::reaper_functions::LICE_IBitmap {
        match self.pointers.LICE_CreateBitmap {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_CreateBitmap)
            )),
            Some(f) => f(mode, w, h),
        }
    }
    pub fn LICE_CreateFont(&self) -> *mut root::reaper_functions::LICE_IFont {
        match self.pointers.LICE_CreateFont {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_CreateFont)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn LICE_DrawCBezier(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        xstart: f64,
        ystart: f64,
        xctl1: f64,
        yctl1: f64,
        xctl2: f64,
        yctl2: f64,
        xend: f64,
        yend: f64,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        aa: bool,
        tol: f64,
    ) {
        match self.pointers.LICE_DrawCBezier {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_DrawCBezier)
            )),
            Some(f) => f(
                dest, xstart, ystart, xctl1, yctl1, xctl2, yctl2, xend, yend, color, alpha, mode,
                aa, tol,
            ),
        }
    }
    pub unsafe fn LICE_DrawChar(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        c: ::std::os::raw::c_char,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_DrawChar {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_DrawChar)
            )),
            Some(f) => f(bm, x, y, c, color, alpha, mode),
        }
    }
    pub unsafe fn LICE_DrawGlyph(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alphas: *mut root::reaper_functions::LICE_pixel_chan,
        glyph_w: ::std::os::raw::c_int,
        glyph_h: ::std::os::raw::c_int,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_DrawGlyph {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_DrawGlyph)
            )),
            Some(f) => f(dest, x, y, color, alphas, glyph_w, glyph_h, alpha, mode),
        }
    }
    pub unsafe fn LICE_DrawRect(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_DrawRect {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_DrawRect)
            )),
            Some(f) => f(dest, x, y, w, h, color, alpha, mode),
        }
    }
    pub unsafe fn LICE_DrawText(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        string: *const ::std::os::raw::c_char,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_DrawText {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_DrawText)
            )),
            Some(f) => f(bm, x, y, string, color, alpha, mode),
        }
    }
    pub unsafe fn LICE_FillCBezier(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        xstart: f64,
        ystart: f64,
        xctl1: f64,
        yctl1: f64,
        xctl2: f64,
        yctl2: f64,
        xend: f64,
        yend: f64,
        yfill: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        aa: bool,
        tol: f64,
    ) {
        match self.pointers.LICE_FillCBezier {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_FillCBezier)
            )),
            Some(f) => f(
                dest, xstart, ystart, xctl1, yctl1, xctl2, yctl2, xend, yend, yfill, color, alpha,
                mode, aa, tol,
            ),
        }
    }
    pub unsafe fn LICE_FillCircle(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        cx: f32,
        cy: f32,
        r: f32,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        aa: bool,
    ) {
        match self.pointers.LICE_FillCircle {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_FillCircle)
            )),
            Some(f) => f(dest, cx, cy, r, color, alpha, mode, aa),
        }
    }
    pub unsafe fn LICE_FillConvexPolygon(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        npoints: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_FillConvexPolygon {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_FillConvexPolygon)
            )),
            Some(f) => f(dest, x, y, npoints, color, alpha, mode),
        }
    }
    pub unsafe fn LICE_FillRect(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_FillRect {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_FillRect)
            )),
            Some(f) => f(dest, x, y, w, h, color, alpha, mode),
        }
    }
    pub unsafe fn LICE_FillTrapezoid(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x1a: ::std::os::raw::c_int,
        x1b: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2a: ::std::os::raw::c_int,
        x2b: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_FillTrapezoid {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_FillTrapezoid)
            )),
            Some(f) => f(dest, x1a, x1b, y1, x2a, x2b, y2, color, alpha, mode),
        }
    }
    pub unsafe fn LICE_FillTriangle(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
        x3: ::std::os::raw::c_int,
        y3: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_FillTriangle {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_FillTriangle)
            )),
            Some(f) => f(dest, x1, y1, x2, y2, x3, y3, color, alpha, mode),
        }
    }
    pub unsafe fn LICE_GetPixel(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> root::reaper_functions::LICE_pixel {
        match self.pointers.LICE_GetPixel {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_GetPixel)
            )),
            Some(f) => f(bm, x, y),
        }
    }
    pub unsafe fn LICE_GradRect(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        dstx: ::std::os::raw::c_int,
        dsty: ::std::os::raw::c_int,
        dstw: ::std::os::raw::c_int,
        dsth: ::std::os::raw::c_int,
        ir: f32,
        ig: f32,
        ib: f32,
        ia: f32,
        drdx: f32,
        dgdx: f32,
        dbdx: f32,
        dadx: f32,
        drdy: f32,
        dgdy: f32,
        dbdy: f32,
        dady: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_GradRect {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_GradRect)
            )),
            Some(f) => f(
                dest, dstx, dsty, dstw, dsth, ir, ig, ib, ia, drdx, dgdx, dbdx, dadx, drdy, dgdy,
                dbdy, dady, mode,
            ),
        }
    }
    pub unsafe fn LICE_Line(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        aa: bool,
    ) {
        match self.pointers.LICE_Line {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_Line)
            )),
            Some(f) => f(dest, x1, y1, x2, y2, color, alpha, mode, aa),
        }
    }
    pub unsafe fn LICE_LineInt(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        aa: bool,
    ) {
        match self.pointers.LICE_LineInt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_LineInt)
            )),
            Some(f) => f(dest, x1, y1, x2, y2, color, alpha, mode, aa),
        }
    }
    pub unsafe fn LICE_LoadPNG(
        &self,
        filename: *const ::std::os::raw::c_char,
        bmp: *mut root::reaper_functions::LICE_IBitmap,
    ) -> *mut root::reaper_functions::LICE_IBitmap {
        match self.pointers.LICE_LoadPNG {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_LoadPNG)
            )),
            Some(f) => f(filename, bmp),
        }
    }
    pub unsafe fn LICE_LoadPNGFromResource(
        &self,
        hInst: root::HINSTANCE,
        resid: *const ::std::os::raw::c_char,
        bmp: *mut root::reaper_functions::LICE_IBitmap,
    ) -> *mut root::reaper_functions::LICE_IBitmap {
        match self.pointers.LICE_LoadPNGFromResource {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_LoadPNGFromResource)
            )),
            Some(f) => f(hInst, resid, bmp),
        }
    }
    pub unsafe fn LICE_MeasureText(
        &self,
        string: *const ::std::os::raw::c_char,
        w: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_MeasureText {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_MeasureText)
            )),
            Some(f) => f(string, w, h),
        }
    }
    pub unsafe fn LICE_MultiplyAddRect(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        rsc: f32,
        gsc: f32,
        bsc: f32,
        asc: f32,
        radd: f32,
        gadd: f32,
        badd: f32,
        aadd: f32,
    ) {
        match self.pointers.LICE_MultiplyAddRect {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_MultiplyAddRect)
            )),
            Some(f) => f(dest, x, y, w, h, rsc, gsc, bsc, asc, radd, gadd, badd, aadd),
        }
    }
    pub unsafe fn LICE_PutPixel(
        &self,
        bm: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        color: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_PutPixel {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_PutPixel)
            )),
            Some(f) => f(bm, x, y, color, alpha, mode),
        }
    }
    pub unsafe fn LICE_RotatedBlit(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        src: *mut root::reaper_functions::LICE_IBitmap,
        dstx: ::std::os::raw::c_int,
        dsty: ::std::os::raw::c_int,
        dstw: ::std::os::raw::c_int,
        dsth: ::std::os::raw::c_int,
        srcx: f32,
        srcy: f32,
        srcw: f32,
        srch: f32,
        angle: f32,
        cliptosourcerect: bool,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        rotxcent: f32,
        rotycent: f32,
    ) {
        match self.pointers.LICE_RotatedBlit {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_RotatedBlit)
            )),
            Some(f) => f(
                dest,
                src,
                dstx,
                dsty,
                dstw,
                dsth,
                srcx,
                srcy,
                srcw,
                srch,
                angle,
                cliptosourcerect,
                alpha,
                mode,
                rotxcent,
                rotycent,
            ),
        }
    }
    pub unsafe fn LICE_RoundRect(
        &self,
        drawbm: *mut root::reaper_functions::LICE_IBitmap,
        xpos: f32,
        ypos: f32,
        w: f32,
        h: f32,
        cornerradius: ::std::os::raw::c_int,
        col: root::reaper_functions::LICE_pixel,
        alpha: f32,
        mode: ::std::os::raw::c_int,
        aa: bool,
    ) {
        match self.pointers.LICE_RoundRect {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_RoundRect)
            )),
            Some(f) => f(drawbm, xpos, ypos, w, h, cornerradius, col, alpha, mode, aa),
        }
    }
    pub unsafe fn LICE_ScaledBlit(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        src: *mut root::reaper_functions::LICE_IBitmap,
        dstx: ::std::os::raw::c_int,
        dsty: ::std::os::raw::c_int,
        dstw: ::std::os::raw::c_int,
        dsth: ::std::os::raw::c_int,
        srcx: f32,
        srcy: f32,
        srcw: f32,
        srch: f32,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.LICE_ScaledBlit {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_ScaledBlit)
            )),
            Some(f) => f(
                dest, src, dstx, dsty, dstw, dsth, srcx, srcy, srcw, srch, alpha, mode,
            ),
        }
    }
    pub unsafe fn LICE_SimpleFill(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        newcolor: root::reaper_functions::LICE_pixel,
        comparemask: root::reaper_functions::LICE_pixel,
        keepmask: root::reaper_functions::LICE_pixel,
    ) {
        match self.pointers.LICE_SimpleFill {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(LICE_SimpleFill)
            )),
            Some(f) => f(dest, x, y, newcolor, comparemask, keepmask),
        }
    }
    pub unsafe fn Loop_OnArrow(
        &self,
        project: *mut root::ReaProject,
        direction: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.Loop_OnArrow {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Loop_OnArrow)
            )),
            Some(f) => f(project, direction),
        }
    }
    pub fn Main_OnCommand(&self, command: ::std::os::raw::c_int, flag: ::std::os::raw::c_int) {
        match self.pointers.Main_OnCommand {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Main_OnCommand)
            )),
            Some(f) => f(command, flag),
        }
    }
    pub unsafe fn Main_OnCommandEx(
        &self,
        command: ::std::os::raw::c_int,
        flag: ::std::os::raw::c_int,
        proj: *mut root::ReaProject,
    ) {
        match self.pointers.Main_OnCommandEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Main_OnCommandEx)
            )),
            Some(f) => f(command, flag, proj),
        }
    }
    pub unsafe fn Main_openProject(&self, name: *const ::std::os::raw::c_char) {
        match self.pointers.Main_openProject {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Main_openProject)
            )),
            Some(f) => f(name),
        }
    }
    pub unsafe fn Main_SaveProject(
        &self,
        proj: *mut root::ReaProject,
        forceSaveAsInOptional: bool,
    ) {
        match self.pointers.Main_SaveProject {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Main_SaveProject)
            )),
            Some(f) => f(proj, forceSaveAsInOptional),
        }
    }
    pub fn Main_UpdateLoopInfo(&self, ignoremask: ::std::os::raw::c_int) {
        match self.pointers.Main_UpdateLoopInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Main_UpdateLoopInfo)
            )),
            Some(f) => f(ignoremask),
        }
    }
    pub unsafe fn MarkProjectDirty(&self, proj: *mut root::ReaProject) {
        match self.pointers.MarkProjectDirty {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MarkProjectDirty)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn MarkTrackItemsDirty(
        &self,
        track: *mut root::MediaTrack,
        item: *mut root::MediaItem,
    ) {
        match self.pointers.MarkTrackItemsDirty {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MarkTrackItemsDirty)
            )),
            Some(f) => f(track, item),
        }
    }
    pub unsafe fn Master_GetPlayRate(&self, project: *mut root::ReaProject) -> f64 {
        match self.pointers.Master_GetPlayRate {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Master_GetPlayRate)
            )),
            Some(f) => f(project),
        }
    }
    pub unsafe fn Master_GetPlayRateAtTime(&self, time_s: f64, proj: *mut root::ReaProject) -> f64 {
        match self.pointers.Master_GetPlayRateAtTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Master_GetPlayRateAtTime)
            )),
            Some(f) => f(time_s, proj),
        }
    }
    pub fn Master_GetTempo(&self) -> f64 {
        match self.pointers.Master_GetTempo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Master_GetTempo)
            )),
            Some(f) => f(),
        }
    }
    pub fn Master_NormalizePlayRate(&self, playrate: f64, isnormalized: bool) -> f64 {
        match self.pointers.Master_NormalizePlayRate {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Master_NormalizePlayRate)
            )),
            Some(f) => f(playrate, isnormalized),
        }
    }
    pub fn Master_NormalizeTempo(&self, bpm: f64, isnormalized: bool) -> f64 {
        match self.pointers.Master_NormalizeTempo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Master_NormalizeTempo)
            )),
            Some(f) => f(bpm, isnormalized),
        }
    }
    pub unsafe fn MB(
        &self,
        msg: *const ::std::os::raw::c_char,
        title: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.MB {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MB)
            )),
            Some(f) => f(msg, title, type_),
        }
    }
    pub unsafe fn MediaItemDescendsFromTrack(
        &self,
        item: *mut root::MediaItem,
        track: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.MediaItemDescendsFromTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MediaItemDescendsFromTrack)
            )),
            Some(f) => f(item, track),
        }
    }
    pub unsafe fn MIDI_CountEvts(
        &self,
        take: *mut root::MediaItem_Take,
        notecntOut: *mut ::std::os::raw::c_int,
        ccevtcntOut: *mut ::std::os::raw::c_int,
        textsyxevtcntOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.MIDI_CountEvts {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_CountEvts)
            )),
            Some(f) => f(take, notecntOut, ccevtcntOut, textsyxevtcntOut),
        }
    }
    pub unsafe fn MIDI_DeleteCC(
        &self,
        take: *mut root::MediaItem_Take,
        ccidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_DeleteCC {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_DeleteCC)
            )),
            Some(f) => f(take, ccidx),
        }
    }
    pub unsafe fn MIDI_DeleteEvt(
        &self,
        take: *mut root::MediaItem_Take,
        evtidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_DeleteEvt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_DeleteEvt)
            )),
            Some(f) => f(take, evtidx),
        }
    }
    pub unsafe fn MIDI_DeleteNote(
        &self,
        take: *mut root::MediaItem_Take,
        noteidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_DeleteNote {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_DeleteNote)
            )),
            Some(f) => f(take, noteidx),
        }
    }
    pub unsafe fn MIDI_DeleteTextSysexEvt(
        &self,
        take: *mut root::MediaItem_Take,
        textsyxevtidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_DeleteTextSysexEvt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_DeleteTextSysexEvt)
            )),
            Some(f) => f(take, textsyxevtidx),
        }
    }
    pub unsafe fn MIDI_DisableSort(&self, take: *mut root::MediaItem_Take) {
        match self.pointers.MIDI_DisableSort {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_DisableSort)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn MIDI_EnumSelCC(
        &self,
        take: *mut root::MediaItem_Take,
        ccidx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.MIDI_EnumSelCC {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_EnumSelCC)
            )),
            Some(f) => f(take, ccidx),
        }
    }
    pub unsafe fn MIDI_EnumSelEvts(
        &self,
        take: *mut root::MediaItem_Take,
        evtidx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.MIDI_EnumSelEvts {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_EnumSelEvts)
            )),
            Some(f) => f(take, evtidx),
        }
    }
    pub unsafe fn MIDI_EnumSelNotes(
        &self,
        take: *mut root::MediaItem_Take,
        noteidx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.MIDI_EnumSelNotes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_EnumSelNotes)
            )),
            Some(f) => f(take, noteidx),
        }
    }
    pub unsafe fn MIDI_EnumSelTextSysexEvts(
        &self,
        take: *mut root::MediaItem_Take,
        textsyxidx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.MIDI_EnumSelTextSysexEvts {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_EnumSelTextSysexEvts)
            )),
            Some(f) => f(take, textsyxidx),
        }
    }
    pub fn MIDI_eventlist_Create(&self) -> *mut root::MIDI_eventlist {
        match self.pointers.MIDI_eventlist_Create {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_eventlist_Create)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn MIDI_eventlist_Destroy(&self, evtlist: *mut root::MIDI_eventlist) {
        match self.pointers.MIDI_eventlist_Destroy {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_eventlist_Destroy)
            )),
            Some(f) => f(evtlist),
        }
    }
    pub unsafe fn MIDI_GetAllEvts(
        &self,
        take: *mut root::MediaItem_Take,
        bufNeedBig: *mut ::std::os::raw::c_char,
        bufNeedBig_sz: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_GetAllEvts {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetAllEvts)
            )),
            Some(f) => f(take, bufNeedBig, bufNeedBig_sz),
        }
    }
    pub unsafe fn MIDI_GetCC(
        &self,
        take: *mut root::MediaItem_Take,
        ccidx: ::std::os::raw::c_int,
        selectedOut: *mut bool,
        mutedOut: *mut bool,
        ppqposOut: *mut f64,
        chanmsgOut: *mut ::std::os::raw::c_int,
        chanOut: *mut ::std::os::raw::c_int,
        msg2Out: *mut ::std::os::raw::c_int,
        msg3Out: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_GetCC {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetCC)
            )),
            Some(f) => f(
                take,
                ccidx,
                selectedOut,
                mutedOut,
                ppqposOut,
                chanmsgOut,
                chanOut,
                msg2Out,
                msg3Out,
            ),
        }
    }
    pub unsafe fn MIDI_GetCCShape(
        &self,
        take: *mut root::MediaItem_Take,
        ccidx: ::std::os::raw::c_int,
        shapeOut: *mut ::std::os::raw::c_int,
        beztensionOut: *mut f64,
    ) -> bool {
        match self.pointers.MIDI_GetCCShape {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetCCShape)
            )),
            Some(f) => f(take, ccidx, shapeOut, beztensionOut),
        }
    }
    pub unsafe fn MIDI_GetEvt(
        &self,
        take: *mut root::MediaItem_Take,
        evtidx: ::std::os::raw::c_int,
        selectedOut: *mut bool,
        mutedOut: *mut bool,
        ppqposOut: *mut f64,
        msg: *mut ::std::os::raw::c_char,
        msg_sz: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_GetEvt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetEvt)
            )),
            Some(f) => f(take, evtidx, selectedOut, mutedOut, ppqposOut, msg, msg_sz),
        }
    }
    pub unsafe fn MIDI_GetGrid(
        &self,
        take: *mut root::MediaItem_Take,
        swingOutOptional: *mut f64,
        noteLenOutOptional: *mut f64,
    ) -> f64 {
        match self.pointers.MIDI_GetGrid {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetGrid)
            )),
            Some(f) => f(take, swingOutOptional, noteLenOutOptional),
        }
    }
    pub unsafe fn MIDI_GetHash(
        &self,
        take: *mut root::MediaItem_Take,
        notesonly: bool,
        hash: *mut ::std::os::raw::c_char,
        hash_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_GetHash {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetHash)
            )),
            Some(f) => f(take, notesonly, hash, hash_sz),
        }
    }
    pub unsafe fn MIDI_GetNote(
        &self,
        take: *mut root::MediaItem_Take,
        noteidx: ::std::os::raw::c_int,
        selectedOut: *mut bool,
        mutedOut: *mut bool,
        startppqposOut: *mut f64,
        endppqposOut: *mut f64,
        chanOut: *mut ::std::os::raw::c_int,
        pitchOut: *mut ::std::os::raw::c_int,
        velOut: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_GetNote {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetNote)
            )),
            Some(f) => f(
                take,
                noteidx,
                selectedOut,
                mutedOut,
                startppqposOut,
                endppqposOut,
                chanOut,
                pitchOut,
                velOut,
            ),
        }
    }
    pub unsafe fn MIDI_GetPPQPos_EndOfMeasure(
        &self,
        take: *mut root::MediaItem_Take,
        ppqpos: f64,
    ) -> f64 {
        match self.pointers.MIDI_GetPPQPos_EndOfMeasure {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetPPQPos_EndOfMeasure)
            )),
            Some(f) => f(take, ppqpos),
        }
    }
    pub unsafe fn MIDI_GetPPQPos_StartOfMeasure(
        &self,
        take: *mut root::MediaItem_Take,
        ppqpos: f64,
    ) -> f64 {
        match self.pointers.MIDI_GetPPQPos_StartOfMeasure {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetPPQPos_StartOfMeasure)
            )),
            Some(f) => f(take, ppqpos),
        }
    }
    pub unsafe fn MIDI_GetPPQPosFromProjQN(
        &self,
        take: *mut root::MediaItem_Take,
        projqn: f64,
    ) -> f64 {
        match self.pointers.MIDI_GetPPQPosFromProjQN {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetPPQPosFromProjQN)
            )),
            Some(f) => f(take, projqn),
        }
    }
    pub unsafe fn MIDI_GetPPQPosFromProjTime(
        &self,
        take: *mut root::MediaItem_Take,
        projtime: f64,
    ) -> f64 {
        match self.pointers.MIDI_GetPPQPosFromProjTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetPPQPosFromProjTime)
            )),
            Some(f) => f(take, projtime),
        }
    }
    pub unsafe fn MIDI_GetProjQNFromPPQPos(
        &self,
        take: *mut root::MediaItem_Take,
        ppqpos: f64,
    ) -> f64 {
        match self.pointers.MIDI_GetProjQNFromPPQPos {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetProjQNFromPPQPos)
            )),
            Some(f) => f(take, ppqpos),
        }
    }
    pub unsafe fn MIDI_GetProjTimeFromPPQPos(
        &self,
        take: *mut root::MediaItem_Take,
        ppqpos: f64,
    ) -> f64 {
        match self.pointers.MIDI_GetProjTimeFromPPQPos {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetProjTimeFromPPQPos)
            )),
            Some(f) => f(take, ppqpos),
        }
    }
    pub unsafe fn MIDI_GetScale(
        &self,
        take: *mut root::MediaItem_Take,
        rootOut: *mut ::std::os::raw::c_int,
        scaleOut: *mut ::std::os::raw::c_int,
        name: *mut ::std::os::raw::c_char,
        name_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_GetScale {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetScale)
            )),
            Some(f) => f(take, rootOut, scaleOut, name, name_sz),
        }
    }
    pub unsafe fn MIDI_GetTextSysexEvt(
        &self,
        take: *mut root::MediaItem_Take,
        textsyxevtidx: ::std::os::raw::c_int,
        selectedOutOptional: *mut bool,
        mutedOutOptional: *mut bool,
        ppqposOutOptional: *mut f64,
        typeOutOptional: *mut ::std::os::raw::c_int,
        msgOptional: *mut ::std::os::raw::c_char,
        msgOptional_sz: *mut ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_GetTextSysexEvt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetTextSysexEvt)
            )),
            Some(f) => f(
                take,
                textsyxevtidx,
                selectedOutOptional,
                mutedOutOptional,
                ppqposOutOptional,
                typeOutOptional,
                msgOptional,
                msgOptional_sz,
            ),
        }
    }
    pub unsafe fn MIDI_GetTrackHash(
        &self,
        track: *mut root::MediaTrack,
        notesonly: bool,
        hash: *mut ::std::os::raw::c_char,
        hash_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_GetTrackHash {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_GetTrackHash)
            )),
            Some(f) => f(track, notesonly, hash, hash_sz),
        }
    }
    pub unsafe fn MIDI_InsertCC(
        &self,
        take: *mut root::MediaItem_Take,
        selected: bool,
        muted: bool,
        ppqpos: f64,
        chanmsg: ::std::os::raw::c_int,
        chan: ::std::os::raw::c_int,
        msg2: ::std::os::raw::c_int,
        msg3: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_InsertCC {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_InsertCC)
            )),
            Some(f) => f(take, selected, muted, ppqpos, chanmsg, chan, msg2, msg3),
        }
    }
    pub unsafe fn MIDI_InsertEvt(
        &self,
        take: *mut root::MediaItem_Take,
        selected: bool,
        muted: bool,
        ppqpos: f64,
        bytestr: *const ::std::os::raw::c_char,
        bytestr_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_InsertEvt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_InsertEvt)
            )),
            Some(f) => f(take, selected, muted, ppqpos, bytestr, bytestr_sz),
        }
    }
    pub unsafe fn MIDI_InsertNote(
        &self,
        take: *mut root::MediaItem_Take,
        selected: bool,
        muted: bool,
        startppqpos: f64,
        endppqpos: f64,
        chan: ::std::os::raw::c_int,
        pitch: ::std::os::raw::c_int,
        vel: ::std::os::raw::c_int,
        noSortInOptional: *const bool,
    ) -> bool {
        match self.pointers.MIDI_InsertNote {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_InsertNote)
            )),
            Some(f) => f(
                take,
                selected,
                muted,
                startppqpos,
                endppqpos,
                chan,
                pitch,
                vel,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn MIDI_InsertTextSysexEvt(
        &self,
        take: *mut root::MediaItem_Take,
        selected: bool,
        muted: bool,
        ppqpos: f64,
        type_: ::std::os::raw::c_int,
        bytestr: *const ::std::os::raw::c_char,
        bytestr_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_InsertTextSysexEvt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_InsertTextSysexEvt)
            )),
            Some(f) => f(take, selected, muted, ppqpos, type_, bytestr, bytestr_sz),
        }
    }
    pub fn midi_reinit(&self) {
        match self.pointers.midi_reinit {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(midi_reinit)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn MIDI_SelectAll(&self, take: *mut root::MediaItem_Take, select: bool) {
        match self.pointers.MIDI_SelectAll {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_SelectAll)
            )),
            Some(f) => f(take, select),
        }
    }
    pub unsafe fn MIDI_SetAllEvts(
        &self,
        take: *mut root::MediaItem_Take,
        buf: *const ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDI_SetAllEvts {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_SetAllEvts)
            )),
            Some(f) => f(take, buf, buf_sz),
        }
    }
    pub unsafe fn MIDI_SetCC(
        &self,
        take: *mut root::MediaItem_Take,
        ccidx: ::std::os::raw::c_int,
        selectedInOptional: *const bool,
        mutedInOptional: *const bool,
        ppqposInOptional: *const f64,
        chanmsgInOptional: *const ::std::os::raw::c_int,
        chanInOptional: *const ::std::os::raw::c_int,
        msg2InOptional: *const ::std::os::raw::c_int,
        msg3InOptional: *const ::std::os::raw::c_int,
        noSortInOptional: *const bool,
    ) -> bool {
        match self.pointers.MIDI_SetCC {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_SetCC)
            )),
            Some(f) => f(
                take,
                ccidx,
                selectedInOptional,
                mutedInOptional,
                ppqposInOptional,
                chanmsgInOptional,
                chanInOptional,
                msg2InOptional,
                msg3InOptional,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn MIDI_SetCCShape(
        &self,
        take: *mut root::MediaItem_Take,
        ccidx: ::std::os::raw::c_int,
        shape: ::std::os::raw::c_int,
        beztension: f64,
        noSortInOptional: *const bool,
    ) -> bool {
        match self.pointers.MIDI_SetCCShape {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_SetCCShape)
            )),
            Some(f) => f(take, ccidx, shape, beztension, noSortInOptional),
        }
    }
    pub unsafe fn MIDI_SetEvt(
        &self,
        take: *mut root::MediaItem_Take,
        evtidx: ::std::os::raw::c_int,
        selectedInOptional: *const bool,
        mutedInOptional: *const bool,
        ppqposInOptional: *const f64,
        msgOptional: *const ::std::os::raw::c_char,
        msgOptional_sz: ::std::os::raw::c_int,
        noSortInOptional: *const bool,
    ) -> bool {
        match self.pointers.MIDI_SetEvt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_SetEvt)
            )),
            Some(f) => f(
                take,
                evtidx,
                selectedInOptional,
                mutedInOptional,
                ppqposInOptional,
                msgOptional,
                msgOptional_sz,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn MIDI_SetItemExtents(
        &self,
        item: *mut root::MediaItem,
        startQN: f64,
        endQN: f64,
    ) -> bool {
        match self.pointers.MIDI_SetItemExtents {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_SetItemExtents)
            )),
            Some(f) => f(item, startQN, endQN),
        }
    }
    pub unsafe fn MIDI_SetNote(
        &self,
        take: *mut root::MediaItem_Take,
        noteidx: ::std::os::raw::c_int,
        selectedInOptional: *const bool,
        mutedInOptional: *const bool,
        startppqposInOptional: *const f64,
        endppqposInOptional: *const f64,
        chanInOptional: *const ::std::os::raw::c_int,
        pitchInOptional: *const ::std::os::raw::c_int,
        velInOptional: *const ::std::os::raw::c_int,
        noSortInOptional: *const bool,
    ) -> bool {
        match self.pointers.MIDI_SetNote {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_SetNote)
            )),
            Some(f) => f(
                take,
                noteidx,
                selectedInOptional,
                mutedInOptional,
                startppqposInOptional,
                endppqposInOptional,
                chanInOptional,
                pitchInOptional,
                velInOptional,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn MIDI_SetTextSysexEvt(
        &self,
        take: *mut root::MediaItem_Take,
        textsyxevtidx: ::std::os::raw::c_int,
        selectedInOptional: *const bool,
        mutedInOptional: *const bool,
        ppqposInOptional: *const f64,
        typeInOptional: *const ::std::os::raw::c_int,
        msgOptional: *const ::std::os::raw::c_char,
        msgOptional_sz: ::std::os::raw::c_int,
        noSortInOptional: *const bool,
    ) -> bool {
        match self.pointers.MIDI_SetTextSysexEvt {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_SetTextSysexEvt)
            )),
            Some(f) => f(
                take,
                textsyxevtidx,
                selectedInOptional,
                mutedInOptional,
                ppqposInOptional,
                typeInOptional,
                msgOptional,
                msgOptional_sz,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn MIDI_Sort(&self, take: *mut root::MediaItem_Take) {
        match self.pointers.MIDI_Sort {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDI_Sort)
            )),
            Some(f) => f(take),
        }
    }
    pub fn MIDIEditor_GetActive(&self) -> root::HWND {
        match self.pointers.MIDIEditor_GetActive {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDIEditor_GetActive)
            )),
            Some(f) => f(),
        }
    }
    pub fn MIDIEditor_GetMode(&self, midieditor: root::HWND) -> ::std::os::raw::c_int {
        match self.pointers.MIDIEditor_GetMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDIEditor_GetMode)
            )),
            Some(f) => f(midieditor),
        }
    }
    pub unsafe fn MIDIEditor_GetSetting_int(
        &self,
        midieditor: root::HWND,
        setting_desc: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        match self.pointers.MIDIEditor_GetSetting_int {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDIEditor_GetSetting_int)
            )),
            Some(f) => f(midieditor, setting_desc),
        }
    }
    pub unsafe fn MIDIEditor_GetSetting_str(
        &self,
        midieditor: root::HWND,
        setting_desc: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDIEditor_GetSetting_str {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDIEditor_GetSetting_str)
            )),
            Some(f) => f(midieditor, setting_desc, buf, buf_sz),
        }
    }
    pub fn MIDIEditor_GetTake(&self, midieditor: root::HWND) -> *mut root::MediaItem_Take {
        match self.pointers.MIDIEditor_GetTake {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDIEditor_GetTake)
            )),
            Some(f) => f(midieditor),
        }
    }
    pub fn MIDIEditor_LastFocused_OnCommand(
        &self,
        command_id: ::std::os::raw::c_int,
        islistviewcommand: bool,
    ) -> bool {
        match self.pointers.MIDIEditor_LastFocused_OnCommand {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDIEditor_LastFocused_OnCommand)
            )),
            Some(f) => f(command_id, islistviewcommand),
        }
    }
    pub fn MIDIEditor_OnCommand(
        &self,
        midieditor: root::HWND,
        command_id: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDIEditor_OnCommand {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDIEditor_OnCommand)
            )),
            Some(f) => f(midieditor, command_id),
        }
    }
    pub unsafe fn MIDIEditor_SetSetting_int(
        &self,
        midieditor: root::HWND,
        setting_desc: *const ::std::os::raw::c_char,
        setting: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.MIDIEditor_SetSetting_int {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MIDIEditor_SetSetting_int)
            )),
            Some(f) => f(midieditor, setting_desc, setting),
        }
    }
    pub unsafe fn mkpanstr(&self, strNeed64: *mut ::std::os::raw::c_char, pan: f64) {
        match self.pointers.mkpanstr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(mkpanstr)
            )),
            Some(f) => f(strNeed64, pan),
        }
    }
    pub unsafe fn mkvolpanstr(&self, strNeed64: *mut ::std::os::raw::c_char, vol: f64, pan: f64) {
        match self.pointers.mkvolpanstr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(mkvolpanstr)
            )),
            Some(f) => f(strNeed64, vol, pan),
        }
    }
    pub unsafe fn mkvolstr(&self, strNeed64: *mut ::std::os::raw::c_char, vol: f64) {
        match self.pointers.mkvolstr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(mkvolstr)
            )),
            Some(f) => f(strNeed64, vol),
        }
    }
    pub fn MoveEditCursor(&self, adjamt: f64, dosel: bool) {
        match self.pointers.MoveEditCursor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MoveEditCursor)
            )),
            Some(f) => f(adjamt, dosel),
        }
    }
    pub unsafe fn MoveMediaItemToTrack(
        &self,
        item: *mut root::MediaItem,
        desttr: *mut root::MediaTrack,
    ) -> bool {
        match self.pointers.MoveMediaItemToTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MoveMediaItemToTrack)
            )),
            Some(f) => f(item, desttr),
        }
    }
    pub fn MuteAllTracks(&self, mute: bool) {
        match self.pointers.MuteAllTracks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(MuteAllTracks)
            )),
            Some(f) => f(mute),
        }
    }
    pub unsafe fn my_getViewport(
        &self,
        r: *mut root::RECT,
        sr: *const root::RECT,
        wantWorkArea: bool,
    ) {
        match self.pointers.my_getViewport {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(my_getViewport)
            )),
            Some(f) => f(r, sr, wantWorkArea),
        }
    }
    pub unsafe fn NamedCommandLookup(
        &self,
        command_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        match self.pointers.NamedCommandLookup {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(NamedCommandLookup)
            )),
            Some(f) => f(command_name),
        }
    }
    pub fn OnPauseButton(&self) {
        match self.pointers.OnPauseButton {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OnPauseButton)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn OnPauseButtonEx(&self, proj: *mut root::ReaProject) {
        match self.pointers.OnPauseButtonEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OnPauseButtonEx)
            )),
            Some(f) => f(proj),
        }
    }
    pub fn OnPlayButton(&self) {
        match self.pointers.OnPlayButton {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OnPlayButton)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn OnPlayButtonEx(&self, proj: *mut root::ReaProject) {
        match self.pointers.OnPlayButtonEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OnPlayButtonEx)
            )),
            Some(f) => f(proj),
        }
    }
    pub fn OnStopButton(&self) {
        match self.pointers.OnStopButton {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OnStopButton)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn OnStopButtonEx(&self, proj: *mut root::ReaProject) {
        match self.pointers.OnStopButtonEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OnStopButtonEx)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn OpenColorThemeFile(&self, fn_: *const ::std::os::raw::c_char) -> bool {
        match self.pointers.OpenColorThemeFile {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OpenColorThemeFile)
            )),
            Some(f) => f(fn_),
        }
    }
    pub unsafe fn OpenMediaExplorer(
        &self,
        mediafn: *const ::std::os::raw::c_char,
        play: bool,
    ) -> root::HWND {
        match self.pointers.OpenMediaExplorer {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OpenMediaExplorer)
            )),
            Some(f) => f(mediafn, play),
        }
    }
    pub unsafe fn OscLocalMessageToHost(
        &self,
        message: *const ::std::os::raw::c_char,
        valueInOptional: *const f64,
    ) {
        match self.pointers.OscLocalMessageToHost {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(OscLocalMessageToHost)
            )),
            Some(f) => f(message, valueInOptional),
        }
    }
    pub unsafe fn parse_timestr(&self, buf: *const ::std::os::raw::c_char) -> f64 {
        match self.pointers.parse_timestr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(parse_timestr)
            )),
            Some(f) => f(buf),
        }
    }
    pub unsafe fn parse_timestr_len(
        &self,
        buf: *const ::std::os::raw::c_char,
        offset: f64,
        modeoverride: ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.parse_timestr_len {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(parse_timestr_len)
            )),
            Some(f) => f(buf, offset, modeoverride),
        }
    }
    pub unsafe fn parse_timestr_pos(
        &self,
        buf: *const ::std::os::raw::c_char,
        modeoverride: ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.parse_timestr_pos {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(parse_timestr_pos)
            )),
            Some(f) => f(buf, modeoverride),
        }
    }
    pub unsafe fn parsepanstr(&self, str: *const ::std::os::raw::c_char) -> f64 {
        match self.pointers.parsepanstr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(parsepanstr)
            )),
            Some(f) => f(str),
        }
    }
    pub unsafe fn PCM_Sink_Create(
        &self,
        filename: *const ::std::os::raw::c_char,
        cfg: *const ::std::os::raw::c_char,
        cfg_sz: ::std::os::raw::c_int,
        nch: ::std::os::raw::c_int,
        srate: ::std::os::raw::c_int,
        buildpeaks: bool,
    ) -> *mut root::PCM_sink {
        match self.pointers.PCM_Sink_Create {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Sink_Create)
            )),
            Some(f) => f(filename, cfg, cfg_sz, nch, srate, buildpeaks),
        }
    }
    pub unsafe fn PCM_Sink_CreateEx(
        &self,
        proj: *mut root::ReaProject,
        filename: *const ::std::os::raw::c_char,
        cfg: *const ::std::os::raw::c_char,
        cfg_sz: ::std::os::raw::c_int,
        nch: ::std::os::raw::c_int,
        srate: ::std::os::raw::c_int,
        buildpeaks: bool,
    ) -> *mut root::PCM_sink {
        match self.pointers.PCM_Sink_CreateEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Sink_CreateEx)
            )),
            Some(f) => f(proj, filename, cfg, cfg_sz, nch, srate, buildpeaks),
        }
    }
    pub unsafe fn PCM_Sink_CreateMIDIFile(
        &self,
        filename: *const ::std::os::raw::c_char,
        cfg: *const ::std::os::raw::c_char,
        cfg_sz: ::std::os::raw::c_int,
        bpm: f64,
        div: ::std::os::raw::c_int,
    ) -> *mut root::PCM_sink {
        match self.pointers.PCM_Sink_CreateMIDIFile {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Sink_CreateMIDIFile)
            )),
            Some(f) => f(filename, cfg, cfg_sz, bpm, div),
        }
    }
    pub unsafe fn PCM_Sink_CreateMIDIFileEx(
        &self,
        proj: *mut root::ReaProject,
        filename: *const ::std::os::raw::c_char,
        cfg: *const ::std::os::raw::c_char,
        cfg_sz: ::std::os::raw::c_int,
        bpm: f64,
        div: ::std::os::raw::c_int,
    ) -> *mut root::PCM_sink {
        match self.pointers.PCM_Sink_CreateMIDIFileEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Sink_CreateMIDIFileEx)
            )),
            Some(f) => f(proj, filename, cfg, cfg_sz, bpm, div),
        }
    }
    pub unsafe fn PCM_Sink_Enum(
        &self,
        idx: ::std::os::raw::c_int,
        descstrOut: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uint {
        match self.pointers.PCM_Sink_Enum {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Sink_Enum)
            )),
            Some(f) => f(idx, descstrOut),
        }
    }
    pub unsafe fn PCM_Sink_GetExtension(
        &self,
        data: *const ::std::os::raw::c_char,
        data_sz: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.PCM_Sink_GetExtension {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Sink_GetExtension)
            )),
            Some(f) => f(data, data_sz),
        }
    }
    pub unsafe fn PCM_Sink_ShowConfig(
        &self,
        cfg: *const ::std::os::raw::c_char,
        cfg_sz: ::std::os::raw::c_int,
        hwndParent: root::HWND,
    ) -> root::HWND {
        match self.pointers.PCM_Sink_ShowConfig {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Sink_ShowConfig)
            )),
            Some(f) => f(cfg, cfg_sz, hwndParent),
        }
    }
    pub unsafe fn PCM_Source_CreateFromFile(
        &self,
        filename: *const ::std::os::raw::c_char,
    ) -> *mut root::PCM_source {
        match self.pointers.PCM_Source_CreateFromFile {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Source_CreateFromFile)
            )),
            Some(f) => f(filename),
        }
    }
    pub unsafe fn PCM_Source_CreateFromFileEx(
        &self,
        filename: *const ::std::os::raw::c_char,
        forcenoMidiImp: bool,
    ) -> *mut root::PCM_source {
        match self.pointers.PCM_Source_CreateFromFileEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Source_CreateFromFileEx)
            )),
            Some(f) => f(filename, forcenoMidiImp),
        }
    }
    pub unsafe fn PCM_Source_CreateFromSimple(
        &self,
        dec: *mut root::ISimpleMediaDecoder,
        fn_: *const ::std::os::raw::c_char,
    ) -> *mut root::PCM_source {
        match self.pointers.PCM_Source_CreateFromSimple {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Source_CreateFromSimple)
            )),
            Some(f) => f(dec, fn_),
        }
    }
    pub unsafe fn PCM_Source_CreateFromType(
        &self,
        sourcetype: *const ::std::os::raw::c_char,
    ) -> *mut root::PCM_source {
        match self.pointers.PCM_Source_CreateFromType {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Source_CreateFromType)
            )),
            Some(f) => f(sourcetype),
        }
    }
    pub unsafe fn PCM_Source_Destroy(&self, src: *mut root::PCM_source) {
        match self.pointers.PCM_Source_Destroy {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Source_Destroy)
            )),
            Some(f) => f(src),
        }
    }
    pub unsafe fn PCM_Source_GetPeaks(
        &self,
        src: *mut root::PCM_source,
        peakrate: f64,
        starttime: f64,
        numchannels: ::std::os::raw::c_int,
        numsamplesperchannel: ::std::os::raw::c_int,
        want_extra_type: ::std::os::raw::c_int,
        buf: *mut f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.PCM_Source_GetPeaks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Source_GetPeaks)
            )),
            Some(f) => f(
                src,
                peakrate,
                starttime,
                numchannels,
                numsamplesperchannel,
                want_extra_type,
                buf,
            ),
        }
    }
    pub unsafe fn PCM_Source_GetSectionInfo(
        &self,
        src: *mut root::PCM_source,
        offsOut: *mut f64,
        lenOut: *mut f64,
        revOut: *mut bool,
    ) -> bool {
        match self.pointers.PCM_Source_GetSectionInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PCM_Source_GetSectionInfo)
            )),
            Some(f) => f(src, offsOut, lenOut, revOut),
        }
    }
    pub unsafe fn PeakBuild_Create(
        &self,
        src: *mut root::PCM_source,
        fn_: *const ::std::os::raw::c_char,
        srate: ::std::os::raw::c_int,
        nch: ::std::os::raw::c_int,
    ) -> *mut root::REAPER_PeakBuild_Interface {
        match self.pointers.PeakBuild_Create {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PeakBuild_Create)
            )),
            Some(f) => f(src, fn_, srate, nch),
        }
    }
    pub unsafe fn PeakBuild_CreateEx(
        &self,
        src: *mut root::PCM_source,
        fn_: *const ::std::os::raw::c_char,
        srate: ::std::os::raw::c_int,
        nch: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> *mut root::REAPER_PeakBuild_Interface {
        match self.pointers.PeakBuild_CreateEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PeakBuild_CreateEx)
            )),
            Some(f) => f(src, fn_, srate, nch, flags),
        }
    }
    pub unsafe fn PeakGet_Create(
        &self,
        fn_: *const ::std::os::raw::c_char,
        srate: ::std::os::raw::c_int,
        nch: ::std::os::raw::c_int,
    ) -> *mut root::REAPER_PeakGet_Interface {
        match self.pointers.PeakGet_Create {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PeakGet_Create)
            )),
            Some(f) => f(fn_, srate, nch),
        }
    }
    pub fn PitchShiftSubModeMenu(
        &self,
        hwnd: root::HWND,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
        submode_sel: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.PitchShiftSubModeMenu {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PitchShiftSubModeMenu)
            )),
            Some(f) => f(hwnd, x, y, mode, submode_sel),
        }
    }
    pub unsafe fn PlayPreview(
        &self,
        preview: *mut root::preview_register_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.PlayPreview {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PlayPreview)
            )),
            Some(f) => f(preview),
        }
    }
    pub unsafe fn PlayPreviewEx(
        &self,
        preview: *mut root::preview_register_t,
        bufflags: ::std::os::raw::c_int,
        MSI: f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.PlayPreviewEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PlayPreviewEx)
            )),
            Some(f) => f(preview, bufflags, MSI),
        }
    }
    pub unsafe fn PlayTrackPreview(
        &self,
        preview: *mut root::preview_register_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.PlayTrackPreview {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PlayTrackPreview)
            )),
            Some(f) => f(preview),
        }
    }
    pub unsafe fn PlayTrackPreview2(
        &self,
        proj: *mut root::ReaProject,
        preview: *mut root::preview_register_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.PlayTrackPreview2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PlayTrackPreview2)
            )),
            Some(f) => f(proj, preview),
        }
    }
    pub unsafe fn PlayTrackPreview2Ex(
        &self,
        proj: *mut root::ReaProject,
        preview: *mut root::preview_register_t,
        flags: ::std::os::raw::c_int,
        msi: f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.PlayTrackPreview2Ex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PlayTrackPreview2Ex)
            )),
            Some(f) => f(proj, preview, flags, msi),
        }
    }
    pub unsafe fn plugin_getapi(
        &self,
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.plugin_getapi {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(plugin_getapi)
            )),
            Some(f) => f(name),
        }
    }
    pub fn plugin_getFilterList(&self) -> *const ::std::os::raw::c_char {
        match self.pointers.plugin_getFilterList {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(plugin_getFilterList)
            )),
            Some(f) => f(),
        }
    }
    pub fn plugin_getImportableProjectFilterList(&self) -> *const ::std::os::raw::c_char {
        match self.pointers.plugin_getImportableProjectFilterList {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(plugin_getImportableProjectFilterList)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn plugin_register(
        &self,
        name: *const ::std::os::raw::c_char,
        infostruct: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        match self.pointers.plugin_register {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(plugin_register)
            )),
            Some(f) => f(name, infostruct),
        }
    }
    pub fn PluginWantsAlwaysRunFx(&self, amt: ::std::os::raw::c_int) {
        match self.pointers.PluginWantsAlwaysRunFx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PluginWantsAlwaysRunFx)
            )),
            Some(f) => f(amt),
        }
    }
    pub fn PreventUIRefresh(&self, prevent_count: ::std::os::raw::c_int) {
        match self.pointers.PreventUIRefresh {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(PreventUIRefresh)
            )),
            Some(f) => f(prevent_count),
        }
    }
    pub unsafe fn projectconfig_var_addr(
        &self,
        proj: *mut root::ReaProject,
        idx: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void {
        match self.pointers.projectconfig_var_addr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(projectconfig_var_addr)
            )),
            Some(f) => f(proj, idx),
        }
    }
    pub unsafe fn projectconfig_var_getoffs(
        &self,
        name: *const ::std::os::raw::c_char,
        szOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.projectconfig_var_getoffs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(projectconfig_var_getoffs)
            )),
            Some(f) => f(name, szOut),
        }
    }
    pub unsafe fn realloc_cmd_ptr(
        &self,
        ptr: *mut *mut ::std::os::raw::c_char,
        ptr_size: *mut ::std::os::raw::c_int,
        new_size: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.realloc_cmd_ptr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(realloc_cmd_ptr)
            )),
            Some(f) => f(ptr, ptr_size, new_size),
        }
    }
    pub fn ReaperGetPitchShiftAPI(
        &self,
        version: ::std::os::raw::c_int,
    ) -> *mut root::IReaperPitchShift {
        match self.pointers.ReaperGetPitchShiftAPI {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ReaperGetPitchShiftAPI)
            )),
            Some(f) => f(version),
        }
    }
    pub unsafe fn ReaScriptError(&self, errmsg: *const ::std::os::raw::c_char) {
        match self.pointers.ReaScriptError {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ReaScriptError)
            )),
            Some(f) => f(errmsg),
        }
    }
    pub unsafe fn RecursiveCreateDirectory(
        &self,
        path: *const ::std::os::raw::c_char,
        ignored: usize,
    ) -> ::std::os::raw::c_int {
        match self.pointers.RecursiveCreateDirectory {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(RecursiveCreateDirectory)
            )),
            Some(f) => f(path, ignored),
        }
    }
    pub fn reduce_open_files(&self, flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        match self.pointers.reduce_open_files {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(reduce_open_files)
            )),
            Some(f) => f(flags),
        }
    }
    pub fn RefreshToolbar(&self, command_id: ::std::os::raw::c_int) {
        match self.pointers.RefreshToolbar {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(RefreshToolbar)
            )),
            Some(f) => f(command_id),
        }
    }
    pub fn RefreshToolbar2(
        &self,
        section_id: ::std::os::raw::c_int,
        command_id: ::std::os::raw::c_int,
    ) {
        match self.pointers.RefreshToolbar2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(RefreshToolbar2)
            )),
            Some(f) => f(section_id, command_id),
        }
    }
    pub unsafe fn relative_fn(
        &self,
        in_: *const ::std::os::raw::c_char,
        out: *mut ::std::os::raw::c_char,
        out_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.relative_fn {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(relative_fn)
            )),
            Some(f) => f(in_, out, out_sz),
        }
    }
    pub unsafe fn RemoveTrackSend(
        &self,
        tr: *mut root::MediaTrack,
        category: ::std::os::raw::c_int,
        sendidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.RemoveTrackSend {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(RemoveTrackSend)
            )),
            Some(f) => f(tr, category, sendidx),
        }
    }
    pub unsafe fn RenderFileSection(
        &self,
        source_filename: *const ::std::os::raw::c_char,
        target_filename: *const ::std::os::raw::c_char,
        start_percent: f64,
        end_percent: f64,
        playrate: f64,
    ) -> bool {
        match self.pointers.RenderFileSection {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(RenderFileSection)
            )),
            Some(f) => f(
                source_filename,
                target_filename,
                start_percent,
                end_percent,
                playrate,
            ),
        }
    }
    pub fn ReorderSelectedTracks(
        &self,
        beforeTrackIdx: ::std::os::raw::c_int,
        makePrevFolder: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.ReorderSelectedTracks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ReorderSelectedTracks)
            )),
            Some(f) => f(beforeTrackIdx, makePrevFolder),
        }
    }
    pub fn Resample_EnumModes(&self, mode: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char {
        match self.pointers.Resample_EnumModes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Resample_EnumModes)
            )),
            Some(f) => f(mode),
        }
    }
    pub fn Resampler_Create(&self) -> *mut root::REAPER_Resample_Interface {
        match self.pointers.Resampler_Create {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Resampler_Create)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn resolve_fn(
        &self,
        in_: *const ::std::os::raw::c_char,
        out: *mut ::std::os::raw::c_char,
        out_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.resolve_fn {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(resolve_fn)
            )),
            Some(f) => f(in_, out, out_sz),
        }
    }
    pub unsafe fn resolve_fn2(
        &self,
        in_: *const ::std::os::raw::c_char,
        out: *mut ::std::os::raw::c_char,
        out_sz: ::std::os::raw::c_int,
        checkSubDirOptional: *const ::std::os::raw::c_char,
    ) {
        match self.pointers.resolve_fn2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(resolve_fn2)
            )),
            Some(f) => f(in_, out, out_sz, checkSubDirOptional),
        }
    }
    pub fn ReverseNamedCommandLookup(
        &self,
        command_id: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.ReverseNamedCommandLookup {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ReverseNamedCommandLookup)
            )),
            Some(f) => f(command_id),
        }
    }
    pub fn ScaleFromEnvelopeMode(&self, scaling_mode: ::std::os::raw::c_int, val: f64) -> f64 {
        match self.pointers.ScaleFromEnvelopeMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ScaleFromEnvelopeMode)
            )),
            Some(f) => f(scaling_mode, val),
        }
    }
    pub fn ScaleToEnvelopeMode(&self, scaling_mode: ::std::os::raw::c_int, val: f64) -> f64 {
        match self.pointers.ScaleToEnvelopeMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ScaleToEnvelopeMode)
            )),
            Some(f) => f(scaling_mode, val),
        }
    }
    pub unsafe fn screenset_register(
        &self,
        id: *mut ::std::os::raw::c_char,
        callbackFunc: *mut ::std::os::raw::c_void,
        param: *mut ::std::os::raw::c_void,
    ) {
        match self.pointers.screenset_register {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(screenset_register)
            )),
            Some(f) => f(id, callbackFunc, param),
        }
    }
    pub unsafe fn screenset_registerNew(
        &self,
        id: *mut ::std::os::raw::c_char,
        callbackFunc: root::screensetNewCallbackFunc,
        param: *mut ::std::os::raw::c_void,
    ) {
        match self.pointers.screenset_registerNew {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(screenset_registerNew)
            )),
            Some(f) => f(id, callbackFunc, param),
        }
    }
    pub unsafe fn screenset_unregister(&self, id: *mut ::std::os::raw::c_char) {
        match self.pointers.screenset_unregister {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(screenset_unregister)
            )),
            Some(f) => f(id),
        }
    }
    pub unsafe fn screenset_unregisterByParam(&self, param: *mut ::std::os::raw::c_void) {
        match self.pointers.screenset_unregisterByParam {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(screenset_unregisterByParam)
            )),
            Some(f) => f(param),
        }
    }
    pub fn screenset_updateLastFocus(&self, prevWin: root::HWND) {
        match self.pointers.screenset_updateLastFocus {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(screenset_updateLastFocus)
            )),
            Some(f) => f(prevWin),
        }
    }
    pub fn SectionFromUniqueID(
        &self,
        uniqueID: ::std::os::raw::c_int,
    ) -> *mut root::KbdSectionInfo {
        match self.pointers.SectionFromUniqueID {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SectionFromUniqueID)
            )),
            Some(f) => f(uniqueID),
        }
    }
    pub unsafe fn SelectAllMediaItems(&self, proj: *mut root::ReaProject, selected: bool) {
        match self.pointers.SelectAllMediaItems {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SelectAllMediaItems)
            )),
            Some(f) => f(proj, selected),
        }
    }
    pub unsafe fn SelectProjectInstance(&self, proj: *mut root::ReaProject) {
        match self.pointers.SelectProjectInstance {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SelectProjectInstance)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn SendLocalOscMessage(
        &self,
        local_osc_handler: *mut ::std::os::raw::c_void,
        msg: *const ::std::os::raw::c_char,
        msglen: ::std::os::raw::c_int,
    ) {
        match self.pointers.SendLocalOscMessage {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SendLocalOscMessage)
            )),
            Some(f) => f(local_osc_handler, msg, msglen),
        }
    }
    pub unsafe fn SetActiveTake(&self, take: *mut root::MediaItem_Take) {
        match self.pointers.SetActiveTake {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetActiveTake)
            )),
            Some(f) => f(take),
        }
    }
    pub fn SetAutomationMode(&self, mode: ::std::os::raw::c_int, onlySel: bool) {
        match self.pointers.SetAutomationMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetAutomationMode)
            )),
            Some(f) => f(mode, onlySel),
        }
    }
    pub unsafe fn SetCurrentBPM(&self, __proj: *mut root::ReaProject, bpm: f64, wantUndo: bool) {
        match self.pointers.SetCurrentBPM {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetCurrentBPM)
            )),
            Some(f) => f(__proj, bpm, wantUndo),
        }
    }
    pub unsafe fn SetCursorContext(
        &self,
        mode: ::std::os::raw::c_int,
        envInOptional: *mut root::TrackEnvelope,
    ) {
        match self.pointers.SetCursorContext {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetCursorContext)
            )),
            Some(f) => f(mode, envInOptional),
        }
    }
    pub fn SetEditCurPos(&self, time: f64, moveview: bool, seekplay: bool) {
        match self.pointers.SetEditCurPos {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetEditCurPos)
            )),
            Some(f) => f(time, moveview, seekplay),
        }
    }
    pub unsafe fn SetEditCurPos2(
        &self,
        proj: *mut root::ReaProject,
        time: f64,
        moveview: bool,
        seekplay: bool,
    ) {
        match self.pointers.SetEditCurPos2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetEditCurPos2)
            )),
            Some(f) => f(proj, time, moveview, seekplay),
        }
    }
    pub unsafe fn SetEnvelopePoint(
        &self,
        envelope: *mut root::TrackEnvelope,
        ptidx: ::std::os::raw::c_int,
        timeInOptional: *mut f64,
        valueInOptional: *mut f64,
        shapeInOptional: *mut ::std::os::raw::c_int,
        tensionInOptional: *mut f64,
        selectedInOptional: *mut bool,
        noSortInOptional: *mut bool,
    ) -> bool {
        match self.pointers.SetEnvelopePoint {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetEnvelopePoint)
            )),
            Some(f) => f(
                envelope,
                ptidx,
                timeInOptional,
                valueInOptional,
                shapeInOptional,
                tensionInOptional,
                selectedInOptional,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn SetEnvelopePointEx(
        &self,
        envelope: *mut root::TrackEnvelope,
        autoitem_idx: ::std::os::raw::c_int,
        ptidx: ::std::os::raw::c_int,
        timeInOptional: *mut f64,
        valueInOptional: *mut f64,
        shapeInOptional: *mut ::std::os::raw::c_int,
        tensionInOptional: *mut f64,
        selectedInOptional: *mut bool,
        noSortInOptional: *mut bool,
    ) -> bool {
        match self.pointers.SetEnvelopePointEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetEnvelopePointEx)
            )),
            Some(f) => f(
                envelope,
                autoitem_idx,
                ptidx,
                timeInOptional,
                valueInOptional,
                shapeInOptional,
                tensionInOptional,
                selectedInOptional,
                noSortInOptional,
            ),
        }
    }
    pub unsafe fn SetEnvelopeStateChunk(
        &self,
        env: *mut root::TrackEnvelope,
        str: *const ::std::os::raw::c_char,
        isundoOptional: bool,
    ) -> bool {
        match self.pointers.SetEnvelopeStateChunk {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetEnvelopeStateChunk)
            )),
            Some(f) => f(env, str, isundoOptional),
        }
    }
    pub unsafe fn SetExtState(
        &self,
        section: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
        persist: bool,
    ) {
        match self.pointers.SetExtState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetExtState)
            )),
            Some(f) => f(section, key, value, persist),
        }
    }
    pub fn SetGlobalAutomationOverride(&self, mode: ::std::os::raw::c_int) {
        match self.pointers.SetGlobalAutomationOverride {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetGlobalAutomationOverride)
            )),
            Some(f) => f(mode),
        }
    }
    pub unsafe fn SetItemStateChunk(
        &self,
        item: *mut root::MediaItem,
        str: *const ::std::os::raw::c_char,
        isundoOptional: bool,
    ) -> bool {
        match self.pointers.SetItemStateChunk {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetItemStateChunk)
            )),
            Some(f) => f(item, str, isundoOptional),
        }
    }
    pub fn SetMasterTrackVisibility(&self, flag: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        match self.pointers.SetMasterTrackVisibility {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMasterTrackVisibility)
            )),
            Some(f) => f(flag),
        }
    }
    pub unsafe fn SetMediaItemInfo_Value(
        &self,
        item: *mut root::MediaItem,
        parmname: *const ::std::os::raw::c_char,
        newvalue: f64,
    ) -> bool {
        match self.pointers.SetMediaItemInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMediaItemInfo_Value)
            )),
            Some(f) => f(item, parmname, newvalue),
        }
    }
    pub unsafe fn SetMediaItemLength(
        &self,
        item: *mut root::MediaItem,
        length: f64,
        refreshUI: bool,
    ) -> bool {
        match self.pointers.SetMediaItemLength {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMediaItemLength)
            )),
            Some(f) => f(item, length, refreshUI),
        }
    }
    pub unsafe fn SetMediaItemPosition(
        &self,
        item: *mut root::MediaItem,
        position: f64,
        refreshUI: bool,
    ) -> bool {
        match self.pointers.SetMediaItemPosition {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMediaItemPosition)
            )),
            Some(f) => f(item, position, refreshUI),
        }
    }
    pub unsafe fn SetMediaItemSelected(&self, item: *mut root::MediaItem, selected: bool) {
        match self.pointers.SetMediaItemSelected {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMediaItemSelected)
            )),
            Some(f) => f(item, selected),
        }
    }
    pub unsafe fn SetMediaItemTake_Source(
        &self,
        take: *mut root::MediaItem_Take,
        source: *mut root::PCM_source,
    ) -> bool {
        match self.pointers.SetMediaItemTake_Source {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMediaItemTake_Source)
            )),
            Some(f) => f(take, source),
        }
    }
    pub unsafe fn SetMediaItemTakeInfo_Value(
        &self,
        take: *mut root::MediaItem_Take,
        parmname: *const ::std::os::raw::c_char,
        newvalue: f64,
    ) -> bool {
        match self.pointers.SetMediaItemTakeInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMediaItemTakeInfo_Value)
            )),
            Some(f) => f(take, parmname, newvalue),
        }
    }
    pub unsafe fn SetMediaTrackInfo_Value(
        &self,
        tr: *mut root::MediaTrack,
        parmname: *const ::std::os::raw::c_char,
        newvalue: f64,
    ) -> bool {
        match self.pointers.SetMediaTrackInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMediaTrackInfo_Value)
            )),
            Some(f) => f(tr, parmname, newvalue),
        }
    }
    pub unsafe fn SetMIDIEditorGrid(&self, project: *mut root::ReaProject, division: f64) {
        match self.pointers.SetMIDIEditorGrid {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMIDIEditorGrid)
            )),
            Some(f) => f(project, division),
        }
    }
    pub unsafe fn SetMixerScroll(
        &self,
        leftmosttrack: *mut root::MediaTrack,
    ) -> *mut root::MediaTrack {
        match self.pointers.SetMixerScroll {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMixerScroll)
            )),
            Some(f) => f(leftmosttrack),
        }
    }
    pub unsafe fn SetMouseModifier(
        &self,
        context: *const ::std::os::raw::c_char,
        modifier_flag: ::std::os::raw::c_int,
        action: *const ::std::os::raw::c_char,
    ) {
        match self.pointers.SetMouseModifier {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetMouseModifier)
            )),
            Some(f) => f(context, modifier_flag, action),
        }
    }
    pub unsafe fn SetOnlyTrackSelected(&self, track: *mut root::MediaTrack) {
        match self.pointers.SetOnlyTrackSelected {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetOnlyTrackSelected)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn SetProjectGrid(&self, project: *mut root::ReaProject, division: f64) {
        match self.pointers.SetProjectGrid {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetProjectGrid)
            )),
            Some(f) => f(project, division),
        }
    }
    pub unsafe fn SetProjectMarker(
        &self,
        markrgnindexnumber: ::std::os::raw::c_int,
        isrgn: bool,
        pos: f64,
        rgnend: f64,
        name: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.SetProjectMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetProjectMarker)
            )),
            Some(f) => f(markrgnindexnumber, isrgn, pos, rgnend, name),
        }
    }
    pub unsafe fn SetProjectMarker2(
        &self,
        proj: *mut root::ReaProject,
        markrgnindexnumber: ::std::os::raw::c_int,
        isrgn: bool,
        pos: f64,
        rgnend: f64,
        name: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.SetProjectMarker2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetProjectMarker2)
            )),
            Some(f) => f(proj, markrgnindexnumber, isrgn, pos, rgnend, name),
        }
    }
    pub unsafe fn SetProjectMarker3(
        &self,
        proj: *mut root::ReaProject,
        markrgnindexnumber: ::std::os::raw::c_int,
        isrgn: bool,
        pos: f64,
        rgnend: f64,
        name: *const ::std::os::raw::c_char,
        color: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.SetProjectMarker3 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetProjectMarker3)
            )),
            Some(f) => f(proj, markrgnindexnumber, isrgn, pos, rgnend, name, color),
        }
    }
    pub unsafe fn SetProjectMarker4(
        &self,
        proj: *mut root::ReaProject,
        markrgnindexnumber: ::std::os::raw::c_int,
        isrgn: bool,
        pos: f64,
        rgnend: f64,
        name: *const ::std::os::raw::c_char,
        color: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.SetProjectMarker4 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetProjectMarker4)
            )),
            Some(f) => f(
                proj,
                markrgnindexnumber,
                isrgn,
                pos,
                rgnend,
                name,
                color,
                flags,
            ),
        }
    }
    pub unsafe fn SetProjectMarkerByIndex(
        &self,
        proj: *mut root::ReaProject,
        markrgnidx: ::std::os::raw::c_int,
        isrgn: bool,
        pos: f64,
        rgnend: f64,
        IDnumber: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        color: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.SetProjectMarkerByIndex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetProjectMarkerByIndex)
            )),
            Some(f) => f(proj, markrgnidx, isrgn, pos, rgnend, IDnumber, name, color),
        }
    }
    pub unsafe fn SetProjectMarkerByIndex2(
        &self,
        proj: *mut root::ReaProject,
        markrgnidx: ::std::os::raw::c_int,
        isrgn: bool,
        pos: f64,
        rgnend: f64,
        IDnumber: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        color: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.SetProjectMarkerByIndex2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetProjectMarkerByIndex2)
            )),
            Some(f) => f(
                proj, markrgnidx, isrgn, pos, rgnend, IDnumber, name, color, flags,
            ),
        }
    }
    pub unsafe fn SetProjExtState(
        &self,
        proj: *mut root::ReaProject,
        extname: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        match self.pointers.SetProjExtState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetProjExtState)
            )),
            Some(f) => f(proj, extname, key, value),
        }
    }
    pub unsafe fn SetRegionRenderMatrix(
        &self,
        proj: *mut root::ReaProject,
        regionindex: ::std::os::raw::c_int,
        track: *mut root::MediaTrack,
        addorremove: ::std::os::raw::c_int,
    ) {
        match self.pointers.SetRegionRenderMatrix {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetRegionRenderMatrix)
            )),
            Some(f) => f(proj, regionindex, track, addorremove),
        }
    }
    pub unsafe fn SetRenderLastError(&self, errorstr: *const ::std::os::raw::c_char) {
        match self.pointers.SetRenderLastError {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetRenderLastError)
            )),
            Some(f) => f(errorstr),
        }
    }
    pub unsafe fn SetTakeStretchMarker(
        &self,
        take: *mut root::MediaItem_Take,
        idx: ::std::os::raw::c_int,
        pos: f64,
        srcposInOptional: *const f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.SetTakeStretchMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTakeStretchMarker)
            )),
            Some(f) => f(take, idx, pos, srcposInOptional),
        }
    }
    pub unsafe fn SetTakeStretchMarkerSlope(
        &self,
        take: *mut root::MediaItem_Take,
        idx: ::std::os::raw::c_int,
        slope: f64,
    ) -> bool {
        match self.pointers.SetTakeStretchMarkerSlope {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTakeStretchMarkerSlope)
            )),
            Some(f) => f(take, idx, slope),
        }
    }
    pub unsafe fn SetTempoTimeSigMarker(
        &self,
        proj: *mut root::ReaProject,
        ptidx: ::std::os::raw::c_int,
        timepos: f64,
        measurepos: ::std::os::raw::c_int,
        beatpos: f64,
        bpm: f64,
        timesig_num: ::std::os::raw::c_int,
        timesig_denom: ::std::os::raw::c_int,
        lineartempo: bool,
    ) -> bool {
        match self.pointers.SetTempoTimeSigMarker {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTempoTimeSigMarker)
            )),
            Some(f) => f(
                proj,
                ptidx,
                timepos,
                measurepos,
                beatpos,
                bpm,
                timesig_num,
                timesig_denom,
                lineartempo,
            ),
        }
    }
    pub fn SetToggleCommandState(
        &self,
        section_id: ::std::os::raw::c_int,
        command_id: ::std::os::raw::c_int,
        state: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.SetToggleCommandState {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetToggleCommandState)
            )),
            Some(f) => f(section_id, command_id, state),
        }
    }
    pub unsafe fn SetTrackAutomationMode(
        &self,
        tr: *mut root::MediaTrack,
        mode: ::std::os::raw::c_int,
    ) {
        match self.pointers.SetTrackAutomationMode {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackAutomationMode)
            )),
            Some(f) => f(tr, mode),
        }
    }
    pub unsafe fn SetTrackColor(&self, track: *mut root::MediaTrack, color: ::std::os::raw::c_int) {
        match self.pointers.SetTrackColor {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackColor)
            )),
            Some(f) => f(track, color),
        }
    }
    pub unsafe fn SetTrackMIDILyrics(
        &self,
        track: *mut root::MediaTrack,
        flag: ::std::os::raw::c_int,
        str: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.SetTrackMIDILyrics {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackMIDILyrics)
            )),
            Some(f) => f(track, flag, str),
        }
    }
    pub unsafe fn SetTrackMIDINoteName(
        &self,
        track: ::std::os::raw::c_int,
        pitch: ::std::os::raw::c_int,
        chan: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.SetTrackMIDINoteName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackMIDINoteName)
            )),
            Some(f) => f(track, pitch, chan, name),
        }
    }
    pub unsafe fn SetTrackMIDINoteNameEx(
        &self,
        proj: *mut root::ReaProject,
        track: *mut root::MediaTrack,
        pitch: ::std::os::raw::c_int,
        chan: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.SetTrackMIDINoteNameEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackMIDINoteNameEx)
            )),
            Some(f) => f(proj, track, pitch, chan, name),
        }
    }
    pub unsafe fn SetTrackSelected(&self, track: *mut root::MediaTrack, selected: bool) {
        match self.pointers.SetTrackSelected {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackSelected)
            )),
            Some(f) => f(track, selected),
        }
    }
    pub unsafe fn SetTrackSendInfo_Value(
        &self,
        tr: *mut root::MediaTrack,
        category: ::std::os::raw::c_int,
        sendidx: ::std::os::raw::c_int,
        parmname: *const ::std::os::raw::c_char,
        newvalue: f64,
    ) -> bool {
        match self.pointers.SetTrackSendInfo_Value {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackSendInfo_Value)
            )),
            Some(f) => f(tr, category, sendidx, parmname, newvalue),
        }
    }
    pub unsafe fn SetTrackSendUIPan(
        &self,
        track: *mut root::MediaTrack,
        send_idx: ::std::os::raw::c_int,
        pan: f64,
        isend: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.SetTrackSendUIPan {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackSendUIPan)
            )),
            Some(f) => f(track, send_idx, pan, isend),
        }
    }
    pub unsafe fn SetTrackSendUIVol(
        &self,
        track: *mut root::MediaTrack,
        send_idx: ::std::os::raw::c_int,
        vol: f64,
        isend: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.SetTrackSendUIVol {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackSendUIVol)
            )),
            Some(f) => f(track, send_idx, vol, isend),
        }
    }
    pub unsafe fn SetTrackStateChunk(
        &self,
        track: *mut root::MediaTrack,
        str: *const ::std::os::raw::c_char,
        isundoOptional: bool,
    ) -> bool {
        match self.pointers.SetTrackStateChunk {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SetTrackStateChunk)
            )),
            Some(f) => f(track, str, isundoOptional),
        }
    }
    pub unsafe fn ShowActionList(&self, caller: *mut root::KbdSectionInfo, callerWnd: root::HWND) {
        match self.pointers.ShowActionList {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ShowActionList)
            )),
            Some(f) => f(caller, callerWnd),
        }
    }
    pub unsafe fn ShowConsoleMsg(&self, msg: *const ::std::os::raw::c_char) {
        match self.pointers.ShowConsoleMsg {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ShowConsoleMsg)
            )),
            Some(f) => f(msg),
        }
    }
    pub unsafe fn ShowMessageBox(
        &self,
        msg: *const ::std::os::raw::c_char,
        title: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.ShowMessageBox {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ShowMessageBox)
            )),
            Some(f) => f(msg, title, type_),
        }
    }
    pub unsafe fn ShowPopupMenu(
        &self,
        name: *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        hwndParentOptional: root::HWND,
        ctxOptional: *mut ::std::os::raw::c_void,
        ctx2Optional: ::std::os::raw::c_int,
        ctx3Optional: ::std::os::raw::c_int,
    ) {
        match self.pointers.ShowPopupMenu {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ShowPopupMenu)
            )),
            Some(f) => f(
                name,
                x,
                y,
                hwndParentOptional,
                ctxOptional,
                ctx2Optional,
                ctx3Optional,
            ),
        }
    }
    pub fn SLIDER2DB(&self, y: f64) -> f64 {
        match self.pointers.SLIDER2DB {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SLIDER2DB)
            )),
            Some(f) => f(y),
        }
    }
    pub unsafe fn SnapToGrid(&self, project: *mut root::ReaProject, time_pos: f64) -> f64 {
        match self.pointers.SnapToGrid {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SnapToGrid)
            )),
            Some(f) => f(project, time_pos),
        }
    }
    pub fn SoloAllTracks(&self, solo: ::std::os::raw::c_int) {
        match self.pointers.SoloAllTracks {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SoloAllTracks)
            )),
            Some(f) => f(solo),
        }
    }
    pub fn Splash_GetWnd(&self) -> root::HWND {
        match self.pointers.Splash_GetWnd {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Splash_GetWnd)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn SplitMediaItem(
        &self,
        item: *mut root::MediaItem,
        position: f64,
    ) -> *mut root::MediaItem {
        match self.pointers.SplitMediaItem {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(SplitMediaItem)
            )),
            Some(f) => f(item, position),
        }
    }
    pub unsafe fn StopPreview(
        &self,
        preview: *mut root::preview_register_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.StopPreview {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(StopPreview)
            )),
            Some(f) => f(preview),
        }
    }
    pub unsafe fn StopTrackPreview(
        &self,
        preview: *mut root::preview_register_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.StopTrackPreview {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(StopTrackPreview)
            )),
            Some(f) => f(preview),
        }
    }
    pub unsafe fn StopTrackPreview2(
        &self,
        proj: *mut ::std::os::raw::c_void,
        preview: *mut root::preview_register_t,
    ) -> ::std::os::raw::c_int {
        match self.pointers.StopTrackPreview2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(StopTrackPreview2)
            )),
            Some(f) => f(proj, preview),
        }
    }
    pub unsafe fn stringToGuid(&self, str: *const ::std::os::raw::c_char, g: *mut root::GUID) {
        match self.pointers.stringToGuid {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(stringToGuid)
            )),
            Some(f) => f(str, g),
        }
    }
    pub fn StuffMIDIMessage(
        &self,
        mode: ::std::os::raw::c_int,
        msg1: ::std::os::raw::c_int,
        msg2: ::std::os::raw::c_int,
        msg3: ::std::os::raw::c_int,
    ) {
        match self.pointers.StuffMIDIMessage {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(StuffMIDIMessage)
            )),
            Some(f) => f(mode, msg1, msg2, msg3),
        }
    }
    pub unsafe fn TakeFX_AddByName(
        &self,
        take: *mut root::MediaItem_Take,
        fxname: *const ::std::os::raw::c_char,
        instantiate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TakeFX_AddByName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_AddByName)
            )),
            Some(f) => f(take, fxname, instantiate),
        }
    }
    pub unsafe fn TakeFX_CopyToTake(
        &self,
        src_take: *mut root::MediaItem_Take,
        src_fx: ::std::os::raw::c_int,
        dest_take: *mut root::MediaItem_Take,
        dest_fx: ::std::os::raw::c_int,
        is_move: bool,
    ) {
        match self.pointers.TakeFX_CopyToTake {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_CopyToTake)
            )),
            Some(f) => f(src_take, src_fx, dest_take, dest_fx, is_move),
        }
    }
    pub unsafe fn TakeFX_CopyToTrack(
        &self,
        src_take: *mut root::MediaItem_Take,
        src_fx: ::std::os::raw::c_int,
        dest_track: *mut root::MediaTrack,
        dest_fx: ::std::os::raw::c_int,
        is_move: bool,
    ) {
        match self.pointers.TakeFX_CopyToTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_CopyToTrack)
            )),
            Some(f) => f(src_take, src_fx, dest_track, dest_fx, is_move),
        }
    }
    pub unsafe fn TakeFX_Delete(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_Delete {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_Delete)
            )),
            Some(f) => f(take, fx),
        }
    }
    pub unsafe fn TakeFX_EndParamEdit(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_EndParamEdit {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_EndParamEdit)
            )),
            Some(f) => f(take, fx, param),
        }
    }
    pub unsafe fn TakeFX_FormatParamValue(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        val: f64,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_FormatParamValue {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_FormatParamValue)
            )),
            Some(f) => f(take, fx, param, val, buf, buf_sz),
        }
    }
    pub unsafe fn TakeFX_FormatParamValueNormalized(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        value: f64,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_FormatParamValueNormalized {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_FormatParamValueNormalized)
            )),
            Some(f) => f(take, fx, param, value, buf, buf_sz),
        }
    }
    pub unsafe fn TakeFX_GetChainVisible(
        &self,
        take: *mut root::MediaItem_Take,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TakeFX_GetChainVisible {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetChainVisible)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn TakeFX_GetCount(&self, take: *mut root::MediaItem_Take) -> ::std::os::raw::c_int {
        match self.pointers.TakeFX_GetCount {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetCount)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn TakeFX_GetEnabled(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_GetEnabled {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetEnabled)
            )),
            Some(f) => f(take, fx),
        }
    }
    pub unsafe fn TakeFX_GetEnvelope(
        &self,
        take: *mut root::MediaItem_Take,
        fxindex: ::std::os::raw::c_int,
        parameterindex: ::std::os::raw::c_int,
        create: bool,
    ) -> *mut root::TrackEnvelope {
        match self.pointers.TakeFX_GetEnvelope {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetEnvelope)
            )),
            Some(f) => f(take, fxindex, parameterindex, create),
        }
    }
    pub unsafe fn TakeFX_GetFloatingWindow(
        &self,
        take: *mut root::MediaItem_Take,
        index: ::std::os::raw::c_int,
    ) -> root::HWND {
        match self.pointers.TakeFX_GetFloatingWindow {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetFloatingWindow)
            )),
            Some(f) => f(take, index),
        }
    }
    pub unsafe fn TakeFX_GetFormattedParamValue(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_GetFormattedParamValue {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetFormattedParamValue)
            )),
            Some(f) => f(take, fx, param, buf, buf_sz),
        }
    }
    pub unsafe fn TakeFX_GetFXGUID(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
    ) -> *mut root::GUID {
        match self.pointers.TakeFX_GetFXGUID {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetFXGUID)
            )),
            Some(f) => f(take, fx),
        }
    }
    pub unsafe fn TakeFX_GetFXName(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_GetFXName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetFXName)
            )),
            Some(f) => f(take, fx, buf, buf_sz),
        }
    }
    pub unsafe fn TakeFX_GetIOSize(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        inputPinsOutOptional: *mut ::std::os::raw::c_int,
        outputPinsOutOptional: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TakeFX_GetIOSize {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetIOSize)
            )),
            Some(f) => f(take, fx, inputPinsOutOptional, outputPinsOutOptional),
        }
    }
    pub unsafe fn TakeFX_GetNamedConfigParm(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        parmname: *const ::std::os::raw::c_char,
        bufOut: *mut ::std::os::raw::c_char,
        bufOut_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_GetNamedConfigParm {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetNamedConfigParm)
            )),
            Some(f) => f(take, fx, parmname, bufOut, bufOut_sz),
        }
    }
    pub unsafe fn TakeFX_GetNumParams(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TakeFX_GetNumParams {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetNumParams)
            )),
            Some(f) => f(take, fx),
        }
    }
    pub unsafe fn TakeFX_GetOffline(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_GetOffline {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetOffline)
            )),
            Some(f) => f(take, fx),
        }
    }
    pub unsafe fn TakeFX_GetOpen(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_GetOpen {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetOpen)
            )),
            Some(f) => f(take, fx),
        }
    }
    pub unsafe fn TakeFX_GetParam(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        minvalOut: *mut f64,
        maxvalOut: *mut f64,
    ) -> f64 {
        match self.pointers.TakeFX_GetParam {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetParam)
            )),
            Some(f) => f(take, fx, param, minvalOut, maxvalOut),
        }
    }
    pub unsafe fn TakeFX_GetParameterStepSizes(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        stepOut: *mut f64,
        smallstepOut: *mut f64,
        largestepOut: *mut f64,
        istoggleOut: *mut bool,
    ) -> bool {
        match self.pointers.TakeFX_GetParameterStepSizes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetParameterStepSizes)
            )),
            Some(f) => f(
                take,
                fx,
                param,
                stepOut,
                smallstepOut,
                largestepOut,
                istoggleOut,
            ),
        }
    }
    pub unsafe fn TakeFX_GetParamEx(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        minvalOut: *mut f64,
        maxvalOut: *mut f64,
        midvalOut: *mut f64,
    ) -> f64 {
        match self.pointers.TakeFX_GetParamEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetParamEx)
            )),
            Some(f) => f(take, fx, param, minvalOut, maxvalOut, midvalOut),
        }
    }
    pub unsafe fn TakeFX_GetParamName(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_GetParamName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetParamName)
            )),
            Some(f) => f(take, fx, param, buf, buf_sz),
        }
    }
    pub unsafe fn TakeFX_GetParamNormalized(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.TakeFX_GetParamNormalized {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetParamNormalized)
            )),
            Some(f) => f(take, fx, param),
        }
    }
    pub unsafe fn TakeFX_GetPinMappings(
        &self,
        tr: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        isoutput: ::std::os::raw::c_int,
        pin: ::std::os::raw::c_int,
        high32OutOptional: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TakeFX_GetPinMappings {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetPinMappings)
            )),
            Some(f) => f(tr, fx, isoutput, pin, high32OutOptional),
        }
    }
    pub unsafe fn TakeFX_GetPreset(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        presetname: *mut ::std::os::raw::c_char,
        presetname_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_GetPreset {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetPreset)
            )),
            Some(f) => f(take, fx, presetname, presetname_sz),
        }
    }
    pub unsafe fn TakeFX_GetPresetIndex(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        numberOfPresetsOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TakeFX_GetPresetIndex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetPresetIndex)
            )),
            Some(f) => f(take, fx, numberOfPresetsOut),
        }
    }
    pub unsafe fn TakeFX_GetUserPresetFilename(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        fn_: *mut ::std::os::raw::c_char,
        fn_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.TakeFX_GetUserPresetFilename {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_GetUserPresetFilename)
            )),
            Some(f) => f(take, fx, fn_, fn_sz),
        }
    }
    pub unsafe fn TakeFX_NavigatePresets(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        presetmove: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_NavigatePresets {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_NavigatePresets)
            )),
            Some(f) => f(take, fx, presetmove),
        }
    }
    pub unsafe fn TakeFX_SetEnabled(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        enabled: bool,
    ) {
        match self.pointers.TakeFX_SetEnabled {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetEnabled)
            )),
            Some(f) => f(take, fx, enabled),
        }
    }
    pub unsafe fn TakeFX_SetNamedConfigParm(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        parmname: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.TakeFX_SetNamedConfigParm {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetNamedConfigParm)
            )),
            Some(f) => f(take, fx, parmname, value),
        }
    }
    pub unsafe fn TakeFX_SetOffline(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        offline: bool,
    ) {
        match self.pointers.TakeFX_SetOffline {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetOffline)
            )),
            Some(f) => f(take, fx, offline),
        }
    }
    pub unsafe fn TakeFX_SetOpen(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        open: bool,
    ) {
        match self.pointers.TakeFX_SetOpen {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetOpen)
            )),
            Some(f) => f(take, fx, open),
        }
    }
    pub unsafe fn TakeFX_SetParam(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        val: f64,
    ) -> bool {
        match self.pointers.TakeFX_SetParam {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetParam)
            )),
            Some(f) => f(take, fx, param, val),
        }
    }
    pub unsafe fn TakeFX_SetParamNormalized(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        value: f64,
    ) -> bool {
        match self.pointers.TakeFX_SetParamNormalized {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetParamNormalized)
            )),
            Some(f) => f(take, fx, param, value),
        }
    }
    pub unsafe fn TakeFX_SetPinMappings(
        &self,
        tr: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        isoutput: ::std::os::raw::c_int,
        pin: ::std::os::raw::c_int,
        low32bits: ::std::os::raw::c_int,
        hi32bits: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_SetPinMappings {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetPinMappings)
            )),
            Some(f) => f(tr, fx, isoutput, pin, low32bits, hi32bits),
        }
    }
    pub unsafe fn TakeFX_SetPreset(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        presetname: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.TakeFX_SetPreset {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetPreset)
            )),
            Some(f) => f(take, fx, presetname),
        }
    }
    pub unsafe fn TakeFX_SetPresetByIndex(
        &self,
        take: *mut root::MediaItem_Take,
        fx: ::std::os::raw::c_int,
        idx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TakeFX_SetPresetByIndex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_SetPresetByIndex)
            )),
            Some(f) => f(take, fx, idx),
        }
    }
    pub unsafe fn TakeFX_Show(
        &self,
        take: *mut root::MediaItem_Take,
        index: ::std::os::raw::c_int,
        showFlag: ::std::os::raw::c_int,
    ) {
        match self.pointers.TakeFX_Show {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeFX_Show)
            )),
            Some(f) => f(take, index, showFlag),
        }
    }
    pub unsafe fn TakeIsMIDI(&self, take: *mut root::MediaItem_Take) -> bool {
        match self.pointers.TakeIsMIDI {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TakeIsMIDI)
            )),
            Some(f) => f(take),
        }
    }
    pub unsafe fn ThemeLayout_GetLayout(
        &self,
        section: *const ::std::os::raw::c_char,
        idx: ::std::os::raw::c_int,
        nameOut: *mut ::std::os::raw::c_char,
        nameOut_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.ThemeLayout_GetLayout {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ThemeLayout_GetLayout)
            )),
            Some(f) => f(section, idx, nameOut, nameOut_sz),
        }
    }
    pub unsafe fn ThemeLayout_GetParameter(
        &self,
        wp: ::std::os::raw::c_int,
        descOutOptional: *mut *const ::std::os::raw::c_char,
        valueOutOptional: *mut ::std::os::raw::c_int,
        defValueOutOptional: *mut ::std::os::raw::c_int,
        minValueOutOptional: *mut ::std::os::raw::c_int,
        maxValueOutOptional: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.ThemeLayout_GetParameter {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ThemeLayout_GetParameter)
            )),
            Some(f) => f(
                wp,
                descOutOptional,
                valueOutOptional,
                defValueOutOptional,
                minValueOutOptional,
                maxValueOutOptional,
            ),
        }
    }
    pub fn ThemeLayout_RefreshAll(&self) {
        match self.pointers.ThemeLayout_RefreshAll {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ThemeLayout_RefreshAll)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn ThemeLayout_SetLayout(
        &self,
        section: *const ::std::os::raw::c_char,
        layout: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.ThemeLayout_SetLayout {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ThemeLayout_SetLayout)
            )),
            Some(f) => f(section, layout),
        }
    }
    pub fn ThemeLayout_SetParameter(
        &self,
        wp: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
        persist: bool,
    ) -> bool {
        match self.pointers.ThemeLayout_SetParameter {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ThemeLayout_SetParameter)
            )),
            Some(f) => f(wp, value, persist),
        }
    }
    pub fn time_precise(&self) -> f64 {
        match self.pointers.time_precise {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(time_precise)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn TimeMap2_beatsToTime(
        &self,
        proj: *mut root::ReaProject,
        tpos: f64,
        measuresInOptional: *const ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.TimeMap2_beatsToTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap2_beatsToTime)
            )),
            Some(f) => f(proj, tpos, measuresInOptional),
        }
    }
    pub unsafe fn TimeMap2_GetDividedBpmAtTime(
        &self,
        proj: *mut root::ReaProject,
        time: f64,
    ) -> f64 {
        match self.pointers.TimeMap2_GetDividedBpmAtTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap2_GetDividedBpmAtTime)
            )),
            Some(f) => f(proj, time),
        }
    }
    pub unsafe fn TimeMap2_GetNextChangeTime(&self, proj: *mut root::ReaProject, time: f64) -> f64 {
        match self.pointers.TimeMap2_GetNextChangeTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap2_GetNextChangeTime)
            )),
            Some(f) => f(proj, time),
        }
    }
    pub unsafe fn TimeMap2_QNToTime(&self, proj: *mut root::ReaProject, qn: f64) -> f64 {
        match self.pointers.TimeMap2_QNToTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap2_QNToTime)
            )),
            Some(f) => f(proj, qn),
        }
    }
    pub unsafe fn TimeMap2_timeToBeats(
        &self,
        proj: *mut root::ReaProject,
        tpos: f64,
        measuresOutOptional: *mut ::std::os::raw::c_int,
        cmlOutOptional: *mut ::std::os::raw::c_int,
        fullbeatsOutOptional: *mut f64,
        cdenomOutOptional: *mut ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.TimeMap2_timeToBeats {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap2_timeToBeats)
            )),
            Some(f) => f(
                proj,
                tpos,
                measuresOutOptional,
                cmlOutOptional,
                fullbeatsOutOptional,
                cdenomOutOptional,
            ),
        }
    }
    pub unsafe fn TimeMap2_timeToQN(&self, proj: *mut root::ReaProject, tpos: f64) -> f64 {
        match self.pointers.TimeMap2_timeToQN {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap2_timeToQN)
            )),
            Some(f) => f(proj, tpos),
        }
    }
    pub unsafe fn TimeMap_curFrameRate(
        &self,
        proj: *mut root::ReaProject,
        dropFrameOutOptional: *mut bool,
    ) -> f64 {
        match self.pointers.TimeMap_curFrameRate {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_curFrameRate)
            )),
            Some(f) => f(proj, dropFrameOutOptional),
        }
    }
    pub fn TimeMap_GetDividedBpmAtTime(&self, time: f64) -> f64 {
        match self.pointers.TimeMap_GetDividedBpmAtTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_GetDividedBpmAtTime)
            )),
            Some(f) => f(time),
        }
    }
    pub unsafe fn TimeMap_GetMeasureInfo(
        &self,
        proj: *mut root::ReaProject,
        measure: ::std::os::raw::c_int,
        qn_startOut: *mut f64,
        qn_endOut: *mut f64,
        timesig_numOut: *mut ::std::os::raw::c_int,
        timesig_denomOut: *mut ::std::os::raw::c_int,
        tempoOut: *mut f64,
    ) -> f64 {
        match self.pointers.TimeMap_GetMeasureInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_GetMeasureInfo)
            )),
            Some(f) => f(
                proj,
                measure,
                qn_startOut,
                qn_endOut,
                timesig_numOut,
                timesig_denomOut,
                tempoOut,
            ),
        }
    }
    pub unsafe fn TimeMap_GetMetronomePattern(
        &self,
        proj: *mut root::ReaProject,
        time: f64,
        pattern: *mut ::std::os::raw::c_char,
        pattern_sz: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TimeMap_GetMetronomePattern {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_GetMetronomePattern)
            )),
            Some(f) => f(proj, time, pattern, pattern_sz),
        }
    }
    pub unsafe fn TimeMap_GetTimeSigAtTime(
        &self,
        proj: *mut root::ReaProject,
        time: f64,
        timesig_numOut: *mut ::std::os::raw::c_int,
        timesig_denomOut: *mut ::std::os::raw::c_int,
        tempoOut: *mut f64,
    ) {
        match self.pointers.TimeMap_GetTimeSigAtTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_GetTimeSigAtTime)
            )),
            Some(f) => f(proj, time, timesig_numOut, timesig_denomOut, tempoOut),
        }
    }
    pub unsafe fn TimeMap_QNToMeasures(
        &self,
        proj: *mut root::ReaProject,
        qn: f64,
        qnMeasureStartOutOptional: *mut f64,
        qnMeasureEndOutOptional: *mut f64,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TimeMap_QNToMeasures {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_QNToMeasures)
            )),
            Some(f) => f(proj, qn, qnMeasureStartOutOptional, qnMeasureEndOutOptional),
        }
    }
    pub fn TimeMap_QNToTime(&self, qn: f64) -> f64 {
        match self.pointers.TimeMap_QNToTime {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_QNToTime)
            )),
            Some(f) => f(qn),
        }
    }
    pub unsafe fn TimeMap_QNToTime_abs(&self, proj: *mut root::ReaProject, qn: f64) -> f64 {
        match self.pointers.TimeMap_QNToTime_abs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_QNToTime_abs)
            )),
            Some(f) => f(proj, qn),
        }
    }
    pub fn TimeMap_timeToQN(&self, tpos: f64) -> f64 {
        match self.pointers.TimeMap_timeToQN {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_timeToQN)
            )),
            Some(f) => f(tpos),
        }
    }
    pub unsafe fn TimeMap_timeToQN_abs(&self, proj: *mut root::ReaProject, tpos: f64) -> f64 {
        match self.pointers.TimeMap_timeToQN_abs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TimeMap_timeToQN_abs)
            )),
            Some(f) => f(proj, tpos),
        }
    }
    pub unsafe fn ToggleTrackSendUIMute(
        &self,
        track: *mut root::MediaTrack,
        send_idx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.ToggleTrackSendUIMute {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ToggleTrackSendUIMute)
            )),
            Some(f) => f(track, send_idx),
        }
    }
    pub unsafe fn Track_GetPeakHoldDB(
        &self,
        track: *mut root::MediaTrack,
        channel: ::std::os::raw::c_int,
        clear: bool,
    ) -> f64 {
        match self.pointers.Track_GetPeakHoldDB {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Track_GetPeakHoldDB)
            )),
            Some(f) => f(track, channel, clear),
        }
    }
    pub unsafe fn Track_GetPeakInfo(
        &self,
        track: *mut root::MediaTrack,
        channel: ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.Track_GetPeakInfo {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Track_GetPeakInfo)
            )),
            Some(f) => f(track, channel),
        }
    }
    pub unsafe fn TrackCtl_SetToolTip(
        &self,
        fmt: *const ::std::os::raw::c_char,
        xpos: ::std::os::raw::c_int,
        ypos: ::std::os::raw::c_int,
        topmost: bool,
    ) {
        match self.pointers.TrackCtl_SetToolTip {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackCtl_SetToolTip)
            )),
            Some(f) => f(fmt, xpos, ypos, topmost),
        }
    }
    pub unsafe fn TrackFX_AddByName(
        &self,
        track: *mut root::MediaTrack,
        fxname: *const ::std::os::raw::c_char,
        recFX: bool,
        instantiate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_AddByName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_AddByName)
            )),
            Some(f) => f(track, fxname, recFX, instantiate),
        }
    }
    pub unsafe fn TrackFX_CopyToTake(
        &self,
        src_track: *mut root::MediaTrack,
        src_fx: ::std::os::raw::c_int,
        dest_take: *mut root::MediaItem_Take,
        dest_fx: ::std::os::raw::c_int,
        is_move: bool,
    ) {
        match self.pointers.TrackFX_CopyToTake {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_CopyToTake)
            )),
            Some(f) => f(src_track, src_fx, dest_take, dest_fx, is_move),
        }
    }
    pub unsafe fn TrackFX_CopyToTrack(
        &self,
        src_track: *mut root::MediaTrack,
        src_fx: ::std::os::raw::c_int,
        dest_track: *mut root::MediaTrack,
        dest_fx: ::std::os::raw::c_int,
        is_move: bool,
    ) {
        match self.pointers.TrackFX_CopyToTrack {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_CopyToTrack)
            )),
            Some(f) => f(src_track, src_fx, dest_track, dest_fx, is_move),
        }
    }
    pub unsafe fn TrackFX_Delete(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_Delete {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_Delete)
            )),
            Some(f) => f(track, fx),
        }
    }
    pub unsafe fn TrackFX_EndParamEdit(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_EndParamEdit {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_EndParamEdit)
            )),
            Some(f) => f(track, fx, param),
        }
    }
    pub unsafe fn TrackFX_FormatParamValue(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        val: f64,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_FormatParamValue {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_FormatParamValue)
            )),
            Some(f) => f(track, fx, param, val, buf, buf_sz),
        }
    }
    pub unsafe fn TrackFX_FormatParamValueNormalized(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        value: f64,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_FormatParamValueNormalized {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_FormatParamValueNormalized)
            )),
            Some(f) => f(track, fx, param, value, buf, buf_sz),
        }
    }
    pub unsafe fn TrackFX_GetByName(
        &self,
        track: *mut root::MediaTrack,
        fxname: *const ::std::os::raw::c_char,
        instantiate: bool,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetByName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetByName)
            )),
            Some(f) => f(track, fxname, instantiate),
        }
    }
    pub unsafe fn TrackFX_GetChainVisible(
        &self,
        track: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetChainVisible {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetChainVisible)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn TrackFX_GetCount(&self, track: *mut root::MediaTrack) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetCount {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetCount)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn TrackFX_GetEnabled(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetEnabled {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetEnabled)
            )),
            Some(f) => f(track, fx),
        }
    }
    pub unsafe fn TrackFX_GetEQ(
        &self,
        track: *mut root::MediaTrack,
        instantiate: bool,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetEQ {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetEQ)
            )),
            Some(f) => f(track, instantiate),
        }
    }
    pub unsafe fn TrackFX_GetEQBandEnabled(
        &self,
        track: *mut root::MediaTrack,
        fxidx: ::std::os::raw::c_int,
        bandtype: ::std::os::raw::c_int,
        bandidx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetEQBandEnabled {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetEQBandEnabled)
            )),
            Some(f) => f(track, fxidx, bandtype, bandidx),
        }
    }
    pub unsafe fn TrackFX_GetEQParam(
        &self,
        track: *mut root::MediaTrack,
        fxidx: ::std::os::raw::c_int,
        paramidx: ::std::os::raw::c_int,
        bandtypeOut: *mut ::std::os::raw::c_int,
        bandidxOut: *mut ::std::os::raw::c_int,
        paramtypeOut: *mut ::std::os::raw::c_int,
        normvalOut: *mut f64,
    ) -> bool {
        match self.pointers.TrackFX_GetEQParam {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetEQParam)
            )),
            Some(f) => f(
                track,
                fxidx,
                paramidx,
                bandtypeOut,
                bandidxOut,
                paramtypeOut,
                normvalOut,
            ),
        }
    }
    pub unsafe fn TrackFX_GetFloatingWindow(
        &self,
        track: *mut root::MediaTrack,
        index: ::std::os::raw::c_int,
    ) -> root::HWND {
        match self.pointers.TrackFX_GetFloatingWindow {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetFloatingWindow)
            )),
            Some(f) => f(track, index),
        }
    }
    pub unsafe fn TrackFX_GetFormattedParamValue(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetFormattedParamValue {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetFormattedParamValue)
            )),
            Some(f) => f(track, fx, param, buf, buf_sz),
        }
    }
    pub unsafe fn TrackFX_GetFXGUID(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
    ) -> *mut root::GUID {
        match self.pointers.TrackFX_GetFXGUID {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetFXGUID)
            )),
            Some(f) => f(track, fx),
        }
    }
    pub unsafe fn TrackFX_GetFXName(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetFXName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetFXName)
            )),
            Some(f) => f(track, fx, buf, buf_sz),
        }
    }
    pub unsafe fn TrackFX_GetInstrument(
        &self,
        track: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetInstrument {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetInstrument)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn TrackFX_GetIOSize(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        inputPinsOutOptional: *mut ::std::os::raw::c_int,
        outputPinsOutOptional: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetIOSize {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetIOSize)
            )),
            Some(f) => f(track, fx, inputPinsOutOptional, outputPinsOutOptional),
        }
    }
    pub unsafe fn TrackFX_GetNamedConfigParm(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        parmname: *const ::std::os::raw::c_char,
        bufOut: *mut ::std::os::raw::c_char,
        bufOut_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetNamedConfigParm {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetNamedConfigParm)
            )),
            Some(f) => f(track, fx, parmname, bufOut, bufOut_sz),
        }
    }
    pub unsafe fn TrackFX_GetNumParams(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetNumParams {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetNumParams)
            )),
            Some(f) => f(track, fx),
        }
    }
    pub unsafe fn TrackFX_GetOffline(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetOffline {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetOffline)
            )),
            Some(f) => f(track, fx),
        }
    }
    pub unsafe fn TrackFX_GetOpen(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetOpen {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetOpen)
            )),
            Some(f) => f(track, fx),
        }
    }
    pub unsafe fn TrackFX_GetParam(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        minvalOut: *mut f64,
        maxvalOut: *mut f64,
    ) -> f64 {
        match self.pointers.TrackFX_GetParam {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetParam)
            )),
            Some(f) => f(track, fx, param, minvalOut, maxvalOut),
        }
    }
    pub unsafe fn TrackFX_GetParameterStepSizes(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        stepOut: *mut f64,
        smallstepOut: *mut f64,
        largestepOut: *mut f64,
        istoggleOut: *mut bool,
    ) -> bool {
        match self.pointers.TrackFX_GetParameterStepSizes {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetParameterStepSizes)
            )),
            Some(f) => f(
                track,
                fx,
                param,
                stepOut,
                smallstepOut,
                largestepOut,
                istoggleOut,
            ),
        }
    }
    pub unsafe fn TrackFX_GetParamEx(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        minvalOut: *mut f64,
        maxvalOut: *mut f64,
        midvalOut: *mut f64,
    ) -> f64 {
        match self.pointers.TrackFX_GetParamEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetParamEx)
            )),
            Some(f) => f(track, fx, param, minvalOut, maxvalOut, midvalOut),
        }
    }
    pub unsafe fn TrackFX_GetParamName(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetParamName {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetParamName)
            )),
            Some(f) => f(track, fx, param, buf, buf_sz),
        }
    }
    pub unsafe fn TrackFX_GetParamNormalized(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
    ) -> f64 {
        match self.pointers.TrackFX_GetParamNormalized {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetParamNormalized)
            )),
            Some(f) => f(track, fx, param),
        }
    }
    pub unsafe fn TrackFX_GetPinMappings(
        &self,
        tr: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        isoutput: ::std::os::raw::c_int,
        pin: ::std::os::raw::c_int,
        high32OutOptional: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetPinMappings {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetPinMappings)
            )),
            Some(f) => f(tr, fx, isoutput, pin, high32OutOptional),
        }
    }
    pub unsafe fn TrackFX_GetPreset(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        presetname: *mut ::std::os::raw::c_char,
        presetname_sz: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_GetPreset {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetPreset)
            )),
            Some(f) => f(track, fx, presetname, presetname_sz),
        }
    }
    pub unsafe fn TrackFX_GetPresetIndex(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        numberOfPresetsOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetPresetIndex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetPresetIndex)
            )),
            Some(f) => f(track, fx, numberOfPresetsOut),
        }
    }
    pub unsafe fn TrackFX_GetRecChainVisible(
        &self,
        track: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetRecChainVisible {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetRecChainVisible)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn TrackFX_GetRecCount(
        &self,
        track: *mut root::MediaTrack,
    ) -> ::std::os::raw::c_int {
        match self.pointers.TrackFX_GetRecCount {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetRecCount)
            )),
            Some(f) => f(track),
        }
    }
    pub unsafe fn TrackFX_GetUserPresetFilename(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        fn_: *mut ::std::os::raw::c_char,
        fn_sz: ::std::os::raw::c_int,
    ) {
        match self.pointers.TrackFX_GetUserPresetFilename {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_GetUserPresetFilename)
            )),
            Some(f) => f(track, fx, fn_, fn_sz),
        }
    }
    pub unsafe fn TrackFX_NavigatePresets(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        presetmove: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_NavigatePresets {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_NavigatePresets)
            )),
            Some(f) => f(track, fx, presetmove),
        }
    }
    pub unsafe fn TrackFX_SetEnabled(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        enabled: bool,
    ) {
        match self.pointers.TrackFX_SetEnabled {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetEnabled)
            )),
            Some(f) => f(track, fx, enabled),
        }
    }
    pub unsafe fn TrackFX_SetEQBandEnabled(
        &self,
        track: *mut root::MediaTrack,
        fxidx: ::std::os::raw::c_int,
        bandtype: ::std::os::raw::c_int,
        bandidx: ::std::os::raw::c_int,
        enable: bool,
    ) -> bool {
        match self.pointers.TrackFX_SetEQBandEnabled {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetEQBandEnabled)
            )),
            Some(f) => f(track, fxidx, bandtype, bandidx, enable),
        }
    }
    pub unsafe fn TrackFX_SetEQParam(
        &self,
        track: *mut root::MediaTrack,
        fxidx: ::std::os::raw::c_int,
        bandtype: ::std::os::raw::c_int,
        bandidx: ::std::os::raw::c_int,
        paramtype: ::std::os::raw::c_int,
        val: f64,
        isnorm: bool,
    ) -> bool {
        match self.pointers.TrackFX_SetEQParam {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetEQParam)
            )),
            Some(f) => f(track, fxidx, bandtype, bandidx, paramtype, val, isnorm),
        }
    }
    pub unsafe fn TrackFX_SetNamedConfigParm(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        parmname: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.TrackFX_SetNamedConfigParm {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetNamedConfigParm)
            )),
            Some(f) => f(track, fx, parmname, value),
        }
    }
    pub unsafe fn TrackFX_SetOffline(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        offline: bool,
    ) {
        match self.pointers.TrackFX_SetOffline {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetOffline)
            )),
            Some(f) => f(track, fx, offline),
        }
    }
    pub unsafe fn TrackFX_SetOpen(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        open: bool,
    ) {
        match self.pointers.TrackFX_SetOpen {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetOpen)
            )),
            Some(f) => f(track, fx, open),
        }
    }
    pub unsafe fn TrackFX_SetParam(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        val: f64,
    ) -> bool {
        match self.pointers.TrackFX_SetParam {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetParam)
            )),
            Some(f) => f(track, fx, param, val),
        }
    }
    pub unsafe fn TrackFX_SetParamNormalized(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        value: f64,
    ) -> bool {
        match self.pointers.TrackFX_SetParamNormalized {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetParamNormalized)
            )),
            Some(f) => f(track, fx, param, value),
        }
    }
    pub unsafe fn TrackFX_SetPinMappings(
        &self,
        tr: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        isoutput: ::std::os::raw::c_int,
        pin: ::std::os::raw::c_int,
        low32bits: ::std::os::raw::c_int,
        hi32bits: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_SetPinMappings {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetPinMappings)
            )),
            Some(f) => f(tr, fx, isoutput, pin, low32bits, hi32bits),
        }
    }
    pub unsafe fn TrackFX_SetPreset(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        presetname: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.TrackFX_SetPreset {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetPreset)
            )),
            Some(f) => f(track, fx, presetname),
        }
    }
    pub unsafe fn TrackFX_SetPresetByIndex(
        &self,
        track: *mut root::MediaTrack,
        fx: ::std::os::raw::c_int,
        idx: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.TrackFX_SetPresetByIndex {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_SetPresetByIndex)
            )),
            Some(f) => f(track, fx, idx),
        }
    }
    pub unsafe fn TrackFX_Show(
        &self,
        track: *mut root::MediaTrack,
        index: ::std::os::raw::c_int,
        showFlag: ::std::os::raw::c_int,
    ) {
        match self.pointers.TrackFX_Show {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackFX_Show)
            )),
            Some(f) => f(track, index, showFlag),
        }
    }
    pub fn TrackList_AdjustWindows(&self, isMinor: bool) {
        match self.pointers.TrackList_AdjustWindows {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackList_AdjustWindows)
            )),
            Some(f) => f(isMinor),
        }
    }
    pub fn TrackList_UpdateAllExternalSurfaces(&self) {
        match self.pointers.TrackList_UpdateAllExternalSurfaces {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(TrackList_UpdateAllExternalSurfaces)
            )),
            Some(f) => f(),
        }
    }
    pub fn Undo_BeginBlock(&self) {
        match self.pointers.Undo_BeginBlock {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_BeginBlock)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn Undo_BeginBlock2(&self, proj: *mut root::ReaProject) {
        match self.pointers.Undo_BeginBlock2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_BeginBlock2)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn Undo_CanRedo2(
        &self,
        proj: *mut root::ReaProject,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.Undo_CanRedo2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_CanRedo2)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn Undo_CanUndo2(
        &self,
        proj: *mut root::ReaProject,
    ) -> *const ::std::os::raw::c_char {
        match self.pointers.Undo_CanUndo2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_CanUndo2)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn Undo_DoRedo2(&self, proj: *mut root::ReaProject) -> ::std::os::raw::c_int {
        match self.pointers.Undo_DoRedo2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_DoRedo2)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn Undo_DoUndo2(&self, proj: *mut root::ReaProject) -> ::std::os::raw::c_int {
        match self.pointers.Undo_DoUndo2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_DoUndo2)
            )),
            Some(f) => f(proj),
        }
    }
    pub unsafe fn Undo_EndBlock(
        &self,
        descchange: *const ::std::os::raw::c_char,
        extraflags: ::std::os::raw::c_int,
    ) {
        match self.pointers.Undo_EndBlock {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_EndBlock)
            )),
            Some(f) => f(descchange, extraflags),
        }
    }
    pub unsafe fn Undo_EndBlock2(
        &self,
        proj: *mut root::ReaProject,
        descchange: *const ::std::os::raw::c_char,
        extraflags: ::std::os::raw::c_int,
    ) {
        match self.pointers.Undo_EndBlock2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_EndBlock2)
            )),
            Some(f) => f(proj, descchange, extraflags),
        }
    }
    pub unsafe fn Undo_OnStateChange(&self, descchange: *const ::std::os::raw::c_char) {
        match self.pointers.Undo_OnStateChange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_OnStateChange)
            )),
            Some(f) => f(descchange),
        }
    }
    pub unsafe fn Undo_OnStateChange2(
        &self,
        proj: *mut root::ReaProject,
        descchange: *const ::std::os::raw::c_char,
    ) {
        match self.pointers.Undo_OnStateChange2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_OnStateChange2)
            )),
            Some(f) => f(proj, descchange),
        }
    }
    pub unsafe fn Undo_OnStateChange_Item(
        &self,
        proj: *mut root::ReaProject,
        name: *const ::std::os::raw::c_char,
        item: *mut root::MediaItem,
    ) {
        match self.pointers.Undo_OnStateChange_Item {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_OnStateChange_Item)
            )),
            Some(f) => f(proj, name, item),
        }
    }
    pub unsafe fn Undo_OnStateChangeEx(
        &self,
        descchange: *const ::std::os::raw::c_char,
        whichStates: ::std::os::raw::c_int,
        trackparm: ::std::os::raw::c_int,
    ) {
        match self.pointers.Undo_OnStateChangeEx {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_OnStateChangeEx)
            )),
            Some(f) => f(descchange, whichStates, trackparm),
        }
    }
    pub unsafe fn Undo_OnStateChangeEx2(
        &self,
        proj: *mut root::ReaProject,
        descchange: *const ::std::os::raw::c_char,
        whichStates: ::std::os::raw::c_int,
        trackparm: ::std::os::raw::c_int,
    ) {
        match self.pointers.Undo_OnStateChangeEx2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(Undo_OnStateChangeEx2)
            )),
            Some(f) => f(proj, descchange, whichStates, trackparm),
        }
    }
    pub fn update_disk_counters(
        &self,
        readamt: ::std::os::raw::c_int,
        writeamt: ::std::os::raw::c_int,
    ) {
        match self.pointers.update_disk_counters {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(update_disk_counters)
            )),
            Some(f) => f(readamt, writeamt),
        }
    }
    pub fn UpdateArrange(&self) {
        match self.pointers.UpdateArrange {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(UpdateArrange)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn UpdateItemInProject(&self, item: *mut root::MediaItem) {
        match self.pointers.UpdateItemInProject {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(UpdateItemInProject)
            )),
            Some(f) => f(item),
        }
    }
    pub fn UpdateTimeline(&self) {
        match self.pointers.UpdateTimeline {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(UpdateTimeline)
            )),
            Some(f) => f(),
        }
    }
    pub unsafe fn ValidatePtr(
        &self,
        pointer: *mut ::std::os::raw::c_void,
        ctypename: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.ValidatePtr {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ValidatePtr)
            )),
            Some(f) => f(pointer, ctypename),
        }
    }
    pub unsafe fn ValidatePtr2(
        &self,
        proj: *mut root::ReaProject,
        pointer: *mut ::std::os::raw::c_void,
        ctypename: *const ::std::os::raw::c_char,
    ) -> bool {
        match self.pointers.ValidatePtr2 {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ValidatePtr2)
            )),
            Some(f) => f(proj, pointer, ctypename),
        }
    }
    pub unsafe fn ViewPrefs(
        &self,
        page: ::std::os::raw::c_int,
        pageByName: *const ::std::os::raw::c_char,
    ) {
        match self.pointers.ViewPrefs {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(ViewPrefs)
            )),
            Some(f) => f(page, pageByName),
        }
    }
    pub unsafe fn WDL_VirtualWnd_ScaledBlitBG(
        &self,
        dest: *mut root::reaper_functions::LICE_IBitmap,
        src: *mut root::reaper_functions::WDL_VirtualWnd_BGCfg,
        destx: ::std::os::raw::c_int,
        desty: ::std::os::raw::c_int,
        destw: ::std::os::raw::c_int,
        desth: ::std::os::raw::c_int,
        clipx: ::std::os::raw::c_int,
        clipy: ::std::os::raw::c_int,
        clipw: ::std::os::raw::c_int,
        cliph: ::std::os::raw::c_int,
        alpha: f32,
        mode: ::std::os::raw::c_int,
    ) -> bool {
        match self.pointers.WDL_VirtualWnd_ScaledBlitBG {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(WDL_VirtualWnd_ScaledBlitBG)
            )),
            Some(f) => f(
                dest, src, destx, desty, destw, desth, clipx, clipy, clipw, cliph, alpha, mode,
            ),
        }
    }
    pub fn GetMidiInput(&self, idx: ::std::os::raw::c_int) -> *mut root::midi_Input {
        match self.pointers.GetMidiInput {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMidiInput)
            )),
            Some(f) => f(idx),
        }
    }
    pub fn GetMidiOutput(&self, idx: ::std::os::raw::c_int) -> *mut root::midi_Output {
        match self.pointers.GetMidiOutput {
            None => panic!(format!(
                "Attempt to use a REAPER function that has not been loaded: {}",
                stringify!(GetMidiOutput)
            )),
            Some(f) => f(idx),
        }
    }
}
#[doc = r" Container for the REAPER function pointers."]
#[derive(Default)]
pub struct ReaperFunctionPointers {
    pub __mergesort: Option<
        unsafe extern "C" fn(
            base: *mut ::std::os::raw::c_void,
            nmemb: usize,
            size: usize,
            cmpfunc: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *const ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
            tmpspace: *mut ::std::os::raw::c_void,
        ),
    >,
    pub AddCustomizableMenu: Option<
        unsafe extern "C" fn(
            menuidstr: *const ::std::os::raw::c_char,
            menuname: *const ::std::os::raw::c_char,
            kbdsecname: *const ::std::os::raw::c_char,
            addtomainmenu: bool,
        ) -> bool,
    >,
    pub AddExtensionsMainMenu: Option<extern "C" fn() -> bool>,
    pub AddMediaItemToTrack:
        Option<unsafe extern "C" fn(tr: *mut root::MediaTrack) -> *mut root::MediaItem>,
    pub AddProjectMarker: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            isrgn: bool,
            pos: f64,
            rgnend: f64,
            name: *const ::std::os::raw::c_char,
            wantidx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub AddProjectMarker2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            isrgn: bool,
            pos: f64,
            rgnend: f64,
            name: *const ::std::os::raw::c_char,
            wantidx: ::std::os::raw::c_int,
            color: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub AddRemoveReaScript: Option<
        unsafe extern "C" fn(
            add: bool,
            sectionID: ::std::os::raw::c_int,
            scriptfn: *const ::std::os::raw::c_char,
            commit: bool,
        ) -> ::std::os::raw::c_int,
    >,
    pub AddTakeToMediaItem:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> *mut root::MediaItem_Take>,
    pub AddTempoTimeSigMarker: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            timepos: f64,
            bpm: f64,
            timesig_num: ::std::os::raw::c_int,
            timesig_denom: ::std::os::raw::c_int,
            lineartempochange: bool,
        ) -> bool,
    >,
    pub adjustZoom: Option<
        extern "C" fn(
            amt: f64,
            forceset: ::std::os::raw::c_int,
            doupd: bool,
            centermode: ::std::os::raw::c_int,
        ),
    >,
    pub AnyTrackSolo: Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> bool>,
    pub APIExists:
        Option<unsafe extern "C" fn(function_name: *const ::std::os::raw::c_char) -> bool>,
    pub APITest: Option<extern "C" fn()>,
    pub ApplyNudge: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            nudgeflag: ::std::os::raw::c_int,
            nudgewhat: ::std::os::raw::c_int,
            nudgeunits: ::std::os::raw::c_int,
            value: f64,
            reverse: bool,
            copies: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub ArmCommand: Option<
        unsafe extern "C" fn(
            cmd: ::std::os::raw::c_int,
            sectionname: *const ::std::os::raw::c_char,
        ),
    >,
    pub Audio_Init: Option<extern "C" fn()>,
    pub Audio_IsPreBuffer: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub Audio_IsRunning: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub Audio_Quit: Option<extern "C" fn()>,
    pub Audio_RegHardwareHook: Option<
        unsafe extern "C" fn(
            isAdd: bool,
            reg: *mut root::audio_hook_register_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub AudioAccessorStateChanged:
        Option<unsafe extern "C" fn(accessor: *mut root::reaper_functions::AudioAccessor) -> bool>,
    pub AudioAccessorUpdate:
        Option<unsafe extern "C" fn(accessor: *mut root::reaper_functions::AudioAccessor)>,
    pub AudioAccessorValidateState:
        Option<unsafe extern "C" fn(accessor: *mut root::reaper_functions::AudioAccessor) -> bool>,
    pub BypassFxAllTracks: Option<extern "C" fn(bypass: ::std::os::raw::c_int)>,
    pub CalculatePeaks: Option<
        unsafe extern "C" fn(
            srcBlock: *mut root::PCM_source_transfer_t,
            pksBlock: *mut root::PCM_source_peaktransfer_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub CalculatePeaksFloatSrcPtr: Option<
        unsafe extern "C" fn(
            srcBlock: *mut root::PCM_source_transfer_t,
            pksBlock: *mut root::PCM_source_peaktransfer_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub ClearAllRecArmed: Option<extern "C" fn()>,
    pub ClearConsole: Option<extern "C" fn()>,
    pub ClearPeakCache: Option<extern "C" fn()>,
    pub ColorFromNative: Option<
        unsafe extern "C" fn(
            col: ::std::os::raw::c_int,
            rOut: *mut ::std::os::raw::c_int,
            gOut: *mut ::std::os::raw::c_int,
            bOut: *mut ::std::os::raw::c_int,
        ),
    >,
    pub ColorToNative: Option<
        extern "C" fn(
            r: ::std::os::raw::c_int,
            g: ::std::os::raw::c_int,
            b: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub CountActionShortcuts: Option<
        unsafe extern "C" fn(
            section: *mut root::KbdSectionInfo,
            cmdID: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub CountAutomationItems:
        Option<unsafe extern "C" fn(env: *mut root::TrackEnvelope) -> ::std::os::raw::c_int>,
    pub CountEnvelopePoints:
        Option<unsafe extern "C" fn(envelope: *mut root::TrackEnvelope) -> ::std::os::raw::c_int>,
    pub CountEnvelopePointsEx: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub CountMediaItems:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub CountProjectMarkers: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            num_markersOut: *mut ::std::os::raw::c_int,
            num_regionsOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub CountSelectedMediaItems:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub CountSelectedTracks:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub CountSelectedTracks2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            wantmaster: bool,
        ) -> ::std::os::raw::c_int,
    >,
    pub CountTakeEnvelopes:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> ::std::os::raw::c_int>,
    pub CountTakes:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> ::std::os::raw::c_int>,
    pub CountTCPFXParms: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            track: *mut root::MediaTrack,
        ) -> ::std::os::raw::c_int,
    >,
    pub CountTempoTimeSigMarkers:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub CountTrackEnvelopes:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub CountTrackMediaItems:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub CountTracks:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub CreateLocalOscHandler: Option<
        unsafe extern "C" fn(
            obj: *mut ::std::os::raw::c_void,
            callback: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub CreateMIDIInput: Option<extern "C" fn(dev: ::std::os::raw::c_int) -> *mut root::midi_Input>,
    pub CreateMIDIOutput: Option<
        unsafe extern "C" fn(
            dev: ::std::os::raw::c_int,
            streamMode: bool,
            msoffset100: *mut ::std::os::raw::c_int,
        ) -> *mut root::midi_Output,
    >,
    pub CreateNewMIDIItemInProj: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            starttime: f64,
            endtime: f64,
            qnInOptional: *const bool,
        ) -> *mut root::MediaItem,
    >,
    pub CreateTakeAudioAccessor: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
        ) -> *mut root::reaper_functions::AudioAccessor,
    >,
    pub CreateTrackAudioAccessor: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
        ) -> *mut root::reaper_functions::AudioAccessor,
    >,
    pub CreateTrackSend: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            desttrInOptional: *mut root::MediaTrack,
        ) -> ::std::os::raw::c_int,
    >,
    pub CSurf_FlushUndo: Option<extern "C" fn(force: bool)>,
    pub CSurf_GetTouchState: Option<
        unsafe extern "C" fn(trackid: *mut root::MediaTrack, isPan: ::std::os::raw::c_int) -> bool,
    >,
    pub CSurf_GoEnd: Option<extern "C" fn()>,
    pub CSurf_GoStart: Option<extern "C" fn()>,
    pub CSurf_NumTracks: Option<extern "C" fn(mcpView: bool) -> ::std::os::raw::c_int>,
    pub CSurf_OnArrow: Option<extern "C" fn(whichdir: ::std::os::raw::c_int, wantzoom: bool)>,
    pub CSurf_OnFwd: Option<extern "C" fn(seekplay: ::std::os::raw::c_int)>,
    pub CSurf_OnFXChange: Option<
        unsafe extern "C" fn(trackid: *mut root::MediaTrack, en: ::std::os::raw::c_int) -> bool,
    >,
    pub CSurf_OnInputMonitorChange: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            monitor: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub CSurf_OnInputMonitorChangeEx: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            monitor: ::std::os::raw::c_int,
            allowgang: bool,
        ) -> ::std::os::raw::c_int,
    >,
    pub CSurf_OnMuteChange: Option<
        unsafe extern "C" fn(trackid: *mut root::MediaTrack, mute: ::std::os::raw::c_int) -> bool,
    >,
    pub CSurf_OnMuteChangeEx: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            mute: ::std::os::raw::c_int,
            allowgang: bool,
        ) -> bool,
    >,
    pub CSurf_OnOscControlMessage:
        Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char, arg: *const f32)>,
    pub CSurf_OnPanChange: Option<
        unsafe extern "C" fn(trackid: *mut root::MediaTrack, pan: f64, relative: bool) -> f64,
    >,
    pub CSurf_OnPanChangeEx: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            pan: f64,
            relative: bool,
            allowGang: bool,
        ) -> f64,
    >,
    pub CSurf_OnPause: Option<extern "C" fn()>,
    pub CSurf_OnPlay: Option<extern "C" fn()>,
    pub CSurf_OnPlayRateChange: Option<extern "C" fn(playrate: f64)>,
    pub CSurf_OnRecArmChange: Option<
        unsafe extern "C" fn(trackid: *mut root::MediaTrack, recarm: ::std::os::raw::c_int) -> bool,
    >,
    pub CSurf_OnRecArmChangeEx: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            recarm: ::std::os::raw::c_int,
            allowgang: bool,
        ) -> bool,
    >,
    pub CSurf_OnRecord: Option<extern "C" fn()>,
    pub CSurf_OnRecvPanChange: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            recv_index: ::std::os::raw::c_int,
            pan: f64,
            relative: bool,
        ) -> f64,
    >,
    pub CSurf_OnRecvVolumeChange: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            recv_index: ::std::os::raw::c_int,
            volume: f64,
            relative: bool,
        ) -> f64,
    >,
    pub CSurf_OnRew: Option<extern "C" fn(seekplay: ::std::os::raw::c_int)>,
    pub CSurf_OnRewFwd:
        Option<extern "C" fn(seekplay: ::std::os::raw::c_int, dir: ::std::os::raw::c_int)>,
    pub CSurf_OnScroll:
        Option<extern "C" fn(xdir: ::std::os::raw::c_int, ydir: ::std::os::raw::c_int)>,
    pub CSurf_OnSelectedChange: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            selected: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub CSurf_OnSendPanChange: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            send_index: ::std::os::raw::c_int,
            pan: f64,
            relative: bool,
        ) -> f64,
    >,
    pub CSurf_OnSendVolumeChange: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            send_index: ::std::os::raw::c_int,
            volume: f64,
            relative: bool,
        ) -> f64,
    >,
    pub CSurf_OnSoloChange: Option<
        unsafe extern "C" fn(trackid: *mut root::MediaTrack, solo: ::std::os::raw::c_int) -> bool,
    >,
    pub CSurf_OnSoloChangeEx: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            solo: ::std::os::raw::c_int,
            allowgang: bool,
        ) -> bool,
    >,
    pub CSurf_OnStop: Option<extern "C" fn()>,
    pub CSurf_OnTempoChange: Option<extern "C" fn(bpm: f64)>,
    pub CSurf_OnTrackSelection: Option<unsafe extern "C" fn(trackid: *mut root::MediaTrack)>,
    pub CSurf_OnVolumeChange: Option<
        unsafe extern "C" fn(trackid: *mut root::MediaTrack, volume: f64, relative: bool) -> f64,
    >,
    pub CSurf_OnVolumeChangeEx: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            volume: f64,
            relative: bool,
            allowGang: bool,
        ) -> f64,
    >,
    pub CSurf_OnWidthChange: Option<
        unsafe extern "C" fn(trackid: *mut root::MediaTrack, width: f64, relative: bool) -> f64,
    >,
    pub CSurf_OnWidthChangeEx: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            width: f64,
            relative: bool,
            allowGang: bool,
        ) -> f64,
    >,
    pub CSurf_OnZoom:
        Option<extern "C" fn(xdir: ::std::os::raw::c_int, ydir: ::std::os::raw::c_int)>,
    pub CSurf_ResetAllCachedVolPanStates: Option<extern "C" fn()>,
    pub CSurf_ScrubAmt: Option<extern "C" fn(amt: f64)>,
    pub CSurf_SetAutoMode: Option<
        unsafe extern "C" fn(
            mode: ::std::os::raw::c_int,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetPlayState: Option<
        unsafe extern "C" fn(
            play: bool,
            pause: bool,
            rec: bool,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetRepeatState:
        Option<unsafe extern "C" fn(rep: bool, ignoresurf: *mut root::IReaperControlSurface)>,
    pub CSurf_SetSurfaceMute: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            mute: bool,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetSurfacePan: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            pan: f64,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetSurfaceRecArm: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            recarm: bool,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetSurfaceSelected: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            selected: bool,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetSurfaceSolo: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            solo: bool,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetSurfaceVolume: Option<
        unsafe extern "C" fn(
            trackid: *mut root::MediaTrack,
            volume: f64,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetTrackListChange: Option<extern "C" fn()>,
    pub CSurf_TrackFromID:
        Option<extern "C" fn(idx: ::std::os::raw::c_int, mcpView: bool) -> *mut root::MediaTrack>,
    pub CSurf_TrackToID: Option<
        unsafe extern "C" fn(track: *mut root::MediaTrack, mcpView: bool) -> ::std::os::raw::c_int,
    >,
    pub DB2SLIDER: Option<extern "C" fn(x: f64) -> f64>,
    pub DeleteActionShortcut: Option<
        unsafe extern "C" fn(
            section: *mut root::KbdSectionInfo,
            cmdID: ::std::os::raw::c_int,
            shortcutidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub DeleteEnvelopePointEx: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
            ptidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub DeleteEnvelopePointRange: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            time_start: f64,
            time_end: f64,
        ) -> bool,
    >,
    pub DeleteEnvelopePointRangeEx: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
            time_start: f64,
            time_end: f64,
        ) -> bool,
    >,
    pub DeleteExtState: Option<
        unsafe extern "C" fn(
            section: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_char,
            persist: bool,
        ),
    >,
    pub DeleteProjectMarker: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            markrgnindexnumber: ::std::os::raw::c_int,
            isrgn: bool,
        ) -> bool,
    >,
    pub DeleteProjectMarkerByIndex: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            markrgnidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub DeleteTakeStretchMarkers: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            idx: ::std::os::raw::c_int,
            countInOptional: *const ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub DeleteTempoTimeSigMarker: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            markerindex: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub DeleteTrack: Option<unsafe extern "C" fn(tr: *mut root::MediaTrack)>,
    pub DeleteTrackMediaItem:
        Option<unsafe extern "C" fn(tr: *mut root::MediaTrack, it: *mut root::MediaItem) -> bool>,
    pub DestroyAudioAccessor:
        Option<unsafe extern "C" fn(accessor: *mut root::reaper_functions::AudioAccessor)>,
    pub DestroyLocalOscHandler:
        Option<unsafe extern "C" fn(local_osc_handler: *mut ::std::os::raw::c_void)>,
    pub DoActionShortcutDialog: Option<
        unsafe extern "C" fn(
            hwnd: root::HWND,
            section: *mut root::KbdSectionInfo,
            cmdID: ::std::os::raw::c_int,
            shortcutidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub Dock_UpdateDockID: Option<
        unsafe extern "C" fn(
            ident_str: *const ::std::os::raw::c_char,
            whichDock: ::std::os::raw::c_int,
        ),
    >,
    pub DockGetPosition:
        Option<extern "C" fn(whichDock: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,
    pub DockIsChildOfDock: Option<
        unsafe extern "C" fn(
            hwnd: root::HWND,
            isFloatingDockerOut: *mut bool,
        ) -> ::std::os::raw::c_int,
    >,
    pub DockWindowActivate: Option<extern "C" fn(hwnd: root::HWND)>,
    pub DockWindowAdd: Option<
        unsafe extern "C" fn(
            hwnd: root::HWND,
            name: *const ::std::os::raw::c_char,
            pos: ::std::os::raw::c_int,
            allowShow: bool,
        ),
    >,
    pub DockWindowAddEx: Option<
        unsafe extern "C" fn(
            hwnd: root::HWND,
            name: *const ::std::os::raw::c_char,
            identstr: *const ::std::os::raw::c_char,
            allowShow: bool,
        ),
    >,
    pub DockWindowRefresh: Option<extern "C" fn()>,
    pub DockWindowRefreshForHWND: Option<extern "C" fn(hwnd: root::HWND)>,
    pub DockWindowRemove: Option<extern "C" fn(hwnd: root::HWND)>,
    pub DuplicateCustomizableMenu: Option<
        unsafe extern "C" fn(
            srcmenu: *mut ::std::os::raw::c_void,
            destmenu: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub EditTempoTimeSigMarker: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            markerindex: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub EnsureNotCompletelyOffscreen: Option<unsafe extern "C" fn(rInOut: *mut root::RECT)>,
    pub EnumerateFiles: Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            fileindex: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub EnumerateSubdirectories: Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            subdirindex: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub EnumPitchShiftModes: Option<
        unsafe extern "C" fn(
            mode: ::std::os::raw::c_int,
            strOut: *mut *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub EnumPitchShiftSubModes: Option<
        extern "C" fn(
            mode: ::std::os::raw::c_int,
            submode: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub EnumProjectMarkers: Option<
        unsafe extern "C" fn(
            idx: ::std::os::raw::c_int,
            isrgnOut: *mut bool,
            posOut: *mut f64,
            rgnendOut: *mut f64,
            nameOut: *mut *const ::std::os::raw::c_char,
            markrgnindexnumberOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub EnumProjectMarkers2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            idx: ::std::os::raw::c_int,
            isrgnOut: *mut bool,
            posOut: *mut f64,
            rgnendOut: *mut f64,
            nameOut: *mut *const ::std::os::raw::c_char,
            markrgnindexnumberOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub EnumProjectMarkers3: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            idx: ::std::os::raw::c_int,
            isrgnOut: *mut bool,
            posOut: *mut f64,
            rgnendOut: *mut f64,
            nameOut: *mut *const ::std::os::raw::c_char,
            markrgnindexnumberOut: *mut ::std::os::raw::c_int,
            colorOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub EnumProjects: Option<
        unsafe extern "C" fn(
            idx: ::std::os::raw::c_int,
            projfnOutOptional: *mut ::std::os::raw::c_char,
            projfnOutOptional_sz: ::std::os::raw::c_int,
        ) -> *mut root::ReaProject,
    >,
    pub EnumProjExtState: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            extname: *const ::std::os::raw::c_char,
            idx: ::std::os::raw::c_int,
            keyOutOptional: *mut ::std::os::raw::c_char,
            keyOutOptional_sz: ::std::os::raw::c_int,
            valOutOptional: *mut ::std::os::raw::c_char,
            valOutOptional_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub EnumRegionRenderMatrix: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            regionindex: ::std::os::raw::c_int,
            rendertrack: ::std::os::raw::c_int,
        ) -> *mut root::MediaTrack,
    >,
    pub EnumTrackMIDIProgramNames: Option<
        unsafe extern "C" fn(
            track: ::std::os::raw::c_int,
            programNumber: ::std::os::raw::c_int,
            programName: *mut ::std::os::raw::c_char,
            programName_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub EnumTrackMIDIProgramNamesEx: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            track: *mut root::MediaTrack,
            programNumber: ::std::os::raw::c_int,
            programName: *mut ::std::os::raw::c_char,
            programName_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub Envelope_Evaluate: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            time: f64,
            samplerate: f64,
            samplesRequested: ::std::os::raw::c_int,
            valueOutOptional: *mut f64,
            dVdSOutOptional: *mut f64,
            ddVdSOutOptional: *mut f64,
            dddVdSOutOptional: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub Envelope_FormatValue: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            value: f64,
            bufOut: *mut ::std::os::raw::c_char,
            bufOut_sz: ::std::os::raw::c_int,
        ),
    >,
    pub Envelope_GetParentTake: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            indexOutOptional: *mut ::std::os::raw::c_int,
            index2OutOptional: *mut ::std::os::raw::c_int,
        ) -> *mut root::MediaItem_Take,
    >,
    pub Envelope_GetParentTrack: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            indexOutOptional: *mut ::std::os::raw::c_int,
            index2OutOptional: *mut ::std::os::raw::c_int,
        ) -> *mut root::MediaTrack,
    >,
    pub Envelope_SortPoints:
        Option<unsafe extern "C" fn(envelope: *mut root::TrackEnvelope) -> bool>,
    pub Envelope_SortPointsEx: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub ExecProcess: Option<
        unsafe extern "C" fn(
            cmdline: *const ::std::os::raw::c_char,
            timeoutmsec: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub file_exists: Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> bool>,
    pub FindTempoTimeSigMarker: Option<
        unsafe extern "C" fn(project: *mut root::ReaProject, time: f64) -> ::std::os::raw::c_int,
    >,
    pub format_timestr: Option<
        unsafe extern "C" fn(
            tpos: f64,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ),
    >,
    pub format_timestr_len: Option<
        unsafe extern "C" fn(
            tpos: f64,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
            offset: f64,
            modeoverride: ::std::os::raw::c_int,
        ),
    >,
    pub format_timestr_pos: Option<
        unsafe extern "C" fn(
            tpos: f64,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
            modeoverride: ::std::os::raw::c_int,
        ),
    >,
    pub FreeHeapPtr: Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void)>,
    pub genGuid: Option<unsafe extern "C" fn(g: *mut root::GUID)>,
    pub get_config_var: Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            szOut: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_config_var_string: Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            bufOut: *mut ::std::os::raw::c_char,
            bufOut_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub get_ini_file: Option<extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub get_midi_config_var: Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            szOut: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetActionShortcutDesc: Option<
        unsafe extern "C" fn(
            section: *mut root::KbdSectionInfo,
            cmdID: ::std::os::raw::c_int,
            shortcutidx: ::std::os::raw::c_int,
            desc: *mut ::std::os::raw::c_char,
            desclen: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetActiveTake:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> *mut root::MediaItem_Take>,
    pub GetAllProjectPlayStates:
        Option<unsafe extern "C" fn(ignoreProject: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub GetAppVersion: Option<extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub GetArmedCommand: Option<
        unsafe extern "C" fn(
            secOut: *mut ::std::os::raw::c_char,
            secOut_sz: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetAudioAccessorEndTime:
        Option<unsafe extern "C" fn(accessor: *mut root::reaper_functions::AudioAccessor) -> f64>,
    pub GetAudioAccessorHash: Option<
        unsafe extern "C" fn(
            accessor: *mut root::reaper_functions::AudioAccessor,
            hashNeed128: *mut ::std::os::raw::c_char,
        ),
    >,
    pub GetAudioAccessorSamples: Option<
        unsafe extern "C" fn(
            accessor: *mut root::reaper_functions::AudioAccessor,
            samplerate: ::std::os::raw::c_int,
            numchannels: ::std::os::raw::c_int,
            starttime_sec: f64,
            numsamplesperchannel: ::std::os::raw::c_int,
            samplebuffer: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetAudioAccessorStartTime:
        Option<unsafe extern "C" fn(accessor: *mut root::reaper_functions::AudioAccessor) -> f64>,
    pub GetAudioDeviceInfo: Option<
        unsafe extern "C" fn(
            attribute: *const ::std::os::raw::c_char,
            desc: *mut ::std::os::raw::c_char,
            desc_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetColorTheme: Option<
        extern "C" fn(idx: ::std::os::raw::c_int, defval: ::std::os::raw::c_int) -> root::INT_PTR,
    >,
    pub GetColorThemeStruct: Option<
        unsafe extern "C" fn(szOut: *mut ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void,
    >,
    pub GetConfigWantsDock: Option<
        unsafe extern "C" fn(ident_str: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub GetContextMenu: Option<extern "C" fn(idx: ::std::os::raw::c_int) -> root::HMENU>,
    pub GetCurrentProjectInLoadSave: Option<extern "C" fn() -> *mut root::ReaProject>,
    pub GetCursorContext: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetCursorContext2: Option<extern "C" fn(want_last_valid: bool) -> ::std::os::raw::c_int>,
    pub GetCursorPosition: Option<extern "C" fn() -> f64>,
    pub GetCursorPositionEx: Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> f64>,
    pub GetDisplayedMediaItemColor:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> ::std::os::raw::c_int>,
    pub GetDisplayedMediaItemColor2: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            take: *mut root::MediaItem_Take,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetEnvelopeInfo_Value: Option<
        unsafe extern "C" fn(
            tr: *mut root::TrackEnvelope,
            parmname: *const ::std::os::raw::c_char,
        ) -> f64,
    >,
    pub GetEnvelopeName: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            bufOut: *mut ::std::os::raw::c_char,
            bufOut_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetEnvelopePoint: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            ptidx: ::std::os::raw::c_int,
            timeOutOptional: *mut f64,
            valueOutOptional: *mut f64,
            shapeOutOptional: *mut ::std::os::raw::c_int,
            tensionOutOptional: *mut f64,
            selectedOutOptional: *mut bool,
        ) -> bool,
    >,
    pub GetEnvelopePointByTime: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            time: f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetEnvelopePointByTimeEx: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
            time: f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetEnvelopePointEx: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
            ptidx: ::std::os::raw::c_int,
            timeOutOptional: *mut f64,
            valueOutOptional: *mut f64,
            shapeOutOptional: *mut ::std::os::raw::c_int,
            tensionOutOptional: *mut f64,
            selectedOutOptional: *mut bool,
        ) -> bool,
    >,
    pub GetEnvelopeScalingMode:
        Option<unsafe extern "C" fn(env: *mut root::TrackEnvelope) -> ::std::os::raw::c_int>,
    pub GetEnvelopeStateChunk: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            strNeedBig: *mut ::std::os::raw::c_char,
            strNeedBig_sz: ::std::os::raw::c_int,
            isundoOptional: bool,
        ) -> bool,
    >,
    pub GetExePath: Option<extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub GetExtState: Option<
        unsafe extern "C" fn(
            section: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub GetFocusedFX: Option<
        unsafe extern "C" fn(
            tracknumberOut: *mut ::std::os::raw::c_int,
            itemnumberOut: *mut ::std::os::raw::c_int,
            fxnumberOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetFreeDiskSpaceForRecordPath: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            pathidx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetFXEnvelope: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fxindex: ::std::os::raw::c_int,
            parameterindex: ::std::os::raw::c_int,
            create: bool,
        ) -> *mut root::TrackEnvelope,
    >,
    pub GetGlobalAutomationOverride: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetHZoomLevel: Option<extern "C" fn() -> f64>,
    pub GetIconThemePointer: Option<
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
    >,
    pub GetIconThemePointerForDPI: Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            dpisc: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetIconThemeStruct: Option<
        unsafe extern "C" fn(szOut: *mut ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void,
    >,
    pub GetInputChannelName:
        Option<extern "C" fn(channelIndex: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char>,
    pub GetInputOutputLatency: Option<
        unsafe extern "C" fn(
            inputlatencyOut: *mut ::std::os::raw::c_int,
            outputLatencyOut: *mut ::std::os::raw::c_int,
        ),
    >,
    pub GetItemEditingTime2: Option<
        unsafe extern "C" fn(
            which_itemOut: *mut *mut root::PCM_source,
            flagsOut: *mut ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub GetItemFromPoint: Option<
        unsafe extern "C" fn(
            screen_x: ::std::os::raw::c_int,
            screen_y: ::std::os::raw::c_int,
            allow_locked: bool,
            takeOutOptional: *mut *mut root::MediaItem_Take,
        ) -> *mut root::MediaItem,
    >,
    pub GetItemProjectContext:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> *mut root::ReaProject>,
    pub GetItemStateChunk: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            strNeedBig: *mut ::std::os::raw::c_char,
            strNeedBig_sz: ::std::os::raw::c_int,
            isundoOptional: bool,
        ) -> bool,
    >,
    pub GetLastColorThemeFile: Option<extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub GetLastMarkerAndCurRegion: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            time: f64,
            markeridxOut: *mut ::std::os::raw::c_int,
            regionidxOut: *mut ::std::os::raw::c_int,
        ),
    >,
    pub GetLastTouchedFX: Option<
        unsafe extern "C" fn(
            tracknumberOut: *mut ::std::os::raw::c_int,
            fxnumberOut: *mut ::std::os::raw::c_int,
            paramnumberOut: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetLastTouchedTrack: Option<extern "C" fn() -> *mut root::MediaTrack>,
    pub GetMainHwnd: Option<extern "C" fn() -> root::HWND>,
    pub GetMasterMuteSoloFlags: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetMasterTrack:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> *mut root::MediaTrack>,
    pub GetMasterTrackVisibility: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetMaxMidiInputs: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetMaxMidiOutputs: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetMediaItem: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            itemidx: ::std::os::raw::c_int,
        ) -> *mut root::MediaItem,
    >,
    pub GetMediaItem_Track:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> *mut root::MediaTrack>,
    pub GetMediaItemInfo_Value: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            parmname: *const ::std::os::raw::c_char,
        ) -> f64,
    >,
    pub GetMediaItemNumTakes:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> ::std::os::raw::c_int>,
    pub GetMediaItemTake: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            tk: ::std::os::raw::c_int,
        ) -> *mut root::MediaItem_Take,
    >,
    pub GetMediaItemTake_Item:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> *mut root::MediaItem>,
    pub GetMediaItemTake_Peaks: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            peakrate: f64,
            starttime: f64,
            numchannels: ::std::os::raw::c_int,
            numsamplesperchannel: ::std::os::raw::c_int,
            want_extra_type: ::std::os::raw::c_int,
            buf: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetMediaItemTake_Source:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> *mut root::PCM_source>,
    pub GetMediaItemTake_Track:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> *mut root::MediaTrack>,
    pub GetMediaItemTakeByGUID: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            guid: *const root::GUID,
        ) -> *mut root::MediaItem_Take,
    >,
    pub GetMediaItemTakeInfo_Value: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            parmname: *const ::std::os::raw::c_char,
        ) -> f64,
    >,
    pub GetMediaItemTrack:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> *mut root::MediaTrack>,
    pub GetMediaSourceFileName: Option<
        unsafe extern "C" fn(
            source: *mut root::PCM_source,
            filenamebuf: *mut ::std::os::raw::c_char,
            filenamebuf_sz: ::std::os::raw::c_int,
        ),
    >,
    pub GetMediaSourceLength: Option<
        unsafe extern "C" fn(source: *mut root::PCM_source, lengthIsQNOut: *mut bool) -> f64,
    >,
    pub GetMediaSourceNumChannels:
        Option<unsafe extern "C" fn(source: *mut root::PCM_source) -> ::std::os::raw::c_int>,
    pub GetMediaSourceParent:
        Option<unsafe extern "C" fn(src: *mut root::PCM_source) -> *mut root::PCM_source>,
    pub GetMediaSourceSampleRate:
        Option<unsafe extern "C" fn(source: *mut root::PCM_source) -> ::std::os::raw::c_int>,
    pub GetMediaSourceType: Option<
        unsafe extern "C" fn(
            source: *mut root::PCM_source,
            typebuf: *mut ::std::os::raw::c_char,
            typebuf_sz: ::std::os::raw::c_int,
        ),
    >,
    pub GetMediaTrackInfo_Value: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            parmname: *const ::std::os::raw::c_char,
        ) -> f64,
    >,
    pub GetMIDIInputName: Option<
        unsafe extern "C" fn(
            dev: ::std::os::raw::c_int,
            nameout: *mut ::std::os::raw::c_char,
            nameout_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetMIDIOutputName: Option<
        unsafe extern "C" fn(
            dev: ::std::os::raw::c_int,
            nameout: *mut ::std::os::raw::c_char,
            nameout_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetMixerScroll: Option<extern "C" fn() -> *mut root::MediaTrack>,
    pub GetMouseModifier: Option<
        unsafe extern "C" fn(
            context: *const ::std::os::raw::c_char,
            modifier_flag: ::std::os::raw::c_int,
            action: *mut ::std::os::raw::c_char,
            action_sz: ::std::os::raw::c_int,
        ),
    >,
    pub GetMousePosition: Option<
        unsafe extern "C" fn(xOut: *mut ::std::os::raw::c_int, yOut: *mut ::std::os::raw::c_int),
    >,
    pub GetNumAudioInputs: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetNumAudioOutputs: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetNumMIDIInputs: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetNumMIDIOutputs: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetNumTracks: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetOS: Option<extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub GetOutputChannelName:
        Option<extern "C" fn(channelIndex: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char>,
    pub GetOutputLatency: Option<extern "C" fn() -> f64>,
    pub GetParentTrack:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> *mut root::MediaTrack>,
    pub GetPeakFileName: Option<
        unsafe extern "C" fn(
            fn_: *const ::std::os::raw::c_char,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ),
    >,
    pub GetPeakFileNameEx: Option<
        unsafe extern "C" fn(
            fn_: *const ::std::os::raw::c_char,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
            forWrite: bool,
        ),
    >,
    pub GetPeakFileNameEx2: Option<
        unsafe extern "C" fn(
            fn_: *const ::std::os::raw::c_char,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
            forWrite: bool,
            peaksfileextension: *const ::std::os::raw::c_char,
        ),
    >,
    pub GetPeaksBitmap: Option<
        unsafe extern "C" fn(
            pks: *mut root::PCM_source_peaktransfer_t,
            maxamp: f64,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            bmp: *mut root::reaper_functions::LICE_IBitmap,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetPlayPosition: Option<extern "C" fn() -> f64>,
    pub GetPlayPosition2: Option<extern "C" fn() -> f64>,
    pub GetPlayPosition2Ex: Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> f64>,
    pub GetPlayPositionEx: Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> f64>,
    pub GetPlayState: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub GetPlayStateEx:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub GetPreferredDiskReadMode: Option<
        unsafe extern "C" fn(
            mode: *mut ::std::os::raw::c_int,
            nb: *mut ::std::os::raw::c_int,
            bs: *mut ::std::os::raw::c_int,
        ),
    >,
    pub GetPreferredDiskReadModePeak: Option<
        unsafe extern "C" fn(
            mode: *mut ::std::os::raw::c_int,
            nb: *mut ::std::os::raw::c_int,
            bs: *mut ::std::os::raw::c_int,
        ),
    >,
    pub GetPreferredDiskWriteMode: Option<
        unsafe extern "C" fn(
            mode: *mut ::std::os::raw::c_int,
            nb: *mut ::std::os::raw::c_int,
            bs: *mut ::std::os::raw::c_int,
        ),
    >,
    pub GetProjectLength: Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> f64>,
    pub GetProjectName: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ),
    >,
    pub GetProjectPath: Option<
        unsafe extern "C" fn(buf: *mut ::std::os::raw::c_char, buf_sz: ::std::os::raw::c_int),
    >,
    pub GetProjectPathEx: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ),
    >,
    pub GetProjectStateChangeCount:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub GetProjectTimeOffset:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, rndframe: bool) -> f64>,
    pub GetProjectTimeSignature: Option<unsafe extern "C" fn(bpmOut: *mut f64, bpiOut: *mut f64)>,
    pub GetProjectTimeSignature2: Option<
        unsafe extern "C" fn(proj: *mut root::ReaProject, bpmOut: *mut f64, bpiOut: *mut f64),
    >,
    pub GetProjExtState: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            extname: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_char,
            valOutNeedBig: *mut ::std::os::raw::c_char,
            valOutNeedBig_sz: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetResourcePath: Option<extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub GetSelectedEnvelope:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> *mut root::TrackEnvelope>,
    pub GetSelectedMediaItem: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            selitem: ::std::os::raw::c_int,
        ) -> *mut root::MediaItem,
    >,
    pub GetSelectedTrack: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            seltrackidx: ::std::os::raw::c_int,
        ) -> *mut root::MediaTrack,
    >,
    pub GetSelectedTrack2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            seltrackidx: ::std::os::raw::c_int,
            wantmaster: bool,
        ) -> *mut root::MediaTrack,
    >,
    pub GetSelectedTrackEnvelope:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> *mut root::TrackEnvelope>,
    pub GetSet_ArrangeView2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            isSet: bool,
            screen_x_start: ::std::os::raw::c_int,
            screen_x_end: ::std::os::raw::c_int,
            start_timeOut: *mut f64,
            end_timeOut: *mut f64,
        ),
    >,
    pub GetSet_LoopTimeRange: Option<
        unsafe extern "C" fn(
            isSet: bool,
            isLoop: bool,
            startOut: *mut f64,
            endOut: *mut f64,
            allowautoseek: bool,
        ),
    >,
    pub GetSet_LoopTimeRange2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            isSet: bool,
            isLoop: bool,
            startOut: *mut f64,
            endOut: *mut f64,
            allowautoseek: bool,
        ),
    >,
    pub GetSetAutomationItemInfo: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
            desc: *const ::std::os::raw::c_char,
            value: f64,
            is_set: bool,
        ) -> f64,
    >,
    pub GetSetAutomationItemInfo_String: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
            desc: *const ::std::os::raw::c_char,
            valuestrNeedBig: *mut ::std::os::raw::c_char,
            is_set: bool,
        ) -> bool,
    >,
    pub GetSetEnvelopeInfo_String: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            parmname: *const ::std::os::raw::c_char,
            stringNeedBig: *mut ::std::os::raw::c_char,
            setNewValue: bool,
        ) -> bool,
    >,
    pub GetSetEnvelopeState: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            str: *mut ::std::os::raw::c_char,
            str_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetSetEnvelopeState2: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            str: *mut ::std::os::raw::c_char,
            str_sz: ::std::os::raw::c_int,
            isundo: bool,
        ) -> bool,
    >,
    pub GetSetItemState: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            str: *mut ::std::os::raw::c_char,
            str_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetSetItemState2: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            str: *mut ::std::os::raw::c_char,
            str_sz: ::std::os::raw::c_int,
            isundo: bool,
        ) -> bool,
    >,
    pub GetSetMediaItemInfo: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            parmname: *const ::std::os::raw::c_char,
            setNewValue: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetSetMediaItemInfo_String: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            parmname: *const ::std::os::raw::c_char,
            stringNeedBig: *mut ::std::os::raw::c_char,
            setNewValue: bool,
        ) -> bool,
    >,
    pub GetSetMediaItemTakeInfo: Option<
        unsafe extern "C" fn(
            tk: *mut root::MediaItem_Take,
            parmname: *const ::std::os::raw::c_char,
            setNewValue: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetSetMediaItemTakeInfo_String: Option<
        unsafe extern "C" fn(
            tk: *mut root::MediaItem_Take,
            parmname: *const ::std::os::raw::c_char,
            stringNeedBig: *mut ::std::os::raw::c_char,
            setNewValue: bool,
        ) -> bool,
    >,
    pub GetSetMediaTrackInfo: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            parmname: *const ::std::os::raw::c_char,
            setNewValue: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetSetMediaTrackInfo_String: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            parmname: *const ::std::os::raw::c_char,
            stringNeedBig: *mut ::std::os::raw::c_char,
            setNewValue: bool,
        ) -> bool,
    >,
    pub GetSetObjectState: Option<
        unsafe extern "C" fn(
            obj: *mut ::std::os::raw::c_void,
            str: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub GetSetObjectState2: Option<
        unsafe extern "C" fn(
            obj: *mut ::std::os::raw::c_void,
            str: *const ::std::os::raw::c_char,
            isundo: bool,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub GetSetProjectAuthor: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            set: bool,
            author: *mut ::std::os::raw::c_char,
            author_sz: ::std::os::raw::c_int,
        ),
    >,
    pub GetSetProjectGrid: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            set: bool,
            divisionInOutOptional: *mut f64,
            swingmodeInOutOptional: *mut ::std::os::raw::c_int,
            swingamtInOutOptional: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetSetProjectInfo: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            desc: *const ::std::os::raw::c_char,
            value: f64,
            is_set: bool,
        ) -> f64,
    >,
    pub GetSetProjectInfo_String: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            desc: *const ::std::os::raw::c_char,
            valuestrNeedBig: *mut ::std::os::raw::c_char,
            is_set: bool,
        ) -> bool,
    >,
    pub GetSetProjectNotes: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            set: bool,
            notesNeedBig: *mut ::std::os::raw::c_char,
            notesNeedBig_sz: ::std::os::raw::c_int,
        ),
    >,
    pub GetSetRepeat: Option<extern "C" fn(val: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,
    pub GetSetRepeatEx: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            val: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetSetTrackGroupMembership: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            groupname: *const ::std::os::raw::c_char,
            setmask: ::std::os::raw::c_uint,
            setvalue: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_uint,
    >,
    pub GetSetTrackGroupMembershipHigh: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            groupname: *const ::std::os::raw::c_char,
            setmask: ::std::os::raw::c_uint,
            setvalue: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_uint,
    >,
    pub GetSetTrackMIDISupportFile: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            track: *mut root::MediaTrack,
            which: ::std::os::raw::c_int,
            filename: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub GetSetTrackSendInfo: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            category: ::std::os::raw::c_int,
            sendidx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
            setNewValue: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetSetTrackSendInfo_String: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            category: ::std::os::raw::c_int,
            sendidx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
            stringNeedBig: *mut ::std::os::raw::c_char,
            setNewValue: bool,
        ) -> bool,
    >,
    pub GetSetTrackState: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            str: *mut ::std::os::raw::c_char,
            str_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetSetTrackState2: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            str: *mut ::std::os::raw::c_char,
            str_sz: ::std::os::raw::c_int,
            isundo: bool,
        ) -> bool,
    >,
    pub GetSubProjectFromSource:
        Option<unsafe extern "C" fn(src: *mut root::PCM_source) -> *mut root::ReaProject>,
    pub GetTake: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            takeidx: ::std::os::raw::c_int,
        ) -> *mut root::MediaItem_Take,
    >,
    pub GetTakeEnvelope: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            envidx: ::std::os::raw::c_int,
        ) -> *mut root::TrackEnvelope,
    >,
    pub GetTakeEnvelopeByName: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            envname: *const ::std::os::raw::c_char,
        ) -> *mut root::TrackEnvelope,
    >,
    pub GetTakeName: Option<
        unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> *const ::std::os::raw::c_char,
    >,
    pub GetTakeNumStretchMarkers:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> ::std::os::raw::c_int>,
    pub GetTakeStretchMarker: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            idx: ::std::os::raw::c_int,
            posOut: *mut f64,
            srcposOutOptional: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetTakeStretchMarkerSlope: Option<
        unsafe extern "C" fn(take: *mut root::MediaItem_Take, idx: ::std::os::raw::c_int) -> f64,
    >,
    pub GetTCPFXParm: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            track: *mut root::MediaTrack,
            index: ::std::os::raw::c_int,
            fxindexOut: *mut ::std::os::raw::c_int,
            parmidxOut: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetTempoMatchPlayRate: Option<
        unsafe extern "C" fn(
            source: *mut root::PCM_source,
            srcscale: f64,
            position: f64,
            mult: f64,
            rateOut: *mut f64,
            targetlenOut: *mut f64,
        ) -> bool,
    >,
    pub GetTempoTimeSigMarker: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            ptidx: ::std::os::raw::c_int,
            timeposOut: *mut f64,
            measureposOut: *mut ::std::os::raw::c_int,
            beatposOut: *mut f64,
            bpmOut: *mut f64,
            timesig_numOut: *mut ::std::os::raw::c_int,
            timesig_denomOut: *mut ::std::os::raw::c_int,
            lineartempoOut: *mut bool,
        ) -> bool,
    >,
    pub GetToggleCommandState:
        Option<extern "C" fn(command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,
    pub GetToggleCommandState2: Option<
        unsafe extern "C" fn(
            section: *mut root::KbdSectionInfo,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetToggleCommandStateEx: Option<
        extern "C" fn(
            section_id: ::std::os::raw::c_int,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetToggleCommandStateThroughHooks: Option<
        unsafe extern "C" fn(
            section: *mut root::KbdSectionInfo,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetTooltipWindow: Option<extern "C" fn() -> root::HWND>,
    pub GetTrack: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            trackidx: ::std::os::raw::c_int,
        ) -> *mut root::MediaTrack,
    >,
    pub GetTrackAutomationMode:
        Option<unsafe extern "C" fn(tr: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub GetTrackColor:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub GetTrackDepth:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub GetTrackEnvelope: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            envidx: ::std::os::raw::c_int,
        ) -> *mut root::TrackEnvelope,
    >,
    pub GetTrackEnvelopeByChunkName: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            cfgchunkname: *const ::std::os::raw::c_char,
        ) -> *mut root::TrackEnvelope,
    >,
    pub GetTrackEnvelopeByName: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            envname: *const ::std::os::raw::c_char,
        ) -> *mut root::TrackEnvelope,
    >,
    pub GetTrackFromPoint: Option<
        unsafe extern "C" fn(
            screen_x: ::std::os::raw::c_int,
            screen_y: ::std::os::raw::c_int,
            infoOutOptional: *mut ::std::os::raw::c_int,
        ) -> *mut root::MediaTrack,
    >,
    pub GetTrackGUID: Option<unsafe extern "C" fn(tr: *mut root::MediaTrack) -> *mut root::GUID>,
    pub GetTrackInfo: Option<
        unsafe extern "C" fn(
            track: root::INT_PTR,
            flags: *mut ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub GetTrackMediaItem: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            itemidx: ::std::os::raw::c_int,
        ) -> *mut root::MediaItem,
    >,
    pub GetTrackMIDILyrics: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            flag: ::std::os::raw::c_int,
            bufWantNeedBig: *mut ::std::os::raw::c_char,
            bufWantNeedBig_sz: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetTrackMIDINoteName: Option<
        extern "C" fn(
            track: ::std::os::raw::c_int,
            pitch: ::std::os::raw::c_int,
            chan: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub GetTrackMIDINoteNameEx: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            track: *mut root::MediaTrack,
            pitch: ::std::os::raw::c_int,
            chan: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub GetTrackMIDINoteRange: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            track: *mut root::MediaTrack,
            note_loOut: *mut ::std::os::raw::c_int,
            note_hiOut: *mut ::std::os::raw::c_int,
        ),
    >,
    pub GetTrackName: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            bufOut: *mut ::std::os::raw::c_char,
            bufOut_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetTrackNumMediaItems:
        Option<unsafe extern "C" fn(tr: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub GetTrackNumSends: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            category: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetTrackReceiveName: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            recv_index: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetTrackReceiveUIMute: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            recv_index: ::std::os::raw::c_int,
            muteOut: *mut bool,
        ) -> bool,
    >,
    pub GetTrackReceiveUIVolPan: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            recv_index: ::std::os::raw::c_int,
            volumeOut: *mut f64,
            panOut: *mut f64,
        ) -> bool,
    >,
    pub GetTrackSendInfo_Value: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            category: ::std::os::raw::c_int,
            sendidx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
        ) -> f64,
    >,
    pub GetTrackSendName: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            send_index: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetTrackSendUIMute: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            send_index: ::std::os::raw::c_int,
            muteOut: *mut bool,
        ) -> bool,
    >,
    pub GetTrackSendUIVolPan: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            send_index: ::std::os::raw::c_int,
            volumeOut: *mut f64,
            panOut: *mut f64,
        ) -> bool,
    >,
    pub GetTrackState: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            flagsOut: *mut ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub GetTrackStateChunk: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            strNeedBig: *mut ::std::os::raw::c_char,
            strNeedBig_sz: ::std::os::raw::c_int,
            isundoOptional: bool,
        ) -> bool,
    >,
    pub GetTrackUIMute:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack, muteOut: *mut bool) -> bool>,
    pub GetTrackUIPan: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            pan1Out: *mut f64,
            pan2Out: *mut f64,
            panmodeOut: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetTrackUIVolPan: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            volumeOut: *mut f64,
            panOut: *mut f64,
        ) -> bool,
    >,
    pub GetUnderrunTime: Option<
        unsafe extern "C" fn(
            audio_xrunOutOptional: *mut ::std::os::raw::c_uint,
            media_xrunOutOptional: *mut ::std::os::raw::c_uint,
            curtimeOutOptional: *mut ::std::os::raw::c_uint,
        ),
    >,
    pub GetUserFileNameForRead: Option<
        unsafe extern "C" fn(
            filenameNeed4096: *mut ::std::os::raw::c_char,
            title: *const ::std::os::raw::c_char,
            defext: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub GetUserInputs: Option<
        unsafe extern "C" fn(
            title: *const ::std::os::raw::c_char,
            num_inputs: ::std::os::raw::c_int,
            captions_csv: *const ::std::os::raw::c_char,
            retvals_csv: *mut ::std::os::raw::c_char,
            retvals_csv_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GoToMarker: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            marker_index: ::std::os::raw::c_int,
            use_timeline_order: bool,
        ),
    >,
    pub GoToRegion: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            region_index: ::std::os::raw::c_int,
            use_timeline_order: bool,
        ),
    >,
    pub GR_SelectColor: Option<
        unsafe extern "C" fn(
            hwnd: root::HWND,
            colorOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GSC_mainwnd: Option<extern "C" fn(t: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,
    pub guidToString:
        Option<unsafe extern "C" fn(g: *const root::GUID, destNeed64: *mut ::std::os::raw::c_char)>,
    pub HasExtState: Option<
        unsafe extern "C" fn(
            section: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub HasTrackMIDIPrograms:
        Option<extern "C" fn(track: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char>,
    pub HasTrackMIDIProgramsEx: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            track: *mut root::MediaTrack,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub Help_Set: Option<
        unsafe extern "C" fn(helpstring: *const ::std::os::raw::c_char, is_temporary_help: bool),
    >,
    pub HiresPeaksFromSource: Option<
        unsafe extern "C" fn(
            src: *mut root::PCM_source,
            block: *mut root::PCM_source_peaktransfer_t,
        ),
    >,
    pub image_resolve_fn: Option<
        unsafe extern "C" fn(
            in_: *const ::std::os::raw::c_char,
            out: *mut ::std::os::raw::c_char,
            out_sz: ::std::os::raw::c_int,
        ),
    >,
    pub InsertAutomationItem: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            pool_id: ::std::os::raw::c_int,
            position: f64,
            length: f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub InsertEnvelopePoint: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            time: f64,
            value: f64,
            shape: ::std::os::raw::c_int,
            tension: f64,
            selected: bool,
            noSortInOptional: *mut bool,
        ) -> bool,
    >,
    pub InsertEnvelopePointEx: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
            time: f64,
            value: f64,
            shape: ::std::os::raw::c_int,
            tension: f64,
            selected: bool,
            noSortInOptional: *mut bool,
        ) -> bool,
    >,
    pub InsertMedia: Option<
        unsafe extern "C" fn(
            file: *const ::std::os::raw::c_char,
            mode: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub InsertMediaSection: Option<
        unsafe extern "C" fn(
            file: *const ::std::os::raw::c_char,
            mode: ::std::os::raw::c_int,
            startpct: f64,
            endpct: f64,
            pitchshift: f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub InsertTrackAtIndex: Option<extern "C" fn(idx: ::std::os::raw::c_int, wantDefaults: bool)>,
    pub IsInRealTimeAudio: Option<extern "C" fn() -> ::std::os::raw::c_int>,
    pub IsItemTakeActiveForPlayback: Option<
        unsafe extern "C" fn(item: *mut root::MediaItem, take: *mut root::MediaItem_Take) -> bool,
    >,
    pub IsMediaExtension:
        Option<unsafe extern "C" fn(ext: *const ::std::os::raw::c_char, wantOthers: bool) -> bool>,
    pub IsMediaItemSelected: Option<unsafe extern "C" fn(item: *mut root::MediaItem) -> bool>,
    pub IsProjectDirty:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub IsREAPER: Option<extern "C" fn() -> bool>,
    pub IsTrackSelected: Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> bool>,
    pub IsTrackVisible:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack, mixer: bool) -> bool>,
    pub joystick_create: Option<
        unsafe extern "C" fn(
            guid: *const root::GUID,
        ) -> *mut root::reaper_functions::joystick_device,
    >,
    pub joystick_destroy:
        Option<unsafe extern "C" fn(device: *mut root::reaper_functions::joystick_device)>,
    pub joystick_enum: Option<
        unsafe extern "C" fn(
            index: ::std::os::raw::c_int,
            namestrOutOptional: *mut *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub joystick_getaxis: Option<
        unsafe extern "C" fn(
            dev: *mut root::reaper_functions::joystick_device,
            axis: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub joystick_getbuttonmask: Option<
        unsafe extern "C" fn(
            dev: *mut root::reaper_functions::joystick_device,
        ) -> ::std::os::raw::c_uint,
    >,
    pub joystick_getinfo: Option<
        unsafe extern "C" fn(
            dev: *mut root::reaper_functions::joystick_device,
            axesOutOptional: *mut ::std::os::raw::c_int,
            povsOutOptional: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub joystick_getpov: Option<
        unsafe extern "C" fn(
            dev: *mut root::reaper_functions::joystick_device,
            pov: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub joystick_update:
        Option<unsafe extern "C" fn(dev: *mut root::reaper_functions::joystick_device) -> bool>,
    pub kbd_enumerateActions: Option<
        unsafe extern "C" fn(
            section: *mut root::KbdSectionInfo,
            idx: ::std::os::raw::c_int,
            nameOut: *mut *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub kbd_formatKeyName:
        Option<unsafe extern "C" fn(ac: *mut root::ACCEL, s: *mut ::std::os::raw::c_char)>,
    pub kbd_getCommandName: Option<
        unsafe extern "C" fn(
            cmd: ::std::os::raw::c_int,
            s: *mut ::std::os::raw::c_char,
            section: *mut root::KbdSectionInfo,
        ),
    >,
    pub kbd_getTextFromCmd: Option<
        unsafe extern "C" fn(
            cmd: root::DWORD,
            section: *mut root::KbdSectionInfo,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub KBD_OnMainActionEx: Option<
        unsafe extern "C" fn(
            cmd: ::std::os::raw::c_int,
            val: ::std::os::raw::c_int,
            valhw: ::std::os::raw::c_int,
            relmode: ::std::os::raw::c_int,
            hwnd: root::HWND,
            proj: *mut root::ReaProject,
        ) -> ::std::os::raw::c_int,
    >,
    pub kbd_OnMidiEvent: Option<
        unsafe extern "C" fn(evt: *mut root::MIDI_event_t, dev_index: ::std::os::raw::c_int),
    >,
    pub kbd_OnMidiList: Option<
        unsafe extern "C" fn(list: *mut root::MIDI_eventlist, dev_index: ::std::os::raw::c_int),
    >,
    pub kbd_ProcessActionsMenu:
        Option<unsafe extern "C" fn(menu: root::HMENU, section: *mut root::KbdSectionInfo)>,
    pub kbd_processMidiEventActionEx: Option<
        unsafe extern "C" fn(
            evt: *mut root::MIDI_event_t,
            section: *mut root::KbdSectionInfo,
            hwndCtx: root::HWND,
        ) -> bool,
    >,
    pub kbd_reprocessMenu:
        Option<unsafe extern "C" fn(menu: root::HMENU, section: *mut root::KbdSectionInfo)>,
    pub kbd_RunCommandThroughHooks: Option<
        unsafe extern "C" fn(
            section: *mut root::KbdSectionInfo,
            actionCommandID: *mut ::std::os::raw::c_int,
            val: *mut ::std::os::raw::c_int,
            valhw: *mut ::std::os::raw::c_int,
            relmode: *mut ::std::os::raw::c_int,
            hwnd: root::HWND,
        ) -> bool,
    >,
    pub kbd_translateAccelerator: Option<
        unsafe extern "C" fn(
            hwnd: root::HWND,
            msg: *mut root::MSG,
            section: *mut root::KbdSectionInfo,
        ) -> ::std::os::raw::c_int,
    >,
    pub kbd_translateMouse: Option<
        unsafe extern "C" fn(
            winmsg: *mut ::std::os::raw::c_void,
            midimsg: *mut ::std::os::raw::c_uchar,
        ) -> bool,
    >,
    pub LICE__Destroy: Option<unsafe extern "C" fn(bm: *mut root::reaper_functions::LICE_IBitmap)>,
    pub LICE__DestroyFont:
        Option<unsafe extern "C" fn(font: *mut root::reaper_functions::LICE_IFont)>,
    pub LICE__DrawText: Option<
        unsafe extern "C" fn(
            font: *mut root::reaper_functions::LICE_IFont,
            bm: *mut root::reaper_functions::LICE_IBitmap,
            str: *const ::std::os::raw::c_char,
            strcnt: ::std::os::raw::c_int,
            rect: *mut root::RECT,
            dtFlags: root::UINT,
        ) -> ::std::os::raw::c_int,
    >,
    pub LICE__GetBits: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub LICE__GetDC:
        Option<unsafe extern "C" fn(bm: *mut root::reaper_functions::LICE_IBitmap) -> root::HDC>,
    pub LICE__GetHeight: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
        ) -> ::std::os::raw::c_int,
    >,
    pub LICE__GetRowSpan: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
        ) -> ::std::os::raw::c_int,
    >,
    pub LICE__GetWidth: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
        ) -> ::std::os::raw::c_int,
    >,
    pub LICE__IsFlipped:
        Option<unsafe extern "C" fn(bm: *mut root::reaper_functions::LICE_IBitmap) -> bool>,
    pub LICE__resize: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub LICE__SetBkColor: Option<
        unsafe extern "C" fn(
            font: *mut root::reaper_functions::LICE_IFont,
            color: root::reaper_functions::LICE_pixel,
        ) -> root::reaper_functions::LICE_pixel,
    >,
    pub LICE__SetFromHFont: Option<
        unsafe extern "C" fn(
            font: *mut root::reaper_functions::LICE_IFont,
            hfont: root::HFONT,
            flags: ::std::os::raw::c_int,
        ),
    >,
    pub LICE__SetTextColor: Option<
        unsafe extern "C" fn(
            font: *mut root::reaper_functions::LICE_IFont,
            color: root::reaper_functions::LICE_pixel,
        ) -> root::reaper_functions::LICE_pixel,
    >,
    pub LICE__SetTextCombineMode: Option<
        unsafe extern "C" fn(
            ifont: *mut root::reaper_functions::LICE_IFont,
            mode: ::std::os::raw::c_int,
            alpha: f32,
        ),
    >,
    pub LICE_Arc: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            cx: f32,
            cy: f32,
            r: f32,
            minAngle: f32,
            maxAngle: f32,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            aa: bool,
        ),
    >,
    pub LICE_Blit: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            src: *mut root::reaper_functions::LICE_IBitmap,
            dstx: ::std::os::raw::c_int,
            dsty: ::std::os::raw::c_int,
            srcx: ::std::os::raw::c_int,
            srcy: ::std::os::raw::c_int,
            srcw: ::std::os::raw::c_int,
            srch: ::std::os::raw::c_int,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_Blur: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            src: *mut root::reaper_functions::LICE_IBitmap,
            dstx: ::std::os::raw::c_int,
            dsty: ::std::os::raw::c_int,
            srcx: ::std::os::raw::c_int,
            srcy: ::std::os::raw::c_int,
            srcw: ::std::os::raw::c_int,
            srch: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_BorderedRect: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            bgcolor: root::reaper_functions::LICE_pixel,
            fgcolor: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_Circle: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            cx: f32,
            cy: f32,
            r: f32,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            aa: bool,
        ),
    >,
    pub LICE_Clear: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            color: root::reaper_functions::LICE_pixel,
        ),
    >,
    pub LICE_ClearRect: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            mask: root::reaper_functions::LICE_pixel,
            orbits: root::reaper_functions::LICE_pixel,
        ),
    >,
    pub LICE_ClipLine: Option<
        unsafe extern "C" fn(
            pX1Out: *mut ::std::os::raw::c_int,
            pY1Out: *mut ::std::os::raw::c_int,
            pX2Out: *mut ::std::os::raw::c_int,
            pY2Out: *mut ::std::os::raw::c_int,
            xLo: ::std::os::raw::c_int,
            yLo: ::std::os::raw::c_int,
            xHi: ::std::os::raw::c_int,
            yHi: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub LICE_Copy: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            src: *mut root::reaper_functions::LICE_IBitmap,
        ),
    >,
    pub LICE_CreateBitmap: Option<
        extern "C" fn(
            mode: ::std::os::raw::c_int,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
        ) -> *mut root::reaper_functions::LICE_IBitmap,
    >,
    pub LICE_CreateFont: Option<extern "C" fn() -> *mut root::reaper_functions::LICE_IFont>,
    pub LICE_DrawCBezier: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            xstart: f64,
            ystart: f64,
            xctl1: f64,
            yctl1: f64,
            xctl2: f64,
            yctl2: f64,
            xend: f64,
            yend: f64,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            aa: bool,
            tol: f64,
        ),
    >,
    pub LICE_DrawChar: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            c: ::std::os::raw::c_char,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_DrawGlyph: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alphas: *mut root::reaper_functions::LICE_pixel_chan,
            glyph_w: ::std::os::raw::c_int,
            glyph_h: ::std::os::raw::c_int,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_DrawRect: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_DrawText: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            string: *const ::std::os::raw::c_char,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_FillCBezier: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            xstart: f64,
            ystart: f64,
            xctl1: f64,
            yctl1: f64,
            xctl2: f64,
            yctl2: f64,
            xend: f64,
            yend: f64,
            yfill: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            aa: bool,
            tol: f64,
        ),
    >,
    pub LICE_FillCircle: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            cx: f32,
            cy: f32,
            r: f32,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            aa: bool,
        ),
    >,
    pub LICE_FillConvexPolygon: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x: *mut ::std::os::raw::c_int,
            y: *mut ::std::os::raw::c_int,
            npoints: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_FillRect: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_FillTrapezoid: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x1a: ::std::os::raw::c_int,
            x1b: ::std::os::raw::c_int,
            y1: ::std::os::raw::c_int,
            x2a: ::std::os::raw::c_int,
            x2b: ::std::os::raw::c_int,
            y2: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_FillTriangle: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x1: ::std::os::raw::c_int,
            y1: ::std::os::raw::c_int,
            x2: ::std::os::raw::c_int,
            y2: ::std::os::raw::c_int,
            x3: ::std::os::raw::c_int,
            y3: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_GetPixel: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
        ) -> root::reaper_functions::LICE_pixel,
    >,
    pub LICE_GradRect: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            dstx: ::std::os::raw::c_int,
            dsty: ::std::os::raw::c_int,
            dstw: ::std::os::raw::c_int,
            dsth: ::std::os::raw::c_int,
            ir: f32,
            ig: f32,
            ib: f32,
            ia: f32,
            drdx: f32,
            dgdx: f32,
            dbdx: f32,
            dadx: f32,
            drdy: f32,
            dgdy: f32,
            dbdy: f32,
            dady: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_Line: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            aa: bool,
        ),
    >,
    pub LICE_LineInt: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x1: ::std::os::raw::c_int,
            y1: ::std::os::raw::c_int,
            x2: ::std::os::raw::c_int,
            y2: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            aa: bool,
        ),
    >,
    pub LICE_LoadPNG: Option<
        unsafe extern "C" fn(
            filename: *const ::std::os::raw::c_char,
            bmp: *mut root::reaper_functions::LICE_IBitmap,
        ) -> *mut root::reaper_functions::LICE_IBitmap,
    >,
    pub LICE_LoadPNGFromResource: Option<
        unsafe extern "C" fn(
            hInst: root::HINSTANCE,
            resid: *const ::std::os::raw::c_char,
            bmp: *mut root::reaper_functions::LICE_IBitmap,
        ) -> *mut root::reaper_functions::LICE_IBitmap,
    >,
    pub LICE_MeasureText: Option<
        unsafe extern "C" fn(
            string: *const ::std::os::raw::c_char,
            w: *mut ::std::os::raw::c_int,
            h: *mut ::std::os::raw::c_int,
        ),
    >,
    pub LICE_MultiplyAddRect: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            rsc: f32,
            gsc: f32,
            bsc: f32,
            asc: f32,
            radd: f32,
            gadd: f32,
            badd: f32,
            aadd: f32,
        ),
    >,
    pub LICE_PutPixel: Option<
        unsafe extern "C" fn(
            bm: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            color: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_RotatedBlit: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            src: *mut root::reaper_functions::LICE_IBitmap,
            dstx: ::std::os::raw::c_int,
            dsty: ::std::os::raw::c_int,
            dstw: ::std::os::raw::c_int,
            dsth: ::std::os::raw::c_int,
            srcx: f32,
            srcy: f32,
            srcw: f32,
            srch: f32,
            angle: f32,
            cliptosourcerect: bool,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            rotxcent: f32,
            rotycent: f32,
        ),
    >,
    pub LICE_RoundRect: Option<
        unsafe extern "C" fn(
            drawbm: *mut root::reaper_functions::LICE_IBitmap,
            xpos: f32,
            ypos: f32,
            w: f32,
            h: f32,
            cornerradius: ::std::os::raw::c_int,
            col: root::reaper_functions::LICE_pixel,
            alpha: f32,
            mode: ::std::os::raw::c_int,
            aa: bool,
        ),
    >,
    pub LICE_ScaledBlit: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            src: *mut root::reaper_functions::LICE_IBitmap,
            dstx: ::std::os::raw::c_int,
            dsty: ::std::os::raw::c_int,
            dstw: ::std::os::raw::c_int,
            dsth: ::std::os::raw::c_int,
            srcx: f32,
            srcy: f32,
            srcw: f32,
            srch: f32,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ),
    >,
    pub LICE_SimpleFill: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            newcolor: root::reaper_functions::LICE_pixel,
            comparemask: root::reaper_functions::LICE_pixel,
            keepmask: root::reaper_functions::LICE_pixel,
        ),
    >,
    pub Loop_OnArrow: Option<
        unsafe extern "C" fn(
            project: *mut root::ReaProject,
            direction: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub Main_OnCommand:
        Option<extern "C" fn(command: ::std::os::raw::c_int, flag: ::std::os::raw::c_int)>,
    pub Main_OnCommandEx: Option<
        unsafe extern "C" fn(
            command: ::std::os::raw::c_int,
            flag: ::std::os::raw::c_int,
            proj: *mut root::ReaProject,
        ),
    >,
    pub Main_openProject: Option<unsafe extern "C" fn(name: *const ::std::os::raw::c_char)>,
    pub Main_SaveProject:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, forceSaveAsInOptional: bool)>,
    pub Main_UpdateLoopInfo: Option<extern "C" fn(ignoremask: ::std::os::raw::c_int)>,
    pub MarkProjectDirty: Option<unsafe extern "C" fn(proj: *mut root::ReaProject)>,
    pub MarkTrackItemsDirty:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack, item: *mut root::MediaItem)>,
    pub Master_GetPlayRate: Option<unsafe extern "C" fn(project: *mut root::ReaProject) -> f64>,
    pub Master_GetPlayRateAtTime:
        Option<unsafe extern "C" fn(time_s: f64, proj: *mut root::ReaProject) -> f64>,
    pub Master_GetTempo: Option<extern "C" fn() -> f64>,
    pub Master_NormalizePlayRate: Option<extern "C" fn(playrate: f64, isnormalized: bool) -> f64>,
    pub Master_NormalizeTempo: Option<extern "C" fn(bpm: f64, isnormalized: bool) -> f64>,
    pub MB: Option<
        unsafe extern "C" fn(
            msg: *const ::std::os::raw::c_char,
            title: *const ::std::os::raw::c_char,
            type_: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub MediaItemDescendsFromTrack: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            track: *mut root::MediaTrack,
        ) -> ::std::os::raw::c_int,
    >,
    pub MIDI_CountEvts: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            notecntOut: *mut ::std::os::raw::c_int,
            ccevtcntOut: *mut ::std::os::raw::c_int,
            textsyxevtcntOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub MIDI_DeleteCC: Option<
        unsafe extern "C" fn(take: *mut root::MediaItem_Take, ccidx: ::std::os::raw::c_int) -> bool,
    >,
    pub MIDI_DeleteEvt: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            evtidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_DeleteNote: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            noteidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_DeleteTextSysexEvt: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            textsyxevtidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_DisableSort: Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take)>,
    pub MIDI_EnumSelCC: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            ccidx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub MIDI_EnumSelEvts: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            evtidx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub MIDI_EnumSelNotes: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            noteidx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub MIDI_EnumSelTextSysexEvts: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            textsyxidx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub MIDI_eventlist_Create: Option<extern "C" fn() -> *mut root::MIDI_eventlist>,
    pub MIDI_eventlist_Destroy: Option<unsafe extern "C" fn(evtlist: *mut root::MIDI_eventlist)>,
    pub MIDI_GetAllEvts: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            bufNeedBig: *mut ::std::os::raw::c_char,
            bufNeedBig_sz: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_GetCC: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            ccidx: ::std::os::raw::c_int,
            selectedOut: *mut bool,
            mutedOut: *mut bool,
            ppqposOut: *mut f64,
            chanmsgOut: *mut ::std::os::raw::c_int,
            chanOut: *mut ::std::os::raw::c_int,
            msg2Out: *mut ::std::os::raw::c_int,
            msg3Out: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_GetCCShape: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            ccidx: ::std::os::raw::c_int,
            shapeOut: *mut ::std::os::raw::c_int,
            beztensionOut: *mut f64,
        ) -> bool,
    >,
    pub MIDI_GetEvt: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            evtidx: ::std::os::raw::c_int,
            selectedOut: *mut bool,
            mutedOut: *mut bool,
            ppqposOut: *mut f64,
            msg: *mut ::std::os::raw::c_char,
            msg_sz: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_GetGrid: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            swingOutOptional: *mut f64,
            noteLenOutOptional: *mut f64,
        ) -> f64,
    >,
    pub MIDI_GetHash: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            notesonly: bool,
            hash: *mut ::std::os::raw::c_char,
            hash_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_GetNote: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            noteidx: ::std::os::raw::c_int,
            selectedOut: *mut bool,
            mutedOut: *mut bool,
            startppqposOut: *mut f64,
            endppqposOut: *mut f64,
            chanOut: *mut ::std::os::raw::c_int,
            pitchOut: *mut ::std::os::raw::c_int,
            velOut: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_GetPPQPos_EndOfMeasure:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take, ppqpos: f64) -> f64>,
    pub MIDI_GetPPQPos_StartOfMeasure:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take, ppqpos: f64) -> f64>,
    pub MIDI_GetPPQPosFromProjQN:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take, projqn: f64) -> f64>,
    pub MIDI_GetPPQPosFromProjTime:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take, projtime: f64) -> f64>,
    pub MIDI_GetProjQNFromPPQPos:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take, ppqpos: f64) -> f64>,
    pub MIDI_GetProjTimeFromPPQPos:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take, ppqpos: f64) -> f64>,
    pub MIDI_GetScale: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            rootOut: *mut ::std::os::raw::c_int,
            scaleOut: *mut ::std::os::raw::c_int,
            name: *mut ::std::os::raw::c_char,
            name_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_GetTextSysexEvt: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            textsyxevtidx: ::std::os::raw::c_int,
            selectedOutOptional: *mut bool,
            mutedOutOptional: *mut bool,
            ppqposOutOptional: *mut f64,
            typeOutOptional: *mut ::std::os::raw::c_int,
            msgOptional: *mut ::std::os::raw::c_char,
            msgOptional_sz: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_GetTrackHash: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            notesonly: bool,
            hash: *mut ::std::os::raw::c_char,
            hash_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_InsertCC: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            selected: bool,
            muted: bool,
            ppqpos: f64,
            chanmsg: ::std::os::raw::c_int,
            chan: ::std::os::raw::c_int,
            msg2: ::std::os::raw::c_int,
            msg3: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_InsertEvt: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            selected: bool,
            muted: bool,
            ppqpos: f64,
            bytestr: *const ::std::os::raw::c_char,
            bytestr_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_InsertNote: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            selected: bool,
            muted: bool,
            startppqpos: f64,
            endppqpos: f64,
            chan: ::std::os::raw::c_int,
            pitch: ::std::os::raw::c_int,
            vel: ::std::os::raw::c_int,
            noSortInOptional: *const bool,
        ) -> bool,
    >,
    pub MIDI_InsertTextSysexEvt: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            selected: bool,
            muted: bool,
            ppqpos: f64,
            type_: ::std::os::raw::c_int,
            bytestr: *const ::std::os::raw::c_char,
            bytestr_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub midi_reinit: Option<extern "C" fn()>,
    pub MIDI_SelectAll: Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take, select: bool)>,
    pub MIDI_SetAllEvts: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            buf: *const ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDI_SetCC: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            ccidx: ::std::os::raw::c_int,
            selectedInOptional: *const bool,
            mutedInOptional: *const bool,
            ppqposInOptional: *const f64,
            chanmsgInOptional: *const ::std::os::raw::c_int,
            chanInOptional: *const ::std::os::raw::c_int,
            msg2InOptional: *const ::std::os::raw::c_int,
            msg3InOptional: *const ::std::os::raw::c_int,
            noSortInOptional: *const bool,
        ) -> bool,
    >,
    pub MIDI_SetCCShape: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            ccidx: ::std::os::raw::c_int,
            shape: ::std::os::raw::c_int,
            beztension: f64,
            noSortInOptional: *const bool,
        ) -> bool,
    >,
    pub MIDI_SetEvt: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            evtidx: ::std::os::raw::c_int,
            selectedInOptional: *const bool,
            mutedInOptional: *const bool,
            ppqposInOptional: *const f64,
            msgOptional: *const ::std::os::raw::c_char,
            msgOptional_sz: ::std::os::raw::c_int,
            noSortInOptional: *const bool,
        ) -> bool,
    >,
    pub MIDI_SetItemExtents:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem, startQN: f64, endQN: f64) -> bool>,
    pub MIDI_SetNote: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            noteidx: ::std::os::raw::c_int,
            selectedInOptional: *const bool,
            mutedInOptional: *const bool,
            startppqposInOptional: *const f64,
            endppqposInOptional: *const f64,
            chanInOptional: *const ::std::os::raw::c_int,
            pitchInOptional: *const ::std::os::raw::c_int,
            velInOptional: *const ::std::os::raw::c_int,
            noSortInOptional: *const bool,
        ) -> bool,
    >,
    pub MIDI_SetTextSysexEvt: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            textsyxevtidx: ::std::os::raw::c_int,
            selectedInOptional: *const bool,
            mutedInOptional: *const bool,
            ppqposInOptional: *const f64,
            typeInOptional: *const ::std::os::raw::c_int,
            msgOptional: *const ::std::os::raw::c_char,
            msgOptional_sz: ::std::os::raw::c_int,
            noSortInOptional: *const bool,
        ) -> bool,
    >,
    pub MIDI_Sort: Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take)>,
    pub MIDIEditor_GetActive: Option<extern "C" fn() -> root::HWND>,
    pub MIDIEditor_GetMode: Option<extern "C" fn(midieditor: root::HWND) -> ::std::os::raw::c_int>,
    pub MIDIEditor_GetSetting_int: Option<
        unsafe extern "C" fn(
            midieditor: root::HWND,
            setting_desc: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub MIDIEditor_GetSetting_str: Option<
        unsafe extern "C" fn(
            midieditor: root::HWND,
            setting_desc: *const ::std::os::raw::c_char,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub MIDIEditor_GetTake:
        Option<extern "C" fn(midieditor: root::HWND) -> *mut root::MediaItem_Take>,
    pub MIDIEditor_LastFocused_OnCommand:
        Option<extern "C" fn(command_id: ::std::os::raw::c_int, islistviewcommand: bool) -> bool>,
    pub MIDIEditor_OnCommand:
        Option<extern "C" fn(midieditor: root::HWND, command_id: ::std::os::raw::c_int) -> bool>,
    pub MIDIEditor_SetSetting_int: Option<
        unsafe extern "C" fn(
            midieditor: root::HWND,
            setting_desc: *const ::std::os::raw::c_char,
            setting: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub mkpanstr: Option<unsafe extern "C" fn(strNeed64: *mut ::std::os::raw::c_char, pan: f64)>,
    pub mkvolpanstr:
        Option<unsafe extern "C" fn(strNeed64: *mut ::std::os::raw::c_char, vol: f64, pan: f64)>,
    pub mkvolstr: Option<unsafe extern "C" fn(strNeed64: *mut ::std::os::raw::c_char, vol: f64)>,
    pub MoveEditCursor: Option<extern "C" fn(adjamt: f64, dosel: bool)>,
    pub MoveMediaItemToTrack: Option<
        unsafe extern "C" fn(item: *mut root::MediaItem, desttr: *mut root::MediaTrack) -> bool,
    >,
    pub MuteAllTracks: Option<extern "C" fn(mute: bool)>,
    pub my_getViewport:
        Option<unsafe extern "C" fn(r: *mut root::RECT, sr: *const root::RECT, wantWorkArea: bool)>,
    pub NamedCommandLookup: Option<
        unsafe extern "C" fn(command_name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub OnPauseButton: Option<extern "C" fn()>,
    pub OnPauseButtonEx: Option<unsafe extern "C" fn(proj: *mut root::ReaProject)>,
    pub OnPlayButton: Option<extern "C" fn()>,
    pub OnPlayButtonEx: Option<unsafe extern "C" fn(proj: *mut root::ReaProject)>,
    pub OnStopButton: Option<extern "C" fn()>,
    pub OnStopButtonEx: Option<unsafe extern "C" fn(proj: *mut root::ReaProject)>,
    pub OpenColorThemeFile:
        Option<unsafe extern "C" fn(fn_: *const ::std::os::raw::c_char) -> bool>,
    pub OpenMediaExplorer: Option<
        unsafe extern "C" fn(mediafn: *const ::std::os::raw::c_char, play: bool) -> root::HWND,
    >,
    pub OscLocalMessageToHost: Option<
        unsafe extern "C" fn(message: *const ::std::os::raw::c_char, valueInOptional: *const f64),
    >,
    pub parse_timestr: Option<unsafe extern "C" fn(buf: *const ::std::os::raw::c_char) -> f64>,
    pub parse_timestr_len: Option<
        unsafe extern "C" fn(
            buf: *const ::std::os::raw::c_char,
            offset: f64,
            modeoverride: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub parse_timestr_pos: Option<
        unsafe extern "C" fn(
            buf: *const ::std::os::raw::c_char,
            modeoverride: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub parsepanstr: Option<unsafe extern "C" fn(str: *const ::std::os::raw::c_char) -> f64>,
    pub PCM_Sink_Create: Option<
        unsafe extern "C" fn(
            filename: *const ::std::os::raw::c_char,
            cfg: *const ::std::os::raw::c_char,
            cfg_sz: ::std::os::raw::c_int,
            nch: ::std::os::raw::c_int,
            srate: ::std::os::raw::c_int,
            buildpeaks: bool,
        ) -> *mut root::PCM_sink,
    >,
    pub PCM_Sink_CreateEx: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            filename: *const ::std::os::raw::c_char,
            cfg: *const ::std::os::raw::c_char,
            cfg_sz: ::std::os::raw::c_int,
            nch: ::std::os::raw::c_int,
            srate: ::std::os::raw::c_int,
            buildpeaks: bool,
        ) -> *mut root::PCM_sink,
    >,
    pub PCM_Sink_CreateMIDIFile: Option<
        unsafe extern "C" fn(
            filename: *const ::std::os::raw::c_char,
            cfg: *const ::std::os::raw::c_char,
            cfg_sz: ::std::os::raw::c_int,
            bpm: f64,
            div: ::std::os::raw::c_int,
        ) -> *mut root::PCM_sink,
    >,
    pub PCM_Sink_CreateMIDIFileEx: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            filename: *const ::std::os::raw::c_char,
            cfg: *const ::std::os::raw::c_char,
            cfg_sz: ::std::os::raw::c_int,
            bpm: f64,
            div: ::std::os::raw::c_int,
        ) -> *mut root::PCM_sink,
    >,
    pub PCM_Sink_Enum: Option<
        unsafe extern "C" fn(
            idx: ::std::os::raw::c_int,
            descstrOut: *mut *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_uint,
    >,
    pub PCM_Sink_GetExtension: Option<
        unsafe extern "C" fn(
            data: *const ::std::os::raw::c_char,
            data_sz: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub PCM_Sink_ShowConfig: Option<
        unsafe extern "C" fn(
            cfg: *const ::std::os::raw::c_char,
            cfg_sz: ::std::os::raw::c_int,
            hwndParent: root::HWND,
        ) -> root::HWND,
    >,
    pub PCM_Source_CreateFromFile: Option<
        unsafe extern "C" fn(filename: *const ::std::os::raw::c_char) -> *mut root::PCM_source,
    >,
    pub PCM_Source_CreateFromFileEx: Option<
        unsafe extern "C" fn(
            filename: *const ::std::os::raw::c_char,
            forcenoMidiImp: bool,
        ) -> *mut root::PCM_source,
    >,
    pub PCM_Source_CreateFromSimple: Option<
        unsafe extern "C" fn(
            dec: *mut root::ISimpleMediaDecoder,
            fn_: *const ::std::os::raw::c_char,
        ) -> *mut root::PCM_source,
    >,
    pub PCM_Source_CreateFromType: Option<
        unsafe extern "C" fn(sourcetype: *const ::std::os::raw::c_char) -> *mut root::PCM_source,
    >,
    pub PCM_Source_Destroy: Option<unsafe extern "C" fn(src: *mut root::PCM_source)>,
    pub PCM_Source_GetPeaks: Option<
        unsafe extern "C" fn(
            src: *mut root::PCM_source,
            peakrate: f64,
            starttime: f64,
            numchannels: ::std::os::raw::c_int,
            numsamplesperchannel: ::std::os::raw::c_int,
            want_extra_type: ::std::os::raw::c_int,
            buf: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub PCM_Source_GetSectionInfo: Option<
        unsafe extern "C" fn(
            src: *mut root::PCM_source,
            offsOut: *mut f64,
            lenOut: *mut f64,
            revOut: *mut bool,
        ) -> bool,
    >,
    pub PeakBuild_Create: Option<
        unsafe extern "C" fn(
            src: *mut root::PCM_source,
            fn_: *const ::std::os::raw::c_char,
            srate: ::std::os::raw::c_int,
            nch: ::std::os::raw::c_int,
        ) -> *mut root::REAPER_PeakBuild_Interface,
    >,
    pub PeakBuild_CreateEx: Option<
        unsafe extern "C" fn(
            src: *mut root::PCM_source,
            fn_: *const ::std::os::raw::c_char,
            srate: ::std::os::raw::c_int,
            nch: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> *mut root::REAPER_PeakBuild_Interface,
    >,
    pub PeakGet_Create: Option<
        unsafe extern "C" fn(
            fn_: *const ::std::os::raw::c_char,
            srate: ::std::os::raw::c_int,
            nch: ::std::os::raw::c_int,
        ) -> *mut root::REAPER_PeakGet_Interface,
    >,
    pub PitchShiftSubModeMenu: Option<
        extern "C" fn(
            hwnd: root::HWND,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            mode: ::std::os::raw::c_int,
            submode_sel: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub PlayPreview: Option<
        unsafe extern "C" fn(preview: *mut root::preview_register_t) -> ::std::os::raw::c_int,
    >,
    pub PlayPreviewEx: Option<
        unsafe extern "C" fn(
            preview: *mut root::preview_register_t,
            bufflags: ::std::os::raw::c_int,
            MSI: f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub PlayTrackPreview: Option<
        unsafe extern "C" fn(preview: *mut root::preview_register_t) -> ::std::os::raw::c_int,
    >,
    pub PlayTrackPreview2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            preview: *mut root::preview_register_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub PlayTrackPreview2Ex: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            preview: *mut root::preview_register_t,
            flags: ::std::os::raw::c_int,
            msi: f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub plugin_getapi: Option<
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
    >,
    pub plugin_getFilterList: Option<extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub plugin_getImportableProjectFilterList:
        Option<extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub plugin_register: Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            infostruct: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub PluginWantsAlwaysRunFx: Option<extern "C" fn(amt: ::std::os::raw::c_int)>,
    pub PreventUIRefresh: Option<extern "C" fn(prevent_count: ::std::os::raw::c_int)>,
    pub projectconfig_var_addr: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            idx: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub projectconfig_var_getoffs: Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            szOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub realloc_cmd_ptr: Option<
        unsafe extern "C" fn(
            ptr: *mut *mut ::std::os::raw::c_char,
            ptr_size: *mut ::std::os::raw::c_int,
            new_size: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub ReaperGetPitchShiftAPI:
        Option<extern "C" fn(version: ::std::os::raw::c_int) -> *mut root::IReaperPitchShift>,
    pub ReaScriptError: Option<unsafe extern "C" fn(errmsg: *const ::std::os::raw::c_char)>,
    pub RecursiveCreateDirectory: Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            ignored: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub reduce_open_files:
        Option<extern "C" fn(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,
    pub RefreshToolbar: Option<extern "C" fn(command_id: ::std::os::raw::c_int)>,
    pub RefreshToolbar2:
        Option<extern "C" fn(section_id: ::std::os::raw::c_int, command_id: ::std::os::raw::c_int)>,
    pub relative_fn: Option<
        unsafe extern "C" fn(
            in_: *const ::std::os::raw::c_char,
            out: *mut ::std::os::raw::c_char,
            out_sz: ::std::os::raw::c_int,
        ),
    >,
    pub RemoveTrackSend: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            category: ::std::os::raw::c_int,
            sendidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub RenderFileSection: Option<
        unsafe extern "C" fn(
            source_filename: *const ::std::os::raw::c_char,
            target_filename: *const ::std::os::raw::c_char,
            start_percent: f64,
            end_percent: f64,
            playrate: f64,
        ) -> bool,
    >,
    pub ReorderSelectedTracks: Option<
        extern "C" fn(
            beforeTrackIdx: ::std::os::raw::c_int,
            makePrevFolder: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub Resample_EnumModes:
        Option<extern "C" fn(mode: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char>,
    pub Resampler_Create: Option<extern "C" fn() -> *mut root::REAPER_Resample_Interface>,
    pub resolve_fn: Option<
        unsafe extern "C" fn(
            in_: *const ::std::os::raw::c_char,
            out: *mut ::std::os::raw::c_char,
            out_sz: ::std::os::raw::c_int,
        ),
    >,
    pub resolve_fn2: Option<
        unsafe extern "C" fn(
            in_: *const ::std::os::raw::c_char,
            out: *mut ::std::os::raw::c_char,
            out_sz: ::std::os::raw::c_int,
            checkSubDirOptional: *const ::std::os::raw::c_char,
        ),
    >,
    pub ReverseNamedCommandLookup:
        Option<extern "C" fn(command_id: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char>,
    pub ScaleFromEnvelopeMode:
        Option<extern "C" fn(scaling_mode: ::std::os::raw::c_int, val: f64) -> f64>,
    pub ScaleToEnvelopeMode:
        Option<extern "C" fn(scaling_mode: ::std::os::raw::c_int, val: f64) -> f64>,
    pub screenset_register: Option<
        unsafe extern "C" fn(
            id: *mut ::std::os::raw::c_char,
            callbackFunc: *mut ::std::os::raw::c_void,
            param: *mut ::std::os::raw::c_void,
        ),
    >,
    pub screenset_registerNew: Option<
        unsafe extern "C" fn(
            id: *mut ::std::os::raw::c_char,
            callbackFunc: root::screensetNewCallbackFunc,
            param: *mut ::std::os::raw::c_void,
        ),
    >,
    pub screenset_unregister: Option<unsafe extern "C" fn(id: *mut ::std::os::raw::c_char)>,
    pub screenset_unregisterByParam:
        Option<unsafe extern "C" fn(param: *mut ::std::os::raw::c_void)>,
    pub screenset_updateLastFocus: Option<extern "C" fn(prevWin: root::HWND)>,
    pub SectionFromUniqueID:
        Option<extern "C" fn(uniqueID: ::std::os::raw::c_int) -> *mut root::KbdSectionInfo>,
    pub SelectAllMediaItems:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, selected: bool)>,
    pub SelectProjectInstance: Option<unsafe extern "C" fn(proj: *mut root::ReaProject)>,
    pub SendLocalOscMessage: Option<
        unsafe extern "C" fn(
            local_osc_handler: *mut ::std::os::raw::c_void,
            msg: *const ::std::os::raw::c_char,
            msglen: ::std::os::raw::c_int,
        ),
    >,
    pub SetActiveTake: Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take)>,
    pub SetAutomationMode: Option<extern "C" fn(mode: ::std::os::raw::c_int, onlySel: bool)>,
    pub SetCurrentBPM:
        Option<unsafe extern "C" fn(__proj: *mut root::ReaProject, bpm: f64, wantUndo: bool)>,
    pub SetCursorContext: Option<
        unsafe extern "C" fn(mode: ::std::os::raw::c_int, envInOptional: *mut root::TrackEnvelope),
    >,
    pub SetEditCurPos: Option<extern "C" fn(time: f64, moveview: bool, seekplay: bool)>,
    pub SetEditCurPos2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            time: f64,
            moveview: bool,
            seekplay: bool,
        ),
    >,
    pub SetEnvelopePoint: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            ptidx: ::std::os::raw::c_int,
            timeInOptional: *mut f64,
            valueInOptional: *mut f64,
            shapeInOptional: *mut ::std::os::raw::c_int,
            tensionInOptional: *mut f64,
            selectedInOptional: *mut bool,
            noSortInOptional: *mut bool,
        ) -> bool,
    >,
    pub SetEnvelopePointEx: Option<
        unsafe extern "C" fn(
            envelope: *mut root::TrackEnvelope,
            autoitem_idx: ::std::os::raw::c_int,
            ptidx: ::std::os::raw::c_int,
            timeInOptional: *mut f64,
            valueInOptional: *mut f64,
            shapeInOptional: *mut ::std::os::raw::c_int,
            tensionInOptional: *mut f64,
            selectedInOptional: *mut bool,
            noSortInOptional: *mut bool,
        ) -> bool,
    >,
    pub SetEnvelopeStateChunk: Option<
        unsafe extern "C" fn(
            env: *mut root::TrackEnvelope,
            str: *const ::std::os::raw::c_char,
            isundoOptional: bool,
        ) -> bool,
    >,
    pub SetExtState: Option<
        unsafe extern "C" fn(
            section: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_char,
            value: *const ::std::os::raw::c_char,
            persist: bool,
        ),
    >,
    pub SetGlobalAutomationOverride: Option<extern "C" fn(mode: ::std::os::raw::c_int)>,
    pub SetItemStateChunk: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            str: *const ::std::os::raw::c_char,
            isundoOptional: bool,
        ) -> bool,
    >,
    pub SetMasterTrackVisibility:
        Option<extern "C" fn(flag: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,
    pub SetMediaItemInfo_Value: Option<
        unsafe extern "C" fn(
            item: *mut root::MediaItem,
            parmname: *const ::std::os::raw::c_char,
            newvalue: f64,
        ) -> bool,
    >,
    pub SetMediaItemLength: Option<
        unsafe extern "C" fn(item: *mut root::MediaItem, length: f64, refreshUI: bool) -> bool,
    >,
    pub SetMediaItemPosition: Option<
        unsafe extern "C" fn(item: *mut root::MediaItem, position: f64, refreshUI: bool) -> bool,
    >,
    pub SetMediaItemSelected:
        Option<unsafe extern "C" fn(item: *mut root::MediaItem, selected: bool)>,
    pub SetMediaItemTake_Source: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            source: *mut root::PCM_source,
        ) -> bool,
    >,
    pub SetMediaItemTakeInfo_Value: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            parmname: *const ::std::os::raw::c_char,
            newvalue: f64,
        ) -> bool,
    >,
    pub SetMediaTrackInfo_Value: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            parmname: *const ::std::os::raw::c_char,
            newvalue: f64,
        ) -> bool,
    >,
    pub SetMIDIEditorGrid:
        Option<unsafe extern "C" fn(project: *mut root::ReaProject, division: f64)>,
    pub SetMixerScroll:
        Option<unsafe extern "C" fn(leftmosttrack: *mut root::MediaTrack) -> *mut root::MediaTrack>,
    pub SetMouseModifier: Option<
        unsafe extern "C" fn(
            context: *const ::std::os::raw::c_char,
            modifier_flag: ::std::os::raw::c_int,
            action: *const ::std::os::raw::c_char,
        ),
    >,
    pub SetOnlyTrackSelected: Option<unsafe extern "C" fn(track: *mut root::MediaTrack)>,
    pub SetProjectGrid: Option<unsafe extern "C" fn(project: *mut root::ReaProject, division: f64)>,
    pub SetProjectMarker: Option<
        unsafe extern "C" fn(
            markrgnindexnumber: ::std::os::raw::c_int,
            isrgn: bool,
            pos: f64,
            rgnend: f64,
            name: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub SetProjectMarker2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            markrgnindexnumber: ::std::os::raw::c_int,
            isrgn: bool,
            pos: f64,
            rgnend: f64,
            name: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub SetProjectMarker3: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            markrgnindexnumber: ::std::os::raw::c_int,
            isrgn: bool,
            pos: f64,
            rgnend: f64,
            name: *const ::std::os::raw::c_char,
            color: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub SetProjectMarker4: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            markrgnindexnumber: ::std::os::raw::c_int,
            isrgn: bool,
            pos: f64,
            rgnend: f64,
            name: *const ::std::os::raw::c_char,
            color: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub SetProjectMarkerByIndex: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            markrgnidx: ::std::os::raw::c_int,
            isrgn: bool,
            pos: f64,
            rgnend: f64,
            IDnumber: ::std::os::raw::c_int,
            name: *const ::std::os::raw::c_char,
            color: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub SetProjectMarkerByIndex2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            markrgnidx: ::std::os::raw::c_int,
            isrgn: bool,
            pos: f64,
            rgnend: f64,
            IDnumber: ::std::os::raw::c_int,
            name: *const ::std::os::raw::c_char,
            color: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub SetProjExtState: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            extname: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_char,
            value: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub SetRegionRenderMatrix: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            regionindex: ::std::os::raw::c_int,
            track: *mut root::MediaTrack,
            addorremove: ::std::os::raw::c_int,
        ),
    >,
    pub SetRenderLastError: Option<unsafe extern "C" fn(errorstr: *const ::std::os::raw::c_char)>,
    pub SetTakeStretchMarker: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            idx: ::std::os::raw::c_int,
            pos: f64,
            srcposInOptional: *const f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub SetTakeStretchMarkerSlope: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            idx: ::std::os::raw::c_int,
            slope: f64,
        ) -> bool,
    >,
    pub SetTempoTimeSigMarker: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            ptidx: ::std::os::raw::c_int,
            timepos: f64,
            measurepos: ::std::os::raw::c_int,
            beatpos: f64,
            bpm: f64,
            timesig_num: ::std::os::raw::c_int,
            timesig_denom: ::std::os::raw::c_int,
            lineartempo: bool,
        ) -> bool,
    >,
    pub SetToggleCommandState: Option<
        extern "C" fn(
            section_id: ::std::os::raw::c_int,
            command_id: ::std::os::raw::c_int,
            state: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub SetTrackAutomationMode:
        Option<unsafe extern "C" fn(tr: *mut root::MediaTrack, mode: ::std::os::raw::c_int)>,
    pub SetTrackColor:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack, color: ::std::os::raw::c_int)>,
    pub SetTrackMIDILyrics: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            flag: ::std::os::raw::c_int,
            str: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub SetTrackMIDINoteName: Option<
        unsafe extern "C" fn(
            track: ::std::os::raw::c_int,
            pitch: ::std::os::raw::c_int,
            chan: ::std::os::raw::c_int,
            name: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub SetTrackMIDINoteNameEx: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            track: *mut root::MediaTrack,
            pitch: ::std::os::raw::c_int,
            chan: ::std::os::raw::c_int,
            name: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub SetTrackSelected:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack, selected: bool)>,
    pub SetTrackSendInfo_Value: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            category: ::std::os::raw::c_int,
            sendidx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
            newvalue: f64,
        ) -> bool,
    >,
    pub SetTrackSendUIPan: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            send_idx: ::std::os::raw::c_int,
            pan: f64,
            isend: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub SetTrackSendUIVol: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            send_idx: ::std::os::raw::c_int,
            vol: f64,
            isend: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub SetTrackStateChunk: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            str: *const ::std::os::raw::c_char,
            isundoOptional: bool,
        ) -> bool,
    >,
    pub ShowActionList:
        Option<unsafe extern "C" fn(caller: *mut root::KbdSectionInfo, callerWnd: root::HWND)>,
    pub ShowConsoleMsg: Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>,
    pub ShowMessageBox: Option<
        unsafe extern "C" fn(
            msg: *const ::std::os::raw::c_char,
            title: *const ::std::os::raw::c_char,
            type_: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub ShowPopupMenu: Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            hwndParentOptional: root::HWND,
            ctxOptional: *mut ::std::os::raw::c_void,
            ctx2Optional: ::std::os::raw::c_int,
            ctx3Optional: ::std::os::raw::c_int,
        ),
    >,
    pub SLIDER2DB: Option<extern "C" fn(y: f64) -> f64>,
    pub SnapToGrid:
        Option<unsafe extern "C" fn(project: *mut root::ReaProject, time_pos: f64) -> f64>,
    pub SoloAllTracks: Option<extern "C" fn(solo: ::std::os::raw::c_int)>,
    pub Splash_GetWnd: Option<extern "C" fn() -> root::HWND>,
    pub SplitMediaItem: Option<
        unsafe extern "C" fn(item: *mut root::MediaItem, position: f64) -> *mut root::MediaItem,
    >,
    pub StopPreview: Option<
        unsafe extern "C" fn(preview: *mut root::preview_register_t) -> ::std::os::raw::c_int,
    >,
    pub StopTrackPreview: Option<
        unsafe extern "C" fn(preview: *mut root::preview_register_t) -> ::std::os::raw::c_int,
    >,
    pub StopTrackPreview2: Option<
        unsafe extern "C" fn(
            proj: *mut ::std::os::raw::c_void,
            preview: *mut root::preview_register_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub stringToGuid:
        Option<unsafe extern "C" fn(str: *const ::std::os::raw::c_char, g: *mut root::GUID)>,
    pub StuffMIDIMessage: Option<
        extern "C" fn(
            mode: ::std::os::raw::c_int,
            msg1: ::std::os::raw::c_int,
            msg2: ::std::os::raw::c_int,
            msg3: ::std::os::raw::c_int,
        ),
    >,
    pub TakeFX_AddByName: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fxname: *const ::std::os::raw::c_char,
            instantiate: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TakeFX_CopyToTake: Option<
        unsafe extern "C" fn(
            src_take: *mut root::MediaItem_Take,
            src_fx: ::std::os::raw::c_int,
            dest_take: *mut root::MediaItem_Take,
            dest_fx: ::std::os::raw::c_int,
            is_move: bool,
        ),
    >,
    pub TakeFX_CopyToTrack: Option<
        unsafe extern "C" fn(
            src_take: *mut root::MediaItem_Take,
            src_fx: ::std::os::raw::c_int,
            dest_track: *mut root::MediaTrack,
            dest_fx: ::std::os::raw::c_int,
            is_move: bool,
        ),
    >,
    pub TakeFX_Delete: Option<
        unsafe extern "C" fn(take: *mut root::MediaItem_Take, fx: ::std::os::raw::c_int) -> bool,
    >,
    pub TakeFX_EndParamEdit: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_FormatParamValue: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            val: f64,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_FormatParamValueNormalized: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            value: f64,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_GetChainVisible:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> ::std::os::raw::c_int>,
    pub TakeFX_GetCount:
        Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> ::std::os::raw::c_int>,
    pub TakeFX_GetEnabled: Option<
        unsafe extern "C" fn(take: *mut root::MediaItem_Take, fx: ::std::os::raw::c_int) -> bool,
    >,
    pub TakeFX_GetEnvelope: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fxindex: ::std::os::raw::c_int,
            parameterindex: ::std::os::raw::c_int,
            create: bool,
        ) -> *mut root::TrackEnvelope,
    >,
    pub TakeFX_GetFloatingWindow: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            index: ::std::os::raw::c_int,
        ) -> root::HWND,
    >,
    pub TakeFX_GetFormattedParamValue: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_GetFXGUID: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
        ) -> *mut root::GUID,
    >,
    pub TakeFX_GetFXName: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_GetIOSize: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            inputPinsOutOptional: *mut ::std::os::raw::c_int,
            outputPinsOutOptional: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TakeFX_GetNamedConfigParm: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
            bufOut: *mut ::std::os::raw::c_char,
            bufOut_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_GetNumParams: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TakeFX_GetOffline: Option<
        unsafe extern "C" fn(take: *mut root::MediaItem_Take, fx: ::std::os::raw::c_int) -> bool,
    >,
    pub TakeFX_GetOpen: Option<
        unsafe extern "C" fn(take: *mut root::MediaItem_Take, fx: ::std::os::raw::c_int) -> bool,
    >,
    pub TakeFX_GetParam: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            minvalOut: *mut f64,
            maxvalOut: *mut f64,
        ) -> f64,
    >,
    pub TakeFX_GetParameterStepSizes: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            stepOut: *mut f64,
            smallstepOut: *mut f64,
            largestepOut: *mut f64,
            istoggleOut: *mut bool,
        ) -> bool,
    >,
    pub TakeFX_GetParamEx: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            minvalOut: *mut f64,
            maxvalOut: *mut f64,
            midvalOut: *mut f64,
        ) -> f64,
    >,
    pub TakeFX_GetParamName: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_GetParamNormalized: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub TakeFX_GetPinMappings: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            isoutput: ::std::os::raw::c_int,
            pin: ::std::os::raw::c_int,
            high32OutOptional: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TakeFX_GetPreset: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            presetname: *mut ::std::os::raw::c_char,
            presetname_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_GetPresetIndex: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            numberOfPresetsOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TakeFX_GetUserPresetFilename: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            fn_: *mut ::std::os::raw::c_char,
            fn_sz: ::std::os::raw::c_int,
        ),
    >,
    pub TakeFX_NavigatePresets: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            presetmove: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_SetEnabled: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            enabled: bool,
        ),
    >,
    pub TakeFX_SetNamedConfigParm: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
            value: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub TakeFX_SetOffline: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            offline: bool,
        ),
    >,
    pub TakeFX_SetOpen: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            open: bool,
        ),
    >,
    pub TakeFX_SetParam: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            val: f64,
        ) -> bool,
    >,
    pub TakeFX_SetParamNormalized: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            value: f64,
        ) -> bool,
    >,
    pub TakeFX_SetPinMappings: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            isoutput: ::std::os::raw::c_int,
            pin: ::std::os::raw::c_int,
            low32bits: ::std::os::raw::c_int,
            hi32bits: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_SetPreset: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            presetname: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub TakeFX_SetPresetByIndex: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            fx: ::std::os::raw::c_int,
            idx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TakeFX_Show: Option<
        unsafe extern "C" fn(
            take: *mut root::MediaItem_Take,
            index: ::std::os::raw::c_int,
            showFlag: ::std::os::raw::c_int,
        ),
    >,
    pub TakeIsMIDI: Option<unsafe extern "C" fn(take: *mut root::MediaItem_Take) -> bool>,
    pub ThemeLayout_GetLayout: Option<
        unsafe extern "C" fn(
            section: *const ::std::os::raw::c_char,
            idx: ::std::os::raw::c_int,
            nameOut: *mut ::std::os::raw::c_char,
            nameOut_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub ThemeLayout_GetParameter: Option<
        unsafe extern "C" fn(
            wp: ::std::os::raw::c_int,
            descOutOptional: *mut *const ::std::os::raw::c_char,
            valueOutOptional: *mut ::std::os::raw::c_int,
            defValueOutOptional: *mut ::std::os::raw::c_int,
            minValueOutOptional: *mut ::std::os::raw::c_int,
            maxValueOutOptional: *mut ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub ThemeLayout_RefreshAll: Option<extern "C" fn()>,
    pub ThemeLayout_SetLayout: Option<
        unsafe extern "C" fn(
            section: *const ::std::os::raw::c_char,
            layout: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub ThemeLayout_SetParameter: Option<
        extern "C" fn(
            wp: ::std::os::raw::c_int,
            value: ::std::os::raw::c_int,
            persist: bool,
        ) -> bool,
    >,
    pub time_precise: Option<extern "C" fn() -> f64>,
    pub TimeMap2_beatsToTime: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            tpos: f64,
            measuresInOptional: *const ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub TimeMap2_GetDividedBpmAtTime:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, time: f64) -> f64>,
    pub TimeMap2_GetNextChangeTime:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, time: f64) -> f64>,
    pub TimeMap2_QNToTime:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, qn: f64) -> f64>,
    pub TimeMap2_timeToBeats: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            tpos: f64,
            measuresOutOptional: *mut ::std::os::raw::c_int,
            cmlOutOptional: *mut ::std::os::raw::c_int,
            fullbeatsOutOptional: *mut f64,
            cdenomOutOptional: *mut ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub TimeMap2_timeToQN:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, tpos: f64) -> f64>,
    pub TimeMap_curFrameRate: Option<
        unsafe extern "C" fn(proj: *mut root::ReaProject, dropFrameOutOptional: *mut bool) -> f64,
    >,
    pub TimeMap_GetDividedBpmAtTime: Option<extern "C" fn(time: f64) -> f64>,
    pub TimeMap_GetMeasureInfo: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            measure: ::std::os::raw::c_int,
            qn_startOut: *mut f64,
            qn_endOut: *mut f64,
            timesig_numOut: *mut ::std::os::raw::c_int,
            timesig_denomOut: *mut ::std::os::raw::c_int,
            tempoOut: *mut f64,
        ) -> f64,
    >,
    pub TimeMap_GetMetronomePattern: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            time: f64,
            pattern: *mut ::std::os::raw::c_char,
            pattern_sz: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TimeMap_GetTimeSigAtTime: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            time: f64,
            timesig_numOut: *mut ::std::os::raw::c_int,
            timesig_denomOut: *mut ::std::os::raw::c_int,
            tempoOut: *mut f64,
        ),
    >,
    pub TimeMap_QNToMeasures: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            qn: f64,
            qnMeasureStartOutOptional: *mut f64,
            qnMeasureEndOutOptional: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub TimeMap_QNToTime: Option<extern "C" fn(qn: f64) -> f64>,
    pub TimeMap_QNToTime_abs:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, qn: f64) -> f64>,
    pub TimeMap_timeToQN: Option<extern "C" fn(tpos: f64) -> f64>,
    pub TimeMap_timeToQN_abs:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject, tpos: f64) -> f64>,
    pub ToggleTrackSendUIMute: Option<
        unsafe extern "C" fn(track: *mut root::MediaTrack, send_idx: ::std::os::raw::c_int) -> bool,
    >,
    pub Track_GetPeakHoldDB: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            channel: ::std::os::raw::c_int,
            clear: bool,
        ) -> f64,
    >,
    pub Track_GetPeakInfo: Option<
        unsafe extern "C" fn(track: *mut root::MediaTrack, channel: ::std::os::raw::c_int) -> f64,
    >,
    pub TrackCtl_SetToolTip: Option<
        unsafe extern "C" fn(
            fmt: *const ::std::os::raw::c_char,
            xpos: ::std::os::raw::c_int,
            ypos: ::std::os::raw::c_int,
            topmost: bool,
        ),
    >,
    pub TrackFX_AddByName: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fxname: *const ::std::os::raw::c_char,
            recFX: bool,
            instantiate: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_CopyToTake: Option<
        unsafe extern "C" fn(
            src_track: *mut root::MediaTrack,
            src_fx: ::std::os::raw::c_int,
            dest_take: *mut root::MediaItem_Take,
            dest_fx: ::std::os::raw::c_int,
            is_move: bool,
        ),
    >,
    pub TrackFX_CopyToTrack: Option<
        unsafe extern "C" fn(
            src_track: *mut root::MediaTrack,
            src_fx: ::std::os::raw::c_int,
            dest_track: *mut root::MediaTrack,
            dest_fx: ::std::os::raw::c_int,
            is_move: bool,
        ),
    >,
    pub TrackFX_Delete: Option<
        unsafe extern "C" fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> bool,
    >,
    pub TrackFX_EndParamEdit: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_FormatParamValue: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            val: f64,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_FormatParamValueNormalized: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            value: f64,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetByName: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fxname: *const ::std::os::raw::c_char,
            instantiate: bool,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_GetChainVisible:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub TrackFX_GetCount:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub TrackFX_GetEnabled: Option<
        unsafe extern "C" fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> bool,
    >,
    pub TrackFX_GetEQ: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            instantiate: bool,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_GetEQBandEnabled: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fxidx: ::std::os::raw::c_int,
            bandtype: ::std::os::raw::c_int,
            bandidx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetEQParam: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fxidx: ::std::os::raw::c_int,
            paramidx: ::std::os::raw::c_int,
            bandtypeOut: *mut ::std::os::raw::c_int,
            bandidxOut: *mut ::std::os::raw::c_int,
            paramtypeOut: *mut ::std::os::raw::c_int,
            normvalOut: *mut f64,
        ) -> bool,
    >,
    pub TrackFX_GetFloatingWindow: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            index: ::std::os::raw::c_int,
        ) -> root::HWND,
    >,
    pub TrackFX_GetFormattedParamValue: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetFXGUID: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
        ) -> *mut root::GUID,
    >,
    pub TrackFX_GetFXName: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetInstrument:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub TrackFX_GetIOSize: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            inputPinsOutOptional: *mut ::std::os::raw::c_int,
            outputPinsOutOptional: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_GetNamedConfigParm: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
            bufOut: *mut ::std::os::raw::c_char,
            bufOut_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetNumParams: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_GetOffline: Option<
        unsafe extern "C" fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> bool,
    >,
    pub TrackFX_GetOpen: Option<
        unsafe extern "C" fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> bool,
    >,
    pub TrackFX_GetParam: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            minvalOut: *mut f64,
            maxvalOut: *mut f64,
        ) -> f64,
    >,
    pub TrackFX_GetParameterStepSizes: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            stepOut: *mut f64,
            smallstepOut: *mut f64,
            largestepOut: *mut f64,
            istoggleOut: *mut bool,
        ) -> bool,
    >,
    pub TrackFX_GetParamEx: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            minvalOut: *mut f64,
            maxvalOut: *mut f64,
            midvalOut: *mut f64,
        ) -> f64,
    >,
    pub TrackFX_GetParamName: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetParamNormalized: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub TrackFX_GetPinMappings: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            isoutput: ::std::os::raw::c_int,
            pin: ::std::os::raw::c_int,
            high32OutOptional: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_GetPreset: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            presetname: *mut ::std::os::raw::c_char,
            presetname_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetPresetIndex: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            numberOfPresetsOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_GetRecChainVisible:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub TrackFX_GetRecCount:
        Option<unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub TrackFX_GetUserPresetFilename: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            fn_: *mut ::std::os::raw::c_char,
            fn_sz: ::std::os::raw::c_int,
        ),
    >,
    pub TrackFX_NavigatePresets: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            presetmove: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_SetEnabled: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            enabled: bool,
        ),
    >,
    pub TrackFX_SetEQBandEnabled: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fxidx: ::std::os::raw::c_int,
            bandtype: ::std::os::raw::c_int,
            bandidx: ::std::os::raw::c_int,
            enable: bool,
        ) -> bool,
    >,
    pub TrackFX_SetEQParam: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fxidx: ::std::os::raw::c_int,
            bandtype: ::std::os::raw::c_int,
            bandidx: ::std::os::raw::c_int,
            paramtype: ::std::os::raw::c_int,
            val: f64,
            isnorm: bool,
        ) -> bool,
    >,
    pub TrackFX_SetNamedConfigParm: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
            value: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub TrackFX_SetOffline: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            offline: bool,
        ),
    >,
    pub TrackFX_SetOpen: Option<
        unsafe extern "C" fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int, open: bool),
    >,
    pub TrackFX_SetParam: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            val: f64,
        ) -> bool,
    >,
    pub TrackFX_SetParamNormalized: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            value: f64,
        ) -> bool,
    >,
    pub TrackFX_SetPinMappings: Option<
        unsafe extern "C" fn(
            tr: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            isoutput: ::std::os::raw::c_int,
            pin: ::std::os::raw::c_int,
            low32bits: ::std::os::raw::c_int,
            hi32bits: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_SetPreset: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            presetname: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub TrackFX_SetPresetByIndex: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            idx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_Show: Option<
        unsafe extern "C" fn(
            track: *mut root::MediaTrack,
            index: ::std::os::raw::c_int,
            showFlag: ::std::os::raw::c_int,
        ),
    >,
    pub TrackList_AdjustWindows: Option<extern "C" fn(isMinor: bool)>,
    pub TrackList_UpdateAllExternalSurfaces: Option<extern "C" fn()>,
    pub Undo_BeginBlock: Option<extern "C" fn()>,
    pub Undo_BeginBlock2: Option<unsafe extern "C" fn(proj: *mut root::ReaProject)>,
    pub Undo_CanRedo2:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> *const ::std::os::raw::c_char>,
    pub Undo_CanUndo2:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> *const ::std::os::raw::c_char>,
    pub Undo_DoRedo2:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub Undo_DoUndo2:
        Option<unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub Undo_EndBlock: Option<
        unsafe extern "C" fn(
            descchange: *const ::std::os::raw::c_char,
            extraflags: ::std::os::raw::c_int,
        ),
    >,
    pub Undo_EndBlock2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            descchange: *const ::std::os::raw::c_char,
            extraflags: ::std::os::raw::c_int,
        ),
    >,
    pub Undo_OnStateChange: Option<unsafe extern "C" fn(descchange: *const ::std::os::raw::c_char)>,
    pub Undo_OnStateChange2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            descchange: *const ::std::os::raw::c_char,
        ),
    >,
    pub Undo_OnStateChange_Item: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            name: *const ::std::os::raw::c_char,
            item: *mut root::MediaItem,
        ),
    >,
    pub Undo_OnStateChangeEx: Option<
        unsafe extern "C" fn(
            descchange: *const ::std::os::raw::c_char,
            whichStates: ::std::os::raw::c_int,
            trackparm: ::std::os::raw::c_int,
        ),
    >,
    pub Undo_OnStateChangeEx2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            descchange: *const ::std::os::raw::c_char,
            whichStates: ::std::os::raw::c_int,
            trackparm: ::std::os::raw::c_int,
        ),
    >,
    pub update_disk_counters:
        Option<extern "C" fn(readamt: ::std::os::raw::c_int, writeamt: ::std::os::raw::c_int)>,
    pub UpdateArrange: Option<extern "C" fn()>,
    pub UpdateItemInProject: Option<unsafe extern "C" fn(item: *mut root::MediaItem)>,
    pub UpdateTimeline: Option<extern "C" fn()>,
    pub ValidatePtr: Option<
        unsafe extern "C" fn(
            pointer: *mut ::std::os::raw::c_void,
            ctypename: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub ValidatePtr2: Option<
        unsafe extern "C" fn(
            proj: *mut root::ReaProject,
            pointer: *mut ::std::os::raw::c_void,
            ctypename: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub ViewPrefs: Option<
        unsafe extern "C" fn(
            page: ::std::os::raw::c_int,
            pageByName: *const ::std::os::raw::c_char,
        ),
    >,
    pub WDL_VirtualWnd_ScaledBlitBG: Option<
        unsafe extern "C" fn(
            dest: *mut root::reaper_functions::LICE_IBitmap,
            src: *mut root::reaper_functions::WDL_VirtualWnd_BGCfg,
            destx: ::std::os::raw::c_int,
            desty: ::std::os::raw::c_int,
            destw: ::std::os::raw::c_int,
            desth: ::std::os::raw::c_int,
            clipx: ::std::os::raw::c_int,
            clipy: ::std::os::raw::c_int,
            clipw: ::std::os::raw::c_int,
            cliph: ::std::os::raw::c_int,
            alpha: f32,
            mode: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetMidiInput: Option<extern "C" fn(idx: ::std::os::raw::c_int) -> *mut root::midi_Input>,
    pub GetMidiOutput: Option<extern "C" fn(idx: ::std::os::raw::c_int) -> *mut root::midi_Output>,
}