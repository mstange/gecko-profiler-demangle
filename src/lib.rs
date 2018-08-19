extern crate cpp_demangle;
extern crate rustc_demangle;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

/// Takes an input string and runs it through a sequence of demanglers. Returns
/// the demangled string if a match was found, or the original string if not.
#[wasm_bindgen]
pub fn demangle_any(name: &str) -> String {
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
