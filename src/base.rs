
#[macro_export]
#[doc(hidden)]
macro_rules! eficall_abi {
    (($($prefix:tt)*),($($suffix:tt)*)) => { $($prefix)* extern "efiapi" $($suffix)* };
}


#[macro_export]
macro_rules! eficall {
    // Muncher
    //
    // The `@munch()` rules are internal and should not be invoked directly. We walk through the
    // input, moving one token after the other from the suffix into the prefix until we find the
    // position where to insert `extern "<abi>"`. This muncher never drops any tokens, hence we
    // can safely match invalid statements just fine, as the compiler will later print proper
    // diagnostics when parsing the macro output.
    // Once done, we invoke the `eficall_abi!{}` macro, which simply inserts the correct ABI.
    (@munch(($($prefix:tt)*),(pub $($suffix:tt)*))) => { eficall!{@munch(($($prefix)* pub),($($suffix)*))} };
    (@munch(($($prefix:tt)*),(unsafe $($suffix:tt)*))) => { eficall!{@munch(($($prefix)* unsafe),($($suffix)*))} };
    (@munch(($($prefix:tt)*),($($suffix:tt)*))) => { eficall_abi!{($($prefix)*),($($suffix)*)} };

    // Entry Point
    //
    // This captures the entire argument and invokes its own TT-muncher, but splits the input into
    // prefix and suffix, so the TT-muncher can walk through it. Note that initially everything is
    // in the suffix and the prefix is empty.
    ($($arg:tt)*) => { eficall!{@munch((),($($arg)*))} };
}

