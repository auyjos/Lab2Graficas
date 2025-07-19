mod framebuffer;
mod line;

use std::{thread, time::Duration};
use raylib::prelude::*;
use framebuffer::Framebuffer;

// Grid dimensions for the Game of Life (use a lower resolution as suggested)
const GRID_WIDTH: u32 = 100;
const GRID_HEIGHT: u32 = 100;
const CELL_SIZE: u32 = 6; // Size of each cell in the grid for display
const BORDER_SIZE: u32 = 50; // Border around the grid

// Color configuration for different organisms
const BACKGROUND_COLOR: Color = Color::PURPLE; // Background color
// Different colors for different types of organisms
const STILL_LIFE_COLOR: Color = Color::GOLD;      // Color for still life forms
const OSCILLATOR_COLOR: Color = Color::LIME;      // Color for oscillators
const SPACESHIP_COLOR: Color = Color::RED;        // Color for spaceships
const CUSTOM_COLOR: Color = Color::SKYBLUE;       // Color for custom patterns
const GENERATED_COLOR: Color = Color::ORANGE;     // Color for dynamically generated cells

// Function to create a block pattern at a specific position
fn create_block(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 1 < GRID_WIDTH as usize && y + 1 < GRID_HEIGHT as usize {
        grid[y][x] = true;
        grid[y][x+1] = true;
        grid[y+1][x] = true;
        grid[y+1][x+1] = true;
    }
}

// Function to create a beehive pattern at a specific position
fn create_beehive(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 3 < GRID_WIDTH as usize && y + 2 < GRID_HEIGHT as usize {
        grid[y][x+1] = true;
        grid[y][x+2] = true;
        grid[y+1][x] = true;
        grid[y+1][x+3] = true;
        grid[y+2][x+1] = true;
        grid[y+2][x+2] = true;
    }
}

// Function to create a loaf pattern at a specific position
fn create_loaf(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 3 < GRID_WIDTH as usize && y + 3 < GRID_HEIGHT as usize {
        grid[y][x+1] = true;
        grid[y][x+2] = true;
        grid[y+1][x] = true;
        grid[y+1][x+3] = true;
        grid[y+2][x+1] = true;
        grid[y+2][x+3] = true;
        grid[y+3][x+2] = true;
    }
}

// Function to create a boat pattern at a specific position
fn create_boat(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 2 < GRID_WIDTH as usize && y + 2 < GRID_HEIGHT as usize {
        grid[y][x] = true;
        grid[y][x+1] = true;
        grid[y+1][x] = true;
        grid[y+1][x+2] = true;
        grid[y+2][x+1] = true;
    }
}

// Function to create a tub pattern at a specific position
fn create_tub(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 2 < GRID_WIDTH as usize && y + 2 < GRID_HEIGHT as usize {
        grid[y][x+1] = true;
        grid[y+1][x] = true;
        grid[y+1][x+2] = true;
        grid[y+2][x+1] = true;
    }
}

// Function to create a blinker pattern at a specific position (oscillator)
fn create_blinker(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 2 < GRID_WIDTH as usize && y < GRID_HEIGHT as usize {
        grid[y][x] = true;
        grid[y][x+1] = true;
        grid[y][x+2] = true;
    }
}

// Function to create a toad pattern at a specific position (oscillator)
fn create_toad(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 3 < GRID_WIDTH as usize && y + 1 < GRID_HEIGHT as usize {
        grid[y][x+1] = true;
        grid[y][x+2] = true;
        grid[y][x+3] = true;
        grid[y+1][x] = true;
        grid[y+1][x+1] = true;
        grid[y+1][x+2] = true;
    }
}

// Function to create a beacon pattern at a specific position (oscillator)
fn create_beacon(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 3 < GRID_WIDTH as usize && y + 3 < GRID_HEIGHT as usize {
        // Top-left block
        grid[y][x] = true;
        grid[y][x+1] = true;
        grid[y+1][x] = true;
        grid[y+1][x+1] = true;
        
        // Bottom-right block
        grid[y+2][x+2] = true;
        grid[y+2][x+3] = true;
        grid[y+3][x+2] = true;
        grid[y+3][x+3] = true;
    }
}

// Function to create a glider pattern at a specific position (spaceship)
fn create_glider(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 2 < GRID_WIDTH as usize && y + 2 < GRID_HEIGHT as usize {
        grid[y][x+1] = true;
        grid[y+1][x+2] = true;
        grid[y+2][x] = true;
        grid[y+2][x+1] = true;
        grid[y+2][x+2] = true;
    }
}

// Function to create a lightweight spaceship (LWSS)
fn create_lwss(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 4 < GRID_WIDTH as usize && y + 3 < GRID_HEIGHT as usize {
        grid[y][x+1] = true;
        grid[y][x+4] = true;
        grid[y+1][x] = true;
        grid[y+2][x] = true;
        grid[y+2][x+4] = true;
        grid[y+3][x] = true;
        grid[y+3][x+1] = true;
        grid[y+3][x+2] = true;
        grid[y+3][x+3] = true;
    }
}

// Function to create a pulsar pattern (large oscillator)
fn create_pulsar(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 12 < GRID_WIDTH as usize && y + 12 < GRID_HEIGHT as usize {
        // Horizontal bars
        for i in [2, 3, 4, 8, 9, 10] {
            for j in [0, 5, 7, 12] {
                grid[y+j][x+i] = true;
                grid[y+i][x+j] = true;
            }
        }
    }
}

// Function to create a Gosper glider gun (creates endless gliders)
fn create_gosper_glider_gun(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 36 < GRID_WIDTH as usize && y + 9 < GRID_HEIGHT as usize {
        // Left block
        grid[y+4][x+0] = true;
        grid[y+4][x+1] = true;
        grid[y+5][x+0] = true;
        grid[y+5][x+1] = true;

        // Left ship
        grid[y+2][x+12] = true;
        grid[y+2][x+13] = true;
        grid[y+3][x+11] = true;
        grid[y+3][x+15] = true;
        grid[y+4][x+10] = true;
        grid[y+4][x+16] = true;
        grid[y+5][x+10] = true;
        grid[y+5][x+14] = true;
        grid[y+5][x+16] = true;
        grid[y+5][x+17] = true;
        grid[y+6][x+10] = true;
        grid[y+6][x+16] = true;
        grid[y+7][x+11] = true;
        grid[y+7][x+15] = true;
        grid[y+8][x+12] = true;
        grid[y+8][x+13] = true;

        // Right ship
        grid[y+0][x+24] = true;
        grid[y+1][x+22] = true;
        grid[y+1][x+24] = true;
        grid[y+2][x+20] = true;
        grid[y+2][x+21] = true;
        grid[y+3][x+20] = true;
        grid[y+3][x+21] = true;
        grid[y+4][x+20] = true;
        grid[y+4][x+21] = true;
        grid[y+5][x+22] = true;
        grid[y+5][x+24] = true;
        grid[y+6][x+24] = true;

        // Right block
        grid[y+2][x+34] = true;
        grid[y+2][x+35] = true;
        grid[y+3][x+34] = true;
        grid[y+3][x+35] = true;
    }
}

// Function to create a penta-decathlon (period 15 oscillator)
fn create_pentadecathlon(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 2 < GRID_WIDTH as usize && y + 9 < GRID_HEIGHT as usize {
        // Central pattern
        for i in 0..8 {
            grid[y+i+1][x+1] = true;
        }
        
        // Top and bottom "bumps"
        grid[y][x] = true;
        grid[y][x+2] = true;
        grid[y+9][x] = true;
        grid[y+9][x+2] = true;
    }
}

// Function to create R-pentomino (creates a lot of activity)
fn create_r_pentomino(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 2 < GRID_WIDTH as usize && y + 2 < GRID_HEIGHT as usize {
        grid[y][x+1] = true;
        grid[y][x+2] = true;
        grid[y+1][x] = true;
        grid[y+1][x+1] = true;
        grid[y+2][x+1] = true;
    }
}

// New pattern: Acorn (sparks long evolution)
fn create_acorn(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pts = [(1,0),(3,0),(0,1),(1,1),(4,1),(5,1),(6,1)];
    for &(dx,dy) in &pts {
        let nx = x + dx;
        let ny = y + dy;
        if nx < GRID_WIDTH as usize && ny < GRID_HEIGHT as usize {
            grid[ny][nx] = true;
        }
    }
}

// New pattern: Diehard (small pattern with long lifespan)
fn create_diehard(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pts = [(0,0),(1,0),(1,1),(5,1),(6,1),(7,1),(6,2)];
    for &(dx,dy) in &pts {
        let nx = x + dx;
        let ny = y + dy;
        if nx < GRID_WIDTH as usize && ny < GRID_HEIGHT as usize {
            grid[ny][nx] = true;
        }
    }
}

// Type of cell to track its origin
#[derive(Clone, Copy, PartialEq)]
enum CellType {
    StillLife,  // Blocks, beehives, etc.
    Oscillator, // Blinkers, toads, etc.
    Spaceship,  // Gliders, LWSS, etc.
    Custom,     // Custom patterns (Gosper gun, etc.)
    Generated,  // Cells that emerge during simulation
    Dead        // Dead cells
}

// Initialize the grid with various life forms
fn initialize_grid() -> (Vec<Vec<bool>>, Vec<Vec<CellType>>) {
    let mut grid = vec![vec![false; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
    let mut cell_types = vec![vec![CellType::Dead; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
    
    // No longer need this with mark_region function
    // Let's keep our grid initialized
    
    // Create still life forms (in the top-left quadrant)
    create_block(&mut grid, 10, 10);
    mark_region(&mut grid, &mut cell_types, 10, 10, 2, 2, CellType::StillLife);
    
    create_beehive(&mut grid, 20, 15);
    mark_region(&mut grid, &mut cell_types, 20, 15, 4, 3, CellType::StillLife);
    
    create_loaf(&mut grid, 30, 10);
    mark_region(&mut grid, &mut cell_types, 30, 10, 4, 4, CellType::StillLife);
    
    create_boat(&mut grid, 40, 10);
    mark_region(&mut grid, &mut cell_types, 40, 10, 3, 3, CellType::StillLife);
    
    create_tub(&mut grid, 10, 20);
    mark_region(&mut grid, &mut cell_types, 10, 20, 3, 3, CellType::StillLife);
    
    // Create oscillators (in the top-right quadrant)
    create_blinker(&mut grid, 60, 10);
    mark_region(&mut grid, &mut cell_types, 60, 10, 3, 1, CellType::Oscillator);
    
    create_toad(&mut grid, 70, 15);
    mark_region(&mut grid, &mut cell_types, 70, 15, 4, 2, CellType::Oscillator);
    
    create_beacon(&mut grid, 70, 25);
    mark_region(&mut grid, &mut cell_types, 70, 25, 4, 4, CellType::Oscillator);
    
    create_pulsar(&mut grid, 55, 40);
    mark_region(&mut grid, &mut cell_types, 55, 40, 13, 13, CellType::Oscillator);
    
    create_pentadecathlon(&mut grid, 85, 10);
    mark_region(&mut grid, &mut cell_types, 85, 10, 3, 10, CellType::Oscillator);
    
    // Create spaceships (in the bottom-left quadrant)
    create_glider(&mut grid, 10, 70);
    mark_region(&mut grid, &mut cell_types, 10, 70, 3, 3, CellType::Spaceship);
    
    create_glider(&mut grid, 20, 80);
    mark_region(&mut grid, &mut cell_types, 20, 80, 3, 3, CellType::Spaceship);
    
    create_lwss(&mut grid, 35, 70);
    mark_region(&mut grid, &mut cell_types, 35, 70, 5, 4, CellType::Spaceship);
    
    // Create custom patterns (in the bottom-right quadrant)
    create_gosper_glider_gun(&mut grid, 55, 80);
    mark_region(&mut grid, &mut cell_types, 55, 80, 36, 9, CellType::Custom);
    
    create_r_pentomino(&mut grid, 30, 40);
    mark_region(&mut grid, &mut cell_types, 30, 40, 3, 3, CellType::Custom);

    // Add cooler patterns
    create_acorn(&mut grid, 20, 50);
    mark_region(&mut grid, &mut cell_types, 20, 50, 7, 2, CellType::Custom);
    create_diehard(&mut grid, 60, 50);
    mark_region(&mut grid, &mut cell_types, 60, 50, 8, 3, CellType::Custom);

    (grid, cell_types)
}

// Helper function to mark a region with a cell type
fn mark_region(grid: &mut Vec<Vec<bool>>, cell_types: &mut Vec<Vec<CellType>>, 
               x: usize, y: usize, width: usize, height: usize, cell_type: CellType) {
    for j in 0..height {
        for i in 0..width {
            let nx = x + i;
            let ny = y + j;
            if nx < GRID_WIDTH as usize && ny < GRID_HEIGHT as usize && grid[ny][nx] {
                cell_types[ny][nx] = cell_type;
            }
        }
    }
}

// Update the grid based on Conway's Game of Life rules
fn update_grid(data: &(Vec<Vec<bool>>, Vec<Vec<CellType>>)) -> (Vec<Vec<bool>>, Vec<Vec<CellType>>) {
    let (grid, cell_types) = data;
    let mut new_grid = vec![vec![false; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
    let mut new_cell_types = vec![vec![CellType::Dead; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
    
    for y in 0..GRID_HEIGHT as usize {
        for x in 0..GRID_WIDTH as usize {
            let mut live_neighbors = 0;
            
            // Count live neighbors (using wrap-around/envolvente approach)
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 { continue; } // Skip self
                    
                    // Calculate neighbor coordinates with wrap-around
                    let nx = (x as i32 + dx).rem_euclid(GRID_WIDTH as i32) as usize;
                    let ny = (y as i32 + dy).rem_euclid(GRID_HEIGHT as i32) as usize;
                    
                    if grid[ny][nx] {
                        live_neighbors += 1;
                    }
                }
            }
            
            // Apply Conway's rules
            if grid[y][x] {
                // Live cell
                if live_neighbors < 2 || live_neighbors > 3 {
                    // Die from underpopulation or overpopulation
                    new_grid[y][x] = false;
                    new_cell_types[y][x] = CellType::Dead;
                } else {
                    // Survive - preserve its type
                    new_grid[y][x] = true;
                    new_cell_types[y][x] = cell_types[y][x];
                }
            } else {
                // Dead cell
                if live_neighbors == 3 {
                    // Reproduction - mark as a new generated cell
                    new_grid[y][x] = true;
                    new_cell_types[y][x] = CellType::Generated;
                }
                // Otherwise stay dead
            }
        }
    }
    
    (new_grid, new_cell_types)
}

// Render the grid onto the framebuffer
fn render_grid(data: &(Vec<Vec<bool>>, Vec<Vec<CellType>>), framebuffer: &mut Framebuffer) {
    framebuffer.clear();
    
    let (grid, cell_types) = data;
    
    // Calculate cell size based on framebuffer dimensions and grid size
    // Leave space for UI and borders
    let fb_width = framebuffer.width() as u32;
    let fb_height = framebuffer.height() as u32;
    
    // Reserve 40px for the UI at the bottom
    let available_height = if fb_height > 40 { fb_height - 40 } else { fb_height };
    
    // Calculate dynamic cell size based on available space
    let horizontal_cell_size = (fb_width - (BORDER_SIZE * 2)) / GRID_WIDTH;
    let vertical_cell_size = (available_height - (BORDER_SIZE * 2)) / GRID_HEIGHT;
    
    // Use the smaller of the two to maintain square cells
    let cell_size = horizontal_cell_size.min(vertical_cell_size).max(1); // Ensure at least 1px
    
    // Calculate border to center the grid
    let horizontal_border = (fb_width - (GRID_WIDTH * cell_size)) / 2;
    let vertical_border = (available_height - (GRID_HEIGHT * cell_size)) / 2;
    
    for y in 0..GRID_HEIGHT as usize {
        for x in 0..GRID_WIDTH as usize {
            if grid[y][x] {
                // Set the color based on the cell type
                match cell_types[y][x] {
                    CellType::StillLife => framebuffer.set_current_color(STILL_LIFE_COLOR),
                    CellType::Oscillator => framebuffer.set_current_color(OSCILLATOR_COLOR),
                    CellType::Spaceship => framebuffer.set_current_color(SPACESHIP_COLOR),
                    CellType::Custom => framebuffer.set_current_color(CUSTOM_COLOR),
                    CellType::Generated => framebuffer.set_current_color(GENERATED_COLOR),
                    _ => framebuffer.set_current_color(Color::WHITE), // Shouldn't happen
                }
                
                // Draw a cell_size x cell_size square for each cell
                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        let px = horizontal_border + (x as u32 * cell_size) + dx;
                        let py = vertical_border + (y as u32 * cell_size) + dy;
                        framebuffer.set_pixel(px, py);
                    }
                }
            }
        }
    }
    
    // Draw a frame around the grid
    framebuffer.set_current_color(Color::WHITE);
    for x in 0..GRID_WIDTH * cell_size + 2 {
        // Top border
        framebuffer.set_pixel(horizontal_border - 1 + x, vertical_border - 1);
        // Bottom border
        framebuffer.set_pixel(horizontal_border - 1 + x, vertical_border + GRID_HEIGHT * cell_size);
    }
    
    for y in 0..GRID_HEIGHT * cell_size + 2 {
        // Left border
        framebuffer.set_pixel(horizontal_border - 1, vertical_border - 1 + y);
        // Right border
        framebuffer.set_pixel(horizontal_border + GRID_WIDTH * cell_size, vertical_border - 1 + y);
    }
}

// Draw the UI directly on the framebuffer
fn draw_ui(framebuffer: &mut Framebuffer, paused: bool, generation: u32, speed: u64, window_width: u32, window_height: u32) {
    // Calculate UI dimensions
    let ui_height = 40;
    let ui_start_y = if window_height > ui_height { window_height - ui_height } else { 0 };
    
    // Draw UI background
    framebuffer.set_current_color(Color::BLACK);
    for y in ui_start_y..window_height {
        for x in 0..window_width {
            framebuffer.set_pixel(x, y);
        }
    }
    
    // Draw status bar separator line
    framebuffer.set_current_color(Color::DARKGRAY);
    for x in 0..window_width {
        framebuffer.set_pixel(x, ui_start_y);
    }
    
    // Draw status indicators using simple ASCII art
    framebuffer.set_current_color(Color::WHITE);
    
    // Determine if we have enough width for full UI
    let compact_ui = window_width < 700;
    
    // Status indicator
    let status_text = if paused { "Status: PAUSED " } else { "Status: RUNNING" };
    draw_text(framebuffer, status_text, 10, (ui_start_y + 10) as i32, 1);
    
    // Generation counter
    let gen_text = format!("Gen: {}", generation);
    let gen_pos_x = if compact_ui { 150 } else { window_width as i32 / 3 };
    draw_text(framebuffer, &gen_text, gen_pos_x, (ui_start_y + 10) as i32, 1);
    
    // Speed indicator
    let speed_text = format!("Speed: {}ms", speed);
    let speed_pos_x = if compact_ui { 250 } else { 2 * window_width as i32 / 3 };
    draw_text(framebuffer, &speed_text, speed_pos_x, (ui_start_y + 10) as i32, 1);
    
    // Controls
    let controls = if compact_ui {
        "SPACE=Pause | R=Reset | S=Step | UP/DOWN=Speed"
    } else {
        "Controls: SPACE=Pause | R=Reset | S=Step | UP/DOWN=Speed"
    };
    draw_text(framebuffer, controls, 10, (ui_start_y + 25) as i32, 1);
    
    // Only show color legend if we have enough space
    if !compact_ui && window_width >= 600 {
        // Legend for colors
        let legend_y = (ui_start_y + 10) as i32;
        let legend_width = if window_width >= 800 { 380 } else { 300 };
        let start_x = window_width - legend_width;
        
        // Adjust spacing based on available width
        let spacing = if window_width >= 800 { 80 } else { 60 };
        
        // Draw color indicators for the legend
        draw_color_square(framebuffer, STILL_LIFE_COLOR, start_x, legend_y - 3, 3);
        draw_text(framebuffer, "SLife", (start_x + 10) as i32, legend_y, 1);
        
        draw_color_square(framebuffer, OSCILLATOR_COLOR, start_x + spacing, legend_y - 3, 3);
        draw_text(framebuffer, "Osc", (start_x + spacing + 10) as i32, legend_y, 1);
        
        draw_color_square(framebuffer, SPACESHIP_COLOR, start_x + 2*spacing, legend_y - 3, 3);
        draw_text(framebuffer, "Ship", (start_x + 2*spacing + 10) as i32, legend_y, 1);
        
        draw_color_square(framebuffer, GENERATED_COLOR, start_x + 3*spacing, legend_y - 3, 3);
        draw_text(framebuffer, "Gen", (start_x + 3*spacing + 10) as i32, legend_y, 1);
    }
}

// Helper function to draw simple text directly on framebuffer
fn draw_text(framebuffer: &mut Framebuffer, text: &str, x: i32, y: i32, scale: u32) {
    let mut current_x = x;
    
    for c in text.chars() {
        // Simple fixed-width character drawing
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | ':' | '.' | ',' | '=' | '|' | '-' | '_' | '+' | '/' => {
                // For each character, draw a simple dot pattern
                // This is a very basic rendering, just to demonstrate
                draw_char(framebuffer, c, current_x, y, scale);
                current_x += 8 * scale as i32;
            }
            _ => current_x += 8 * scale as i32, // Skip unknown chars
        }
    }
}

// Draw a simple character directly on the framebuffer
fn draw_char(framebuffer: &mut Framebuffer, c: char, x: i32, y: i32, scale: u32) {
    // This is a very basic implementation just to show text
    // For each character, we'll just draw a few pixels to represent it
    // A proper implementation would use a font bitmap
    
    // Define a simple 5x7 font for basic characters
    let pattern = match c {
        'A' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3), (4,3), (0,4), (4,4), (0,5), (4,5), (0,6), (4,6)],
        'B' => vec![(0,0), (1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3), (0,4), (4,4), (0,5), (4,5), (0,6), (1,6), (2,6), (3,6)],
        'C' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (0,3), (0,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        'D' => vec![(0,0), (1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (0,6), (1,6), (2,6), (3,6)],
        'E' => vec![(0,0), (1,0), (2,0), (3,0), (4,0), (0,1), (0,2), (0,3), (1,3), (2,3), (3,3), (0,4), (0,5), (0,6), (1,6), (2,6), (3,6), (4,6)],
        'F' => vec![(0,0), (1,0), (2,0), (3,0), (4,0), (0,1), (0,2), (0,3), (1,3), (2,3), (3,3), (0,4), (0,5), (0,6)],
        'G' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (0,3), (0,4), (3,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        'H' => vec![(0,0), (4,0), (0,1), (4,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3), (4,3), (0,4), (4,4), (0,5), (4,5), (0,6), (4,6)],
        'I' => vec![(0,0), (1,0), (2,0), (3,0), (4,0), (2,1), (2,2), (2,3), (2,4), (2,5), (0,6), (1,6), (2,6), (3,6), (4,6)],
        'J' => vec![(0,0), (1,0), (2,0), (3,0), (4,0), (2,1), (2,2), (2,3), (2,4), (0,5), (2,5), (1,6)],
        'K' => vec![(0,0), (4,0), (0,1), (3,1), (0,2), (2,2), (0,3), (1,3), (0,4), (2,4), (0,5), (3,5), (0,6), (4,6)],
        'L' => vec![(0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (1,6), (2,6), (3,6), (4,6)],
        'M' => vec![(0,0), (4,0), (0,1), (1,1), (3,1), (4,1), (0,2), (2,2), (4,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (0,6), (4,6)],
        'N' => vec![(0,0), (4,0), (0,1), (1,1), (4,1), (0,2), (2,2), (4,2), (0,3), (2,3), (4,3), (0,4), (3,4), (4,4), (0,5), (3,5), (4,5), (0,6), (4,6)],
        'O' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        'P' => vec![(0,0), (1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3), (0,4), (0,5), (0,6)],
        'Q' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (0,3), (4,3), (0,4), (2,4), (4,4), (0,5), (3,5), (1,6), (2,6), (4,6)],
        'R' => vec![(0,0), (1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3), (0,4), (2,4), (0,5), (3,5), (0,6), (4,6)],
        'S' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (1,3), (2,3), (3,3), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        'T' => vec![(0,0), (1,0), (2,0), (3,0), (4,0), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6)],
        'U' => vec![(0,0), (4,0), (0,1), (4,1), (0,2), (4,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        'V' => vec![(0,0), (4,0), (0,1), (4,1), (0,2), (4,2), (0,3), (4,3), (1,4), (3,4), (1,5), (3,5), (2,6)],
        'W' => vec![(0,0), (4,0), (0,1), (4,1), (0,2), (4,2), (0,3), (4,3), (0,4), (2,4), (4,4), (0,5), (2,5), (4,5), (1,6), (3,6)],
        'X' => vec![(0,0), (4,0), (0,1), (4,1), (1,2), (3,2), (2,3), (1,4), (3,4), (0,5), (4,5), (0,6), (4,6)],
        'Y' => vec![(0,0), (4,0), (0,1), (4,1), (1,2), (3,2), (2,3), (2,4), (2,5), (2,6)],
        'Z' => vec![(0,0), (1,0), (2,0), (3,0), (4,0), (4,1), (3,2), (2,3), (1,4), (0,5), (0,6), (1,6), (2,6), (3,6), (4,6)],
        'a' => vec![(1,2), (2,2), (3,2), (0,3), (4,3), (1,4), (2,4), (3,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6), (4,6)],
        'b' => vec![(0,0), (0,1), (0,2), (1,2), (2,2), (3,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (0,6), (1,6), (2,6), (3,6)],
        'c' => vec![(1,2), (2,2), (3,2), (0,3), (0,4), (0,5), (1,6), (2,6), (3,6)],
        'd' => vec![(4,0), (4,1), (1,2), (2,2), (3,2), (4,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6), (4,6)],
        'e' => vec![(1,2), (2,2), (3,2), (0,3), (4,3), (0,4), (1,4), (2,4), (3,4), (4,4), (0,5), (1,6), (2,6), (3,6)],
        'f' => vec![(2,0), (3,0), (1,1), (1,2), (0,3), (1,3), (2,3), (1,4), (1,5), (1,6)],
        'g' => vec![(1,2), (2,2), (3,2), (4,2), (0,3), (4,3), (0,4), (4,4), (1,5), (2,5), (3,5), (4,5), (4,6), (1,7), (2,7), (3,7)],
        'h' => vec![(0,0), (0,1), (0,2), (1,2), (2,2), (3,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (0,6), (4,6)],
        'i' => vec![(2,0), (2,2), (2,3), (2,4), (2,5), (2,6)],
        'j' => vec![(3,0), (3,2), (3,3), (3,4), (3,5), (0,6), (1,6), (2,6)],
        'k' => vec![(0,0), (0,1), (0,2), (3,2), (0,3), (2,3), (0,4), (1,4), (0,5), (2,5), (0,6), (3,6)],
        'l' => vec![(1,0), (1,1), (1,2), (1,3), (1,4), (1,5), (2,6)],
        'm' => vec![(0,2), (2,2), (4,2), (0,3), (1,3), (3,3), (4,3), (0,4), (2,4), (4,4), (0,5), (2,5), (4,5), (0,6), (2,6), (4,6)],
        'n' => vec![(0,2), (1,2), (2,2), (3,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (0,6), (4,6)],
        'o' => vec![(1,2), (2,2), (3,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        'p' => vec![(0,2), (1,2), (2,2), (3,2), (0,3), (4,3), (0,4), (4,4), (0,5), (1,5), (2,5), (3,5), (0,6), (0,7), (0,8)],
        'q' => vec![(1,2), (2,2), (3,2), (4,2), (0,3), (4,3), (0,4), (4,4), (1,5), (2,5), (3,5), (4,5), (4,6), (4,7), (4,8)],
        'r' => vec![(0,2), (1,2), (2,2), (3,2), (0,3), (4,3), (0,4), (0,5), (0,6)],
        's' => vec![(1,2), (2,2), (3,2), (4,2), (0,3), (1,4), (2,4), (3,4), (4,5), (0,6), (1,6), (2,6), (3,6)],
        't' => vec![(2,0), (2,1), (1,2), (2,2), (3,2), (2,3), (2,4), (2,5), (3,6)],
        'u' => vec![(0,2), (4,2), (0,3), (4,3), (0,4), (4,4), (0,5), (3,5), (4,5), (1,6), (2,6), (4,6)],
        'v' => vec![(0,2), (4,2), (0,3), (4,3), (0,4), (4,4), (1,5), (3,5), (2,6)],
        'w' => vec![(0,2), (2,2), (4,2), (0,3), (2,3), (4,3), (0,4), (2,4), (4,4), (0,5), (2,5), (4,5), (1,6), (3,6)],
        'x' => vec![(0,2), (4,2), (1,3), (3,3), (2,4), (1,5), (3,5), (0,6), (4,6)],
        'y' => vec![(0,2), (4,2), (0,3), (4,3), (0,4), (4,4), (1,5), (2,5), (3,5), (4,5), (4,6), (1,7), (2,7), (3,7)],
        'z' => vec![(0,2), (1,2), (2,2), (3,2), (4,2), (3,3), (2,4), (1,5), (0,6), (1,6), (2,6), (3,6), (4,6)],
        '0' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (3,2), (4,2), (0,3), (2,3), (4,3), (0,4), (1,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        '1' => vec![(2,0), (1,1), (2,1), (0,2), (2,2), (2,3), (2,4), (2,5), (0,6), (1,6), (2,6), (3,6), (4,6)],
        '2' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (4,2), (2,3), (3,3), (1,4), (0,5), (0,6), (1,6), (2,6), (3,6), (4,6)],
        '3' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (4,2), (2,3), (3,3), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        '4' => vec![(3,0), (2,1), (3,1), (1,2), (3,2), (0,3), (3,3), (0,4), (1,4), (2,4), (3,4), (4,4), (3,5), (3,6)],
        '5' => vec![(0,0), (1,0), (2,0), (3,0), (4,0), (0,1), (0,2), (0,3), (1,3), (2,3), (3,3), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        '6' => vec![(1,0), (2,0), (3,0), (0,1), (0,2), (0,3), (1,3), (2,3), (3,3), (0,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        '7' => vec![(0,0), (1,0), (2,0), (3,0), (4,0), (4,1), (3,2), (2,3), (2,4), (2,5), (2,6)],
        '8' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (1,3), (2,3), (3,3), (0,4), (4,4), (0,5), (4,5), (1,6), (2,6), (3,6)],
        '9' => vec![(1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (4,2), (1,3), (2,3), (3,3), (4,3), (4,4), (4,5), (1,6), (2,6), (3,6)],
        ' ' => vec![],
        ':' => vec![(2,1), (2,2), (2,4), (2,5)],
        '.' => vec![(2,5), (2,6)],
        ',' => vec![(2,5), (2,6), (1,7)],
        '=' => vec![(0,2), (1,2), (2,2), (3,2), (4,2), (0,4), (1,4), (2,4), (3,4), (4,4)],
        '|' => vec![(2,0), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6)],
        '-' => vec![(0,3), (1,3), (2,3), (3,3), (4,3)],
        '_' => vec![(0,6), (1,6), (2,6), (3,6), (4,6)],
        '+' => vec![(2,1), (2,2), (0,3), (1,3), (2,3), (3,3), (4,3), (2,4), (2,5)],
        '/' => vec![(4,0), (4,1), (3,2), (2,3), (1,4), (0,5), (0,6)],
        _ => vec![], // Empty for unknown characters
    };
    
    // Draw the character pattern
    for (dx, dy) in pattern {
        let px = x + (dx * scale as i32);
        let py = y + (dy * scale as i32);
        
        // Draw a small square for each pixel of the character
        for sy in 0..scale {
            for sx in 0..scale {
                if px >= 0 && py >= 0 {
                    framebuffer.set_pixel((px + sx as i32) as u32, (py + sy as i32) as u32);
                }
            }
        }
    }
}

// Draw a colored square for the legend
fn draw_color_square(framebuffer: &mut Framebuffer, color: Color, x: u32, y: i32, size: u32) {
    framebuffer.set_current_color(color);
    for dy in 0..size*2 {
        for dx in 0..size*2 {
            framebuffer.set_pixel(x + dx, y as u32 + dy);
        }
    }
}

fn main() {
    let mut window_width = (GRID_WIDTH * CELL_SIZE) + (BORDER_SIZE * 2);
    let mut window_height = (GRID_HEIGHT * CELL_SIZE) + (BORDER_SIZE * 2) + 40; // Extra space for UI

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Conway's Game of Life - Jose")
        .log_level(TraceLogLevel::LOG_WARNING)
        .resizable()
        .build();
        
    let mut framebuffer = Framebuffer::new(window_width, window_height, BACKGROUND_COLOR);
    framebuffer.set_background_color(BACKGROUND_COLOR);
    framebuffer.clear();
    
    // Initialize the game grid and cell types
    let mut grid_data = initialize_grid();
    
    // Render the initial state
    render_grid(&grid_data, &mut framebuffer);
    
    // Simulation control
    let mut paused = false;
    let mut step_mode = false;
    let mut step_requested = false;
    let mut speed = 100; // milliseconds between updates
    let mut generation = 0;
    
    // Main game loop
    while !window.window_should_close() {
        // Check if window was resized
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();
        
        // Check for window resize
        if current_width as u32 != window_width || current_height as u32 != window_height {
            // Update window dimensions
            window_width = current_width as u32;
            window_height = current_height as u32;
            
            // Create a new framebuffer with the new size
            framebuffer = Framebuffer::new(window_width, window_height, BACKGROUND_COLOR);
            framebuffer.set_background_color(BACKGROUND_COLOR);
            
            // Re-render the current state
            render_grid(&grid_data, &mut framebuffer);
            draw_ui(&mut framebuffer, paused, generation, speed, window_width, window_height);
        }
        
        // Check for user input without drawing
        let mut input = window.get_key_pressed();
        
        // Handle input
        while let Some(key) = input {
            match key {
                KeyboardKey::KEY_SPACE => paused = !paused,
                KeyboardKey::KEY_S => {
                    step_mode = true;
                    step_requested = true;
                },
                KeyboardKey::KEY_R => {
                    // Reset the simulation
                    grid_data = initialize_grid();
                    generation = 0;
                },
                KeyboardKey::KEY_UP => {
                    // Increase simulation speed
                    if speed > 20 {
                        speed -= 20;
                    }
                },
                KeyboardKey::KEY_DOWN => {
                    // Decrease simulation speed
                    speed += 20;
                },
                _ => {}
            }
            input = window.get_key_pressed();
        }
        
        // Update the game state if not paused or if step requested
        if (!paused || (step_mode && step_requested)) && !window.window_should_close() {
            grid_data = update_grid(&grid_data);
            generation += 1;
            step_requested = false; // Reset step flag
        }
        
        // Render everything to our framebuffer
        render_grid(&grid_data, &mut framebuffer);
        
        // Draw the UI directly on our framebuffer
        draw_ui(&mut framebuffer, paused, generation, speed, window_width, window_height);
        
        // Display the framebuffer with everything on it
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        // Add a delay to slow down the simulation
        thread::sleep(Duration::from_millis(speed));
    }
}
