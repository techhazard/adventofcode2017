use super::*;

named!(group_start, do_parse!(
        not!(exclamation)
        >> curly_open
        >> (b!("start"))
    ));

named!(group_end, do_parse!(
        not!(exclamation)
        >> curly_close
        >> (b!("end"))
    ));

named!(group_item, not!(group_end));

named!(group_contents<&[u8], Vec<&[u8]> >, many0!(group_item));


named!(group, do_parse!(
        group_start
        >> group_contents
        >> group_end
        >> (b!(""))
));

#[cfg(test)]
mod tests {
    use nom;

    #[test]
    fn group_start() {
        make_test!(group_start, ["{}", Done, "}", "start"]);
        make_test!(group_start, ["!{}", Not ]);
        make_test!(group_start, ["A", Tag]);
    }
    #[test]
    fn group_end() {
        make_test!(group_end, ["}a", Done, "a", "end"]);
        make_test!(group_end, ["!}a", Not]);
        make_test!(group_end, ["A", Tag]);
    }

}
