use ppm;
use line::Line;
use point::Color;
use render;
use consts::*;

/// work2: Generate pretty line pattern using Bresenham's Line Algorithm (in line.rs).
pub fn run() {
    ppm::make_ppm(|image: &mut Vec<Vec<Color>>| {
        for i in 0..(IHEIGHT / 20) {
            // down-right lines
            render::line(
                image,
                Line::xyxy(0, i * 1, IWIDTH - 1, i * 19),
                Color::white());
            // down-left lines
            render::line(
                image,
                Line::xyxy(IWIDTH - 1, i * 1, 0, i * 19),
                Color::white());
            // up-right lines
            render::line(
                image,
                Line::xyxy(0, IHEIGHT - 1 - i * 1, IWIDTH - 1, IHEIGHT - 1 - i * 19),
                Color::white());
            // up-left lines
            render::line(
                image,
                Line::xyxy(IWIDTH - 1, IHEIGHT - 1 - i * 1, 0, IHEIGHT - 1 - i * 19),
                Color::white());
        }
    });
}

