use crate::Widget;
/// Legacy sizing trait placeholder.
///
/// Prefer [`crate::modifiers::Dimensionable`] for concrete CSS sizing APIs.
pub trait Sizable: Widget {
    /// Set the widget width.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Icon::new(Lucide::Search).width(24);
    /// ```
    fn width(&mut self, width: u32) -> &mut Self {
        self
    }
    /// Set the widget height.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Icon::new(Lucide::Search).height(24);
    /// ```
    fn height(&mut self, height: u32) -> &mut Self {
        self
    }
    /// Set the widget aspect ratio.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Icon::new(Lucide::Search).aspect_ratio(1.0);
    /// ```
    fn aspect_ratio(&mut self, aspect_ratio: f32) -> &mut Self {
        self
    }
}
