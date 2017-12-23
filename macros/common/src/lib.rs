extern crate pmutil;
extern crate proc_macro2;
extern crate proc_macro;
extern crate quote;
extern crate syn;
use pmutil::synom_ext::FromSpan;
use proc_macro2::Span;

pub mod prelude;

pub fn call_site<T: FromSpan>() -> T {
    FromSpan::from_span(Span::call_site())
}

/// `attr` - tokens inside `#[]`. e.g. `derive(EqIgnoreSpan)`, ast_node
pub fn print<T: Into<proc_macro2::TokenStream>>(
    attr: &'static str,
    t: T,
) -> proc_macro::TokenStream {
    use std::env;

    let tokens = t.into();

    match env::var("PRINT_GENERATED") {
        Ok(ref s) if s == "1" || attr == s => {}
        _ => return tokens.into(),
    }

    println!("\n\tOutput of #[{}]:\n\t {}", attr, tokens);
    tokens.into()
}

/// fail! is a panic! with location reporting.
#[macro_export]
macro_rules! fail {
    ($($args:tt)+) => {{
        panic!("{}\n --> {}:{}:{}", format_args!($($args)*), file!(), line!(), column!());
    }};
}

#[macro_export]
macro_rules! unimplemented {
    ($($args:tt)+) => {{
        fail!("not yet implemented: {}", format_args!($($args)*));
    }};
}

#[macro_export]
macro_rules! unreachable {
    () => {{
        fail!("internal error: unreacable");
    }};
    ($($args:tt)+) => {{
        fail!("internal error: unreacable\n{}", format_args!($($args)*));
    }};
}