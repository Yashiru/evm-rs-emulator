// TODO(phase-1): Each `allow` below flags a class of lints that Phase 1 of
// the cleanup plan (see PLAN.md) will eliminate by fixing the underlying
// code. They are silenced here so the strict CI gate (`clippy -D warnings`)
// can be introduced immediately without blocking the existing codebase.
// Remove these as the corresponding code is cleaned up.
#![allow(
    clippy::clone_on_copy,
    clippy::expect_fun_call,
    clippy::get_first,
    clippy::len_zero,
    clippy::manual_is_multiple_of,
    clippy::manual_strip,
    clippy::match_single_binding,
    clippy::missing_safety_doc,
    clippy::needless_borrow,
    clippy::needless_borrows_for_generic_args,
    clippy::needless_late_init,
    clippy::needless_question_mark,
    clippy::new_without_default,
    clippy::nonminimal_bool,
    clippy::print_in_format_impl,
    clippy::print_literal,
    clippy::single_char_add_str,
    clippy::to_string_in_format_args,
    clippy::unit_arg,
    clippy::unnecessary_fallible_conversions,
    clippy::unnecessary_to_owned,
    clippy::unnecessary_unwrap,
    clippy::unused_enumerate_index,
    clippy::useless_conversion,
    noop_method_call
)]

mod core_module;

/* ---------------------------------- Core ---------------------------------- */
pub use core_module::memory::Memory;
pub use core_module::op_codes;
pub use core_module::runner::Runner;
pub use core_module::stack::Stack;
pub use core_module::state::EvmState;

/* ---------------------------------- Utils --------------------------------- */
pub use core_module::utils::bytes;
pub use core_module::utils::debug;
pub use core_module::utils::environment;
pub use core_module::utils::errors;
