use std::collections::HashMap;

struct Log {
    total_minutes: u64,
    minute_count: HashMap<u8, u16>,
}

impl Log {
    fn new() -> Self {
        Log {
            total_minutes: 0,
            minute_count: HashMap::new(),
        }
    }

    fn mark_asleep(&mut self, start: u8, end: u8) {
        for i in start..end {
            self.total_minutes += 1;
            let count = self.minute_count.entry(i).or_insert(0);
            *count += 1;
        }
    }

    fn most_asleep_minute_and_count(&self) -> (&u8, &u16) {
        self.minute_count.iter().max_by_key(|x| x.1).unwrap()
    }
}

pub struct Schedule {
    schedule: HashMap<u32, Log>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            schedule: HashMap::new(),
        }
    }

    pub fn mark_asleep(&mut self, guard: u32, from: u8, to: u8) {
        let guard_log = self.schedule.entry(guard).or_insert(Log::new());
        guard_log.mark_asleep(from, to);
    }

    pub fn longest_sleeper_and_minute(&self) -> Option<(u32, u8)> {
        self.schedule
            .iter()
            .max_by_key(|x| x.1.total_minutes)
            .map(|(id, log)| (*id, *log.most_asleep_minute_and_count().0))
    }

    pub fn habitual_sleeper_and_minute(&self) -> Option<(u32, u8)> {
        self.schedule
            .iter()
            .map(|(id, log)| {
                let (minute, count) = log.most_asleep_minute_and_count();
                (id, minute, count)
            })
            .max_by_key(|x| x.2)
            .map(|(id, minute, _count)| (*id, *minute))
    }
}
