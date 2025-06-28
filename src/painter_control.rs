use std::path::Path;

use appcui::prelude::*;

use super::DrawingObject;
use super::Selection;

#[CustomControl(overwrite = OnPaint + OnMouseEvent + OnResize + OnKeyPressed)]
pub struct PainterControl {
    surface: Surface,
    scrollbars: ScrollBars,
    selection: Selection,
    drawwing_object: DrawingObject,
}

impl PainterControl {
    pub fn new(width: u32, height: u32) -> Self {
        let mut me = Self {
            base: ControlBase::with_focus_overlay(Layout::new("d:c")),
            surface: Surface::new(width, height),
            scrollbars: ScrollBars::new(true),
            selection: Selection::new(),
            drawwing_object: DrawingObject::Selection,
        };
        me.set_components_toolbar_margins(3, 5);
        me
    }

    pub fn clear_surface(&mut self) {
        self.surface.clear(char!("' ',black,black"));
    }

    pub fn load_from_file(&mut self, file: &Path) -> Result<(), String> {
        if let Ok(surface) = Surface::from_file(file) {
            self.surface = surface;
            self.scrollbars.resize(
                self.surface.size().width as u64,
                self.surface.size().height as u64,
                &self.base,
            );
            Ok(())
        } else {
            Err(format!(
                "Failed to load surface from file '{}'",
                file.display()
            ))
        }
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        self.surface
            .save(path)
            .map_err(|e| format!("Failed to save surface to file '{}': {}", path.display(), e))
    }
    pub fn reset(&mut self, d: DrawingObject) {
        self.selection = Selection::new();
        self.drawwing_object = d;
    }
    pub fn update_rectangle_properties(&mut self, fore: Color, back: Color, line_type: LineType) {
        if let DrawingObject::Rectangle(ref mut rect) = self.drawwing_object {
            rect.fore = fore;
            rect.back = back;
            rect.line_type = line_type;
        }
    }
    pub fn update_fillrectangle_properties(&mut self, fore: Color, back: Color, ch: char, flags: CharFlags) {
        if let DrawingObject::FillRectangle(ref mut fill_rect) = self.drawwing_object {
            fill_rect.fore = fore;
            fill_rect.back = back;
            fill_rect.ch = ch;
            fill_rect.flags = flags;
        }
    }
    pub fn write_current_object(&mut self) {
        if self.selection.is_visible() {
            self.drawwing_object
                .paint(&mut self.surface, self.selection.rect());
            self.selection = Selection::new();
        }
    }
}

impl OnPaint for PainterControl {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.has_focus() {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        let o = self.scrollbars.offset();
        surface.draw_surface(o.x, o.y, &self.surface);
        surface.set_origin(o.x, o.y);
        if self.selection.is_visible() {
            self.drawwing_object.paint(surface, self.selection.rect());
        }
        self.selection.paint(surface, theme);
    }
}

impl OnMouseEvent for PainterControl {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            return EventProcessStatus::Processed;
        }
        if self.selection.process_mouse_event(event) {
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Released(_) => {
                self.write_current_object();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnKeyPressed for PainterControl {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") => {
                self.scrollbars.set_indexes(
                    self.scrollbars.horizontal_index(),
                    self.scrollbars.vertical_index().saturating_sub(1),
                );
                EventProcessStatus::Processed
            }
            key!("Down") => {
                self.scrollbars.set_indexes(
                    self.scrollbars.horizontal_index(),
                    self.scrollbars.vertical_index() + 1,
                );
                EventProcessStatus::Processed
            }
            key!("Left") => {
                self.scrollbars.set_indexes(
                    self.scrollbars.horizontal_index().saturating_sub(1),
                    self.scrollbars.vertical_index(),
                );
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.scrollbars.set_indexes(
                    self.scrollbars.horizontal_index() + 1,
                    self.scrollbars.vertical_index(),
                );
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnResize for PainterControl {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.scrollbars.resize(100, 100, &self.base);
    }
}
