use image::{ImageBuffer, Rgb};

pub fn render(s: &str) {
    let w = 800u32;
    let h = 800u32;
    let step = 4i32; // length of line
    let mut img = ImageBuffer::from_pixel(w, h, Rgb([255u8, 255u8, 255u8]));

    // directions
    let dx = [0i32, 1, 1, 1, 0, -1, -1, -1];
    let dy = [-1i32, -1, 0, 1, 1, 1, 0, -1];

    let mut x = (w / 2) as i32;
    let mut y = ((h / 2)+350) as i32;
    let mut dir: usize = 0;
    let mut stack = [(0i32, 0i32, 0usize); 2000];
    let mut top = 0usize;

    for c in s.chars() {
        if c == 'F' || c == 'B' || c == 'C' {
            for i in 0..step {
                let nx = x + dx[dir] * i;
                let ny = y + dy[dir] * i;
                // check bounds
                if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                    img.put_pixel(nx as u32, ny as u32, Rgb([0u8, 0u8, 0u8]));
                }
            }
            x += dx[dir] * step;
            y += dy[dir] * step;
        } else if c == '+' {
            dir = (dir + 1) % 8;
        } else if c == '-' {
            dir = (dir + 7) % 8;
        } else if c == '[' {
            stack[top] = (x, y, dir); // store position
            top += 1;
        } else if c == ']' {
            top -= 1;
            let s = stack[top]; // return to position
            x = s.0;
            y = s.1;
            dir = s.2;
        }
    }

    img.save("output.png").unwrap();
    println!("saved output.png");
}
