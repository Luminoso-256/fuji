//per the MacPascal book (1985) quickdraw is in two "units". we however, don't care.

use std::collections::HashMap;
use std::future::Future;
use std::process::Output;

use macroquad::prelude::*;

//a point is composed of an x and a y, both ints
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Pen {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub struct RenderQueue {
    pub(crate) lines: Vec<RqLine>,

}

pub struct RqLine {
    start: Vec2,
    end: Vec2,
}

pub struct RqRect{
    x:i32,
    y:i32,
    w:i32,
    h:i32
}

pub enum FillRectPattern{
    White,
    Black,
    Gray,
    LtGray, //Light Gray
    DkGray //dark gray
}

//due to reference things, this is handled as a struct/impl
//really would be way better as a class, but this is rust's closest analog to that
pub struct QuickDraw {
    pub(crate) pen: Pen,
    pub(crate) render_queue: RenderQueue,

}

impl QuickDraw {
    /*==Internal functionality==*/

    //[Internal] Call every frame to draw things.
    pub fn int_frame(&self) {
        clear_background(WHITE);
        for line in &self.render_queue.lines {
            draw_line(line.start.x as f32, line.start.y as f32, line.end.x as f32, line.end.y as f32, 1.0, BLACK);
        }



    }
    /*==The QuickDraw API==*/

    //moves the pen to the given cords.
    pub fn MoveTo(&mut self, x: i32, y: i32) {
        self.pen.x = x;
        self.pen.y = y;
    }

    //moves the pen relative to current cords
    pub fn Move(&mut self, x: i32, y: i32) {
        self.pen.x += x;
        self.pen.y += y;
    }

    //draws a line between the pens current position and the relative cords specified
    pub fn Line(&mut self, x: i32, y: i32) {
        self.render_queue.lines.push(
            RqLine {
                start: vec2(self.pen.x as f32, self.pen.y as f32),
                end: vec2(x as f32, y as f32),
            }
        );
        self.pen.x = x;
        self.pen.y = y;
    }

    //draws a line between the pens current position and the absolute cords specified
    pub fn LineTo(&mut self, x: i32, y: i32) {
        self.render_queue.lines.push(
            RqLine {
                start: vec2(self.pen.x as f32, self.pen.y as f32),
                end: vec2((self.pen.x + x) as f32, (self.pen.y + y) as f32),
            }
        );
        self.pen.x = x;
        self.pen.y = y;
    }

    //TODO: Implement this once proper sizing and scaling for the Macroquad window is builtin
    pub fn SetDrawingRect(){}

    //TODO: Implement this with SetDrawingRect as part of screen controls
    pub fn ShowDrawing(&mut self){}




}


