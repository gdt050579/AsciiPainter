use appcui::prelude::*;

enum Status {
    None,
    DuringCreation,
    Drag,
    ResizeTopLeft,
    ResizeTopRight,
    ResizeBottomLeft,
    ResizeBottomRight,
    ResizeUpperMargin,
    ResizeLowerMargin,
    ResizeLeftMargin,
    ResizeRightMargin,
    Visible,
}

enum MousePosInRect {
    Inside,
    Outside,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    LeftMargin,
    RightMargin,
    TopMargin,
    BottomMargin,
}

pub struct Selection {
    r: Rect,
    status: Status,
    start_point: Point,
}

impl Selection {
    pub(crate) fn new() -> Self {
        Self {
            r: Rect::new(0, 0, 0, 0),
            status: Status::None,
            start_point: Point::new(0, 0),
        }
    }
    pub(crate) fn is_visible(&self) -> bool {
        !matches!(self.status, Status::None)
    }
    pub(crate) fn is_during_creation(&self) -> bool {
        matches!(self.status, Status::DuringCreation)
    }
    pub(crate) fn rect(&self) -> Rect {
        Rect::new(
            self.r.left() + 1,
            self.r.top() + 1,
            self.r.right() - 1,
            self.r.bottom() - 1,
        )
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        let r = self.r;
        let ch = char!(".,gray,black");
        let marker = Character::new(
            SpecialChar::BlockCentered,
            Color::Yellow,
            Color::Black,
            CharFlags::None,
        );
        surface.fill_horizontal_line(r.left(), r.top(), r.right(), ch);
        surface.fill_horizontal_line(r.left(), r.bottom(), r.right(), ch);
        surface.fill_vertical_line(r.left(), r.top(), r.bottom(), ch);
        surface.fill_vertical_line(r.right(), r.top(), r.bottom(), ch);
        surface.write_char(r.left(), r.top(), marker);
        surface.write_char(r.left(), r.bottom(), marker);
        surface.write_char(r.right(), r.top(), marker);
        surface.write_char(r.right(), r.bottom(), marker);
        surface.write_char(r.center_x(), r.top(), marker);
        surface.write_char(r.center_x(), r.bottom(), marker);
        surface.write_char(r.left(), r.center_y(), marker);
        surface.write_char(r.right(), r.center_y(), marker);
    }
    fn mouse_pos_in_rect(&self, point: Point) -> MousePosInRect {
        let r = self.r;
        if point.x >= r.left()
            && point.x <= r.right()
            && point.y >= r.top()
            && point.y <= r.bottom()
        {
            if point.x == r.left() && point.y == r.top() {
                MousePosInRect::TopLeft
            } else if point.x == r.right() && point.y == r.top() {
                MousePosInRect::TopRight
            } else if point.x == r.left() && point.y == r.bottom() {
                MousePosInRect::BottomLeft
            } else if point.x == r.right() && point.y == r.bottom() {
                MousePosInRect::BottomRight
            } else if point.x == r.center_x() && point.y == r.top() {
                MousePosInRect::TopMargin
            } else if point.x == r.center_x() && point.y == r.bottom() {
                MousePosInRect::BottomMargin
            } else if point.y == r.center_y() && point.x == r.left() {
                MousePosInRect::LeftMargin
            } else if point.y == r.center_y() && point.x == r.right() {
                MousePosInRect::RightMargin
            } else {
                MousePosInRect::Inside
            }
        } else {
            MousePosInRect::Outside
        }
    }
    fn on_mouse_pressed(&mut self, data: &MouseEventData) -> bool {
        let p = Point::new(data.x, data.y);
        match self.status {
            Status::None => {
                self.start_point = p;
                self.status = Status::DuringCreation;
                self.r = Rect::new(p.x, p.y, p.x, p.y);
                true
            }
            Status::Visible => match self.mouse_pos_in_rect(p) {
                MousePosInRect::Inside => {
                    self.status = Status::Drag;
                    self.start_point = Point::new(p.x - self.r.left(), p.y - self.r.top());
                    true
                }
                MousePosInRect::TopLeft => {
                    self.status = Status::ResizeTopLeft;
                    true
                }
                MousePosInRect::TopRight => {
                    self.status = Status::ResizeTopRight;
                    true
                }
                MousePosInRect::BottomLeft => {
                    self.status = Status::ResizeBottomLeft;
                    true
                }
                MousePosInRect::BottomRight => {
                    self.status = Status::ResizeBottomRight;
                    true
                }
                MousePosInRect::LeftMargin => {
                    self.status = Status::ResizeLeftMargin;
                    true
                }
                MousePosInRect::RightMargin => {
                    self.status = Status::ResizeRightMargin;
                    true
                }
                MousePosInRect::TopMargin => {
                    self.status = Status::ResizeUpperMargin;
                    true
                }
                MousePosInRect::BottomMargin => {
                    self.status = Status::ResizeLowerMargin;
                    true
                }
                MousePosInRect::Outside => false,
            },
            _ => false,
        }
    }
    fn on_mouse_drag(&mut self, data: &MouseEventData) -> bool {
        match self.status {
            Status::DuringCreation => {
                let l = self.start_point.x.min(data.x);
                let r = self.start_point.x.max(data.x);
                let t = self.start_point.y.min(data.y);
                let b = self.start_point.y.max(data.y);
                self.r = Rect::new(l, t, r, b);
                true
            }
            Status::Drag => {
                let x = data.x - self.start_point.x;
                let y = data.y - self.start_point.y;
                let w = self.r.width() as u16;
                let h = self.r.height() as u16;
                self.r = Rect::with_size(x, y, w, h);
                true
            }
            Status::ResizeTopLeft => {
                let x = data.x.min(self.r.right() - 2);
                let y = data.y.min(self.r.bottom() - 2);
                self.r = Rect::new(x, y, self.r.right(), self.r.bottom());
                true
            }
            Status::ResizeTopRight => {
                let x = data.x.max(self.r.left() + 2);
                let y = data.y.min(self.r.bottom() - 2);
                self.r = Rect::new(self.r.left(), y, x, self.r.bottom());
                true
            }
            Status::ResizeBottomLeft => {
                let x = data.x.min(self.r.right() - 2);
                let y = data.y.max(self.r.top() + 2);
                self.r = Rect::new(x, self.r.top(), self.r.right(), y);
                true
            }
            Status::ResizeBottomRight => {
                let x = data.x.max(self.r.left() + 2);
                let y = data.y.max(self.r.top() + 2);
                self.r = Rect::new(self.r.left(), self.r.top(), x, y);
                true
            }
            Status::ResizeUpperMargin => {
                let y = data.y.min(self.r.bottom() - 2);
                self.r = Rect::new(self.r.left(), y, self.r.right(), self.r.bottom());
                true
            }
            Status::ResizeLowerMargin => {
                let y = data.y.max(self.r.top() + 2);
                self.r = Rect::new(self.r.left(), self.r.top(), self.r.right(), y);
                true
            }
            Status::ResizeLeftMargin => {
                let x = data.x.min(self.r.right() - 2);
                self.r = Rect::new(x, self.r.top(), self.r.right(), self.r.bottom());
                true
            }
            Status::ResizeRightMargin => {
                let x = data.x.max(self.r.left() + 2);
                self.r = Rect::new(self.r.left(), self.r.top(), x, self.r.bottom());
                true
            }
            _ => false,
        }
    }
    fn on_mouse_released(&mut self, data: &MouseEventData) -> bool {
        match self.status {
            Status::DuringCreation => {
                let r = self.r;
                self.r = Rect::new(r.left() - 1, r.top() - 1, r.right() + 1, r.bottom() + 1);
                self.status = Status::Visible;
                true
            }
            Status::Drag
            | Status::ResizeBottomLeft
            | Status::ResizeTopLeft
            | Status::ResizeTopRight
            | Status::ResizeBottomRight
            | Status::ResizeLeftMargin
            | Status::ResizeRightMargin
            | Status::ResizeLowerMargin
            | Status::ResizeUpperMargin => {
                self.status = Status::Visible;
                true
            }
            Status::Visible => false,
            Status::None => false,
        }
    }
    pub(crate) fn process_mouse_event(&mut self, evnt: &MouseEvent) -> bool {
        match evnt {
            MouseEvent::Pressed(data) => self.on_mouse_pressed(data),
            MouseEvent::Drag(data) => self.on_mouse_drag(data),
            MouseEvent::Released(data) => self.on_mouse_released(data),
            _ => false,
        }
    }
}
