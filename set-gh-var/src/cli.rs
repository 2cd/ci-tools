use std::env::args;

/// First, get all args, then set the cmd to be the first arg.
/// The original args are like: `./set-gh-env K1 V1`
///
/// Since the first arg has been taken away, the remaining ones are `[K1, V1]`.
/// If the number of remaining args is less than 2, panic and the usage is printed.
pub(crate) fn get_args() -> (String, Vec<String>) {
    let mut all_args = args();
    let cmd = all_args
        .next()
        .expect("Failed to get cmd");
    let args = all_args.collect::<Vec<_>>();

    if args.len() < 2 {
        panic!(
            r#"
    Error: The len of args < 2!

    Usage:
        {} Key Value
    or:
        set-gh-env K1 V1 K2 V2
        set-gh-output Key1 "Value 01" Key2 Value2
    ------
    "#,
            cmd
        )
    }
    (cmd, args)
}
