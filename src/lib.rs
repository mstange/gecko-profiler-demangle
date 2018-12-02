extern crate cpp_demangle;
extern crate msvc_demangler;
extern crate rustc_demangle;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use msvc_demangler::DemangleFlags;

/// Takes an input string and runs it through a sequence of demanglers. Returns
/// the demangled string if a match was found, or the original string if not.
#[wasm_bindgen]
pub fn demangle_any(name: &str) -> String {
    if name.starts_with('?') {
        let flags = DemangleFlags::NO_ACCESS_SPECIFIERS
            | DemangleFlags::NO_FUNCTION_RETURNS
            | DemangleFlags::NO_MEMBER_TYPE
            | DemangleFlags::NO_MS_KEYWORDS
            | DemangleFlags::NO_THISTYPE
            | DemangleFlags::NO_CLASS_TYPE
            | DemangleFlags::SPACE_AFTER_COMMA
            | DemangleFlags::HUG_TYPE;
        return msvc_demangler::demangle(&name, flags).unwrap_or_else(|_| name.to_string());
    }
    if let Ok(demangled_symbol) = rustc_demangle::try_demangle(name) {
        return format!("{:#}", demangled_symbol);
    }

    if let Ok(demangled_symbol) = cpp_demangle::Symbol::new(name) {
        return format!("{}", demangled_symbol);
    }

    if name.starts_with('_') {
        return name.split_at(1).1.to_owned();
    }

    name.to_owned()
}
