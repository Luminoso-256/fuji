mod system;
//Handles APIs built into the Mac OS rom or system (ex. QuickDraw's units)
mod api;

use macroquad::prelude::*;
use std::io::{self, Read};
use crate::api::quickdraw::{Pen, RenderQueue, QuickDraw};

fn readstr() ->  String{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {}
        Err(error) => println!("error: {}", error),
    }
    return input.replace("\n", "");
}


fn main() {
    println!("Welcome to *Fuji* | A compatability layer for very old Mac OS programs");
    println!("Navigate around the filesystem provided below. \n to execute a program, use the run command");
    loop{
        println!("fuji> ");
        let mut input = readstr();
        let command = input.as_str();
        match command {
            "test>gui" => {

                macroquad::Window::new("Fuji | GUI Window", gui_main());
            },
            "test>hfs" => {
                crate::system::hfs::read_disk("C:\\Users\\Lumen\\Downloads\\Apps4\\apps4.dsk")
            }
            _ => println!("[Err] Invalid command given")
        }
    }
}
async fn gui_main(){
    let mut qd = api::quickdraw::QuickDraw{
        pen: Pen{x:0,y:0},
        render_queue:RenderQueue{ lines: vec![] }
    };
    qd.Line(30,30);
    loop{
        qd.int_frame();
        next_frame().await
    }
}
