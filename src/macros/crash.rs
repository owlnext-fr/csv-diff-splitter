#[macro_export]
/// macro crash!()
/// 
/// This will print an error to the standard output, then system exit with the given error code.
/// 
/// Usage: `crash!(String, i32)`
/// 
/// Arguemnts:
/// 
/// - 0 - the error message to display to the user.
/// - 1 - the error code to dispatch to system.
macro_rules! crash{
    ($message:expr,$status_code:expr)=>{
        {
            error!("{}", $message);
            std::process::exit($status_code);
        }
    }
}