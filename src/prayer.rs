use crate::calculations::AstronomicalMeasures;
use crate::event::Event;
use crate::Config;
use chrono::{Days, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

// #[derive(PartialEq)]
pub struct Prayer {
    event: Event,
    date: NaiveDateTime,
    measures: AstronomicalMeasures,
    config: Config,
}
impl PartialEq for Prayer {
    fn eq(&self, other: &Self) -> bool {
        self.event() == other.event() && self.date_time() == other.date_time()
    }
}
impl Prayer {
    fn new_from_measures(event: Event, measures: AstronomicalMeasures, config: &Config) -> Prayer {
        Self {
            event,
            date: measures.date_time(event),
            measures,
            config: config.clone(),
        }
    }
    pub fn new(event: Event, date: NaiveDate, config: &Config) -> Prayer {
        let measures = AstronomicalMeasures::new(date, config);
        Self::new_from_measures(event, measures, config)
    }

    fn new_from_date(&self, event: Event) -> Prayer {
        Self::new_from_measures(event, self.measures.clone(), &self.config)
    }

    pub fn event(&self) -> Event {
        self.event
    }
    pub fn date_time(&self) -> NaiveDateTime {
        self.date
    }
    pub fn date(&self) -> NaiveDate {
        self.date.date()
    }
    pub fn time(&self) -> NaiveTime {
        self.date.time()
    }

    pub fn previous(&self) -> Prayer {
        let previous_prayer = self.new_from_date(self.event.previous());
        if previous_prayer.time() <= self.time() {
            return previous_prayer;
        }

        let previous_date = self
            .date()
            .checked_sub_days(Days::new(1))
            .expect("Overflow when subtracting days");
        Self::new(self.event.previous(), previous_date, &self.config)
    }

    pub fn next(&self) -> Prayer {
        let next_prayer = self.new_from_date(self.event.next());
        if next_prayer.date_time().time() >= self.date_time().time() {
            return next_prayer;
        }

        let next_date = self
            .date()
            .checked_add_days(Days::new(1))
            .expect("Overflow when adding days");
        Self::new(self.event.next(), next_date, &self.config)
    }

    // Returns the time remaining for the next prayer to happen
    pub fn time_remaining(&self) -> Duration {
        let duration = self
            .date_time()
            .signed_duration_since(Local::now().naive_local());

        // The time remaining should not be negative
        if duration < Duration::zero() {
            return Duration::zero();
        }
        duration
    }

    // Returns true if the time of the prayer passed
    pub fn time_has_passed(&self) -> bool {
        self.time_remaining() <= Duration::zero()
    }

    pub fn text_duration(&self) -> String {
        let time_remaining = self.time_remaining();
        let in_or_since = if self.time_has_passed() {
            "since"
        } else {
            "in"
        };

        format!(
            "{} {in_or_since} {:02}H{:02}",
            self.event(),
            time_remaining.num_hours(),
            time_remaining.num_minutes() % 60
        )
    }
    pub fn text_time(&self) -> String {
        format!("{} at {}", self.event(), self.time())
        // format!("{} at {} the {}", self.event(), self.time(), self.date())
    }
}
