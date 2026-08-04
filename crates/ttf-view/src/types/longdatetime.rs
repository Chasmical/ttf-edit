use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use std::fmt;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct LongDateTime([u8; 8]);

const EPOCH_NAIVE: NaiveDateTime =
    NaiveDate::from_ymd_opt(1904, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();

impl LongDateTime {
    pub const EPOCH: DateTime<Utc> = EPOCH_NAIVE.and_utc();

    pub const fn new(datetime: DateTime<Utc>) -> Self {
        let delta = datetime.naive_utc().signed_duration_since(EPOCH_NAIVE);
        Self::from_epoch_seconds(delta.num_seconds())
    }
    pub const fn datetime(&self) -> Option<DateTime<Utc>> {
        let delta = TimeDelta::try_seconds(self.epoch_seconds())?;
        Some(EPOCH_NAIVE.checked_add_signed(delta)?.and_utc())
    }

    pub const fn from_epoch_seconds(secs: i64) -> Self {
        Self(i64::to_be_bytes(secs))
    }
    pub const fn epoch_seconds(&self) -> i64 {
        i64::from_be_bytes(self.0)
    }

    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0
    }
}

impl fmt::Debug for LongDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.datetime() {
            Some(datetime) => datetime.fmt(f),
            // TODO: maybe try to represent the out-of-range date?
            None => write!(f, "{:#010X}", self.epoch_seconds()),
        }
    }
}

// TODO: When [u8; 8]'s Default is constified, replace with #[derive_const]
#[allow(clippy::derivable_impls)]
const impl Default for LongDateTime {
    fn default() -> Self {
        Self([0; 8])
    }
}

// Note: PartialEq + Eq impls need to be explicit, because [u8; 8] is compared unsignedly.
const impl PartialOrd for LongDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
const impl Ord for LongDateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.epoch_seconds().cmp(&other.epoch_seconds())
    }
}

const impl From<DateTime<Utc>> for LongDateTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self::new(value)
    }
}
const impl TryFrom<LongDateTime> for DateTime<Utc> {
    type Error = ();
    fn try_from(value: LongDateTime) -> Result<Self, Self::Error> {
        value.datetime().ok_or(())
    }
}
