use core::ops::{Bound, RangeBounds as _};

use crate::interval::{Endpoint, Interval};

pub trait IntervalLike<K> {
    fn left(&self) -> Endpoint<&K>;

    fn right(&self) -> Endpoint<&K>;

    fn into_interval(self) -> Interval<K>;

    fn into_range_bounds(self) -> (Bound<K>, Bound<K>)
    where
        Self: Sized,
    {
        let interval = self.into_interval();
        let start = interval.left.into_range_bound();
        let end = interval.right.into_range_bound();
        (start, end)
    }
}

impl<K> IntervalLike<K> for (Endpoint<K>, Endpoint<K>) {
    fn left(&self) -> Endpoint<&K> {
        self.0.as_ref()
    }

    fn right(&self) -> Endpoint<&K> {
        self.1.as_ref()
    }

    fn into_interval(self) -> Interval<K> {
        let (left, right) = self;
        Interval { left, right }
    }
}

macro_rules! impl_for_range {
    ($ty:ty; $left_endpoint:ident $( ($left:ident) )?, $right_endpoint:ident $( ($right:ident) )?) => {
        impl_for_range!(
            $ty;
            |self| Interval {
                left: impl_for_range!(@endpoint self $left_endpoint $( ( $left ) )? ),
                right: impl_for_range!(@endpoint self $right_endpoint $( ( $right ) )? ),
            }
        );
    };

    ($ty:ty; |$self:ident| $into_interval:expr) => {
        impl<K> IntervalLike<K> for $ty {
            fn left(&self) -> Endpoint<&K> {
                self.start_bound().into()
            }

            fn right(&self) -> Endpoint<&K> {
                self.end_bound().into()
            }

            fn into_interval($self) -> Interval<K> {
                $into_interval
            }
        }
    };

    (@endpoint $self:ident $endpoint:ident $( ( $side:ident ) )?) => {
        Endpoint::$endpoint $(($self.$side))?
    };
}

// a..b
impl_for_range!(core::range::Range<K>; Closed(start), Open(end));
impl_for_range!(core::ops::Range<K>; Closed(start), Open(end));

// a..
impl_for_range!(core::range::RangeFrom<K>; Closed(start), Infinity);
impl_for_range!(core::ops::RangeFrom<K>; Closed(start), Infinity);

// ..
impl_for_range!(core::range::RangeFull; Infinity, Infinity);

// a..=b
impl_for_range!(core::range::RangeInclusive<K>; Closed(start), Closed(last));
impl_for_range!(core::ops::RangeInclusive<K>; |self| {
    let (left, right) = self.into_inner();
    Interval {
        left: Endpoint::Closed(left),
        right: Endpoint::Closed(right),
    }
});

// ..b
impl_for_range!(core::range::RangeTo<K>; Infinity, Open(end));

// ..=b
impl_for_range!(core::range::RangeToInclusive<K>; Infinity, Closed(last));
impl_for_range!(core::ops::RangeToInclusive<K>; Infinity, Closed(end));
