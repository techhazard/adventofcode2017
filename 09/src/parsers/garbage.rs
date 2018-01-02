use super::*;

named!(garbage_start, do_parse!(
        not!(exclamation)
        >> angle_open
        >> (b!("start"))
    ));

named!(garbage_end, do_parse!(
        not!(exclamation)
        >> angle_close
        >> (b!("end"))
    ));

named!(garbage_item, not!(garbage_end));

named!(garbage_contents<&[u8], Vec<&[u8]> >, many0!(garbage_item));


named!(garbage, do_parse!(
        garbage_start
        >> garbage_contents
        >> garbage_end
        >> (b!(""))
));

#[cfg(test)]
mod tests {
    use nom;

    #[test]
    fn garbage_start() {
        make_test!(garbage_start, ["<>", Done, ">", "start"]);
        make_test!(garbage_start, ["!<>", Not ]);
        make_test!(garbage_start, ["A", Tag]);
    }

    #[test]
    fn garbage_end() {
        make_test!(garbage_end, [">a", Done, "a", "end"]);
        make_test!(garbage_end, ["!>a", Not]);
        make_test!(garbage_end, ["A", Tag]);
    }

    #[test]
    fn garbage_end() {
        make_test!()
    }

}
