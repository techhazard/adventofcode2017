#![feature(trace_macros)]
#[macro_use]
extern crate nom;

/// transform strings into bytestrings
/// previously I wrote:
/// ```
/// assert_eq!(exclamation(&b"r"[..]), Done(&b"r"[..], &b""[..]));
/// ```
///
/// but all those `&b"something"[..]`'s are not really readable
/// so I wrote this to make it easier
/// `b"something"[..]`'s are of type `&[u8]`
/// and `&str` can be transformed into `&[u8]`
/// by using `.as_bytes()`.
///
/// this macro essentialy appends `.as_bytes()`
/// `b!("bla")` -> `"bla".as_bytes()`
///
/// # Examples
/// ```
/// # #[macro_use]
/// # extern crate advent9;
/// # fn main() {
///
/// let bytes : &[u8] = b!("this is a bytestring");
/// assert_eq!(bytes, [116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 98, 121, 116, 101, 115, 116, 114, 105, 110, 103]);
///
/// println!("{:?}", bytes);
/// // prints:
/// // [116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 98, 121, 116, 101, 115, 116, 114, 105, 110, 103]
/// # }
/// ```
#[macro_export] //bla
macro_rules! b {
    ($string:expr) => {
        $string.as_bytes()
    }
}

/// use this macro to test nom parsers
/// ```
/// make_test!(take_i, ["input", Done, "nput", "i"]);
/// ```
/// // turns into
/// ```
/// assert_eq!(
///     super::curly_open(b!("{}")),
///     nome::IResult::Done(b!("nput"), b!("i"))
/// );
/// ```
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate advent9;
///
/// #[macro_use]
/// extern crate nom;
///
/// named!(curly_open, tag!("{"));
/// named!(exclamation, tag!("!"));
/// named!(escape,
///        do_parse!(
///             exclamation >>
///             take!(1) >>
///             (b!(""))
///        )
/// );
///
/// # /*
/// #[cfg(test)]
/// # */
/// mod tests {
///     use nom;
///
///     make_test!(escape, ["!!", Done, "", ""]);
/// }
///
/// # fn main() {
/// #     tests::escape();
/// # }
/// ```
#[macro_export]
macro_rules! make_test {

     ($funcname:ident, $($testcase:tt),+) => {
      /*
        #[test]
      */
      pub
         fn $funcname() {
             make_testcases!($funcname, $($testcase),+);
         }
     };

     ($should:ident $funcname:ident, $($testcase:tt),+) => {
         #[$should]
         #[test]
         fn $funcname() {
             make_testcases!($funcname, $($testcase),+);
         }
     };

 }



#[macro_export]
 macro_rules! make_testcases {

     ($funcname:ident, $testcase:tt, $($testcases:tt),+) => {
             make_testcases!($funcname, $testcase);
             make_testcases!($funcname, $($testcases),+);
     };

     ($funcname:ident, [$input:expr, $outputType:ident, $($output:expr),+]) => {
         assert_eq!(
             super::$funcname($input.as_bytes())
             ,
             nom::IResult::$outputType($($output.as_bytes()),+)
         );
     };
 }


