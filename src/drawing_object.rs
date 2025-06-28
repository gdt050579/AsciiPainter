use appcui::prelude::*;

pub struct RectangleObject {
    pub fore: Color,
    pub back: Color,
    pub line_type: LineType,
}

impl Default for RectangleObject {
    fn default() -> Self {
        Self {
            fore: Color::White,
            back: Color::Black,
            line_type: LineType::Single,
        }
    }
}

pub struct FillRectangleObject {
    pub fore: Color,
    pub back: Color,
    pub ch: char,
    pub flags: CharFlags,
}

impl Default for FillRectangleObject {
    fn default() -> Self {
        Self {
            fore: Color::White,
            back: Color::Black,
            ch: ' ',
            flags: CharFlags::None,
        }
    }
}

pub enum DrawingObject {
    Selection,
    Rectangle(RectangleObject),
    FillRectangle(FillRectangleObject),
}

impl DrawingObject {
    pub fn paint(&self, surface: &mut Surface, rect: Rect) {
        match self {
            DrawingObject::Selection => {}
            DrawingObject::Rectangle(rectangle) => {
                surface.draw_rect(
                    rect,
                    rectangle.line_type,
                    CharAttribute::with_color(rectangle.fore, rectangle.back),
                );
            }
            DrawingObject::FillRectangle(fill_rect) => {
                surface.fill_rect(
                    rect,
                    Character::new(
                        fill_rect.ch,
                        fill_rect.fore,
                        fill_rect.back,
                        fill_rect.flags,
                    ),
                );
            }
        }
    }
}
