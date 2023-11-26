use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Deref, Sub};

#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BoundedFloat<const LOW: i16, const HIGH: i16> {
    num: f32,
}

impl<const LOW: i16, const HIGH: i16> Debug for BoundedFloat<LOW, HIGH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.?}", self.num)
    }
}

impl<const LOW: i16, const HIGH: i16> Display for BoundedFloat<LOW, HIGH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl<const LOW: i16, const HIGH: i16> Sub for BoundedFloat<LOW, HIGH> {
    type Output = f32;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

impl<const LOW: i16, const HIGH: i16> Add for BoundedFloat<LOW, HIGH> {
    type Output = f32;

    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}

impl<const LOW: i16, const HIGH: i16> Eq for BoundedFloat<LOW, HIGH> {}

impl<const LOW: i16, const HIGH: i16> Ord for BoundedFloat<LOW, HIGH> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("should always be successful")
    }
}

impl<const LOW: i16, const HIGH: i16> Hash for BoundedFloat<LOW, HIGH> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let raw_bytes = self.to_be_bytes();
        u32::from_be_bytes(raw_bytes).hash(state);
    }
}

impl<const LOW: i16, const HIGH: i16> Deref for BoundedFloat<LOW, HIGH> {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.num
    }
}

impl<const LOW: i16, const HIGH: i16> BoundedFloat<LOW, HIGH> {
    pub const MIN: BoundedFloat<LOW, HIGH> = BoundedFloat { num: LOW as f32 };

    pub const MAX: BoundedFloat<LOW, HIGH> = BoundedFloat { num: HIGH as f32 };

    /// Creates a bounded float value, if the given val is within the range given
    pub fn new(val: f32) -> Option<Self> {
        if val >= LOW as f32 && val <= HIGH as f32 {
            Some(Self { num: val })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::bounded_float::BoundedFloat;
    use std::collections::BTreeSet;

    #[test]
    fn bounded_floats() {
        BoundedFloat::<0, 1>::new(0.1).expect("is valid");
        assert!(BoundedFloat::<0, 1>::new(1.1).is_none());
    }

    #[test]
    fn bounded_floats_inclusive() {
        BoundedFloat::<0, 1>::new(0.).expect("is valid");
        BoundedFloat::<0, 1>::new(1.).expect("is valid");
    }

    #[test]
    fn bounded_float_rejects_nan() {
        assert!(BoundedFloat::<0, 1>::new(f32::NAN).is_none());
    }

    #[test]
    fn bounded_float_btree() {
        let mut map = BTreeSet::<BoundedFloat<0, 1>>::new();
        map.insert(BoundedFloat::new(0.7).unwrap());
        map.insert(BoundedFloat::new(0.03).unwrap());
        map.insert(BoundedFloat::new(0.9).unwrap());
        map.insert(BoundedFloat::new(0.8).unwrap());
        map.insert(BoundedFloat::new(0.2).unwrap());

        map.iter()
            .inspect(|f| println!("{f:#?}"))
            .try_fold(
                f32::MIN,
                |acc, &next| {
                    if *next > acc {
                        Ok(*next)
                    } else {
                        Err(())
                    }
                },
            )
            .expect("should be okay");
    }
}
