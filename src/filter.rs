use std::env::{self, consts::*};

pub fn evaluate(source: &str) -> Result<bool, Box<rhai::EvalAltResult>> {
    let mut engine = rhai::Engine::new();
    engine
        .register_fn("arch", || ARCH)
        .register_fn("dll_extension", || DLL_EXTENSION)
        .register_fn("dll_prefix", || DLL_PREFIX)
        .register_fn("dll_suffix", || DLL_SUFFIX)
        .register_fn("env", |key: rhai::ImmutableString| {
            env::var(key.as_str()).unwrap_or_default()
        })
        .register_fn("env_present", |key: rhai::ImmutableString| {
            env::var(key.as_str()).is_err_and(|e| e == env::VarError::NotPresent)
        })
        .register_fn("exe_extension", || EXE_EXTENSION)
        .register_fn("exe_suffix", || EXE_SUFFIX)
        .register_fn("family", || FAMILY)
        .register_fn("os", || OS);

    engine.eval_expression(source)
}
