pub(crate) fn calc_circle(num_players: usize, radius: f64) -> Vec<(f64, f64)> {
    let angle_diff = -(360.0 / num_players as f64);
    let mut res: Vec<(f64, f64)> = vec![];

    for i in 0..num_players {
        let angle = ((i as f64) * angle_diff - 90.0).to_radians();
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        res.push((x, y));
    }
    res
}
