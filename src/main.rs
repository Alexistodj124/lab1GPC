mod framebuffer;
mod line;
mod polygon;
mod bmp;

use crate::polygon::Polygon;
use crate::framebuffer::Framebuffer;
use crate::line::Line;

fn fill_polygon(framebuffer: &mut Framebuffer, points: &[(usize, usize)]) {
    if points.len() < 3 {
        return;
    }

    let height = framebuffer.height;

    let mut edges: Vec<Vec<usize>> = vec![vec![]; height];
    
    for i in 0..points.len() {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % points.len()];

        if y0 == y1 {
            continue;
        }

        let (x0, y0, x1, y1) = if y0 < y1 {
            (x0, y0, x1, y1)
        } else {
            (x1, y1, x0, y0)
        };

        let mut x = x0 as isize;
        let mut y = y0;
        let dx = (x1 as isize - x0 as isize).abs();
        let dy = (y1 as isize - y0 as isize).abs();
        let mut err = dx - dy;

        let x_step = if x0 < x1 { 1 } else { -1 };
        let y_step = if y0 < y1 { 1 } else { -1 };

        edges[y].push(x as usize);

        while y != y1 {
            let e2 = err * 2;
            if e2 > -dy {
                err -= dy;
                x += x_step;
            }
            if e2 < dx {
                err += dx;
                y = (y as isize + y_step) as usize;
                edges[y].push(x as usize);
            }
        }
    }

    for y in 0..edges.len() {
        if edges[y].is_empty() {
            continue;
        }
        edges[y].sort();

        for chunk in edges[y].chunks(2) {
            if chunk.len() < 2 {
                continue;
            }
            let x_start = chunk[0];
            let x_end = chunk[1];

            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y, framebuffer.get_current_color());
            }
        }
    }
}

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0x00FFFF);


    let poly1 = vec![
        (165, 380), 
        (185, 360), 
        (180, 330), 
        (207, 345), 
        (233, 330),
        (230, 360), 
        (250, 380), 
        (220, 385), 
        (205, 410), 
        (193, 383)
    ];

    fill_polygon(&mut framebuffer, &poly1);


    framebuffer.set_current_color(0xFFFFFF);

    framebuffer.polygon(&poly1);

    framebuffer.flip_horizontal();
    framebuffer.flip_vertical();

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}
