gecko-profiler-demangle
=======================

This crate, or npm package (depending on where you're reading this), contains a wrapper function `demangle_any` for C++ and rust name demangling. This wrapper function is intended to be used through WASM in [the Firefox profiler project](https://profiler.firefox.com/).

Usage instructions
------------------

 1. Have your webpack configuration set up to understand .wasm file loading and
    async imports. This may require adding `'.wasm'` to the list of resolve
    extensions in your `webpack.config.js`, and it may require the use of the
    babel plugin `syntax-dynamic-import` so that the `import()` function is
    recognized.
 2. Add a dependency to the [npm module `gecko-profiler-demangle`](https://www.npmjs.com/package/gecko-profiler-demangle).
 3. In your JavaScript code:
    ```js
    import('gecko-profiler-demangle').then(demanglerModule => {
      const mangledNames = [
        '_ZN3art10ArenaStack19AllocWithMemoryToolEjNS_14ArenaAllocKindE',
        'pthread_setspecific', '_ZN3foo3barE', '_ZN3foo17h05af221e174051e9E'
      ];
      const demangledNames = mangledNames.map(n => demanglerModule.demangle_any(n));
      console.log(demangledNames);
      // ['art::ArenaStack::AllocWithMemoryTool(unsigned int, art::ArenaAllocKind)',
      //  'pthread_setspecific', 'foo::bar', 'foo']
    });
    ```

Build instructions
------------------

To rebuild this npm module from the original Rust code, clone [the repo](https://github.com/mstange/gecko-profiler-demangle/) and proceed as follows:

```bash
$ cargo +nightly install wasm-pack --force
$ wasm-pack build --out-name index
$ # Now grab the files you need from pkg/ and move them where you want them.
```

Publishing instructions
-----------------------

To publish a new version of this module on npm, do the following:

 1. Update the version number in `Cargo.toml`. The same version will be used for the npm package.
 2. Manually edit `pkg/package.json` to add a `"main": "index.js",` entry, see [this comment](https://github.com/mstange/gecko-profiler-demangle/issues/1#issuecomment-849023815) for background.
 3. Manually edit `pkg/package.json` to add an `"index_bg.js",` entry to the `"files"` list, see [this comment](https://github.com/mstange/gecko-profiler-demangle/issues/2#issuecomment-854959688) for background.
 4. Run `wasm-pack publish`.
