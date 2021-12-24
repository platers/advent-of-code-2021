// inp w
// x = (z % 26) + ?
// z /= 1 || 26
// if x != w:
// z = 26z + w + ?

fn exec(w: i128, z: i128, x_off: i128, y_off: i128, z_div: i128) -> (i128, bool) {
    let x = (z % 26) + x_off;
    let mut z = z / z_div;
    if x != w {
        z = 26 * z + w + y_off;
    }
    // println!("x {}", x);
    (z, x == w)
}

const x_offsets: [i128; 14] = [12, 11, 10, 10, -16, 14, 12, -4, 15, -7, -8, -4, -15, -8];
const y_offsets: [i128; 14] = [6, 12, 5, 10, 7, 0, 4, 12, 14, 13, 10, 11, 9, 9];
const z_divs: [i128; 14] = [1, 1, 1, 1, 26, 1, 1, 26, 1, 26, 26, 26, 26, 26];

fn eval(input: u128) -> (i128, usize) {
    let mut z = 0;
    let mut matches = 0;
    let digits = input.to_string().chars().map(|c| c.to_digit(10).unwrap() as i128).collect::<Vec<i128>>();
    for i in 0..x_offsets.len() {
        let t = exec(digits[i], z, x_offsets[i], y_offsets[i], z_divs[i]) ;
        z = t.0;
        let b = t.1;
        matches += b as i128;
        if matches + 13 - (i as i128) < 7 {
            return (z, i);
        }
    }
    (z, 14)
}
fn main() {
    let mut i = 99999999999999;
    let mut min_val = i as i128;
    while i > 0 { 
        match eval(i) {
            (0, _) => {
                println!("{}", i);
                break;
            },
            (v, j) => {
                // decrement digit j
                i -= 10u128.pow(13 - j as u32) as u128;
                if v < min_val {
                    min_val = v;
                }
                println!("{} {}", i, min_val);
            }
        }
    }
}