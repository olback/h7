use chrono::Datelike;

use {
    crate::globals,
    chrono::{NaiveDateTime, Timelike},
    cortex_m::interrupt,
    stm32h7xx_hal::{
        pac,
        rcc::backup,
        rcc::CoreClocks,
        rtc::{self, RtcClock},
    },
};

const DEFAULT_TIMESTAMP: embedded_sdmmc::Timestamp = embedded_sdmmc::Timestamp {
    year_since_1970: 0,
    zero_indexed_month: 0,
    zero_indexed_day: 0,
    hours: 0,
    minutes: 0,
    seconds: 0,
};

pub struct TimeSource;

impl TimeSource {
    pub fn rtc(
        rtc: pac::RTC,
        prec: backup::Rtc,
        clock_source: RtcClock,
        clocks: &CoreClocks,
    ) -> rtc::Rtc {
        rtc::Rtc::open_or_init(rtc, prec, clock_source, clocks)
    }

    pub fn set_date_time(dt: chrono::NaiveDateTime) -> Result<(), ()> {
        interrupt::free(|_| match unsafe { globals::RTC.as_mut() } {
            Some(rtc) => {
                rtc.set_date_time(dt);
                Ok(())
            }
            None => Err(()),
        })
    }

    pub fn get_date_time() -> Option<NaiveDateTime> {
        interrupt::free(|_| unsafe { globals::RTC.as_ref().map(|dt| dt.date_time()).flatten() })
    }
}

impl embedded_sdmmc::TimeSource for TimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        match { Self::get_date_time() } {
            Some(date_time) => embedded_sdmmc::Timestamp {
                year_since_1970: (date_time.year() - 1970) as u8,
                zero_indexed_month: date_time.month0() as u8,
                zero_indexed_day: date_time.day0() as u8,
                hours: date_time.hour() as u8,
                minutes: date_time.minute() as u8,
                seconds: date_time.second() as u8,
            },
            None => DEFAULT_TIMESTAMP,
        }
    }
}