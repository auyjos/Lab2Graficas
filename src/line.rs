use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
) {
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    // Calculate deltas
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    
    // Determine step direction
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    
    // Initialize error term
    let mut err = dx - dy;

    loop {
        // Draw pixel at current position
        framebuffer.set_pixel(x0 as u32, y0 as u32);

        // Check if we've reached the end point
        if x0 == x1 && y0 == y1 {
            break;
        }

        // Calculate error for next step
        let e2 = 2 * err;
        
        // Step in x direction
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        
        // Step in y direction
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}
