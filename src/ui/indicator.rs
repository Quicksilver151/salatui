use crate::*;

/// passive timeline char, a thin line hugging the left cell edge so it aligns
/// with the marker triangles' straight edge
pub const AXIS: &str = "▏";
/// top half of the fused arrow, sits on the current prayer row
pub const MARKER_CURRENT: &str = "◣";
/// bottom half of the fused arrow, sits on the next prayer row
pub const MARKER_NEXT: &str = "◤";
/// single Powerline arrow shown while a prayer is inside the notification
/// window (its `>` shape points at the prayer's row)
pub const MARKER_SINGLE: &str = "\u{E0B0}";

/// rows of the prayer list: [Fajr, Sun, Dhuhur, Asr, Magrib, Isha]
pub const PRAYER_ROW_COUNT: usize = 6;

/// the two rows hosting the marker arrow, `current` on top / `next` below
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    pub current: usize,
    pub next: usize,
}

/// where the marker arrow(s) go for the configured indicator variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorMarker {
    /// fused `◣`/`◤` arrow marking the current→next boundary
    Pair(Pair),
    /// lone arrow pointing at the prayer inside the notification window
    Single(usize),
}

/// what to draw in the menu for the configured indicator variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IndicatorData {
    /// row to reverse-highlight, if any
    pub highlight: Option<usize>,
    /// rows hosting the arrow(s), if any
    pub marker: Option<IndicatorMarker>,
}

/// compute the indicator for `now_min` (minutes since local midnight).
///
/// `upcoming` is the first prayer strictly after `now_min` (mirrors
/// [`PrayerTimes::get_current_index`]); `last_passed` wraps so before Fajr and
/// after Isha it lands on Isha.
///
/// The `Inbetween` marker is the fused current→next arrow, wrapping to the
/// overnight boundary before Fajr / after Isha. While any prayer sits within
/// `window_min` minutes of `now_min` (the absolute notification offset), it
/// collapses to a single `MARKER_SINGLE` arrow on the nearest such prayer
/// (offset 0 => only exactly on the prayer's minute).
pub fn indicator(
    mode: TimeIndicator,
    times: &PrayerTimes,
    now_min: u32,
    window_min: u32,
) -> IndicatorData {
    let upcoming = times
        .to_vec()
        .into_iter()
        .position(|t| t > now_min)
        .unwrap_or(0);
    let last_passed = (upcoming + 5) % PRAYER_ROW_COUNT;

    match mode {
        TimeIndicator::Empty => IndicatorData { highlight: None, marker: None },
        TimeIndicator::Current => IndicatorData { highlight: Some(last_passed), marker: None },
        TimeIndicator::Next => IndicatorData { highlight: Some(upcoming), marker: None },
        TimeIndicator::Inbetween => IndicatorData {
            highlight: None,
            marker: Some(match prayers_within(times, now_min, window_min) {
                Some(row) => IndicatorMarker::Single(row),
                None => IndicatorMarker::Pair(Pair { current: last_passed, next: upcoming }),
            }),
        },
    }
}

/// nearest prayer within `window` minutes of `now_min` (first row wins ties)
fn prayers_within(times: &PrayerTimes, now_min: u32, window: u32) -> Option<usize> {
    times
        .to_vec()
        .into_iter()
        .enumerate()
        .filter(|&(_, t)| now_min.abs_diff(t) <= window)
        .min_by_key(|&(_, t)| now_min.abs_diff(t))
        .map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::*;

    // from_vec reads prayers from list[2..=7]: Fajr 293, Sun 365, Dhuhur 736,
    // Asr 932, Magrib 1098, Isha 1171
    fn times() -> PrayerTimes {
        PrayerTimes::from_vec(vec![77, 225, 293, 365, 736, 932, 1098, 1171])
    }

    #[test]
    fn empty_has_no_marker_or_highlight() {
        let data = indicator(TimeIndicator::Empty, &times(), 800, 0);
        assert_eq!(data, IndicatorData { highlight: None, marker: None });
    }

    #[test]
    fn current_highlights_last_passed_at_midday() {
        // 13:20 -> Dhuhur(736) passed, Asr(932) upcoming => current is Dhuhur
        let data = indicator(TimeIndicator::Current, &times(), 800, 0);
        assert_eq!(data.highlight, Some(2));
        assert_eq!(data.marker, None);
    }

    #[test]
    fn next_highlights_upcoming_at_midday() {
        let data = indicator(TimeIndicator::Next, &times(), 800, 0);
        assert_eq!(data.highlight, Some(3));
        assert_eq!(data.marker, None);
    }

    #[test]
    fn inbetween_marks_current_and_next_at_midday() {
        let data = indicator(TimeIndicator::Inbetween, &times(), 800, 0);
        assert_eq!(data.highlight, None);
        assert_eq!(data.marker, Some(IndicatorMarker::Pair(Pair { current: 2, next: 3 })));
    }

    #[test]
    fn before_fajr_wraps_to_overnight_boundary() {
        // 01:40 < Fajr(293) => upcoming 0, last_passed Isha(5)
        assert_eq!(indicator(TimeIndicator::Current, &times(), 100, 0).highlight, Some(5));
        let data = indicator(TimeIndicator::Inbetween, &times(), 100, 0);
        assert_eq!(data.highlight, None);
        assert_eq!(data.marker, Some(IndicatorMarker::Pair(Pair { current: 5, next: 0 })));
        assert_eq!(indicator(TimeIndicator::Next, &times(), 100, 0).highlight, Some(0));
    }

    #[test]
    fn after_isha_wraps_to_overnight_boundary() {
        // 21:40 > Isha(1171) => upcoming 0
        assert_eq!(indicator(TimeIndicator::Current, &times(), 1300, 0).highlight, Some(5));
        let data = indicator(TimeIndicator::Inbetween, &times(), 1300, 0);
        assert_eq!(data.highlight, None);
        assert_eq!(data.marker, Some(IndicatorMarker::Pair(Pair { current: 5, next: 0 })));
        assert_eq!(indicator(TimeIndicator::Next, &times(), 1300, 0).highlight, Some(0));
    }

    #[test]
    fn exact_minute_shows_single_arrow() {
        // 12:16 == Dhuhur(736) => window 0 still matches
        let data = indicator(TimeIndicator::Inbetween, &times(), 736, 0);
        assert_eq!(data.highlight, None);
        assert_eq!(data.marker, Some(IndicatorMarker::Single(2)));
        // one minute later the exact-match window no longer contains Dhuhur
        let data = indicator(TimeIndicator::Inbetween, &times(), 737, 0);
        assert_eq!(data.marker, Some(IndicatorMarker::Pair(Pair { current: 2, next: 3 })));
    }

    #[test]
    fn in_window_before_prayer_shows_single() {
        // 12:11, five minutes before Dhuhur(736) with window 5
        let data = indicator(TimeIndicator::Inbetween, &times(), 731, 5);
        assert_eq!(data.highlight, None);
        assert_eq!(data.marker, Some(IndicatorMarker::Single(2)));
    }

    #[test]
    fn in_window_after_prayer_shows_single() {
        // 12:21, five minutes after Dhuhur(736) with window 5
        let data = indicator(TimeIndicator::Inbetween, &times(), 741, 5);
        assert_eq!(data.highlight, None);
        assert_eq!(data.marker, Some(IndicatorMarker::Single(2)));
    }

    #[test]
    fn outside_window_keeps_pair() {
        // 13:20, neither Fajr nor Asr within 5 minutes
        let data = indicator(TimeIndicator::Inbetween, &times(), 800, 5);
        assert_eq!(data.highlight, None);
        assert_eq!(data.marker, Some(IndicatorMarker::Pair(Pair { current: 2, next: 3 })));
    }

    #[test]
    fn nearest_prayer_wins_when_windows_overlap() {
        // 05:30, 37 min to Fajr(293) and 35 min to Sun(365) with window 40
        let data = indicator(TimeIndicator::Inbetween, &times(), 330, 40);
        assert_eq!(data.marker, Some(IndicatorMarker::Single(1)));
    }
}