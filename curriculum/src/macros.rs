/// A macro to create a course.
/// `$interval` is in days
/// `$duration` is in minutes
pub macro course(
    $name: literal,
    $place: literal,
    ($($tutor_name: literal),*$(,)?),
    $(
        (
            (
                $year: literal,
                $month: literal,
                $day: literal,
                $hour: literal,
                $minute: literal,
                $second: literal
            ),
            $num: literal,
            $interval: literal
        )
    ),+,
    $duration: literal
) {
    $crate::course::Course::builder()
        .name($name)
        .place($place)
        .tutors([$($crate::tutor::Tutor::builder().name($tutor_name).build()),*])
        .periods(
            [].into_iter()
                $(.chain({
                    let date = $crate::NaiveDate::from_ymd_opt($year, $month, $day).unwrap();
                    (0..$num)
                        .filter_map(|i| date.checked_add_days($crate::Days::new(i * $interval)))
                        .filter_map(|d| d.and_hms_opt($hour, $minute, $second))
                        .map(|ts| {
                            $crate::Period::builder()
                                .start(ts)
                                .duration($crate::TimeDelta::minutes($duration))
                                .build()
                        })
                        .collect::<Vec<_>>()
                }))+
        )
        .build()
}
