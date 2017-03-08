use point::Point;

#[derive(Clone, Copy)]
pub struct Line {
    pub x0: i64,
    pub y0: i64,
    pub x1: i64,
    pub y1: i64,
}

impl Line {
    pub fn xyxy(x0: i64, y0: i64, x1: i64, y1: i64) -> Line {
        Line { x0: x0, y0: y0, x1: x1, y1: y1 }
    }

    pub fn reversed(self) -> Line {
        Line::xyxy(self.x1, self.y1, self.x0, self.y0)
    }

    pub fn startpoint(self) -> Point {
        Point::xy(self.x0, self.y0)
    }

    #[allow(dead_code)]
    pub fn endpoint(self) -> Point {
        Point::xy(self.x1, self.y1)
    }

    pub fn dx(self) -> i64 {
        return self.x1 - self.x0;
    }

    pub fn dy(self) -> i64 {
        return self.y1 - self.y0;
    }
}

