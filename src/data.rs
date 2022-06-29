use rand::prelude::*;

pub fn generate_data<T: Sized + Copy + From<i32>, const ATTRS: usize>(
    rows: u32,
) -> Vec<[T; ATTRS]> {
    let mut data: Vec<[T; ATTRS]> = Vec::new();

    for index in 0..=rows - 1 {
        data.push([(index as i32).into(); ATTRS]);
    }

    data
}

pub fn generate_random_data<const ATTRS: usize>(rows: &u32) -> Vec<[i32; ATTRS]> {
    let mut random_generator = StdRng::seed_from_u64(0);

    let mut data: Vec<[i32; ATTRS]> = Vec::new();
    for _ in 0..=rows - 1 {
        let mut row: [i32; ATTRS] = [0.into(); ATTRS];
        for i in 0..ATTRS {
            row[i] = random_generator.next_u32() as i32;
        }
        data.push(row);
    }

    data
}
