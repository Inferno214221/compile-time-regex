#[doc(hidden)]
#[allow(non_snake_case)]
mod __regex_Email {
    use crate::group::CaptureGroup;

    #[derive(Default, Clone, Copy)]
    pub struct EmailByteClass0;
    impl crate::matcher::Class<u8> for EmailByteClass0 {
        const ENTRIES: &[crate::matcher::ClassEntry<u8>] = &[
            crate::matcher::ClassEntry {
                value: 48u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 57u8,
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 65u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 90u8,
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 95u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 97u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 122u8,
                is_upper_bound: true,
            },
        ];
    }
    #[derive(Default, Clone, Copy)]
    pub struct EmailByteClass1;
    impl crate::matcher::Class<u8> for EmailByteClass1 {
        const ENTRIES: &[crate::matcher::ClassEntry<u8>] = &[
            crate::matcher::ClassEntry {
                value: 48u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 57u8,
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 65u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 90u8,
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 95u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 97u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 122u8,
                is_upper_bound: true,
            },
        ];
    }
    #[derive(Default, Clone, Copy)]
    pub struct EmailByteClass2;
    impl crate::matcher::Class<u8> for EmailByteClass2 {
        const ENTRIES: &[crate::matcher::ClassEntry<u8>] = &[
            crate::matcher::ClassEntry {
                value: 48u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 57u8,
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 65u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 90u8,
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 95u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 97u8,
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 122u8,
                is_upper_bound: true,
            },
        ];
    }
    #[derive(Default, Clone, Copy)]
    pub struct EmailScalarClass0;
    impl crate::matcher::Class<char> for EmailScalarClass0 {
        const ENTRIES: &[crate::matcher::ClassEntry<char>] = &[
            crate::matcher::ClassEntry {
                value: '0',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: '9',
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 'A',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'Z',
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: '_',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'a',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'z',
                is_upper_bound: true,
            },
        ];
    }
    #[derive(Default, Clone, Copy)]
    pub struct EmailScalarClass1;
    impl crate::matcher::Class<char> for EmailScalarClass1 {
        const ENTRIES: &[crate::matcher::ClassEntry<char>] = &[
            crate::matcher::ClassEntry {
                value: '0',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: '9',
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 'A',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'Z',
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: '_',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'a',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'z',
                is_upper_bound: true,
            },
        ];
    }
    #[derive(Default, Clone, Copy)]
    pub struct EmailScalarClass2;
    impl crate::matcher::Class<char> for EmailScalarClass2 {
        const ENTRIES: &[crate::matcher::ClassEntry<char>] = &[
            crate::matcher::ClassEntry {
                value: '0',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: '9',
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: 'A',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'Z',
                is_upper_bound: true,
            },
            crate::matcher::ClassEntry {
                value: '_',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'a',
                is_upper_bound: false,
            },
            crate::matcher::ClassEntry {
                value: 'z',
                is_upper_bound: true,
            },
        ];
    }
    #[derive(Default, Clone, Copy)]
    pub struct EmailLiteral0;
    impl crate::matcher::Literal for EmailLiteral0 {
        const LITERAL: &[u8] = b"@";
    }
    #[derive(Default, Clone, Copy)]
    pub struct EmailLiteral1;
    impl crate::matcher::Literal for EmailLiteral1 {
        const LITERAL: &[u8] = b".";
    }
    ///A macro-generated regular expression matching the pattern: `(\w+)@(?<domain>(\w+)(\.\w+)?)` with flags: []. See the [`Regex`](crate::expr::Regex) trait for associated matching and capturing functions.
    #[derive(Clone, Copy)]
    pub struct Email;
    impl crate::expr::Regex<u8, 5> for Email {
        type Pattern = crate::matcher::Then<
            u8,
            crate::matcher::CaptureGroup<
                u8,
                crate::matcher::QuantifierNOrMore<
                    u8,
                    crate::matcher::ClassMatcher<u8, EmailByteClass0>,
                    1usize,
                >,
                1usize,
            >,
            crate::matcher::Then<
                u8,
                crate::matcher::LiteralMatcher<EmailLiteral0>,
                crate::matcher::CaptureGroup<
                    u8,
                    crate::matcher::Then<
                        u8,
                        crate::matcher::CaptureGroup<
                            u8,
                            crate::matcher::QuantifierNOrMore<
                                u8,
                                crate::matcher::ClassMatcher<
                                    u8,
                                    EmailByteClass1,
                                >,
                                1usize,
                            >,
                            3usize,
                        >,
                        crate::matcher::QuantifierNToM<
                            u8,
                            crate::matcher::CaptureGroup<
                                u8,
                                crate::matcher::Then<
                                    u8,
                                    crate::matcher::LiteralMatcher<
                                        EmailLiteral1,
                                    >,
                                    crate::matcher::QuantifierNOrMore<
                                        u8,
                                        crate::matcher::ClassMatcher<
                                            u8,
                                            EmailByteClass2,
                                        >,
                                        1usize,
                                    >,
                                >,
                                4usize,
                            >,
                            0usize,
                            1usize,
                        >,
                    >,
                    2usize,
                >,
            >,
        >;
        type Anchors = crate::anchor::MinLen<3usize>;
        type Capture<'a, S: crate::haystack::HaystackSlice<'a>> = EmailCapture<
            'a,
            S,
            true,
        >;
    }
    impl crate::expr::Regex<char, 5> for Email {
        type Pattern = crate::matcher::Then<
            char,
            crate::matcher::CaptureGroup<
                char,
                crate::matcher::QuantifierNOrMore<
                    char,
                    crate::matcher::ClassMatcher<
                        char,
                        EmailScalarClass0,
                    >,
                    1usize,
                >,
                1usize,
            >,
            crate::matcher::Then<
                char,
                crate::matcher::LiteralMatcher<EmailLiteral0>,
                crate::matcher::CaptureGroup<
                    char,
                    crate::matcher::Then<
                        char,
                        crate::matcher::CaptureGroup<
                            char,
                            crate::matcher::QuantifierNOrMore<
                                char,
                                crate::matcher::ClassMatcher<
                                    char,
                                    EmailScalarClass1,
                                >,
                                1usize,
                            >,
                            3usize,
                        >,
                        crate::matcher::QuantifierNToM<
                            char,
                            crate::matcher::CaptureGroup<
                                char,
                                crate::matcher::Then<
                                    char,
                                    crate::matcher::LiteralMatcher<
                                        EmailLiteral1,
                                    >,
                                    crate::matcher::QuantifierNOrMore<
                                        char,
                                        crate::matcher::ClassMatcher<
                                            char,
                                            EmailScalarClass2,
                                        >,
                                        1usize,
                                    >,
                                >,
                                4usize,
                            >,
                            0usize,
                            1usize,
                        >,
                    >,
                    2usize,
                >,
            >,
        >;
        type Anchors = crate::anchor::MinLen<3usize>;
        type Capture<'a, S: crate::haystack::HaystackSlice<'a>> = EmailCapture<
            'a,
            S,
            true,
        >;
    }
    impl ::std::fmt::Debug for Email {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(
                format_args!(
                    "/{0:?}/",
                    <Self as crate::expr::Regex<
                        char,
                        5,
                    >>::Pattern::default(),
                ),
            )
        }
    }
    /**A macro-generated type that holds 5 captures for the associated regex, [`Email`]. If present, named groups can be retrieved through their associated method.

Capture types include methods to retrieve individual captures as a slice of the original haystack, or as the underlying [`Range<usize>`](::std::range::Range), which can be used to manually slice the haystack (without risk of panicking in the case of `&str`).

As is common with regular expressions, group `0` refers to the whole match (and is therefore aliased as [`EmailCapture::whole_match`]).*/
    #[derive(Debug, Clone)]
    pub struct EmailCapture<
        'a,
        S: crate::haystack::HaystackSlice<'a>,
        const UTF8: bool,
    >(
        S,
        ::std::ops::Range<usize>,
        ::std::ops::Range<usize>,
        ::std::ops::Range<usize>,
        ::std::ops::Range<usize>,
        ::std::option::Option<::std::ops::Range<usize>>,
        ::std::marker::PhantomData<&'a ()>,
    );
    impl<
        'a,
        S: crate::haystack::HaystackSlice<'a>,
        const U: bool,
    > EmailCapture<'a, S, U> {
        pub fn cap_0(&self) -> CaptureGroup<'a, S, U> {
            CaptureGroup::new(
                self.0.slice_with(self.cap_0_range())
            )
        }
        pub fn cap_0_range(&self) -> ::std::ops::Range<usize> {
            self.1.clone()
        }
        pub fn cap_1(&self) -> S {
            self.0.slice_with(self.cap_1_range())
        }
        pub fn cap_1_range(&self) -> ::std::ops::Range<usize> {
            self.2.clone()
        }
        pub fn cap_2(&self) -> S {
            self.0.slice_with(self.cap_2_range())
        }
        pub fn cap_2_range(&self) -> ::std::ops::Range<usize> {
            self.3.clone()
        }
        pub fn cap_3(&self) -> S {
            self.0.slice_with(self.cap_3_range())
        }
        pub fn cap_3_range(&self) -> ::std::ops::Range<usize> {
            self.4.clone()
        }
        pub fn cap_4(&self) -> ::std::option::Option<S> {
            self.cap_4_range().map(|r| self.0.slice_with(r))
        }
        pub fn cap_4_range(
            &self,
        ) -> ::std::option::Option<::std::ops::Range<usize>> {
            self.5.clone()
        }
        pub fn whole_match(&self) -> S {
            self.0.slice_with(self.whole_match_range())
        }
        pub fn whole_match_range(&self) -> ::std::ops::Range<usize> {
            self.1.clone()
        }
        pub fn domain(&self) -> S {
            self.0.slice_with(self.domain_range())
        }
        pub fn domain_range(&self) -> ::std::ops::Range<usize> {
            self.3.clone()
        }
    }
    impl<
        'a,
        S: crate::haystack::HaystackSlice<'a>,
        const U: bool,
    > crate::expr::Capture<'a, S> for EmailCapture<'a, S, U> {
        fn whole_match_range(&self) -> ::std::ops::Range<usize> {
            self.1.clone()
        }
        fn whole_match(&self) -> S {
            self.0.slice_with(self.whole_match_range())
        }
    }
    impl<
        'a,
        S: crate::haystack::HaystackSlice<'a>,
        const U: bool,
    > crate::expr::FromRanges<'a, S, 5> for EmailCapture<'a, S, U> {
        fn from_ranges(
            ranges: [::std::option::Option<::std::ops::Range<usize>>; 5],
            hay: S,
        ) -> ::std::option::Option<Self> {
            let [c0, c1, c2, c3, c4] = ranges;
            ::std::option::Option::Some(
                Self(hay, c0?, c1?, c2?, c3?, c4, ::std::marker::PhantomData),
            )
        }
    }
}
#[doc(inline)]
#[allow(unused)]
pub use __regex_Email::{Email, EmailCapture};
