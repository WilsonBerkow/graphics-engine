use ppm;
use line::Line;
use point::Color;
use render;
use consts::*;

/// work2: Generate pretty line pattern using Bresenham's Line Algorithm (in line.rs).
pub fn run() {
    ppm::make_ppm(|image: &mut Vec<Vec<Color>>| {
        for i in 0..(HEIGHT / 10) {
            render::line(
                image,
                Line::xyxy(WIDTH / 2, HEIGHT - 1, 0, i * 10),
                Color::rgb(255, 0, 0));
        }
        for i in 0..(HEIGHT / 10) {
            render::line(
                image,
                Line::xyxy(WIDTH / 2, HEIGHT - 1, WIDTH - 1, i * 10),
                Color::rgb(255, 0, 0));
        }
        for i in 0..(HEIGHT / 20) {
            // down-right lines
            render::line(
                image,
                Line::xyxy(0, i * 1, WIDTH - 1, i * 19),
                Color::white());
            // down-left lines
            render::line(
                image,
                Line::xyxy(WIDTH - 1, i * 1, 0, i * 19),
                Color::white());
            // up-right lines
            render::line(
                image,
                Line::xyxy(0, HEIGHT - 1 - i * 1, WIDTH - 1, HEIGHT - 1 - i * 19),
                Color::white());
            // up-left lines
            render::line(
                image,
                Line::xyxy(WIDTH - 1, HEIGHT - 1 - i * 1, 0, HEIGHT - 1 - i * 19),
                Color::white());
        }
        for i in 0..(WIDTH / 10) {
            render::line(
                image,
                Line::xyxy(WIDTH / 2, HEIGHT - 1, i * 10, 0),
                Color::rgb(255, 0, 0));
        }
    });
}

