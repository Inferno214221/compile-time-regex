use std::ops::ControlFlow::*;

use super::*;
use crate::haystack::{Haystack, IntoHaystack};

mod start {
    use super::*;

    #[test]
    fn assert() {
        let mut hay = "hay".into_haystack();
        assert_eq!(Start::assert(&hay), Continue(true));
        hay.progress();
        assert_eq!(Start::assert(&hay), Break(()));
        hay.progress();
        assert_eq!(Start::assert(&hay), Break(()));
        hay.rollback(0);
        assert_eq!(Start::assert(&hay), Continue(true));
    }

    #[test]
    fn assert_fixed() {
        let mut hay = "hay".into_haystack();
        assert!(Start::assert_fixed(&hay));
        hay.progress();
        assert!(Start::assert_fixed(&hay));
        hay.progress();
        assert!(Start::assert_fixed(&hay));
    }
}

mod min_len {
    use super::*;

    #[test]
    fn assert() {
        let mut hay = "hay".into_haystack();
        assert_eq!(MinLen::<2>::assert(&hay), Continue(true));
        assert_eq!(MinLen::<3>::assert(&hay), Continue(true));
        assert_eq!(MinLen::<4>::assert(&hay), Break(()));
        hay.progress();
        assert_eq!(MinLen::<2>::assert(&hay), Continue(true));
        assert_eq!(MinLen::<3>::assert(&hay), Break(()));
        hay.progress();
        hay.progress();
        assert_eq!(MinLen::<1>::assert(&hay), Break(()));
    }

    #[test]
    fn assert_fixed() {
        let mut hay = "hay".into_haystack();
        assert!(MinLen::<2>::assert_fixed(&hay));
        assert!(MinLen::<3>::assert_fixed(&hay));
        assert!(!MinLen::<4>::assert_fixed(&hay));
        hay.progress();
        assert!(MinLen::<2>::assert_fixed(&hay));
        assert!(!MinLen::<3>::assert_fixed(&hay));
        hay.progress();
        hay.progress();
        assert!(!MinLen::<1>::assert_fixed(&hay));
    }
}

mod max_len {
    use super::*;

    #[test]
    fn assert() {
        let mut hay = "hay".into_haystack();
        assert_eq!(MaxLen::<1>::assert(&hay), Continue(true));
        assert_eq!(MaxLen::<2>::assert(&hay), Continue(true));
        assert_eq!(MaxLen::<3>::assert(&hay), Continue(true));
        hay.progress();
        assert_eq!(MaxLen::<1>::assert(&hay), Continue(true));
        assert_eq!(MaxLen::<2>::assert(&hay), Continue(true));
        hay.progress();
        hay.progress();
        assert_eq!(MaxLen::<1>::assert(&hay), Continue(true));
    }

    #[test]
    fn assert_fixed() {
        let mut hay = "hay".into_haystack();
        assert!(!MaxLen::<1>::assert_fixed(&hay));
        assert!(!MaxLen::<2>::assert_fixed(&hay));
        assert!(MaxLen::<3>::assert_fixed(&hay));
        hay.progress();
        assert!(!MaxLen::<1>::assert_fixed(&hay));
        assert!(MaxLen::<2>::assert_fixed(&hay));
        hay.progress();
        hay.progress();
        assert!(MaxLen::<1>::assert_fixed(&hay));
    }
}

mod end_and_max_len {
    use super::*;

    #[test]
    fn assert() {
        let mut hay = "hay".into_haystack();
        assert_eq!(EndAndMaxLen::<1>::assert(&hay), Continue(false));
        assert_eq!(EndAndMaxLen::<2>::assert(&hay), Continue(false));
        assert_eq!(EndAndMaxLen::<3>::assert(&hay), Continue(true));
        hay.progress();
        assert_eq!(EndAndMaxLen::<1>::assert(&hay), Continue(false));
        assert_eq!(EndAndMaxLen::<2>::assert(&hay), Continue(true));
        hay.progress();
        hay.progress();
        assert_eq!(EndAndMaxLen::<1>::assert(&hay), Continue(true));
    }

    #[test]
    fn assert_fixed() {
        let mut hay = "hay".into_haystack();
        assert!(!EndAndMaxLen::<1>::assert_fixed(&hay));
        assert!(!EndAndMaxLen::<2>::assert_fixed(&hay));
        assert!(EndAndMaxLen::<3>::assert_fixed(&hay));
        hay.progress();
        assert!(!EndAndMaxLen::<1>::assert_fixed(&hay));
        assert!(EndAndMaxLen::<2>::assert_fixed(&hay));
        hay.progress();
        hay.progress();
        assert!(EndAndMaxLen::<1>::assert_fixed(&hay));
    }
}

mod pair {
    use super::*;

    type Min2Max3 = AnchorPair<MinLen<2>, EndAndMaxLen<3>>;
    type Strict2 = AnchorPair<MinLen<2>, EndAndMaxLen<2>>;
    type StartEnd2 = AnchorPair<Start, EndAndMaxLen<2>>;

    #[test]
    fn assert() {
        let mut hay = "hay".into_haystack();
        assert_eq!(Min2Max3::assert(&hay), Continue(true));
        assert_eq!(Strict2::assert(&hay), Continue(false));
        assert_eq!(StartEnd2::assert(&hay), Continue(false));
        hay.progress();
        assert_eq!(Min2Max3::assert(&hay), Continue(true));
        assert_eq!(Strict2::assert(&hay), Continue(true));
        assert_eq!(StartEnd2::assert(&hay), Break(()));
        hay.progress();
        assert_eq!(Min2Max3::assert(&hay), Break(()));
        assert_eq!(Strict2::assert(&hay), Break(()));
        assert_eq!(StartEnd2::assert(&hay), Break(()));
    }

    #[test]
    fn assert_fixed() {
        let mut hay = "hay".into_haystack();
        assert!(Min2Max3::assert_fixed(&hay));
        assert!(!Strict2::assert_fixed(&hay));
        assert!(!StartEnd2::assert_fixed(&hay));
        hay.progress();
        assert!(Min2Max3::assert_fixed(&hay));
        assert!(Strict2::assert_fixed(&hay));
        assert!(StartEnd2::assert_fixed(&hay));
        hay.progress();
        assert!(!Min2Max3::assert_fixed(&hay));
        assert!(!Strict2::assert_fixed(&hay));
        assert!(StartEnd2::assert_fixed(&hay));
    }
}

mod set {
    use super::*;

    type StartMin2Max3 = AnchorSet<Start, MinLen<2>, MaxLen<3>>;
    type StartMin2EndMax3 = AnchorSet<Start, MinLen<2>, EndAndMaxLen<3>>;
    type StartMin1EndMax2 = AnchorSet<Start, MinLen<1>, EndAndMaxLen<2>>;

    #[test]
    fn assert() {
        let mut hay = "hay".into_haystack();
        assert_eq!(StartMin2Max3::assert(&hay), Continue(true));
        assert_eq!(StartMin2EndMax3::assert(&hay), Continue(true));
        assert_eq!(StartMin1EndMax2::assert(&hay), Continue(false));
        hay.progress();
        assert_eq!(StartMin2Max3::assert(&hay), Break(()));
        assert_eq!(StartMin2EndMax3::assert(&hay), Break(()));
        assert_eq!(StartMin1EndMax2::assert(&hay), Break(()));
        hay.progress();
        assert_eq!(StartMin2Max3::assert(&hay), Break(()));
        assert_eq!(StartMin2EndMax3::assert(&hay), Break(()));
        assert_eq!(StartMin1EndMax2::assert(&hay), Break(()));

        let mut hay = "ha".into_haystack();
        assert_eq!(StartMin2Max3::assert(&hay), Continue(true));
        assert_eq!(StartMin2EndMax3::assert(&hay), Continue(true));
        assert_eq!(StartMin1EndMax2::assert(&hay), Continue(true));
        hay.progress();
        assert_eq!(StartMin2Max3::assert(&hay), Break(()));
        assert_eq!(StartMin2EndMax3::assert(&hay), Break(()));
        assert_eq!(StartMin1EndMax2::assert(&hay), Break(()));
    }

    #[test]
    fn assert_fixed() {
        let mut hay = "hay".into_haystack();
        assert!(StartMin2Max3::assert_fixed(&hay));
        assert!(StartMin2EndMax3::assert_fixed(&hay));
        assert!(!StartMin1EndMax2::assert_fixed(&hay));
        hay.progress();
        assert!(StartMin2Max3::assert_fixed(&hay));
        assert!(StartMin2EndMax3::assert_fixed(&hay));
        assert!(StartMin1EndMax2::assert_fixed(&hay));
        hay.progress();
        assert!(!StartMin2Max3::assert_fixed(&hay));
        assert!(!StartMin2EndMax3::assert_fixed(&hay));
        assert!(StartMin1EndMax2::assert_fixed(&hay));

        let mut hay = "ha".into_haystack();
        assert!(StartMin2Max3::assert_fixed(&hay));
        assert!(StartMin2EndMax3::assert_fixed(&hay));
        assert!(StartMin1EndMax2::assert_fixed(&hay));
        hay.progress();
        assert!(!StartMin2Max3::assert_fixed(&hay));
        assert!(!StartMin2EndMax3::assert_fixed(&hay));
        assert!(StartMin1EndMax2::assert_fixed(&hay));
    }
}
