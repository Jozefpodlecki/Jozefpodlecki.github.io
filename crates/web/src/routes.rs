use yew_router::Routable;
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")] Home,
    #[at("/about")] About,
    #[at("/projects")] Projects,
    #[at("/projects/:slug")] Project { slug: String },
    #[not_found]
    #[at("/404")] NotFound,
}
