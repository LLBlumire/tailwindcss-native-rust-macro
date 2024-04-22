use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use std::path::PathBuf;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, LitStr, Token};

/// Keywords used internally by the application.
mod kw {
    syn::custom_keyword!(config);
    syn::custom_keyword!(config_env);
    syn::custom_keyword!(input);
    syn::custom_keyword!(input_env);
    syn::custom_keyword!(tailwindcss_bin);
    syn::custom_keyword!(tailwindcss_bin_env);
}

enum Argument {
    Config {
        _kw_token: kw::config,
        _colon_token: Token![:],
        value: LitStr,
    },
    ConfigEnv {
        _kw_token: kw::config_env,
        _colon_token: Token![:],
        value: LitStr,
    },
    Input {
        _kw_token: kw::input,
        _colon_token: Token![:],
        value: LitStr,
    },
    InputEnv {
        _kw_token: kw::input_env,
        _colon_token: Token![:],
        value: LitStr,
    },
    TailwindCssBin {
        _kw_token: kw::tailwindcss_bin,
        _colon_token: Token![:],
        value: LitStr,
    },
    TailwindCssBinEnv {
        _kw_token: kw::tailwindcss_bin_env,
        _colon_token: Token![:],
        value: LitStr,
    },
}
impl Parse for Argument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(kw::config) {
            Ok(Argument::Config {
                _kw_token: input.parse()?,
                _colon_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead1.peek(kw::config_env) {
            Ok(Argument::ConfigEnv {
                _kw_token: input.parse()?,
                _colon_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead1.peek(kw::input) {
            Ok(Argument::Input {
                _kw_token: input.parse()?,
                _colon_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead1.peek(kw::input_env) {
            Ok(Argument::InputEnv {
                _kw_token: input.parse()?,
                _colon_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead1.peek(kw::tailwindcss_bin) {
            Ok(Argument::TailwindCssBin {
                _kw_token: input.parse()?,
                _colon_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead1.peek(kw::tailwindcss_bin_env) {
            Ok(Argument::TailwindCssBinEnv {
                _kw_token: input.parse()?,
                _colon_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead1.error())
        }
    }
}
impl Argument {
    fn as_config(&self) -> Option<&LitStr> {
        match self {
            Argument::Config { value, .. } => Some(value),
            _ => None,
        }
    }
    fn as_config_env(&self) -> Option<&LitStr> {
        match self {
            Argument::ConfigEnv { value, .. } => Some(value),
            _ => None,
        }
    }
    fn as_input(&self) -> Option<&LitStr> {
        match self {
            Argument::Input { value, .. } => Some(value),
            _ => None,
        }
    }
    fn as_input_env(&self) -> Option<&LitStr> {
        match self {
            Argument::InputEnv { value, .. } => Some(value),
            _ => None,
        }
    }
    fn as_tailwindcss_bin(&self) -> Option<&LitStr> {
        match self {
            Argument::TailwindCssBin { value, .. } => Some(value),
            _ => None,
        }
    }
    fn as_tailwindcss_bin_env(&self) -> Option<&LitStr> {
        match self {
            Argument::TailwindCssBinEnv { value, .. } => Some(value),
            _ => None,
        }
    }
}

struct MacroArgs {
    args: Punctuated<Argument, Token![,]>,
}
impl Parse for MacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(MacroArgs {
            args: Punctuated::parse_terminated(input)?,
        })
    }
}

/// The `include_tailwind!` macro expects to be passed a set of arguments that
/// will govern how it behaves. The expected format is as follows:
///
/// ```rust
/// include_tailwind! {
///     config: "path/to/tailwind.config.js",
///     input: "path/to/tailwind.input.js",
///     tailwindcss_bin: "/path/to/tailwindcss/bin/tailwindcss"
/// }
/// ```
///
/// If a relative path is given, it will be taken relative to the
/// `CARGO_MANIFEST_DIR`.
///
/// The macro will then compile to an inline string representing the output from
/// tailwindcss. This can then be embeded in and returned by (with necessary CSS
/// headers) your web server framework of choice.
///
/// If any of the arguments are not present, they will be read from a
/// corresponding environment variable:
///
///  - TAILWINDCSS_CONFIG
///  - TAILWINDCSS_INPUT
///  - TAILWINDCSS_BIN
///
/// If you would like to override the environment variable being read from,
/// you may do that with the `_env` parameters.
///
/// ```rust
/// include_tailwind! {
///     config_env: "MY_TAILWINDCSS_CONFIG_ENV_VAR",
///     input_env: "MY_TAILWINDCSS_INPUT_ENV_VAR",
///     tailwindcss_bin: "MY_TAILWINDCSS_BIN_ENV_VAR"
/// }
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn include_tailwind(item: TokenStream) -> TokenStream {
    let MacroArgs { args } = parse_macro_input!(item as MacroArgs);
    // Unwrap is okay as long as we are building from inside CARGO... Who isn't?
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap()
        .parse::<PathBuf>()
        .unwrap();
    let args = args.iter().collect::<Vec<_>>();
    let config = args.iter().copied().filter_map(Argument::as_config).next();
    let config_env = args
        .iter()
        .copied()
        .filter_map(Argument::as_config_env)
        .next();
    let input = args.iter().copied().filter_map(Argument::as_input).next();
    let input_env = args
        .iter()
        .copied()
        .filter_map(Argument::as_input_env)
        .next();
    let tailwindcss_bin = args
        .iter()
        .copied()
        .filter_map(Argument::as_tailwindcss_bin)
        .next();
    let tailwindcss_bin_env = args
        .iter()
        .copied()
        .filter_map(Argument::as_tailwindcss_bin_env)
        .next();

    let config_env = config_env.map(LitStr::value);
    let config_env = config_env.as_deref().unwrap_or("TAILWINDCSS_CONFIG");
    let input_env = input_env.map(LitStr::value);
    let input_env = input_env.as_deref().unwrap_or("TAILWINDCSS_INPUT");
    let tailwindcss_bin_env = tailwindcss_bin_env.map(LitStr::value);
    let tailwindcss_bin_env = tailwindcss_bin_env.as_deref().unwrap_or("TAILWINDCSS_BIN");

    let config = match config {
        Some(config) => config.value(),
        None => match std::env::var(config_env) {
            Ok(config) => config,
            Err(e) => abort_call_site!(format!(
                "Required `config` arg or `TAILWINDCSS_CONFIG` env var: {}",
                e
            )),
        },
    };

    let input = match input {
        Some(input) => input.value(),
        None => match std::env::var(input_env) {
            Ok(input) => input,
            Err(e) => abort_call_site!(format!(
                "Required `input` arg or `TAILWINDCSS_INPUT` env var: {}",
                e
            )),
        },
    };

    let tailwindcss_bin = match tailwindcss_bin {
        Some(tailwindcss_bin) => tailwindcss_bin.value(),
        None => match std::env::var(tailwindcss_bin_env) {
            Ok(bin) => bin,
            Err(e) => abort_call_site!(format!(
                "Required `tailwindcss_bin` arg or `TAILWINDCSS_BIN` env var: {}",
                e
            )),
        },
    };

    let mut config_path = match config.parse::<PathBuf>() {
        Ok(path) => path,
        Err(e) => abort_call_site!(format!("Provided config is not a path: {}", e)),
    };
    if config_path.is_relative() {
        config_path = manifest_path.join(config_path);
    }
    if !config_path.exists() {
        abort!(
            config,
            format!(
                "Config path does not exist: {}",
                config_path.to_string_lossy()
            )
        )
    }

    let mut input_path = match input.parse::<PathBuf>() {
        Ok(path) => path,
        Err(e) => abort_call_site!(format!("Provided input is not a path: {}", e)),
    };
    if input_path.is_relative() {
        input_path = manifest_path.join(input_path);
    }
    if !input_path.exists() {
        abort!(
            input,
            format!(
                "Input path does not exist: {}",
                input_path.to_string_lossy()
            )
        )
    }

    let mut tailwindcss_bin_path = match tailwindcss_bin.parse::<PathBuf>() {
        Ok(path) => path,
        Err(e) => abort!(
            tailwindcss_bin,
            format!("Provided tailwindcss_bin is not a path: {}", e)
        ),
    };
    if tailwindcss_bin_path.is_relative() {
        tailwindcss_bin_path = manifest_path.join(tailwindcss_bin_path);
    }
    if !tailwindcss_bin_path.exists() {
        abort!(
            tailwindcss_bin,
            format!(
                "The tailwindcss_bin path does not exist: {}",
                tailwindcss_bin_path.to_string_lossy()
            )
        )
    }

    let tw_proc_output = std::process::Command::new(tailwindcss_bin_path)
        .arg("-c")
        .arg(config_path)
        .arg("-i")
        .arg(input_path)
        .arg("--minify")
        .output();

    let tw_proc_output = match tw_proc_output {
        Ok(tw_proc) => tw_proc,
        Err(e) => abort_call_site!(format!("Tailwind proc did not run correctly: {e}")),
    };

    if !tw_proc_output.status.success() {
        abort_call_site!(format!("Tailwind proc did not run correctly."))
    }

    let tw_content_str = match std::str::from_utf8(&tw_proc_output.stdout) {
        Ok(content) => content,
        Err(e) => abort_call_site!(format!("Generated file is not utf8: {}", e)),
    };

    let tw_content_lit = LitStr::new(tw_content_str, Span::call_site());

    quote! {
        #tw_content_lit
    }
    .into()
}
