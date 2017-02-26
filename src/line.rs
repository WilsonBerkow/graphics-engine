use point::Point;

#[derive(Clone, Copy)]
pub struct Line {
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
}

impl Line {
    pub fn origin_to(x: usize, y: usize) -> Line {
        Line { x0: 0, y0: 0, x1: x, y1: y }
    }

    pub fn from_to(p0: Point, p1: Point) -> Line {
        Line { x0: p0.x, y0: p0.y, x1: p1.x, y1: p1.y }
    }

    pub fn xyxy(x0: usize, y0: usize, x1: usize, y1: usize) -> Line {
        Line { x0: x0, y0: y0, x1: x1, y1: y1 }
    }

    pub fn from_by(x: usize, y: usize, dx: i64, dy: i64) -> Line {
        Line {
            x0: x,
            y0: y,
            x1: (x as i64 + dx) as usize,
            y1: (y as i64 + dy) as usize,
        }
    }

    pub fn reversed(self) -> Line {
        Line::xyxy(self.x1, self.y1, self.x0, self.y0)
    }

    pub fn startpoint(self) -> Point {
        Point::xy(self.x0, self.y0)
    }

    pub fn endpoint(self) -> Point {
        Point::xy(self.x1, self.y1)
    }

    pub fn dx(self) -> i64 {
        return self.x1 as i64 - self.x0 as i64;
    }

    pub fn dy(self) -> i64 {
        return self.y1 as i64 - self.y0 as i64;
    }
}

