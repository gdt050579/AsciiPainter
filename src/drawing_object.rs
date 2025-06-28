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

pub struct LineObject {
    pub fore: Color,
    pub back: Color,
    pub line_type: LineType,
    pub vertical: bool,
}
impl Default for LineObject {
    fn default() -> Self {
        Self {
            fore: Color::White,
            back: Color::Black,
            line_type: LineType::Single,
            vertical: false,
        }
    }
}

pub struct TextObject {
    pub txt: String,
    pub fore: Color,
    pub back: Color,
    pub flags: CharFlags,
}
impl Default for TextObject {
    fn default() -> Self {
        Self {
            txt: String::new(),
            fore: Color::White,
            back: Color::Black,
            flags: CharFlags::None,
        }
    }
}

pub enum DrawingObject {
    Selection,
    Rectangle(RectangleObject),
    FillRectangle(FillRectangleObject),
    Line(LineObject),
    Text(TextObject),
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
            DrawingObject::Line(line) => {
                if line.vertical {
                    surface.draw_vertical_line(
                        rect.center_x(),
                        rect.top(),
                        rect.bottom(),
                        line.line_type,
                        CharAttribute::with_color(line.fore, line.back),
                    );
                } else {
                    surface.draw_horizontal_line(
                        rect.left(),
                        rect.center_y(),
                        rect.right(),
                        line.line_type,
                        CharAttribute::with_color(line.fore, line.back),
                    );
                }
            }
            DrawingObject::Text(text) => {
                let tf = TextFormatBuilder::new()
                    .position(rect.left(), rect.top())
                    .attribute(CharAttribute::new(text.fore,text.back,text.flags))
                    .wrap_type(WrapType::WordWrap(rect.width() as u16))
                    .build();
                surface.write_text(&text.txt, &tf);
            }
        }
    }
}
