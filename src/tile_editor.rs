use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint + OnMouseEvent + OnResize)]
pub struct TileEditor {
    tile: BitTileU128,
    scrollbars: ScrollBars,
    hovered: Option<(u32, u32)>,
}
impl TileEditor {
    pub fn new(size: Size) -> Self {
        Self {
            base: ControlBase::with_focus_overlay(Layout::fill()),
            tile: BitTileU128::new(size.width as u8, size.height as u8).unwrap(),
            scrollbars: ScrollBars::new(true),
            hovered: None,
        }
    }
    fn mouse_to_pos(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        let w = self.tile.width() as i32;
        let h = self.tile.height() as i32;
        let tx = (x - 2) / 3;
        let ty = (y - 1) / 2;
        if tx >= 0 && tx < w && ty >= 0 && ty < h {
            Some((tx as u32, ty as u32))
        } else {
            None
        }
    }
}
impl OnPaint for TileEditor {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.has_focus() {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        surface.clear(char!("' ',white,black"));
        let o = self.scrollbars.offset();
        surface.set_origin(o.x, o.y);

        let w = self.tile.width() as i32;
        let h = self.tile.height() as i32;
        let attr = charattr!("gray,black");
        for x in 0..=w {
            surface.draw_vertical_line_with_size(
                x * 3 + 2,
                1,
                (h * 2) as u32,
                LineType::Single,
                attr,
            );
        }
        for y in 0..=h {
            surface.draw_horizontal_line_with_size(
                2,
                y * 2 + 1,
                (w * 3) as u32,
                LineType::Single,
                attr,
            );
        }
        for x in 0..w {
            for y in 0..h {
                surface.write_char(
                    x * 3 + 2,
                    y * 2 + 1,
                    Character::with_attributes(SpecialChar::BoxCrossSingleLine, attr),
                );
                if self.tile.get(x as u32, y as u32).unwrap_or(false) {
                    surface.write_string(
                        x * 3 + 3,
                        y * 2 + 2,
                        "  ",
                        charattr!("white,white"),
                        false,
                    );
                }
            }
        }
        let ch_t = Character::with_attributes('┬', attr);
        let ch_b = Character::with_attributes('┴', attr);
        for x in 0..=w {
            surface.write_char(x * 3 + 2, 1, ch_t);
            surface.write_char(x * 3 + 2, h * 2 + 1, ch_b);
        }
        let ch_l = Character::with_attributes('├', attr);
        let ch_r = Character::with_attributes('┤', attr);
        for y in 0..=h {
            surface.write_char(2, y * 2 + 1, ch_l);
            surface.write_char(w * 3 + 2, y * 2 + 1, ch_r);
        }
        surface.write_char(2, 1, Character::with_attributes('┌', attr));
        surface.write_char(w * 3 + 2, 1, Character::with_attributes('┐', attr));
        surface.write_char(2, h * 2 + 1, Character::with_attributes('└', attr));
        surface.write_char(w * 3 + 2, h * 2 + 1, Character::with_attributes('┘', attr));
        if let Some((hx, hy)) = self.hovered {
            let r = Rect::with_size(hx as i32 * 3 + 2, hy as i32 * 2 + 1, 4, 3);
            surface.draw_rect(r, LineType::Single, charattr!("yellow,black"));
        }
    }
}
impl OnResize for TileEditor {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        let w = self.tile.width() as i32;
        let h = self.tile.height() as i32;
        self.scrollbars
            .resize((w * 3 + 3) as u64, (h * 2 + 2) as u64, &self.base);
    }
}

impl OnMouseEvent for TileEditor {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.hovered = None;
                EventProcessStatus::Processed
            }
            MouseEvent::Over(point) => {
                let x = point.x - self.scrollbars.offset().x;
                let y = point.y - self.scrollbars.offset().y;
                let new_hovered = self.mouse_to_pos(x, y);
                if new_hovered != self.hovered {
                    self.hovered = new_hovered;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Pressed(mouse_event_data) => {
                let x = mouse_event_data.x - self.scrollbars.offset().x;
                let y = mouse_event_data.y - self.scrollbars.offset().y;
                if let Some((tx, ty)) = self.mouse_to_pos(x, y) {
                    if mouse_event_data.button == MouseButton::Left {
                        let _ = self.tile.set(tx, ty, true);
                    } else if mouse_event_data.button == MouseButton::Right {
                        let _ = self.tile.set(tx, ty, false);
                    }
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
