#[macro_export]
macro_rules! crash{
    ($message:expr,$status_code:expr)=>{
        {
            error!("{}", $message);
            std::process::exit($status_code);
        }
    }
}