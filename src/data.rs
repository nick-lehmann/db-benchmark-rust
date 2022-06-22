pub fn generate_data<T: Sized + Copy + From<i32>, const ATTRS: usize>(
    rows: u32,
) -> Vec<[T; ATTRS]> {
    let mut data: Vec<[T; ATTRS]> = Vec::new();

    for index in 0..=rows - 1 {
        data.push([(index as i32).into(); ATTRS]);
    }

    data
}
