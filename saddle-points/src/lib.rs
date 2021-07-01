pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    input.into_iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, point)| {
            let mut column_elements = input.iter().map(|row| row.iter().nth(j).unwrap());

            let row_cond = row.iter().all(|x| point >= x);
            let col_cond = column_elements.all(|x| point <= x);

            if row_cond && col_cond {
                result.push((i, j));
            }
        })
    });

    result
}
