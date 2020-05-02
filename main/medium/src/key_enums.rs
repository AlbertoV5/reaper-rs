use crate::{concat_c_strs, ReaperStringArg};
use c_str_macro::c_str;
use reaper_rs_low::raw;
use reaper_rs_low::raw::{MediaTrack, GUID};
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::ptr::{null_mut, NonNull};

/// Track info key which you can pass to [`get_set_media_track_info()`].
///
/// Please raise a *reaper-rs* issue if you find that an enum variant is missing!
///
/// [`get_set_media_track_info()`]: struct.ReaperFunctions.html#method.get_set_media_track_info
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TrackInfoKey<'a> {
    /// Parent track (read-only).
    ParTrack,
    /// Parent project (read-only).
    Project,
    /// Track name (on master returns `null`).
    Name,
    /// Track icon.
    ///
    /// Full file name or relative to resource path / data / track icons.
    Icon,
    /// Layout name.
    McpLayout,
    /// Layout name.
    TcpLayout,
    /// Extension-specific persistent data.
    Ext(Cow<'a, CStr>),
    /// 6-byte GUID, can query or update.
    ///
    /// If using a `_string()` function, GUID is a string `{xyz-...}`.
    Guid,
    /// Muted.
    Mute,
    /// Track phase inverted.
    Phase,
    /// Track number
    ///
    /// 1-based, read-only, returns the i32 directly.
    ///
    /// - 0 → not found
    /// - -1 → master track
    TrackNumber,
    /// Soloed.
    ///
    /// - 0 → not soloed
    /// - 1 → soloed
    /// - 2 → soloed in place
    /// - 5 → safe soloed
    /// - 6 → safe soloed in place
    Solo,
    /// FX enabled.
    ///
    /// - 0 → bypassed
    /// - != 0 → FX active
    FxEn,
    /// Record armed.
    ///
    /// - 0 → not record armed
    /// - 1 → record armed
    RecArm,
    /// Record input.
    ///
    /// - <0 → no input
    /// - 0..=n → mono hardware input
    /// - 512 + n → rearoute input
    /// - &1024 → stereo input pair
    /// - &4096 → MIDI input, if set then low 5 bits represent channel (0 → all, 1 - 16 → only
    ///   channel), next 6 bits represent physical input (63 → all, 62 → VKB)
    RecInput,
    /// Record mode.
    ///
    /// - 0 → input
    /// - 1 → stereo out
    /// - 2 → none
    /// - 3 → stereo out with latency compensation
    /// - 4 → midi output
    /// - 5 → mono out
    /// - 6 → mono out with latency compensation
    /// - 7 → MIDI overdub
    /// - 8 → MIDI replace
    RecMode,
    /// Record monitoring.
    ///
    /// - 0 → off
    /// - 1 → normal
    /// - 2 → not when playing (tape style)
    RecMon,
    /// Monitor items while recording.
    ///
    /// - 0 → off
    /// - 1 → on
    RecMonItems,
    /// Track automation mode.
    ///
    /// - 0 → trim/off
    /// - 1 → read
    /// - 2 → touch
    /// - 3 → write
    /// - 4 → latch
    AutoMode,
    /// Number of track channels.
    ///
    /// 2 - 64, even numbers only.
    Nchan,
    /// Track selected.
    ///
    /// - 0 → unselected
    /// - 1 → selected
    Selected,
    /// Current TCP window height in pixels including envelopes (read-only).
    WndH,
    /// Current TCP window height in pixels not including envelopes (read-only).
    TcpH,
    /// Current TCP window Y-position in pixels relative to top of arrange view (read-only).
    TcpY,
    /// Current MCP X-position in pixels relative to mixer container.
    McpX,
    /// Current MCP Y-position in pixels relative to mixer container.
    McpY,
    /// Current MCP width in pixels.
    McpW,
    /// Current MCP height in pixels.
    McpH,
    /// Folder depth change.
    ///
    /// - 0 → normal
    /// - 1 → track is a folder parent
    /// - -1 → track is the last in the innermost folder
    /// - -2 → track is the last in the innermost and next-innermost folders
    /// - ...
    FolderDepth,
    /// Folder compacted state (only valid on folders).
    ///
    /// - 0 → normal
    /// - 1 → small
    /// - 2 → tiny children
    FolderCompact,
    /// Track midi hardware output index.
    ///
    /// Low 5 bits are which channels (1..=16, 0 → all), next 5 bits are output device index
    /// (0..=31). < 0 means disabled.
    MidiHwOut,
    /// Track performance flags.
    ///
    /// &1 → no media buffering
    /// &2 → no anticipative FX
    PerfFlags,
    /// Custom color.
    ///
    /// `<OS dependent color> | 0x100000` (i.e. `ColorToNative(r, g, b) | 0x100000`).
    /// If you don't do `| 0x100000`, then it will not be used, but will store the color anyway.
    CustomColor,
    /// Custom height override for TCP window.
    ///
    /// 0 for none, otherwise size in pixels.
    HeightOverride,
    /// Track height lock.
    ///
    /// Must set [`HeightOverride`] before locking.
    ///
    /// [`HeightOverride`]: #variant.HeightOverride
    HeightLock,
    /// Trim volume of track.
    ///
    /// - 0 → -inf
    /// - 0.5 → -6dB
    /// - 1 → +0dB
    /// - 2 → +6dB
    /// - ...
    Vol,
    /// Trim pan of track
    ///
    /// -1..=1.
    Pan,
    /// Width of track
    ///
    /// -1..=1.
    Width,
    /// Dual pan position 1.
    ///
    /// -1..=1, only if [`PanMode`] == 6.
    ///
    /// [`PanMode`]: #variant.PanMode
    DualPanL,
    /// Dual pan position 2.
    ///
    /// -1..=1, only if [`PanMode`] == 6.
    ///
    /// [`PanMode`]: #variant.PanMode
    DualPanR,
    /// Pan mode.
    ///
    /// - 0 → classic 3.x
    /// - 3 → new balance
    /// - 5 → stereo pan
    /// - 6 → dual pan
    PanMode,
    /// Pan law.
    ///
    /// - < 0 → project default
    /// - 1 → +0 dB
    /// - ...
    PanLaw,
    /// TrackEnvelope (read only).
    Env(EnvChunkName<'a>),
    /// Track control panel visible in mixer.
    ///
    /// Do not use on master track.
    ShowInMixer,
    /// Track control panel visible in arrange view.
    ///
    /// Do not use on master track.
    ShowInTcp,
    /// Track sends audio to parent.
    MainSend,
    /// Channel offset of track send to parent.
    MainSendOffs,
    /// Track free item positioning enabled
    ///
    /// Call [`update_timeline`] after changing.
    ///
    /// [`update_timeline`]: struct.Reaper.html#method.update_timeline
    FreeMode,
    /// Track timebase.
    ///
    /// - -1 → project default
    /// - 0 → time
    /// - 1 → beats (position, length, rate)
    /// - 2 → beats (position only)
    BeatAttachMode,
    /// Scale of FX and send area in MCP.
    ///
    /// - 0 → minimum allowed
    /// - 1 → maximum allowed
    McpFxSendScale,
    /// Scale of send area as proportion of the FX and send total area.
    ///
    /// - 0 → minimum allowed
    /// - 1 → maximum allowed
    McpSendRgnScale,
    /// Track playback offset state.
    ///
    /// - &1 → bypassed
    /// - &2 → offset
    ///
    /// Value is measured in samples (otherwise measured in seconds).
    PlayOffsetFlag,
    /// Track playback offset.
    ///
    /// Units depend on [`PlayOffsetFlag`].
    ///
    /// [`PlayOffsetFlag`]: #variant.PlayOffsetFlag
    PlayOffset,
    /// If a variant is missing in this enum, you can use this custom one as a resort.
    Custom(Cow<'a, CStr>),
}

impl<'a> TrackInfoKey<'a> {
    pub fn ext(key: impl Into<ReaperStringArg<'a>>) -> TrackInfoKey<'a> {
        TrackInfoKey::Ext(key.into().into_inner())
    }

    pub fn custom(key: impl Into<ReaperStringArg<'a>>) -> TrackInfoKey<'a> {
        TrackInfoKey::Custom(key.into().into_inner())
    }
}

impl<'a> From<TrackInfoKey<'a>> for Cow<'a, CStr> {
    fn from(value: TrackInfoKey<'a>) -> Self {
        use TrackInfoKey::*;
        match value {
            FreeMode => c_str!("B_FREEMODE").into(),
            HeightLock => c_str!("B_HEIGHTLOCK").into(),
            MainSend => c_str!("B_MAINSEND").into(),
            Mute => c_str!("B_MUTE").into(),
            Phase => c_str!("B_PHASE").into(),
            ShowInMixer => c_str!("B_SHOWINMIXER").into(),
            ShowInTcp => c_str!("B_SHOWINTCP").into(),
            BeatAttachMode => c_str!("C_BEATATTACHMODE").into(),
            MainSendOffs => c_str!("C_MAINSEND_OFFS").into(),
            DualPanL => c_str!("D_DUALPANL").into(),
            DualPanR => c_str!("D_DUALPANR").into(),
            Pan => c_str!("D_PAN").into(),
            PanLaw => c_str!("D_PANLAW").into(),
            PlayOffset => c_str!("D_PLAY_OFFSET").into(),
            Vol => c_str!("D_VOL").into(),
            Width => c_str!("D_WIDTH").into(),
            McpFxSendScale => c_str!("F_MCP_FXSEND_SCALE").into(),
            McpSendRgnScale => c_str!("F_MCP_SENDRGN_SCALE").into(),
            Guid => c_str!("GUID").into(),
            AutoMode => c_str!("I_AUTOMODE").into(),
            CustomColor => c_str!("I_CUSTOMCOLOR").into(),
            FolderCompact => c_str!("I_FOLDERCOMPACT").into(),
            FolderDepth => c_str!("I_FOLDERDEPTH").into(),
            FxEn => c_str!("I_FXEN").into(),
            HeightOverride => c_str!("I_HEIGHTOVERRIDE").into(),
            McpH => c_str!("I_MCPH").into(),
            McpW => c_str!("I_MCPW").into(),
            McpX => c_str!("I_MCPX").into(),
            McpY => c_str!("I_MCPY").into(),
            MidiHwOut => c_str!("I_MIDIHWOUT").into(),
            Nchan => c_str!("I_NCHAN").into(),
            PanMode => c_str!("I_PANMODE").into(),
            PerfFlags => c_str!("I_PERFFLAGS").into(),
            PlayOffsetFlag => c_str!("I_PLAY_OFFSET_FLAG").into(),
            RecArm => c_str!("I_RECARM").into(),
            RecInput => c_str!("I_RECINPUT").into(),
            RecMode => c_str!("I_RECMODE").into(),
            RecMon => c_str!("I_RECMON").into(),
            RecMonItems => c_str!("I_RECMONITEMS").into(),
            Selected => c_str!("I_SELECTED").into(),
            Solo => c_str!("I_SOLO").into(),
            TcpH => c_str!("I_TCPH").into(),
            TcpY => c_str!("I_TCPY").into(),
            WndH => c_str!("I_WNDH").into(),
            TrackNumber => c_str!("IP_TRACKNUMBER").into(),
            Env(env_chunk_name) => {
                let cow: Cow<CStr> = env_chunk_name.into();
                concat_c_strs(c_str!("P_ENV:<"), cow.as_ref()).into()
            }
            Ext(extension_specific_key) => {
                concat_c_strs(c_str!("P_EXT:"), extension_specific_key.as_ref()).into()
            }
            Icon => c_str!("P_ICON").into(),
            McpLayout => c_str!("P_MCP_LAYOUT").into(),
            Name => c_str!("P_NAME").into(),
            ParTrack => c_str!("P_PARTRACK").into(),
            Project => c_str!("P_PROJECT").into(),
            TcpLayout => c_str!("P_TCP_LAYOUT").into(),
            Custom(key) => key,
        }
    }
}

mod private {
    use crate::TrInfo;
    use std::ptr::NonNull;

    pub trait Sealed {}
    impl<'a, T> Sealed for TrInfo<'a, *mut T> {}
    impl<'a, T> Sealed for TrInfo<'a, *const T> {}
    impl<'a, T> Sealed for TrInfo<'a, Option<NonNull<T>>> {}
    impl<'a> Sealed for TrInfo<'a, i32> {}
}

pub trait TrackInfo<'a, T>: private::Sealed {
    fn ptr_as_value(ptr: *mut c_void) -> T;
    fn key(self) -> TrackInfoKey<'a>;
    fn value_as_ptr(&self) -> *mut c_void;
}

pub(crate) struct TrInfo<'a, T> {
    pub(crate) key: TrackInfoKey<'a>,
    pub(crate) value: T,
}

pub mod track_infos {
    use crate::TrackInfoKey::*;
    use crate::{
        EnvChunkName, MediaTrack, ReaProject, ReaperStringArg, TrInfo, TrackInfo, TrackInfoKey,
    };
    use reaper_rs_low::raw;
    use std::os::raw::{c_char, c_void};
    use std::ptr::null_mut;

    pub fn get_par_track<'a>() -> impl TrackInfo<'a, Option<MediaTrack>> {
        TrInfo {
            key: ParTrack,
            value: None,
        }
    }

    /// In REAPER < 5.95 this returns `None`.
    pub fn get_project<'a>() -> impl TrackInfo<'a, Option<ReaProject>> {
        TrInfo {
            key: TrackInfoKey::Project,
            value: None,
        }
    }

    pub fn get_name<'a>() -> impl TrackInfo<'a, *const c_char> {
        TrInfo {
            key: Name,
            value: null_mut() as *const c_char,
        }
    }

    pub fn set_name<'a>(value: *const c_char) -> impl TrackInfo<'a, *const c_char> {
        TrInfo { key: Name, value }
    }

    pub fn ext<'a>(
        key: impl Into<ReaperStringArg<'a>>,
        value: *mut c_char,
    ) -> impl TrackInfo<'a, *mut c_char> {
        TrInfo {
            key: TrackInfoKey::ext(key),
            value,
        }
    }
    pub fn main_send_offs<'a>(value: *mut c_char) -> impl TrackInfo<'a, *mut c_char> {
        TrInfo {
            key: MainSendOffs,
            value,
        }
    }
    pub fn beat_attach_mode<'a>(value: *mut c_char) -> impl TrackInfo<'a, *mut c_char> {
        TrInfo {
            key: BeatAttachMode,
            value,
        }
    }

    pub fn icon<'a>(value: *const c_char) -> impl TrackInfo<'a, *const c_char> {
        TrInfo { key: Icon, value }
    }
    pub fn mcp_layout<'a>(value: *const c_char) -> impl TrackInfo<'a, *const c_char> {
        TrInfo {
            key: McpLayout,
            value,
        }
    }
    pub fn tcp_layout<'a>(value: *const c_char) -> impl TrackInfo<'a, *const c_char> {
        TrInfo {
            key: TcpLayout,
            value,
        }
    }
    pub fn rec_mon<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: RecMon, value }
    }

    pub fn rec_input<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: RecInput,
            value,
        }
    }

    pub fn solo<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: Solo, value }
    }
    pub fn fx_en<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: FxEn, value }
    }
    pub fn rec_mode<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: RecMode,
            value,
        }
    }
    pub fn rec_mon_items<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: RecMonItems,
            value,
        }
    }
    pub fn auto_mode<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: AutoMode,
            value,
        }
    }
    pub fn nchan<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: Nchan, value }
    }
    pub fn selected<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: Selected,
            value,
        }
    }
    pub fn wnd_h<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: WndH, value }
    }
    pub fn tcp_h<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: TcpH, value }
    }
    pub fn tcp_y<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: TcpY, value }
    }
    pub fn mcp_x<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: McpX, value }
    }
    pub fn mcp_y<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: McpY, value }
    }
    pub fn mcp_w<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: McpW, value }
    }
    pub fn mcp_h<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo { key: McpH, value }
    }
    pub fn folder_depth<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: FolderDepth,
            value,
        }
    }
    pub fn folder_compact<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: FolderCompact,
            value,
        }
    }
    pub fn midi_hw_out<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: MidiHwOut,
            value,
        }
    }
    pub fn perf_flags<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: PerfFlags,
            value,
        }
    }
    pub fn custom_color<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: CustomColor,
            value,
        }
    }
    pub fn height_override<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: HeightOverride,
            value,
        }
    }
    pub fn pan_mode<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: PanMode,
            value,
        }
    }
    pub fn play_offset_flag<'a>(value: *mut i32) -> impl TrackInfo<'a, *mut i32> {
        TrInfo {
            key: PlayOffsetFlag,
            value,
        }
    }
    pub fn track_number<'a>(value: i32) -> impl TrackInfo<'a, i32> {
        TrInfo {
            key: TrackNumber,
            value,
        }
    }
    pub fn guid<'a>(value: *mut raw::GUID) -> impl TrackInfo<'a, *mut raw::GUID> {
        TrInfo { key: Guid, value }
    }
    pub fn mute<'a>(value: *mut bool) -> impl TrackInfo<'a, *mut bool> {
        TrInfo { key: Mute, value }
    }
    pub fn phase<'a>(value: *mut bool) -> impl TrackInfo<'a, *mut bool> {
        TrInfo { key: Phase, value }
    }
    pub fn height_lock<'a>(value: *mut bool) -> impl TrackInfo<'a, *mut bool> {
        TrInfo {
            key: HeightLock,
            value,
        }
    }
    pub fn show_in_mixer<'a>(value: *mut bool) -> impl TrackInfo<'a, *mut bool> {
        TrInfo {
            key: ShowInMixer,
            value,
        }
    }
    pub fn show_in_tcp<'a>(value: *mut bool) -> impl TrackInfo<'a, *mut bool> {
        TrInfo {
            key: ShowInTcp,
            value,
        }
    }
    pub fn main_send<'a>(value: *mut bool) -> impl TrackInfo<'a, *mut bool> {
        TrInfo {
            key: MainSend,
            value,
        }
    }
    pub fn free_mode<'a>(value: *mut bool) -> impl TrackInfo<'a, *mut bool> {
        TrInfo {
            key: FreeMode,
            value,
        }
    }
    pub fn vol<'a>(value: *mut f64) -> impl TrackInfo<'a, *mut f64> {
        TrInfo { key: Vol, value }
    }
    pub fn pan<'a>(value: *mut f64) -> impl TrackInfo<'a, *mut f64> {
        TrInfo { key: Pan, value }
    }
    pub fn width<'a>(value: *mut f64) -> impl TrackInfo<'a, *mut f64> {
        TrInfo { key: Width, value }
    }
    pub fn dual_pan_l<'a>(value: *mut f64) -> impl TrackInfo<'a, *mut f64> {
        TrInfo {
            key: DualPanL,
            value,
        }
    }
    pub fn dual_pan_r<'a>(value: *mut f64) -> impl TrackInfo<'a, *mut f64> {
        TrInfo {
            key: DualPanR,
            value,
        }
    }
    pub fn pan_law<'a>(value: *mut f64) -> impl TrackInfo<'a, *mut f64> {
        TrInfo { key: PanLaw, value }
    }
    pub fn play_offset<'a>(value: *mut f64) -> impl TrackInfo<'a, *mut f64> {
        TrInfo {
            key: PlayOffset,
            value,
        }
    }
    pub fn env(
        name: EnvChunkName,
        value: *mut raw::TrackEnvelope,
    ) -> impl TrackInfo<*mut raw::TrackEnvelope> {
        TrInfo {
            key: Env(name),
            value,
        }
    }

    pub fn mcp_fx_send_scale<'a>(value: *mut f32) -> impl TrackInfo<'a, *mut f32> {
        TrInfo {
            key: McpFxSendScale,
            value,
        }
    }
    pub fn mcp_send_rgn_scale<'a>(value: *mut f32) -> impl TrackInfo<'a, *mut f32> {
        TrInfo {
            key: McpSendRgnScale,
            value,
        }
    }

    pub fn custom<'a>(
        key: impl Into<ReaperStringArg<'a>>,
        value: *mut c_void,
    ) -> impl TrackInfo<'a, *mut c_void> {
        TrInfo {
            key: TrackInfoKey::custom(key),
            value,
        }
    }
}

impl<'a, T> TrackInfo<'a, *mut T> for TrInfo<'a, *mut T> {
    fn ptr_as_value(ptr: *mut c_void) -> *mut T {
        ptr as _
    }

    fn key(self) -> TrackInfoKey<'a> {
        self.key
    }

    fn value_as_ptr(&self) -> *mut c_void {
        self.value as _
    }
}

impl<'a, T> TrackInfo<'a, *const T> for TrInfo<'a, *const T> {
    fn ptr_as_value(ptr: *mut c_void) -> *const T {
        ptr as _
    }

    fn key(self) -> TrackInfoKey<'a> {
        self.key
    }

    fn value_as_ptr(&self) -> *mut c_void {
        self.value as _
    }
}

impl<'a, T> TrackInfo<'a, Option<NonNull<T>>> for TrInfo<'a, Option<NonNull<T>>> {
    fn ptr_as_value(ptr: *mut c_void) -> Option<NonNull<T>> {
        NonNull::new(ptr as *mut T)
    }

    fn key(self) -> TrackInfoKey<'a> {
        self.key
    }

    fn value_as_ptr(&self) -> *mut c_void {
        match self.value {
            None => null_mut(),
            Some(v) => v.as_ptr() as _,
        }
    }
}

impl<'a> TrackInfo<'a, i32> for TrInfo<'a, i32> {
    fn ptr_as_value(ptr: *mut c_void) -> i32 {
        ptr as _
    }

    fn key(self) -> TrackInfoKey<'a> {
        self.key
    }

    fn value_as_ptr(&self) -> *mut c_void {
        self.value as _
    }
}

/// All the possible track send info keys which you can pass to `Reaper::get_set_track_send_info()`.
///
/// The variants are named exactly like the strings which will be passed to the low-level REAPER
/// function because the medium-level API is designed to still be close to the raw REAPER API.  
///
/// Please raise a reaper-rs issue if you find that an enum variant is missing!
#[derive(Clone, Debug)]
pub enum TrackSendInfoKey<'a> {
    Mono,
    Mute,
    Phase,
    Pan,
    PanLaw,
    Vol,
    AutoMode,
    DstChan,
    MidiFlags,
    SendMode,
    SrcChan,
    DestTrack,
    SrcTrack,
    Env(EnvChunkName<'a>),
    Ext(Cow<'a, CStr>),
    /// If a variant is missing in this enum, you can use this custom one as a resort.
    Custom(Cow<'a, CStr>),
}

impl<'a> TrackSendInfoKey<'a> {
    pub fn p_ext(key: impl Into<ReaperStringArg<'a>>) -> TrackSendInfoKey<'a> {
        TrackSendInfoKey::Ext(key.into().into_inner())
    }

    pub fn custom(key: impl Into<ReaperStringArg<'a>>) -> TrackSendInfoKey<'a> {
        TrackSendInfoKey::Custom(key.into().into_inner())
    }
}

impl<'a> From<TrackSendInfoKey<'a>> for Cow<'a, CStr> {
    fn from(value: TrackSendInfoKey<'a>) -> Self {
        use TrackSendInfoKey::*;
        match value {
            Mono => c_str!("B_MONO").into(),
            Mute => c_str!("B_MUTE").into(),
            Phase => c_str!("B_PHASE").into(),
            Pan => c_str!("D_PAN").into(),
            PanLaw => c_str!("D_PANLAW").into(),
            Vol => c_str!("D_VOL").into(),
            AutoMode => c_str!("I_AUTOMODE").into(),
            DstChan => c_str!("I_DSTCHAN").into(),
            MidiFlags => c_str!("I_MIDIFLAGS").into(),
            SendMode => c_str!("I_SENDMODE").into(),
            SrcChan => c_str!("I_SRCCHAN").into(),
            DestTrack => c_str!("P_DESTTRACK").into(),
            SrcTrack => c_str!("P_SRCTRACK").into(),
            Env(env_chunk_name) => {
                let cow: Cow<CStr> = env_chunk_name.into();
                concat_c_strs(c_str!("P_ENV:<"), cow.as_ref()).into()
            }
            Ext(key) => concat_c_strs(c_str!("P_EXT:"), key.as_ref()).into(),
            Custom(key) => key.into(),
        }
    }
}

/// Common envelope chunk names which you can pass to `TrackInfoKey::P_ENV()`.
///
/// The variants are named exactly like the strings which will be passed to the low-level REAPER
/// function because the medium-level API is designed to still be close to the raw REAPER API.  
///
/// Please raise a reaper-rs issue if you find that an enum variant is missing!
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum EnvChunkName<'a> {
    /// Volume (Pre-FX)
    VolEnv,
    /// Pan (Pre-FX)
    PanEnv,
    /// Volume
    VolEnv2,
    /// Pan
    PanEnv2,
    /// Width (Pre-FX)
    WidthEnv,
    /// Width
    WidthEnv2,
    /// Trim Volume
    VolEnv3,
    /// Mute
    MuteEnv,
    /// Use this for all non-common envelope names.
    Custom(Cow<'a, CStr>),
}

impl<'a> EnvChunkName<'a> {
    pub fn custom(name: impl Into<ReaperStringArg<'a>>) -> Self {
        Self::Custom(name.into().into_inner())
    }
}

impl<'a> From<EnvChunkName<'a>> for Cow<'a, CStr> {
    fn from(value: EnvChunkName<'a>) -> Self {
        use EnvChunkName::*;
        match value {
            VolEnv => c_str!("VOLENV").into(),
            PanEnv => c_str!("PANENV").into(),
            VolEnv2 => c_str!("VOLENV2").into(),
            PanEnv2 => c_str!("PANENV2").into(),
            WidthEnv => c_str!("WIDTHENV").into(),
            WidthEnv2 => c_str!("WIDTHENV2").into(),
            VolEnv3 => c_str!("VOLENV3").into(),
            MuteEnv => c_str!("MUTEENV").into(),
            Custom(name) => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_track_info_key() {
        use TrackInfoKey::*;
        assert_eq!(Cow::from(Mute).as_ref(), c_str!("B_MUTE"));
        assert_eq!(
            Cow::from(Env(EnvChunkName::VolEnv)).as_ref(),
            c_str!("P_ENV:<VOLENV")
        );
        assert_eq!(
            Cow::from(Env(EnvChunkName::Custom(c_str!("MYENV").into()))).as_ref(),
            c_str!("P_ENV:<MYENV")
        );
        assert_eq!(
            Cow::from(TrackInfoKey::ext("SWS_FOO")).as_ref(),
            c_str!("P_EXT:SWS_FOO")
        );
        assert_eq!(
            Cow::from(TrackInfoKey::custom(c_str!("BLA"))).as_ref(),
            c_str!("BLA")
        );
    }
}
