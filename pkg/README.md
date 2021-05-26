gecko-profiler-demangle
=======================

This crate contains a wrapper function for C++ and rust name demangling. This
wrapper function is intended to be used through WASM in
[the perf.html project](https://perf-html.io/).

Build instructions
------------------

```bash
$ cargo +nightly install wasm-pack --force
$ wasm-pack build --out-name index
$ # Now grab the files you need from pkg/ and move them where you want them.
```

Usage instructions
------------------

 1. Have your webpack configuration set up to understand .wasm file loading and
    async imports. This may require adding `'.wasm'` to the list of resolve
    extensions in your `webpack.config.js`, and it may require the use of the
    babel plugin `syntax-dynamic-import` so that the `import()` function is
    recognized.
 2. Put the files `gecko_profiler_demangle.js` and `gecko_profiler_demangle_bg.wasm`
    into a directory where you can import them from.
 3. In your JavaScript code:
    ```js
    import('./gecko_profiler_demangle').then(demanglerModule => {
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