use aoc;
use num::Integer;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::f64::consts::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> aoc::Result<()> {
    let map = read_map("input")?;
    let station = find_station(&map);
    if let Some(asteroid) = find_nth_asteroid(station, &map, 200) {
        println!("200th: {}", asteroid.x * 100 + asteroid.y);
    }

    Ok(())
}

fn read_map(path: &str) -> aoc::Result<Vec<Asteroid>> {
    let input = File::open(path)?;
    let mut buffered = BufReader::new(input);

    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut y = 0;
    let mut buf = String::new();

    while let Ok(n) = buffered.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        asteroids.extend(buf.chars().enumerate().filter_map(|(x, c)| {
            if c == '#' {
                Some(Asteroid {
                    x: x as isize,
                    y: y as isize,
                })
            } else {
                None
            }
        }));

        buf.clear();
        y += 1;
    }

    Ok(asteroids)
}

fn find_station(asteroids: &Vec<Asteroid>) -> Asteroid {
    let mut max_visible = 0;
    let mut candidate = Asteroid { x: -1, y: -1 };

    for a in asteroids {
        let mut visible_asteroids: HashSet<Gradient> = HashSet::new();
        for b in asteroids {
            if a == b {
                continue;
            }

            let g = Gradient::new(a, b);
            visible_asteroids.insert(g);
        }

        let total_visible = visible_asteroids.len();
        if total_visible > max_visible {
            max_visible = total_visible;
            candidate = *a;
        }
    }

    println!("Max visible: {}", max_visible);
    candidate
}

fn find_nth_asteroid(station: Asteroid, asteroids: &Vec<Asteroid>, n: usize) -> Option<Asteroid> {
    let mut targets: HashMap<Gradient, BinaryHeap<Target>> = HashMap::new();

    for a in asteroids {
        if *a == station {
            continue;
        }

        let g = Gradient::new(&station, a);
        targets
            .entry(g)
            .or_insert(BinaryHeap::new())
            .push(Target::new(&station, a));
    }

    let mut target_list: Vec<Vec<Target>> = targets
        .values()
        .map(|h| h.clone().into_sorted_vec())
        .collect();

    target_list.sort_by(|a, b| {
        let aa = &a[0];
        let bb = &b[0];

        aa.angle.partial_cmp(&bb.angle).unwrap()
    });

    let mut counter = 0;
    let mut j = 0;
    let mut candidate: Option<Asteroid> = None;
    while counter < n {
        for i in 0..target_list.len() {
            if counter == n {
                break;
            }

            if let Some(t) = target_list[i].get(j) {
                counter += 1;
                candidate = Some(t.asteroid);
            }
        }

        j += 1;
    }

    candidate
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Gradient(isize, isize);

impl Gradient {
    fn new(a: &Asteroid, b: &Asteroid) -> Gradient {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let g = dx.gcd(&dy);
        Gradient(dx / g, dy / g)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Asteroid {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Target {
    asteroid: Asteroid,
    angle: f64,
    distance: isize,
}

impl Target {
    fn new(station: &Asteroid, asteroid: &Asteroid) -> Target {
        let (distance, angle) = Self::distance_and_angle(station, asteroid);
        Target {
            asteroid: *asteroid,
            distance,
            angle,
        }
    }

    fn distance_and_angle(station: &Asteroid, asteroid: &Asteroid) -> (isize, f64) {
        let x = asteroid.x - station.x;
        let y = station.y - asteroid.y;
        let dist = x.pow(2) + y.pow(2);

        let angle = (y as f64).atan2(x as f64);
        let angle = if angle >= 0.0 && angle <= FRAC_PI_2 {
            FRAC_PI_2 - angle
        } else if angle > FRAC_PI_2 && angle <= PI {
            (2.0 * PI) - (angle - FRAC_PI_2)
        } else {
            FRAC_PI_2 + (-angle)
        };

        (dist, angle)
    }
}

impl Ord for Target {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Target {}
