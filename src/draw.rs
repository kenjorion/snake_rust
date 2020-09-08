use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 15.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}
/// function `draw_block` draw the block 
    /// # Arguments
    /// * `color` - Color
    /// * ` x`  - i32
    /// * `y` - i32
    /// * ` con` - Context
    /// * `g` -  G2d
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
   
}
/// function `draw_rectangle` draw the rectangle implemented by piston_window
    /// # Arguments
    /// * `color` - Color
    /// * ` x`  - i32
    /// * `y` - i32
    /// * `width` - i32
    /// * `height`- i32
    /// * ` con` - i32
    /// * `g` -G2d
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
