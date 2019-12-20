const START: u32 = 137683;
const END: u32 = 596253;

fn main() {
    {
        let mut code = Code::from(START);
        let count = code.count_valid_combinations(END, is_valid_part1);
        println!("Part1: {}", count);
    }

    {
        let mut code = Code::from(START);
        let count = code.count_valid_combinations(END, is_valid_part2);
        println!("Part2: {}", count);
    }
}

#[derive(Debug)]
struct Code {
    digits: Vec<u8>,
}

impl Code {
    fn from(num: u32) -> Code {
        let mut digits: Vec<u8> = Vec::new();

        let mut divisor = 1;
        while num >= divisor * 10 {
            divisor *= 10;
        }

        let mut temp = num;
        while divisor > 0 {
            let d = temp / divisor;
            temp %= divisor;
            divisor /= 10;
            digits.push(d as u8);
        }

        Code { digits }
    }

    fn value(&self) -> u32 {
        let m = 10u32.pow((self.digits.len() - 1) as u32);
        self.digits
            .iter()
            .scan(m, |state, &d| {
                let x = d as u32 * (*state);
                *state = *state / 10;
                Some(x)
            })
            .sum()
    }

    fn count_valid_combinations<F>(&mut self, upper_limit: u32, is_valid: F) -> u32
    where
        F: Fn(&Vec<u8>) -> bool,
    {
        self.move_to_first_possible_code();

        let mut count = 0;
        while self.value() <= upper_limit {
            if is_valid(&self.digits) {
                count += 1;
            }

            let mut idx = self.digits.len() - 1;
            while self.digits[idx] + 1 > 9 {
                idx -= 1;
            }

            let v = self.digits[idx] + 1;
            for i in idx..self.digits.len() {
                self.digits[i] = v;
            }
        }

        count
    }

    fn move_to_first_possible_code(&mut self) {
        let mut prev = self.digits[0];
        let mut idx = 0;

        for i in 1..self.digits.len() {
            if self.digits[i] < prev {
                idx = i;
                break;
            }
            prev = self.digits[i];
        }

        for i in idx..self.digits.len() {
            self.digits[i] = prev;
        }
    }
}

fn is_valid_part1(digits: &Vec<u8>) -> bool {
    let mut seen_run = false;
    let mut prev = digits[0];

    for i in 1..digits.len() {
        if prev > digits[i] {
            return false;
        }

        if prev == digits[i] {
            seen_run = true;
        }

        prev = digits[i]
    }

    seen_run
}

fn is_valid_part2(digits: &Vec<u8>) -> bool {
    let mut prev = digits[0];
    let mut curr_group_size = 1;
    let mut seen_valid_run = false;

    for i in 1..digits.len() {
        if prev > digits[i] {
            return false;
        }

        if prev == digits[i] {
            curr_group_size += 1;
        } else {
            if curr_group_size == 2 {
                seen_valid_run = true;
            }
            curr_group_size = 1;
        }

        prev = digits[i]
    }

    if curr_group_size == 2 {
        seen_valid_run = true
    }

    seen_valid_run
}
