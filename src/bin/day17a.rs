fn main() {
    let x_min = 144;
    let x_max = 178;
    let y_min = -100;
    let y_max = -76;
    let mut vy_max = 0;
    for init_vy in 0..200 {
        for init_vx in 0..100 {
            let mut x = 0;
            let mut y = 0;
            let mut vy = init_vy;
            let mut vx = init_vx;
            while y >= y_min && x <= x_max {
                if y <= y_max && x >= x_min {
                    vy_max = init_vy;
                    break;
                }
                x += vx;
                y += vy;
                vy -= 1;
                if vx > 0 {
                    vx -= 1;
                }
            }
        }
    }

    println!("{}", vy_max * (vy_max + 1) / 2);
}
