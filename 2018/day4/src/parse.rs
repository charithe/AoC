use nom::types::CompleteStr;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    StartShift(u32),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub event: Event,
}

fn to_uint<T>(input: CompleteStr) -> Result<T, T::Err>
where
    T: FromStr,
{
    input.parse::<T>()
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(num_u8<CompleteStr, u8>, map_res!(take_while!(is_digit), to_uint::<u8>));

named!(num_u16<CompleteStr, u16>, map_res!(take_while!(is_digit), to_uint::<u16>));

named!(num_u32<CompleteStr, u32>, map_res!(take_while!(is_digit), to_uint::<u32>));

named!(event_start_shift<CompleteStr, Event>,
       ws!(do_parse!(
            tag!("Guard") >>
            tag!("#") >>
            id: num_u32 >>
            tag!("begins") >>
            tag!("shift") >>
            (Event::StartShift(id))
       ))
);

named!(event_fall_asleep<CompleteStr, Event>,
       ws!(do_parse!(
               tag!("falls") >>
               tag!("asleep") >>
               (Event::FallAsleep)
       ))
);

named!(event_wake_up<CompleteStr, Event>,
       ws!(do_parse!(
               tag!("wakes") >>
               tag!("up") >>
               (Event::WakeUp)
       ))
);

named!(parse_entry<CompleteStr, Entry>,
       do_parse!(
               tag!("[") >>
               year: num_u16 >>
               tag!("-") >>
               month: num_u8 >>
               tag!("-") >>
               day: num_u8 >>
               tag!(" ") >>
               hour: num_u8 >>
               tag!(":") >>
               minute: num_u8 >>
               tag!("]") >>
               tag!(" ") >>
               event: alt!(event_start_shift | event_fall_asleep | event_wake_up) >>
               (Entry{ year, month, day, hour, minute, event })
       )
);

pub fn entry(line: &str) -> Option<Entry> {
    parse_entry(CompleteStr(line)).ok().map(|x| x.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shift_start() {
        assert_eq!(
            parse_entry(CompleteStr("[1518-03-19 00:02] Guard #647 begins shift")),
            Ok((
                CompleteStr(""),
                Entry {
                    year: 1518,
                    month: 3,
                    day: 19,
                    hour: 0,
                    minute: 2,
                    event: Event::StartShift(647),
                }
            ))
        )
    }

    #[test]
    fn test_parse_fall_asleep() {
        assert_eq!(
            parse_entry(CompleteStr("[1518-11-04 00:24] falls asleep")),
            Ok((
                CompleteStr(""),
                Entry {
                    year: 1518,
                    month: 11,
                    day: 4,
                    hour: 0,
                    minute: 24,
                    event: Event::FallAsleep,
                }
            ))
        )
    }

    #[test]
    fn test_parse_wake_up() {
        assert_eq!(
            parse_entry(CompleteStr("[1518-05-07 00:26] wakes up")),
            Ok((
                CompleteStr(""),
                Entry {
                    year: 1518,
                    month: 5,
                    day: 7,
                    hour: 0,
                    minute: 26,
                    event: Event::WakeUp,
                }
            ))
        )
    }
}
