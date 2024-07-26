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

    framebuffer.set_current_color(0xFF0000);

    let poly2 = vec![
        (321, 335),
        (288, 286),
        (339, 251),
        (374, 302)
    ];

    fill_polygon(&mut framebuffer, &poly2);

    framebuffer.set_current_color(0xFFFFFF);

    framebuffer.polygon(&poly2);
    
    framebuffer.set_current_color(0x0000FF);

    let poly3 = vec![
        (377, 249),
        (411, 197),
        (436, 249)
    ];

    fill_polygon(&mut framebuffer, &poly3);

    framebuffer.set_current_color(0xFFFFFF);

    framebuffer.polygon(&poly3);

    framebuffer.set_current_color(0x00FF00);

    let poly4 = vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180)
    ];

    fill_polygon(&mut framebuffer, &poly4);

    framebuffer.set_current_color(0xFFFFFF);

    framebuffer.polygon(&poly4);

    framebuffer.set_current_color(0x000000);

    let poly5 = vec![
        (682, 175),
        (708, 120),
        (735, 148),
        (739, 170)
    ];

    fill_polygon(&mut framebuffer, &poly5);

    framebuffer.set_current_color(0xFFFFFF);

    framebuffer.polygon(&poly5);
    

    framebuffer.flip_horizontal();
    framebuffer.flip_vertical();

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}
