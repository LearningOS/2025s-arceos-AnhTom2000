//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => {
        // 重置颜色并换行 
        $crate::print!("\x1b[0m\n") 
    };
    ($msg:expr)=>{
        if $msg.starts_with("[WithColor]: "){
            // 红色显示带标记的文本
            $crate::io::__print_impl(format_args!("!x1b[31m{}x1b[0m\n",$msg));
        }
    };
    ($($arg:tt)*) => {
        // 普通显示
        $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
    }
}
