use std::borrow::Cow;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// This type is used for all medium-level API function parameters which are strings. The C++
/// REAPER SDK generally expects UTF-8 encoded C-Strings (`*const c_char`). It wouldn't be
/// convenient if we just keep it that way in Rust.
///
/// At the very least we should provide the possibility to pass `&CStr`. That gives us type safety:
/// It's a reference instead of a pointer so we can assume it's not stale. Also, the `CStr` type
/// gives some guarantees, for example that there are no intermediate zero bytes (which would make
/// the string end abruptly in C world). Because `&CStr` is the closest thing to a `*const c_char`
/// (infact it's the same + some type safety guarantees), `ReaperStringArg` is just a wrapper around
/// it. That answers the medium-level API's claim to be still as close to the original REAPER SDK as
/// possible.
///
/// Well, actually it's a wrapper around a `Cow<CStr>` because it also needs to be able to hold
/// conversion results. For improved convenience, `ReaperStringArg` offers conversions from regular
/// Rust string types. Because medium-level API functions take string parameters as `impl
/// Into<ReaperStringArg>`, they *just work* with regular Rust strings. This conversion is not
/// entirely without cost because it needs to check for intermediate zero bytes and append a zero
/// byte (which demands a copy if a borrowed string is passed)! Therefore, if you want to be sure to
/// not waste any performance, just pass a `&CStr`, then there's no extra cost involved. In many
/// scenarios this is probably over optimization, but the point is, you *can* go the zero-cost way,
/// if you want. Fortunately, the string encoding itself doesn't need to be converted because REAPER
/// expects UTF-8 encoding as well.
///
/// In the *reaper-rs*  code base you will find many examples that pass `c_str!("...")` to string
/// parameters. This macro from the [c_str_macro crate](https://crates.io/crates/c_str_macro)
/// creates static (UTF-8 encoded) `&CStr` literals, just as `"..."` creates `&str` literals.
/// Because those literals are embedded in the plug-in code, no heap-space allocation or conversion
/// is necessary at all. If you want, you can do the same with literals.
pub struct ReaperStringArg<'a>(Cow<'a, CStr>);

impl<'a> ReaperStringArg<'a> {
    pub(super) fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }
}

impl<'a> From<&'a CStr> for ReaperStringArg<'a> {
    fn from(s: &'a CStr) -> Self {
        ReaperStringArg(s.into())
    }
}

impl<'a> From<CString> for ReaperStringArg<'a> {
    fn from(s: CString) -> Self {
        ReaperStringArg(s.into())
    }
}

impl<'a> From<Cow<'a, CStr>> for ReaperStringArg<'a> {
    fn from(s: Cow<'a, CStr>) -> Self {
        match s.into() {
            Cow::Borrowed(b) => b.into(),
            Cow::Owned(o) => o.into(),
        }
    }
}

impl<'a> From<&'a str> for ReaperStringArg<'a> {
    fn from(s: &'a str) -> Self {
        // Requires copying
        ReaperStringArg(CString::new(s).unwrap().into())
    }
}

impl<'a> From<String> for ReaperStringArg<'a> {
    fn from(s: String) -> Self {
        // Doesn't require copying because we own the string now
        ReaperStringArg(CString::new(s).unwrap().into())
    }
}

impl<'a> From<Cow<'a, str>> for ReaperStringArg<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        match s.into() {
            Cow::Borrowed(b) => b.into(),
            Cow::Owned(o) => o.into(),
        }
    }
}

/// This string type is used in the medium-level API to expose strings which are owned by REAPER
/// itself.
pub struct ReaperStringVal<'a>(pub(super) &'a CStr);

impl<'a> From<ReaperStringVal<'a>> for &'a CStr {
    fn from(v: ReaperStringVal<'a>) -> Self {
        v.0
    }
}

impl<'a> From<ReaperStringVal<'a>> for CString {
    fn from(v: ReaperStringVal<'a>) -> Self {
        v.0.to_owned()
    }
}