use rocket::response::Redirect;
use viewy::prelude::*;
use viewy::widgets::nav::{Nav, NavItem, NavLevel, NavOrientation};

use crate::ui::layouts::default_layout::default_layout;

#[derive(Clone, Copy)]
enum NavSection {
    Code,
    Issues,
    PullRequests,
}

impl NavSection {
    fn title(self) -> &'static str {
        match self {
            Self::Code => "Code",
            Self::Issues => "Issues",
            Self::PullRequests => "Pull requests",
        }
    }

    fn body(self) -> &'static str {
        match self {
            Self::Code => "This page demonstrates how the navigation highlights the current route.",
            Self::Issues => {
                "Switch between pages to verify that the active item follows the browser URL."
            }
            Self::PullRequests => {
                "Each route re-renders the same nav structure while the active state is resolved client-side."
            }
        }
    }
}

fn build_nav(orientation: NavOrientation, level: NavLevel, section: &NavSection) -> VStack {
    let mut stack = VStack::new(Alignment::Stretch);
    stack
        .as_card(CardStyle::OutlinedRaised)
        .append_child({
            let mut nav = Nav::new(orientation);
            nav.level(level)
                .add_item({
                    let mut item = NavItem::new("Code", uri!(nav_code()));
                    item.icon(Lucide::Code);
                    item
                })
                .add_item({
                    let mut item = NavItem::new("Issues", uri!(nav_issues()));
                    item.icon(Lucide::CircleDot);
                    item
                })
                .add_item({
                    let mut item = NavItem::new("Pull requests", uri!(nav_pull_requests()));
                    item.icon(Lucide::GitPullRequestArrow);
                    item
                });

            nav
        })
        .append_child({
            let mut content = VStack::new(Alignment::Stretch);
            content
                .gap(vec![scale(3)])
                .padding(vec![scale(5)])
                .append_child(Text::new(section.title(), TextStyle::H2))
                .append_child(Text::new(section.body(), TextStyle::Body));
            content
        });
    stack
}

fn nav_demo_page(section: NavSection) -> Page<'static> {
    Page::with_title("Nav demo").with_layout(default_layout()).with_content({
        let mut stack = VStack::new(Alignment::Stretch);
        stack
            .gap(vec![scale(5)])
            .append_child(Text::new("Navigation", TextStyle::H1))
            .append_child(Text::new(
                "This example uses three distinct routes so the active state can be observed during real page navigation.",
                TextStyle::Body,
            ))
            .append_child(Text::new("Primary / Horizontal", TextStyle::Label))
            .append_child(build_nav(NavOrientation::Horizontal, NavLevel::Primary, &section))
            .append_child(Text::new("Primary / Vertical", TextStyle::Label))
            .append_child(build_nav(NavOrientation::Vertical, NavLevel::Primary, &section))
            .append_child(Text::new("Secondary / Horizontal", TextStyle::Label))
            .append_child(build_nav(NavOrientation::Horizontal, NavLevel::Secondary, &section))
            .append_child(Text::new("Secondary / Vertical", TextStyle::Label))
            .append_child(build_nav(NavOrientation::Vertical, NavLevel::Secondary, &section))
            .append_child(Text::new("Tertiary / Horizontal", TextStyle::Label))
            .append_child(build_nav(NavOrientation::Horizontal, NavLevel::Tertiary, &section))
            .append_child(Text::new("Tertiary / Vertical", TextStyle::Label))
            .append_child(build_nav(NavOrientation::Vertical, NavLevel::Tertiary, &section));
        stack
    })
}

#[get("/nav")]
pub fn nav_default() -> Redirect {
    Redirect::to(uri!(nav_code()))
}
#[get("/nav/code")]
pub fn nav_code() -> Page<'static> {
    nav_demo_page(NavSection::Code)
}

#[get("/nav/issues")]
pub fn nav_issues() -> Page<'static> {
    nav_demo_page(NavSection::Issues)
}

#[get("/nav/pull-requests")]
pub fn nav_pull_requests() -> Page<'static> {
    nav_demo_page(NavSection::PullRequests)
}
