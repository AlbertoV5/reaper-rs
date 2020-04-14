//! Provides all functions from `reaper_plugin_functions.h` with the following small improvements:
//! TODO-medium
//! Doc: Explain that returning CString instead of String is because we also expect CStrings
//! as ideal arguments (for good reasons). It would not be symmetric to return Strings then.
//! Should be similar to Lua API, not too far away because there are lots of tutorials already.
//! At first I didn't have the enums in place and was closer to the low-level API. At the point when
//! I changed to enums I could watch how my high-level API code gets cleaner, more understandable
//! and often also less. In some cases I also discovered API usage bugs because of that. So I think
//! enums are a good choice here. Again, you can always resort to low-level API.
//! I think that with the right abstractions in place, you can build sophisticated extensions much
//! easier, faster and with less bugs because there's no need to take care of the same low-level
//! stuff again and again.
//! The medium-level API offers much more type safety and convenience. Still stays close to
//! original REAPER API and the ultimate goal is to expose every function with every possible
//! calling style of the low-level API, just
//! in a bit nicer and type-safe manner - so that at the end you don't have to resort to the
//! low-level API anymore and this gets a complete replacement. Some low-level functions can't be
//! rewritten in a type-safe way. In this case, new convenience functions are introduced.
//! Quite likely that someone who uses Rust (instead of e.g. Lua) does it also because of
//! performance reasons. medium-level API can be considered the first API. The low-level one is not
//! really supposed to be used directly. It's important that the "first" API still is sensitive
//! about performance and doesn't do too much extra.
//!
//! - Note about strings (both return and parameter)!
//! - When I say "index", I always mean zero-based
//!
//! - Snake-case function and parameter names
//! - Use bool instead of i32 as return value type for functions with obvious "yes or no" result
//! - Use ReaperStringPtr instead of raw c_char pointers as return value type (offers convenience
//!   functions)
//! - Use ReaperVoidPtr instead of raw c_void pointers as return value type (offers convenience
//!   functions)
//! - Use return values instead of output parameters
//! - When there are string output parameters which can be passed a null pointer, trigger this null
//!   pointer case when a buffer size of 0 is passed, also use Cow in this case in order to have a
//!   cheap empty string in null-pointer case
//! - When there are both return values and output parameters, return a tuple if there's just one
//!   output parameter and a struct if there are many output parameters
//! - In all REAPER functions which can fail (mostly indicated by returning false or -1), return
//!   Result
//! - In all REAPER functions which return things that might not be present, return Option
//! - Panics if function not available (we should make sure on plug-in load that all necessary
//!   functions are available)
//! - More restrictive number types where safely applicable (for increased safety, e.g. u32 instead
//!   of i32). In the unlikely case that the value range has to be extended in future, it's just a
//!   matter of removing safe casts on user-side code.
// TODO-medium In Rust get_ prefix is not idiomatic. On the other hand, the convention talks
//  about exposing members only. Channel is not a member. However I also don't want to
//  expose the information if it's a member or not. get_ has an advantage in IDEs and also
//  prevents ambiguities if the noun can sound like a verb.

mod constants;
pub use constants::*;

mod control_surface;
pub use control_surface::*;

mod midi;
pub use midi::*;

mod reaper;
pub use reaper::*;

mod util;
pub use util::*;

mod string_types;
pub use string_types::*;

mod recording_input;
pub use recording_input::*;

mod automation_mode;
pub use automation_mode::*;

mod message_box;
pub use message_box::*;

mod ptr_wrappers;
pub use ptr_wrappers::*;
