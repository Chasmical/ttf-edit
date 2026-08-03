use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use std::fmt;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LongDateTime([u8; 8]);

const impl Default for LongDateTime {
    fn default() -> Self {
        Self::new(Self::EPOCH)
    }
}

const EPOCH_NAIVE: NaiveDateTime =
    NaiveDate::from_ymd_opt(1904, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();

impl LongDateTime {
    pub const EPOCH: DateTime<Utc> = EPOCH_NAIVE.and_utc();

    pub const fn from_epoch_seconds(secs: i64) -> Self {
        Self(i64::to_be_bytes(secs))
    }
    pub const fn epoch_seconds(&self) -> i64 {
        i64::from_be_bytes(self.0)
    }

    pub const fn new(datetime: DateTime<Utc>) -> Self {
        let delta = datetime.naive_utc().signed_duration_since(EPOCH_NAIVE);
        Self::from_epoch_seconds(delta.num_seconds())
    }
    pub const fn to_datetime(&self) -> Option<DateTime<Utc>> {
        let delta = TimeDelta::try_seconds(self.epoch_seconds())?;
        Some(EPOCH_NAIVE.checked_add_signed(delta)?.and_utc())
    }
}

impl fmt::Debug for LongDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_datetime() {
            Some(datetime) => datetime.fmt(f),
            // TODO: maybe try to represent the out-of-range date?
            None => write!(f, "{:#X}", u64::from_be_bytes(self.0)),
        }
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
        value.to_datetime().ok_or(())
    }
}
