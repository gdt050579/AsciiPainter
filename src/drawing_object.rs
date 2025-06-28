use appcui::prelude::*;

pub struct Rectangle {
    pub r: Rect,
    pub fore: Color,
    pub back: Color,
    pub line_type: LineType,
}

pub enum DrawingObject {
    Selection,
    Rectangle(Rectangle),
}

impl DrawingObject {
    pub fn paint(&self, surface: &mut Surface, theme: &Theme) {
        match self {
            DrawingObject::Selection => {}
            DrawingObject::Rectangle(rectangle) => {
                surface.draw_rect(
                    rectangle.r,
                    rectangle.line_type,
                    CharAttribute::with_color(rectangle.fore, rectangle.back),
                );
            }
        }
    }
}
