fn main() {
    // let min_x = 20;
    // let max_x = 30;
    // let min_y = -10;
    // let max_y = -5;
    let min_x = 25;
    let max_x = 67;
    let min_y = -260;
    let max_y = -200;

    // Find the lowest possible x to save time. Probably not a big deal. But whatever.
    fn gauss_num(x: i32) -> i32{
        (x as f32 * ((x + 1).abs() as f32 / 2.0)) as i32
    }
    let mut x = 1;
    let mut lowest_x = 0;
    loop {
        let new_num = gauss_num(x);
        if new_num >= min_x && new_num <= max_x {
            lowest_x = x;
            break;
        }
        if new_num > max_x {
            break;
        }
        x += 1;
    }
    // println!("LOW: {}", lowest_x);

    let mut hits: Vec<(i32, i32)> = vec![];
    let highest_y = (min_y as i32).abs() - 1;
    // for new_x in 1..(max_x + 1) {  // Try different powers. Target is always to the right of launcher and at least 1 space away.
    for new_x in lowest_x..(max_x + 1) {  // Try different powers. Target is always to the right of launcher and at least 1 space away.
        let mut new_y = highest_y;  // Start with the highest angle/power, and then lower from there.
        loop {  // Try different angles.
            let mut adj_x = new_x.clone();
            let mut adj_y = new_y.clone();
            let mut pos = (adj_x, adj_y);  // First step, since loop starts with checking.
            loop {  // Generate steps.
                if pos.0 >= min_x && pos.0 <= max_x && pos.1 >= min_y && pos.1 <= max_y {  // In the box?
                    // println!("HIT: {:?}", (new_x, new_y));
                    hits.push((new_x, new_y));
                    break;  // Don't need to keep checking steps. (Probably also reduces doubles on slow velocities?)
                }

                // Adjust for drag and gravity for next step.
                adj_y -= 1;
                if adj_x > 0 { adj_x -= 1; }

                // Find next step location.
                pos.0 += adj_x;
                pos.1 += adj_y;

                if pos.0 > max_x || pos.1 < min_y { break; } // Shot beyond the target?
            }

            new_y -= 1;  // Next lower angle.
            if new_y < min_y { break; } // Beyond the possible limits?
        }
    }

    // println!("HITS: {:?}", hits);
    println!("NUM HITS: {}", hits.len());
}