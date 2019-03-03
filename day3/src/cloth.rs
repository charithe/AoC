pub struct Cloth {
    repr: Vec<Vec<u8>>,
}

impl Cloth {
    pub fn of_size(width: usize, height: usize) -> Cloth {
        let mut columns: Vec<Vec<u8>> = Vec::with_capacity(height);
        for _i in 0..height {
            let mut row: Vec<u8> = Vec::with_capacity(width);
            row.resize(width, 0);
            columns.push(row);
        }

        Cloth { repr: columns }
    }

    pub fn add_claim(&mut self, x: u32, y: u32, width: u32, height: u32) {
        for i in y..(y + height) {
            if let Some(row) = self.repr.get_mut(i as usize) {
                for j in x..(x + width) {
                    if let Some(elem) = row.get_mut(j as usize) {
                        *elem += 1
                    }
                }
            }
        }
    }

    pub fn find_overlaps(&self) -> u64 {
        self.repr.iter().fold(0, |acc, x| {
            let overlaps = x.iter().filter(|&v| *v > 1.into()).count();
            acc + overlaps as u64
        })
    }

    pub fn has_overlapped(&self, x: u32, y: u32, width: u32, height: u32) -> bool {
        let rows = &self.repr[y as usize..(y + height) as usize];
        rows.iter().all(|row| {
            let cols = &row[x as usize..(x + width) as usize];
            cols.iter().all(|&col| col == 1)
        })
    }
}
