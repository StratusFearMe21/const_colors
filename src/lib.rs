#[macro_export]
macro_rules! end {
    () => {
        r#"[0m"#
    };
}

#[macro_export]
macro_rules! bold {
    () => {
        r#"[1m"#
    };
}

#[macro_export]
macro_rules! dim {
    () => {
        r#"[2m"#
    };
}

#[macro_export]
macro_rules! italic {
    () => {
        r#"[3m"#
    };
}

#[macro_export]
macro_rules! underlined {
    () => {
        r#"[4m"#
    };
}

#[macro_export]
macro_rules! blink {
    () => {
        r#"[5m"#
    };
}

#[macro_export]
macro_rules! reverse {
    () => {
        r#"[7m"#
    };
}

#[macro_export]
macro_rules! hidden {
    () => {
        r#"[8m"#
    };
}

#[macro_export]
macro_rules! black {
    () => {
        r#"[30m"#
    };
}

#[macro_export]
macro_rules! red {
    () => {
        r#"[31m"#
    };
}

#[macro_export]
macro_rules! green {
    () => {
        r#"[32m"#
    };
}

#[macro_export]
macro_rules! yellow {
    () => {
        r#"[33m"#
    };
}

#[macro_export]
macro_rules! blue {
    () => {
        r#"[34m"#
    };
}

#[macro_export]
macro_rules! magenta {
    () => {
        r#"[35m"#
    };
}

#[macro_export]
macro_rules! cyan {
    () => {
        r#"[36m"#
    };
}

#[macro_export]
macro_rules! white {
    () => {
        r#"[37m"#
    };
}

#[macro_export]
macro_rules! color256 {
    ($a:expr) => {
        concat!(r#"[38;5;"#, $a, r#"m"#)
    };
}

#[macro_export]
macro_rules! on_black {
    () => {
        r#"[40m"#
    };
}

#[macro_export]
macro_rules! on_red {
    () => {
        r#"[41m"#
    };
}

#[macro_export]
macro_rules! on_green {
    () => {
        r#"[42m"#
    };
}

#[macro_export]
macro_rules! on_yellow {
    () => {
        r#"[43m"#
    };
}

#[macro_export]
macro_rules! on_blue {
    () => {
        r#"[44m"#
    };
}

#[macro_export]
macro_rules! on_magenta {
    () => {
        r#"[45m"#
    };
}

#[macro_export]
macro_rules! on_cyan {
    () => {
        r#"[46m"#
    };
}

#[macro_export]
macro_rules! on_white {
    () => {
        r#"[47m"#
    };
}

#[macro_export]
macro_rules! on_color256 {
    ($a:expr) => {
        concat!(r#"[48;5;"#, $a, r#"m"#)
    };
}
