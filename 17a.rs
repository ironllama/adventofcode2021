fn main() {

    // Highest trajectory is regardless of x value, so we only need the y. And for highest, we want the furthest from 0, or the lowest point on the target.
    let min_y: i32 = -260;

    fn gauss_conseq_sum(x: i32) -> i32{
        (x as f32 * ((x + 1).abs() as f32 / 2.0)) as i32
    }

    // Since it's the first to cross the 0 line, it's 1 faster than the initial trajectory. Or initial trajectory was 1 less.
    // Gravity reduces by 1 each step until 0, so adding them all up is like Gauss's Sequential Number Sum.
    println!("HEIGHT: {}", gauss_conseq_sum(min_y.abs() - 1));
}