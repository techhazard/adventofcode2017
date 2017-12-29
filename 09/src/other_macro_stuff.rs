    // from
    // ```
    // test_parser!(take_i, ["input", "n", "i"]);
    // ```
    // to
    // ```
    // #[test]
    // fn take_i() {
    //     assert_eq!(
    //         super::take_i(b!("input")),
    //         nom::IResult::Done(b!("nput"),b!("i"))
    //     );
    // }
    // ```
    //
    //
    // from
    // ```
    // test_parser!(take_i, ["input", "n", "i"], ["isnot", "snot", "i"]);
    // ```
    // to
    // ```
    // #[test]
    // fn take_i() {
    //     assert_eq!(
    //         super::take_i(b!("input")),
    //         nom::IResult::Done(b!("nput"),b!("i"))
    //     );
    //     assert_eq!(
    //         super::take_i(b!("isnot")),
    //         nom::IResult::Done(b!("snot"),b!("i"))
    //     );
    // }
    // ```

//    macro_rules! test_parser {
//        ($funcname:ident, $($testcases:tt)+) => {
//            #[test]
//            fn $funcname() {
//                make_test!($funcname, $($testcases)*);
//            }
//        };
//    }
//    macro_rules! test_parser_fail {
//        ($funcname:ident, $testcases:tt) => {
//            #[test]
//            #[should_fail]
//            fn $funcname() {
//                make_test!($funcname, $testcases);
//            }
//        };
//    }
//    macro_rules! test_parser_panic {
//        ($funcname:ident, $testcases:tt) => {
//            #[test]
//            #[should_panic]
//            fn $funcname() {
//                make_test!($funcname, $testcases);
//            }
//        };
//    }


    // use this macro to test nom parsers
    // ```
    // make_test!(take_i, ["input", Done, "nput", "i"]);
    // ``````
    // // turns into
    // ```
    // assert_eq!(
    //     super::curly_open(b!("{}")),
    //     nome::IResult::Done(b!("nput"), b!("i"))
    // );
    // ```
