#[cfg(windows)]
macro_rules! PATH_SEPARATOR {
    () => {
        r"\"
    };
}

#[allow(dead_code)]
#[cfg(not(windows))]
macro_rules! PATH_SEPARATOR {
    () => {
        r"/"
    };
}

#[allow(unused)]
macro_rules! PATH_OF {
    ($($comp:expr),+ ; $tail:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"),PATH_SEPARATOR!(),$($comp, PATH_SEPARATOR!()),+, $tail)
    };
}

macro_rules! PATH_TO_DOCUMENTATION {
    ($($comp:expr),* => $tail:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            PATH_SEPARATOR!(),
            "resources",
            PATH_SEPARATOR!(),
            "documentation",
            PATH_SEPARATOR!(),
            $tail
        )
    };
}

#[allow(unused)]
pub(crate) use PATH_OF;
pub(crate) use PATH_SEPARATOR;
pub(crate) use PATH_TO_DOCUMENTATION;
