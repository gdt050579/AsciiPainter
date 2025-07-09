use std::path::Path;

use appcui::prelude::*;

use crate::drawing_object::SelectionObject;

use super::DrawingObject;
use super::Selection;

#[CustomControl(overwrite = OnPaint + OnMouseEvent + OnResize + OnKeyPressed)]
pub struct PainterControl {
    surface: Surface,
    scrollbars: ScrollBars,
    selection: Selection,
    drawwing_object: DrawingObject,
    is_move_dragging: bool,
    move_drag_start: Point,
    move_drag_initial_offset: Point,
    undo_stack: Vec<Surface>,
    redo_stack: Vec<Surface>,
    max_undo_levels: usize,
    clipboard: Option<Surface>,
}

impl PainterControl {
    pub fn new(width: u32, height: u32) -> Self {
        let mut me = Self {
            base: ControlBase::with_focus_overlay(Layout::new("d:c")),
            surface: Surface::new(width, height),
            scrollbars: ScrollBars::new(true),
            selection: Selection::new(true),
            drawwing_object: DrawingObject::Selection(SelectionObject::default()),
            is_move_dragging: false,
            move_drag_start: Point::new(0, 0),
            move_drag_initial_offset: Point::new(0, 0),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_undo_levels: 50,
            clipboard: None,
        };
        me.set_components_toolbar_margins(3, 5);
        me.scrollbars.resize(
            me.surface.size().width as u64,
            me.surface.size().height as u64,
            &me.base,
        );
        me.save_state();
        me
    }
    pub fn from_path(path: &Path) -> Option<Self> {
        if let Ok(surface) = Surface::from_file(path) {
            let mut me = Self {
                base: ControlBase::with_focus_overlay(Layout::new("d:c")),
                surface,
                scrollbars: ScrollBars::new(true),
                selection: Selection::new(true),
                drawwing_object: DrawingObject::Selection(SelectionObject::default()),
                is_move_dragging: false,
                move_drag_start: Point::new(0, 0),
                move_drag_initial_offset: Point::new(0, 0),
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),
                max_undo_levels: 50,
                clipboard: None,
            };
            me.set_components_toolbar_margins(3, 5);
            me.scrollbars.resize(
                me.surface.size().width as u64,
                me.surface.size().height as u64,
                &me.base,
            );
            me.save_state();
            Some(me)
        } else {
            None
        }
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
        let is_selecton = matches!(d, DrawingObject::Selection(_));
        self.selection = Selection::new(!is_selecton);
        self.drawwing_object = d;
    }
    pub fn update_rectangle_properties(&mut self, fore: Color, back: Color, line_type: LineType) {
        if let DrawingObject::Rectangle(ref mut rect) = self.drawwing_object {
            rect.fore = fore;
            rect.back = back;
            rect.line_type = line_type;
        }
    }
    pub fn update_fillrectangle_properties(
        &mut self,
        fore: Color,
        back: Color,
        ch: char,
        flags: CharFlags,
    ) {
        if let DrawingObject::FillRectangle(ref mut fill_rect) = self.drawwing_object {
            fill_rect.fore = fore;
            fill_rect.back = back;
            fill_rect.ch = ch;
            fill_rect.flags = flags;
        }
    }
    pub fn update_line_properties(
        &mut self,
        fore: Color,
        back: Color,
        line_type: LineType,
        vertical: bool,
    ) {
        if let DrawingObject::Line(ref mut line) = self.drawwing_object {
            line.fore = fore;
            line.back = back;
            line.line_type = line_type;
            line.vertical = vertical;
        }
    }
    pub fn update_text_properties(
        &mut self,
        txt: String,
        fore: Color,
        back: Color,
        flags: CharFlags,
    ) {
        if let DrawingObject::Text(ref mut text) = self.drawwing_object {
            text.txt = txt;
            text.fore = fore;
            text.back = back;
            text.flags = flags;
        }
    }
    pub fn write_current_object(&mut self) {
        if self.selection.is_visible() {
            self.save_state();

            self.drawwing_object
                .paint(&mut self.surface, self.selection.rect());
            self.drawwing_object.clear();
            self.selection.reset();
        }
    }
    pub fn cancel_selection(&mut self) {
        if self.selection.is_visible() {
            self.drawwing_object.clear();
            self.selection.reset();
        }
    }

    pub fn save_state(&mut self) {
        let size = self.surface.size();
        let mut surface_copy = Surface::new(size.width, size.height);

        for y in 0..size.height as i32 {
            for x in 0..size.width as i32 {
                if let Some(ch) = self.surface.char(x, y) {
                    surface_copy.write_char(x, y, *ch);
                }
            }
        }

        self.undo_stack.push(surface_copy);
        self.redo_stack.clear();

        if self.undo_stack.len() > self.max_undo_levels {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some(previous_surface) = self.undo_stack.pop() {
            let size = self.surface.size();
            let mut current_surface_copy = Surface::new(size.width, size.height);

            for y in 0..size.height as i32 {
                for x in 0..size.width as i32 {
                    if let Some(ch) = self.surface.char(x, y) {
                        current_surface_copy.write_char(x, y, *ch);
                    }
                }
            }

            self.redo_stack.push(current_surface_copy);

            if self.redo_stack.len() > self.max_undo_levels {
                self.redo_stack.remove(0);
            }

            self.surface = previous_surface;
            self.selection.reset();
            self.drawwing_object.clear();
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next_surface) = self.redo_stack.pop() {
            let size = self.surface.size();
            let mut current_surface_copy = Surface::new(size.width, size.height);

            for y in 0..size.height as i32 {
                for x in 0..size.width as i32 {
                    if let Some(ch) = self.surface.char(x, y) {
                        current_surface_copy.write_char(x, y, *ch);
                    }
                }
            }

            self.undo_stack.push(current_surface_copy);

            if self.undo_stack.len() > self.max_undo_levels {
                self.undo_stack.remove(0);
            }

            self.surface = next_surface;
            self.selection.reset();
            self.drawwing_object.clear();
            true
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn copy_selection(&mut self) {
        if self.selection.is_visible()
            && matches!(self.drawwing_object, DrawingObject::Selection(_))
        {
            let rect = self.selection.rect();
            let mut clipboard_surface = Surface::new(rect.width(), rect.height());

            for y in 0..rect.height() as i32 {
                for x in 0..rect.width() as i32 {
                    if let Some(ch) = self.surface.char(rect.left() + x, rect.top() + y) {
                        clipboard_surface.write_char(x, y, *ch);
                    }
                }
            }

            self.clipboard = Some(clipboard_surface);
        }
    }

    pub fn paste_from_clipboard(&mut self) {
        if self.clipboard.is_none() || !self.selection.is_visible() {
            return;
        }

        self.save_state();
        let rect = self.selection.rect();

        // Get clipboard dimensions first
        let (clipboard_width, clipboard_height) = if let Some(clipboard_surface) = &self.clipboard {
            (
                clipboard_surface.size().width,
                clipboard_surface.size().height,
            )
        } else {
            return;
        };

        // Paste the clipboard content at the selection position
        for y in 0..clipboard_height as i32 {
            for x in 0..clipboard_width as i32 {
                let target_x = rect.left() + x;
                let target_y = rect.top() + y;

                // Check bounds
                if target_x >= 0
                    && target_y >= 0
                    && target_x < self.surface.size().width as i32
                    && target_y < self.surface.size().height as i32
                {
                    if let Some(clipboard_surface) = &self.clipboard {
                        if let Some(ch) = clipboard_surface.char(x, y) {
                            self.surface.write_char(target_x, target_y, *ch);
                        }
                    }
                }
            }
        }

        // Clear the selection after pasting
        self.selection.reset();
        self.drawwing_object.clear();
    }

    fn adjust_mouse_event_for_scroll(&self, event: &MouseEvent) -> MouseEvent {
        let offset = self.scrollbars.offset();
        match event {
            MouseEvent::Pressed(data) => MouseEvent::Pressed(MouseEventData {
                x: data.x - offset.x,
                y: data.y - offset.y,
                button: data.button,
                modifier: data.modifier,
            }),
            MouseEvent::Released(data) => MouseEvent::Released(MouseEventData {
                x: data.x - offset.x,
                y: data.y - offset.y,
                button: data.button,
                modifier: data.modifier,
            }),
            MouseEvent::Drag(data) => MouseEvent::Drag(MouseEventData {
                x: data.x - offset.x,
                y: data.y - offset.y,
                button: data.button,
                modifier: data.modifier,
            }),
            _ => *event,
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
        let is_move_mode = matches!(self.drawwing_object, DrawingObject::Move(_));

        match event {
            MouseEvent::Pressed(data) if is_move_mode => {
                self.is_move_dragging = true;
                self.move_drag_start = Point::new(data.x, data.y);
                self.move_drag_initial_offset = Point::new(
                    self.scrollbars.horizontal_index() as i32,
                    self.scrollbars.vertical_index() as i32,
                );
                return EventProcessStatus::Processed;
            }
            MouseEvent::Drag(data) if self.is_move_dragging => {
                let dx = self.move_drag_start.x - data.x;
                let dy = self.move_drag_start.y - data.y;

                let new_h_index = (self.move_drag_initial_offset.x + dx).max(0) as u64;
                let new_v_index = (self.move_drag_initial_offset.y + dy).max(0) as u64;

                self.scrollbars.set_indexes(new_h_index, new_v_index);
                return EventProcessStatus::Processed;
            }
            MouseEvent::Released(_) if self.is_move_dragging => {
                self.is_move_dragging = false;
                return EventProcessStatus::Processed;
            }
            _ => {}
        }

        if self.is_move_dragging {
            return EventProcessStatus::Processed;
        }

        if self.scrollbars.process_mouse_event(event) {
            return EventProcessStatus::Processed;
        }

        if is_move_mode {
            return EventProcessStatus::Processed;
        }

        let adjusted_event = self.adjust_mouse_event_for_scroll(event);
        let during_creation = self.selection.is_during_creation();
        if self.selection.process_mouse_event(&adjusted_event) {
            if during_creation && self.selection.is_visible() {
                // tocmai am creat o selectie noua
                self.drawwing_object
                    .on_finish_selection(&self.surface, self.selection.rect());
            }
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
            key!("Escape") => {
                self.cancel_selection();
                EventProcessStatus::Processed
            }
            key!("Enter") => {
                self.write_current_object();
                EventProcessStatus::Processed
            }
            key!("Ctrl+Z") => {
                if self.undo() {
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            key!("Ctrl+Shift+Z") => {
                if self.redo() {
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            key!("Ctrl+Y") => {
                if self.redo() {
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            key!("Ctrl+C") => {
                if self.selection.is_visible()
                    && matches!(self.drawwing_object, DrawingObject::Selection(_))
                {
                    self.copy_selection();
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            key!("Ctrl+V") => {
                if self.clipboard.is_some() && self.selection.is_visible() {
                    self.paste_from_clipboard();
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnResize for PainterControl {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.scrollbars.resize(
            self.surface.size().width as u64,
            self.surface.size().height as u64,
            &self.base,
        );
    }
}
