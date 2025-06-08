use crate::node::Node;

pub trait Renderable {
    fn render(self) -> Node;

    fn component_name(&self) -> &str {
        "View"
    }
}

impl<R> Renderable for &R
where
    R: Renderable + Clone,
{
    fn render(self) -> Node {
        self.clone().render()
    }
}

impl<R> Renderable for &mut R
where
    R: Renderable + Clone,
{
    fn render(self) -> Node {
        self.clone().render()
    }
}

impl<R> Renderable for Box<R>
where
    R: Renderable + Clone,
{
    fn render(self) -> Node {
        (*self).render()
    }
}

pub trait IntoPopup {
    fn render(self) -> Node;
    fn attach_to(&mut self, el: &str) -> &mut Self;
}

impl<R> IntoPopup for &R
where
    R: IntoPopup + Clone,
{
    fn render(self) -> Node {
        self.clone().render()
    }

    fn attach_to(&mut self, el: &str) -> &mut Self {
        self.attach_to(el)
    }
}

impl<R> IntoPopup for &mut R
where
    R: IntoPopup + Clone,
{
    fn render(self) -> Node {
        self.clone().render()
    }

    fn attach_to(&mut self, el: &str) -> &mut Self {
        (**self).attach_to(el);
        self
    }
}

impl<R> IntoPopup for Box<R>
where
    R: IntoPopup + Clone,
{
    fn render(self) -> Node {
        (*self).render()
    }

    fn attach_to(&mut self, el: &str) -> &mut Self {
        (*self).attach_to(el);
        self
    }
}

pub trait IntoPopover {
    fn render(self) -> Node;
    fn attach_to(&mut self, el: &str) -> &mut Self;
}

impl<R> IntoPopover for &R
where
    R: IntoPopover + Clone,
{
    fn render(self) -> Node {
        self.clone().render()
    }

    fn attach_to(&mut self, el: &str) -> &mut Self {
        (*self).attach_to(el);
        self
    }
}

impl<R> IntoPopover for &mut R
where
    R: IntoPopover + Clone,
{
    fn render(self) -> Node {
        self.clone().render()
    }

    fn attach_to(&mut self, el: &str) -> &mut Self {
        (*self).attach_to(el);
        self
    }
}

impl<R> IntoPopover for Box<R>
where
    R: IntoPopover + Clone,
{
    fn render(self) -> Node {
        (*self).render()
    }

    fn attach_to(&mut self, el: &str) -> &mut Self {
        (*self).attach_to(el);
        self
    }
}
