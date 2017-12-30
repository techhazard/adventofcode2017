named!(curly_open, tag!("{"));
named!(exclamation, tag!("!"));
named!(escape,
       do_parse!(
            exclamation >>
            take!(1) >>
            (b!(""))
       )
);


#[cfg(test)]
mod tests {
    use nom;
    #[test]
    fn curly_open() {
        make_test!(curly_open, ["{}", Done, "}", "{"]);
        make_test!(curly_open, ["A", Tag]);
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
