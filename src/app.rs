use log::{debug, info};
use yew::*;

use crate::{api::get_summary, dark_mode_toggle::DarkModeToggle, experience::ExperienceSection, icons::*, loader::Loader, models::{FetchError, Summary}};

pub enum Message {
    #[allow(dead_code)]
    Loading,
    Loaded(Summary),
    Error(FetchError),
}
pub struct App {
    summary: Option<Summary>,
    error: Option<FetchError>
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        {
            let link = context.link();
            link.send_future(async {
                match get_summary().await {
                    Ok(summary) => {
                        Message::Loaded(summary)
                    },
                    Err(err) => {
                        Message::Error(err)
                    },
                } 
            });
        }

        Self {
            summary: None,
            error: None
        }
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Message::Loading => {
                false
            }
            Message::Loaded(summary) => {
               self.summary = Some(summary);
               true
            }
            Message::Error(err) => {
                debug!("{:?}", err);
                self.error = Some(err);
                true
            },
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {

        match &self.summary {
            Some(summary) => {
                html! {
                    <div class="">
                        <div class="h-8 absolute top-1 right-1">
                            <DarkModeToggle/>
                        </div>
                        <div class="flex flex-col items-center justify-between h-screen bg-gradient-to-t from-black via-gray-900 to-gray-800">
                            <div class="bg-white dark:bg-black max-w-[800px] w-full p-8 rounded-lg shadow-lg m-8 flex flex-col justify-between h-full border dark:border-white">
                                <section class="py-5">
                                    <span class="font-[oswald] block text-3xl font-bold text-gray-900 dark:text-gray-200">{summary.full_name.clone()}</span>
                                    <span class="font-[oswald] block text-xl text-gray-700 dark:text-gray-200">{summary.role.clone()}</span>

                                    <div class="flex justify-start gap-1 mt-2">
                                        <a class="dark:text-white" href={summary.social.linkedin.clone()}><Linkedin/></a>
                                        <a class="dark:text-white" href={summary.social.github.clone()}><Github/></a>
                                        <a class="dark:text-white" href={summary.social.duolingo.clone()}><Duolingo/></a>
                                    </div>
                                </section>

                                <section class="py-5">
                                    <h1 class="font-[oswald] text-2xl font-bold text-gray-900 dark:text-gray-200">{"Projects"}</h1>
                                </section>

                                <ExperienceSection items={summary.experience.clone()}/>

                                <footer class="font-[roboto] py-2 mt-auto text-center text-sm text-gray-500 dark:text-gray-300">
                                    <span>{"© Jozef Podlecki 2025"}</span>
                                </footer>
                            </div>
                        </div>
                    </div>
                }
            },
            None => {
                html! {
                    <div>
                        <div class="h-8 absolute top-1 right-1">
                            <DarkModeToggle/>
                        </div>
                        <div class="flex flex-col items-center justify-between h-screen bg-gradient-to-t from-black via-gray-900 to-gray-800">
                            <div class="bg-white dark:bg-black max-w-[800px] w-full p-8 rounded-lg shadow-lg m-16 flex flex-col justify-between h-full border dark:border-white">
                                <Loader/>
                            </div>
                        </div>
                    </div>
                }
            } 
        }
    }
}