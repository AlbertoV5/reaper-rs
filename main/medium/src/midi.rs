use helgoboss_midi::{ShortMessage, U7};
use reaper_rs_low::raw;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::ptr::NonNull;

// # Internals exposed: no | vtable: yes (Rust => REAPER)
//
// This is like a MediaTrack object in that it wraps a raw pointer. Like KbdSectionInfo, it must not
// be copied because it's reference-only. It is reference-only so we can offer a medium-level API
// for it that doesn't require unsafe code.
// ALternative name: MidiIn
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct MidiInput(pub(crate) NonNull<raw::midi_Input>);

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct MidiOutput(pub(crate) NonNull<raw::midi_Output>);

impl MidiInput {
    // In the past this function was unsafe, didn't return the event list and expected a closure
    // instead that could do something with the event list. That's not necessary anymore since we
    // ensure in get_midi_input() that we only ever publish valid MidiInput instances, and those
    // only by a very short-lived reference that's not possible to cache anywhere. That makes it
    // possible to bind the lifetime of the event list to that MidiInput and everything is fine!
    // Thanks Rust! If we would return an owned event list, we would waste performance because we
    // would need to copy all events first. That would be especially bad because this code code
    // typically runs in the audio thread and therefore has real-time requirements.
    // Should we mark this as unsafe because it can crash if accessed wrongly from UI thread instead
    // of audio thread? I don't think so, then we would have to mark most functions as unsafe
    // because most functions can only be called from UI thread.
    // TODO-low In theory we could prevent undefined behavior by always checking the thread at
    //  first.
    pub fn get_read_buf(&self) -> MidiEventList<'_> {
        let raw_evt_list = unsafe { self.0.as_ref() }.GetReadBuf();
        MidiEventList::new(unsafe { &*raw_evt_list })
    }
}

// # Internals exposed: no | vtable: yes (Rust => REAPER)
// ALternative name: MidiEvtList
pub struct MidiEventList<'a>(&'a raw::MIDI_eventlist);

impl<'a> MidiEventList<'a> {
    pub(super) fn new(raw_evt_list: &'a raw::MIDI_eventlist) -> Self {
        MidiEventList(raw_evt_list)
    }

    pub fn enum_items(&self, bpos: u32) -> MidiEventListIterator<'a> {
        MidiEventListIterator {
            raw_list: self.0,
            bpos: bpos as i32,
        }
    }
}

pub struct MidiEventListIterator<'a> {
    raw_list: &'a raw::MIDI_eventlist,
    bpos: i32,
}

impl<'a> Iterator for MidiEventListIterator<'a> {
    type Item = MidiEvent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let raw_evt = unsafe { self.raw_list.EnumItems(&mut self.bpos as *mut c_int) };
        if raw_evt.is_null() {
            // No MIDI events left
            return None;
        }
        let evt = unsafe { MidiEvent::new(&*raw_evt) };
        Some(evt)
    }
}

// # Internals exposed: yes | vtable: no
// Represents a borrowed reference to a MIDI event from REAPER. Cheap to copy because it's just a
// wrapper around MIDI_event_t.
// ALternative name: MidiEvt
// TODO-low Can be converted into an owned MIDI event in case it needs to live longer than REAPER
//  keeps  the event around.
#[derive(Clone, Copy)]
pub struct MidiEvent<'a>(&'a raw::MIDI_event_t);

impl<'a> MidiEvent<'a> {
    pub unsafe fn new(raw_evt: &'a raw::MIDI_event_t) -> Self {
        MidiEvent(raw_evt)
    }

    pub fn get_frame_offset(&self) -> u32 {
        self.0.frame_offset as u32
    }

    pub fn get_message(&self) -> MidiMessage<'a> {
        MidiMessage::new(self.0)
    }
}

impl<'a> From<MidiEvent<'a>> for &'a raw::MIDI_event_t {
    fn from(outer: MidiEvent<'a>) -> Self {
        outer.0
    }
}

pub struct MidiMessage<'a>(&'a raw::MIDI_event_t);

impl<'a> MidiMessage<'a> {
    pub(super) fn new(raw_evt: &'a raw::MIDI_event_t) -> Self {
        MidiMessage(raw_evt)
    }
}

impl<'a> ShortMessage for MidiMessage<'a> {
    fn status_byte(&self) -> u8 {
        self.0.midi_message[0]
    }

    fn data_byte_1(&self) -> U7 {
        unsafe { U7::new_unchecked(self.0.midi_message[1]) }
    }

    fn data_byte_2(&self) -> U7 {
        unsafe { U7::new_unchecked(self.0.midi_message[2]) }
    }
}
