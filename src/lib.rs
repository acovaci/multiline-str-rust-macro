#[macro_export]
macro_rules! multiline_str {
    ( $head_first:literal $( , $head_line:literal )* $( ; $par_first:literal $( , $par_line:literal )* )* ) => {
        concat!($head_first $(, " ", $head_line)* $(, "\n", $par_first $(, " ", $par_line)* )* )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_multiline_str() {
        let s = multiline_str! {
            "abc",
            "def",
            "ghi"
        };
        assert_eq!(s, "abc def ghi");
    }

    #[test]
    fn test_multiline_str_paragraph() {
        let s = multiline_str! {
            "abc",
            "def";
            "ghi"
        };
        assert_eq!(s, "abc def\nghi");
    }
}
