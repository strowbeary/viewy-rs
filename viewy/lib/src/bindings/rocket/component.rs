use std::fmt::format;

use rocket::{Data, Request, Route, async_trait, http::Method, outcome::Outcome, route::Handler};

use crate::Component;
/*
#[async_trait]
impl<T> Handler for T
where
    T: Component,
{
    async fn handle<'r, 'life0, 'life1>(
        &'life0 self,
        request: &'r Request<'life1>,
        data: Data<'r>,
    ) -> Outcome<'r>
    where
        'r: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}

impl<T> Into<Route> for T
where
    T: Component,
{
    fn into(self) -> Route {
        let component_name = T::name();
        let mut route = Route::new(
            Method::Post,
            &format!("/v/components/{}", component_name),
            self,
        );

        route
    }
}
 */
