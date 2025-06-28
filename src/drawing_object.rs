use appcui::prelude::*;

pub struct RectangleObject {
    pub fore: Color,
    pub back: Color,
    pub line_type: LineType,
}

pub enum DrawingObject {
    Selection,
    Rectangle(RectangleObject),
}

impl DrawingObject {
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, rect: Rect) {
        match self {
            DrawingObject::Selection => {}
            DrawingObject::Rectangle(rectangle) => {
                surface.draw_rect(
                    rect,
                    rectangle.line_type,
                    CharAttribute::with_color(rectangle.fore, rectangle.back),
                );
            }
        }
    }
}
