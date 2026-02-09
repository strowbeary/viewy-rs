use crate::Widget;
pub trait Sizable: Widget {
    fn width(&mut self, width: u32) -> &mut Self {
        self
    }
    fn height(&mut self, height: u32) -> &mut Self {
        self
    }
    fn aspect_ratio(&mut self, aspect_ratio: f32) -> &mut Self {
        self
    }
}
