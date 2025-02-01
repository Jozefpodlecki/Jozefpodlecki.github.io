use yew::*;

use crate::icons::{Github, Linkedin};

#[function_component(App)]
pub fn app() -> Html {
   
    html! {
        <div class="flex flex-col items-center justify-between h-screen bg-gradient-to-t from-black via-gray-900 to-gray-800">
            <div class="bg-white max-w-[600px] w-full p-8 rounded-lg shadow-lg m-16 flex flex-col justify-between h-full">

                <section class="py-5">
                    <span class="font-[oswald] block text-3xl font-bold text-gray-900">{"Józef Podlecki"}</span>
                    <span class="font-[oswald] block text-xl text-gray-700">{"Software Developer"}</span>

                    <div class="flex justify-start gap-1 mt-2">
                        <a href="https://www.linkedin.com/in/jozef-witold-podlecki/"><Linkedin/></a>
                        <a href="https://github.com/Jozefpodlecki"><Github/></a>
                    </div>
                </section>

                <section class="py-5">
                    <h1 class="font-[oswald] text-2xl font-bold text-gray-900">{"Projects"}</h1>
                </section>

                <section class="py-5">
                    <h1 class="font-[oswald] text-2xl font-bold text-gray-900">{"Experience"}</h1>
                    <div class="border-b border-gray-300 pb-4 mt-4">
                        <div class="flex justify-between items-center">
                            <div>
                                <h3 class="font-[roboto] font-semibold text-xl text-gray-800">{"BJSS"}</h3>
                                <p class="font-[roboto] text-gray-500 text-sm italic">{"Software Engineer · Full-time · Hybrid"}</p>
                            </div>
                            <p class="text-gray-500 text-sm">{"Jan 2022 – Present"}</p>
                        </div>
                        <p class="font-[roboto] text-gray-500 text-sm">{"Leeds, UK"}</p>
                        <ul class="mt-2 list-none list-inside space-y-2">
                            <li class="font-[roboto] text-gray-700 text-sm">{"Contributed to a multi-year digital transformation project for a leading UK gambling company."}</li>
                            <li class="font-[roboto] text-gray-700 text-sm">{"Extracted database calls from legacy monolithic applications and migrated them to a microservices architecture."}</li>
                            <li class="font-[roboto] text-gray-700 text-sm">{"Contributed to the migration of legacy stored procedures powering SSRS reports to GCP BigQuery."}</li>
                        </ul>
                    </div>
                    <div class="border-b border-gray-300 pb-4 mt-4">
                    <div class="flex justify-between items-center">
                        <div>
                            <h3 class="font-[roboto] font-semibold text-xl text-gray-800">{"Vismo"}</h3>
                            <p class="font-[roboto] text-gray-500 text-sm italic">{".NET Developer · Full-time · Hybrid"}</p>
                        </div>
                        <p class="font-[roboto] text-gray-500 text-sm">{"Aug 2020 – Jan 2022"}</p>
                    </div>
                    <p class="font-[roboto] text-gray-500 text-sm">{"York, UK"}</p>
                    <ul class="mt-2 list-none list-inside space-y-2">
                        <li class="font-[roboto] text-gray-700 text-sm">{"Worked on intranet asp net core web app and powering it .net modules."}</li>
                        <li class="font-[roboto] text-gray-700 text-sm">{"Performed ETL operations."}</li>
                        <li class="font-[roboto] text-gray-700 text-sm">{"Triaged and troubleshooted issues reported by customers."}</li>
                    </ul>
                    </div>
                </section>


                <footer id="contact" class="py-2 mt-auto text-center text-sm text-gray-500">
                    <span>{"© Jozef Podlecki 2025"}</span>
                </footer>
            </div>
        </div>
    }

    // html! {
    //     <div class="flex flex-col items-center justify-between h-screen bg-[#0a192f]">
    //         <div class="bg-[#FFF] max-w-[600px] h-full w-full p-8 rounded-lg shadow-lg m-16">
    //         <div class="">
    //         </div>
    //             <section class="py-10 space-y-2">
    //                 <span class="block text-3xl font-bold text-gray-900">{"Jozef Podlecki"}</span>
    //                 <span class="block text-xl text-gray-700">{"Software Developer"}</span>

    //                 <div class="flex justify-start gap-6 mt-8">
    //                     <a href="https://www.linkedin.com/in/jozef-witold-podlecki/"><Linkedin/></a>
    //                     <a href="https://github.com/Jozefpodlecki"><Github/></a>
    //                 </div>
    //             </section>

    //             <section class="py-10">
    //                 <h1 class="text-3xl font-bold text-gray-900">{"Work Experience"}</h1>
    //             </section>
                    

    //             <footer class="py-2 mt-auto text-center text-sm text-gray-500">
    //                 <span class="text-xl text-gray-500">{"Jozef Podlecki 2025"}</span>
    //             </footer>
    //         </div>
    //     </div>
    // }


}