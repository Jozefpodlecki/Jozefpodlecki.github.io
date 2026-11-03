use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconData};
use crate::content::SiteData;
use crate::features::experience::ExperienceSection;
use crate::features::education::EducationSection;
use crate::routes::Route;
use crate::features::projects::ProjectCollection;
use crate::features::travel::TravelMap;
use crate::features::motion::MotionReveal;
use crate::features::hero::HeroFlowField;

#[derive(Properties, PartialEq)]
struct SocialLinksProps {
    github: String,
    linkedin: String,
}

#[function_component(SocialLinks)]
fn social_links(props: &SocialLinksProps) -> Html {
    html! {
        <nav data-component="social-links" class="mt-[18px] flex items-center gap-[18px]" aria-label="Social profiles and CV">
            <a
                data-social="github"
                href={props.github.clone()}
                target="_blank"
                rel="noopener noreferrer"
                aria-label="GitHub"
                title="GitHub"
                class="grid size-[38px] select-none place-items-center rounded-full border border-divider bg-card/35 text-ink no-underline backdrop-blur-sm transition-colors duration-200 hover:border-highlight hover:text-highlight focus-visible:outline-2 focus-visible:outline-offset-[3px] focus-visible:outline-highlight"
            >
                <Icon data={IconData::SIMPLE_ICONS_GITHUB} width="21" height="21"/>
            </a>
            <a
                data-social="linkedin"
                href={props.linkedin.clone()}
                target="_blank"
                rel="noopener noreferrer"
                aria-label="LinkedIn"
                title="LinkedIn"
                class="grid size-[38px] select-none place-items-center rounded-full border border-divider bg-card/35 text-ink no-underline backdrop-blur-sm transition-colors duration-200 hover:border-highlight hover:text-highlight focus-visible:outline-2 focus-visible:outline-offset-[3px] focus-visible:outline-highlight"
            >
                <Icon data={IconData::SIMPLE_ICONS_LINKEDIN} width="21" height="21"/>
            </a>
            <a
                data-social="cv"
                href="Jozef_Podlecki_Resume_2026_11_03_Public.pdf"
                target="_blank"
                rel="noopener noreferrer"
                aria-label="Open CV PDF"
                title="Open CV PDF"
                class="relative grid size-[38px] select-none place-items-center rounded-full border border-divider bg-card/35 text-ink no-underline backdrop-blur-sm transition-colors duration-200 hover:border-highlight hover:text-highlight focus-visible:outline-2 focus-visible:outline-offset-[3px] focus-visible:outline-highlight"
            >
                <Icon
                    data={IconData::LUCIDE_DOWNLOAD}
                    width="19"
                    height="19"
                />
                <span
                    aria-hidden="true"
                    class="pointer-events-none absolute -bottom-1 -right-1 rounded bg-highlight px-1 py-px text-[9px] font-bold leading-none text-on-accent shadow-sm"
                >
                    {"CV"}
                </span>
            </a>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
struct HeroProps {
    intro: String,
    github: String,
    linkedin: String,
}

#[function_component(Hero)]
fn hero(props: &HeroProps) -> Html {
    html! {
        <section data-component="home-hero" class="relative select-none grid grid-cols-1 items-center gap-[clamp(18px,4vw,56px)] py-8 pb-10 min-[641px]:pt-[clamp(28px,5vh,54px)] min-[641px]:pb-[clamp(34px,6vh,64px)] min-[851px]:grid-cols-[minmax(0,1.2fr)_minmax(0,0.8fr)]">
            <div class="max-w-[640px]">
                <h1 class="!text-[clamp(2.2rem,10vw,3.3rem)] !leading-[1.04] !tracking-[-0.055em] sm:!text-[clamp(2.55rem,4.65vw,4.8rem)]">
                    {"Hi, I'm Józef"}
                    <span class="block text-highlight">
                        {"I build software"}
                    </span>
                </h1>
                <p class="max-w-[530px] text-[clamp(0.98rem,1.35vw,1.1rem)] leading-[1.65] text-ink">{&props.intro}</p>
                <div class="mt-[22px] flex flex-wrap items-center gap-[18px]">
                    <Link<Route>
                        to={Route::Projects}
                        classes="inline-flex select-none items-center gap-2 rounded-[5px] bg-highlight px-5 py-[13px] font-bold text-on-accent no-underline transition-opacity hover:opacity-90"
                    >
                        {"View my projects"}
                        <Icon data={IconData::LUCIDE_ARROW_RIGHT} width="16" height="16"/>
                    </Link<Route>>
                    <Link<Route>
                        to={Route::About}
                        classes="inline-flex select-none items-center rounded-[5px] border border-divider bg-card/80 px-5 py-[13px] font-bold text-ink no-underline transition-colors hover:border-highlight hover:bg-card hover:text-highlight focus-visible:outline-2 focus-visible:outline-highlight"
                    >
                        {"About me"}
                    </Link<Route>>
                </div>
                <SocialLinks github={props.github.clone()} linkedin={props.linkedin.clone()}/>
            </div>
            <HeroFlowField/>
        </section>
    }
}

#[function_component(FeaturedProjects)]
fn featured_projects() -> Html {
    html! {
        <section data-component="featured-projects" class="section relative pb-[70px] pt-9">
            <div class="mb-[22px] flex flex-wrap items-center justify-between gap-[15px] [&>h2]:m-0">
                <h2 class="!m-0 select-none text-2xl font-semibold tracking-tight sm:text-3xl">{"Featured projects"}</h2>
                <Link<Route>
                    to={Route::Projects}
                    classes="inline-flex items-center gap-2 text-sm font-medium no-underline hover:underline"
                >
                    {"View all projects"}
                    <Icon data={IconData::LUCIDE_ARROW_RIGHT} width="16" height="16" />
                </Link<Route>>
            </div>
            <ProjectCollection limit={3}/>
        </section>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let data = use_context::<SiteData>().expect("SiteData provider");
    html! {
        <main data-component="home-page" class="relative min-h-[65vh]">
            <MotionReveal>
                <Hero
                    intro={data.profile.intro.clone()}
                    github={data.profile.social.github.clone()}
                    linkedin={data.profile.social.linkedin.clone()}
                />
            </MotionReveal>
            <FeaturedProjects/>
            <ExperienceSection items={data.experience.iter().take(2).cloned().collect::<Vec<_>>()}/>
        </main>
    }
}

#[function_component(AboutPortrait)]
fn about_portrait() -> Html {
    let failed = use_state(|| false);
    let loaded = use_state(|| false);
    let onload = {
        let loaded = loaded.clone();
        Callback::from(move |_: Event| loaded.set(true))
    };
    let onerror = {
        let failed = failed.clone();
        Callback::from(move |_: Event| failed.set(true))
    };

    html! {
        <figure
            data-component="about-portrait"
            data-image-state={if *failed { "unavailable" } else if *loaded { "ready" } else { "loading" }}
            class="m-0 w-full max-w-[340px] select-none justify-self-center lg:justify-self-end"
        >
            if *failed {
                <div
                    data-component="portrait-fallback"
                    role="img"
                    aria-label="Portrait unavailable"
                    class="flex aspect-[4/5] w-full flex-col items-center justify-center gap-4 rounded-2xl border border-divider bg-layer/80 px-6 text-center shadow-xl sm:aspect-square lg:aspect-[4/5]"
                >
                    <span
                        aria-hidden="true"
                        class="grid size-20 place-items-center rounded-full border border-highlight/30 bg-highlight/10 font-semibold text-3xl tracking-tight text-highlight"
                    >
                        {"JP"}
                    </span>
                    <span class="text-sm text-subtle">{"Portrait unavailable"}</span>
                </div>
            } else {
                <img
                    data-component="portrait-image"
                    draggable="false"
                    class="aspect-[4/5] w-full rounded-2xl border border-divider object-cover object-center shadow-2xl shadow-black/20 sm:aspect-square lg:aspect-[4/5]"
                    src="avatar.jpg"
                    alt="Portrait of Józef Podlecki"
                    width="360"
                    height="450"
                    loading="eager"
                    onload={onload}
                    onerror={onerror}
                />
            }
        </figure>
    }
}

#[derive(Properties, PartialEq)]
struct AboutIntroProps {
    paragraphs: Vec<String>,
}

#[function_component(AboutIntro)]
fn about_intro(props: &AboutIntroProps) -> Html {
    html! {
        <header
            data-component="about-intro"
            class="relative my-8 select-none grid items-center gap-8 overflow-hidden rounded-2xl border border-divider bg-card/80 p-6 shadow-xl backdrop-blur-md sm:p-10 lg:grid-cols-[minmax(0,1.3fr)_minmax(240px,0.7fr)] lg:gap-12"
        >
            <div class="max-w-[620px]">
                <h1 class="!mb-5 !mt-0 !text-[clamp(2.75rem,5vw,4.25rem)] !leading-[1.08] text-highlight">{"A bit about me."}</h1>
                <div data-component="about-biography" class="flex max-w-[65ch] flex-col gap-4 text-base leading-relaxed text-subtle sm:text-lg">
                    {for props.paragraphs.iter().map(|paragraph| html! { <p class="m-0">{paragraph}</p> })}
                </div>
            </div>
            <AboutPortrait/>
        </header>
    }
}

#[function_component(About)]
pub fn about() -> Html {
    let data = use_context::<SiteData>().expect("SiteData provider");
    html! {
        <main data-component="about-page" class="min-h-[65vh]">
            <MotionReveal>
                <AboutIntro paragraphs={data.profile.about.clone()}/>
            </MotionReveal>
            <ExperienceSection items={data.experience.clone()} detailed={true}/>
            <EducationSection items={data.profile.education.clone()}/>
            <TravelMap/>
        </main>
    }
}
