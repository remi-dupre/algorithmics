#[macro_export]
macro_rules! lines {
    ( $line: literal ) => {
        $line
    };
    ( $line: literal $( $tail: literal )+ ) => {
        concat!($line, "\n", $crate::lines!($( $tail )+))
    };
}
