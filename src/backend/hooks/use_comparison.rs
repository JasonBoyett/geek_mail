use dioxus::prelude::*;

/// Compares a signal with type T and a value of type T and returns
/// true if they are equal, false otherwise.
pub fn use_comparison<T>(signal: Signal<T>, val: T) -> bool {
    let value_signal = use_signal(|| val);
    signal == value_signal
}
