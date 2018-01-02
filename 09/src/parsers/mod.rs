mod groups;
mod garbage;


named!(curly_open, tag!("{"));
named!(curly_close, tag!("}"));

named!(angle_open, tag!("<"));
named!(angle_close, tag!(">"));

named!(exclamation, tag!("!"));
named!(escape,
       do_parse!(
            exclamation
            >> take!(1)
            >> (b!(""))
));

#[cfg(test)]
mod tests {
    use nom;
    #[test]
    fn curly_open() {
        make_test!(curly_open, ["{}", Done, "}", "{"]);
        make_test!(curly_open, ["A", Tag]);
    }
    #[test]
    fn curly_close() {
        make_test!(curly_close, ["}a", Done, "a", "}"]);
        make_test!(curly_close, ["A", Tag]);
    }

    #[test]
    fn angle_open() {
        make_test!(angle_open, ["<>", Done, ">", "<"]);
        make_test!(angle_open, ["A", Tag]);
    }
    #[test]
    fn angle_close() {
        make_test!(angle_close, [">a", Done, "a", ">"]);
        make_test!(angle_close, ["A", Tag]);
    }

    #[test]
    fn exclamation() {
        make_test!(exclamation, ["!!", Done, "!", "!"]);
        make_test!(exclamation, ["A", Tag]);
    }
    fn escape() {
        make_test!(escape, ["!!", Done, "", ""]);
        make_test!(escape, ["!<", Done, "", ""]);
        make_test!(escape, ["!>", Done, "", ""]);
        make_test!(escape, ["!{", Done, "", ""]);
        make_test!(escape, ["!}", Done, "", ""]);
        make_test!(escape, ["!!", Done, "a", ""]);
        make_test!(escape, ["A", Tag]);
    }
}
