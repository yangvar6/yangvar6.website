//! # A website
//! that just exists for now, no flexy / fancy things etc.
//!
//! - [x] bootstrap
//! - [x] [`thaw`] ui
//! - [x] vibecode old site to new one with some AI
//! - [x] simple css
//! - [x] complex tailwind
//! - ~~[ ] fork [`thaw`] ui into `/crates/thaw` and adapt to me if needed~~
//! - [ ] glsl backgroung
//! - [x] link tree replacement with all links that are out there
//! - [ ] push to github pages

#![allow(non_snake_case)]
#![feature(cold_path)]
#![feature(allocator_api)]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use chrono::Datelike;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod consts;
mod glsl;

// MARK: MAIN

/// 32 bit WASM environment btw... (finally)
///
/// ## During development run next things in separete terminals:
/// - watch tailwind updates
///     - tailwindcss -i style/input.css -o style/output.css --optimize -m --watch
/// - watch website updates
///     - trunk serve --open
fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <Router>


                <nav>
                    /* nav idk */
                </nav>


                <main class="flex flex-col min-h-screen bg-[#181818]">
                    // <Header />
                    <HeaderMinimal />

                    <div class="flex flex-col flex-grow">

                        // <Routes fallback=NotFoundPage>
                        <Routes fallback=MainPage> // there is no escape AHAHAHAH HAHAH AHAAA
                            <Route path=path!("/") view=MainPage />
                        </Routes>

                    </div>

                    // <Footer />
                    <FooterMinimal />
                    // <Background />
                </main>


            </Router>
        }
    })
}

// MARK: LAYOUT
#[component]
fn Header() -> impl IntoView {
    view! {
        <header id="header" class="sticky top-0">
            <div class=" m-[8px] bg-black text-white">
                <div class="py-[12px] flex flex-col justify-between items-center">

                    <ul class="flex space-x-6">
                        <li><a href="#home" class="hover:underline">"Home"</a></li>
                        <li><a href="#about" class="hover:underline">"About"</a></li>
                        <li><a href="#projects" class="hover:underline">"Projects"</a></li>
                        <li><a href="#contact" class="hover:underline">"Contact"</a></li>
                    </ul>

                </div>
            </div>
        </header>
    }
}

#[component]
fn HeaderMinimal() -> impl IntoView {
    view! {
        <header id="header" class="sticky top-0">
            <div class=" m-[8px] bg-black text-white">
                <PageContentWidth class="py-[12px] flex flex-col justify-between w-[80%]" kind=PageContentWidthKind::Big>

                    <ul class="flex space-x-6">
                        <a>"Y6"</a>
                    </ul>

                </PageContentWidth>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full">
            <div class=" m-[8px] bg-black text-white">
                <PageContentWidth class="py-[46px] flex flex-col justify-between w-[80%]" kind=PageContentWidthKind::Big>
                    <div class="flex flex-row">
                        <div class="flex flex-col w-[30%]">
                            <h2>"Some important ass information"</h2>
                            <p class="text-[8px] text-justify">"Aenean at rhoncus mauris. Curabitur urna elit, dignissim at eros nec, rutrum blandit magna. Pellentesque eget sodales erat. Pellentesque non tellus elementum ipsum porta viverra. Sed luctus metus pharetra tristique venenatis. Vestibulum posuere efficitur molestie. Mauris volutpat, urna at bibendum tempor, metus mi porttitor lacus, eget feugiat lorem mi a leo. Etiam hendrerit ut orci quis hendrerit. Aliquam porta a magna vitae sagittis"</p>
                            <p class="text-[8px] text-justify">"AA porta viverra. Sed luctus metus pharetra tristique venenatis. Vestibulum posuere efficitur molestie. Mauris volutpat, urna at bibendum tempor, metus mi porttitor lacus, eget feugiat lorem mi a leo. Etiam hendrerit ut orci quis hendrerit. Aliquam porta a magna vitae sagittis"</p>
                        </div>
                        <div class="w-[20vh]"/>
                        <div class="flex flex-row w-full justify-between items-start">
                            <ul class="flex flex-col gap-1 mb-4 md:mb-0">
                                <li><h2 class="text-yellow-400">"Future"</h2></li>
                                <li><div class="h-[5px]"/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                            </ul>
                            <ul class="flex flex-col gap-1 mb-4 md:mb-0">
                                <li><h2 class="text-yellow-400">"Future"</h2></li>
                                <li><div class="h-[5px]"/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                            </ul>
                            <ul class="flex flex-col gap-1 mb-4 md:mb-0">
                                <li><h2 class="text-yellow-400">"Future"</h2></li>
                                <li><div class="h-[5px]"/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                                <li><Future/></li>
                            </ul>
                            <ul class="flex flex-col gap-1 mb-4 md:mb-0">
                                <li><h2 class="text-yellow-400">"Socials"</h2></li>
                                <li><div class="h-[5px]"/></li>
                                <li><a href=consts::socials::YOUTUBE>"YouTube"</a></li>
                                <li><a href=consts::socials::XDOTCOM>"x.com"</a></li>
                                <li><a href=consts::socials::TIKTOK>"TikTok"</a></li>
                                <li><a href=consts::socials::GITHUB>"GitHub"</a></li>
                                <li><a href=consts::socials::MAILTO>"Mail"</a></li>
                            </ul>
                        </div>
                    </div>
                    <div class="h-[20px]"/>
                    <p class="text-sm text-right">"Yan B. 2025 All rights reserved."</p>
                </PageContentWidth>
            </div>
        </footer>
    }
}

#[component]
fn FooterMinimal() -> impl IntoView {
    let current_year = chrono::Utc::now().year();

    view! {
        <footer class="w-full">
            <div class="m-[8px] bg-black text-white">
                <PageContentWidth class="py-[46px] flex flex-col justify-between w-[80%]" kind=PageContentWidthKind::Big>
                    <div class="flex flex-row justify-end">
                        // <div class="flex flex-col w-[30%]">
                        //     <h2>"Some important ass information"</h2>
                        //     <p class="text-[8px] text-justify">"Aenean at rhoncus mauris. Curabitur urna elit, dignissim at eros nec, rutrum blandit magna. Pellentesque eget sodales erat. Pellentesque non tellus elementum ipsum porta viverra. Sed luctus metus pharetra tristique venenatis. Vestibulum posuere efficitur molestie. Mauris volutpat, urna at bibendum tempor, metus mi porttitor lacus, eget feugiat lorem mi a leo. Etiam hendrerit ut orci quis hendrerit. Aliquam porta a magna vitae sagittis"</p>
                        //     <p class="text-[8px] text-justify">"AA porta viverra. Sed luctus metus pharetra tristique venenatis. Vestibulum posuere efficitur molestie. Mauris volutpat, urna at bibendum tempor, metus mi porttitor lacus, eget feugiat lorem mi a leo. Etiam hendrerit ut orci quis hendrerit. Aliquam porta a magna vitae sagittis"</p>
                        // </div>
                        // <div class="w-[20vh]"/>
                        // <div class="flex flex-row w-full justify-between items-start">
                            // <ul class="flex flex-col gap-1 mb-4 md:mb-0">
                            //     <li><h2 class="text-yellow-400">"Future"</h2></li>
                            //     <li><div class="h-[5px]"/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            // </ul>
                            // <ul class="flex flex-col gap-1 mb-4 md:mb-0">
                            //     <li><h2 class="text-yellow-400">"Future"</h2></li>
                            //     <li><div class="h-[5px]"/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            // </ul>
                            // <ul class="flex flex-col gap-1 mb-4 md:mb-0">
                            //     <li><h2 class="text-yellow-400">"Future"</h2></li>
                            //     <li><div class="h-[5px]"/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            //     <li><Future/></li>
                            // </ul>

                            <ul class="flex flex-col gap-1 mb-4 md:mb-0">
                                <li><h2 class="text-yellow-400">"Socials"</h2></li>
                                <li><div class="h-[5px]"/></li>
                                <li><a href=consts::socials::YOUTUBE>"YouTube"</a></li>
                                <li><a href=consts::socials::INSTAGRAM>"Instagram"</a></li>
                                <li><a href=consts::socials::XDOTCOM>"x.com"</a></li>
                                <li><a href=consts::socials::TIKTOK>"TikTok"</a></li>
                                <li><a href=consts::socials::GITHUB>"GitHub"</a></li>
                                <li><a href=consts::socials::MAILTO>"Mail"</a></li>
                            </ul>

                        // </div>
                    </div>
                    <div class="h-[20px]"/>
                    <div class="flex flex-row">
                        // <p class="text-sm text-right">"Yan B. 2025 All rights reserved."</p>
                        <p class="text-sm">"Yan B. " {current_year} ""</p> // minimal
                        <p class="text-sm text-black">"All rights reserved."</p>
                    </div>
                </PageContentWidth>
            </div>
        </footer>
    }
}

// MARK: COMPONENTS

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SectionDividerKind {
    Big,
    Small,
}

#[component]
fn SectionDivider(#[prop(optional, into)] kind: Option<SectionDividerKind>) -> impl IntoView {
    let kind = kind.unwrap_or(SectionDividerKind::Big);

    // let class = format!(
    //     "h-[{h}vh]",
    //     h = match kind {
    //         SectionDividerKind::Big => "26",
    //         SectionDividerKind::Small => "6",
    //     }
    // );

    let class = match (kind,) {
        (SectionDividerKind::Big,) => "h-[26vh]",
        (SectionDividerKind::Small,) => "h-[6vh]",
    };

    view! {
        <div class=class/>
    }
}

#[component]
fn Background() -> impl IntoView {
    view! {
        <section class="z-[-10000000] fixed inset-0 bg-gray-900">
            <glsl::BackgroundCanvas />
        </section>
    }
}

#[component]
fn Future() -> impl IntoView {
    view! {
        <div class=format!("w-[80px] h-[6px] bg-gray-100 animate-pulse duration-[1500ms]")/>
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum PageContentWidthKind {
    #[default]
    Medium,
    Small,
    Big,
    Full,
}

#[component]
fn PageContentWidth(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] kind: Option<PageContentWidthKind>,
    children: Children,
) -> impl IntoView {
    let kind = kind.unwrap_or(PageContentWidthKind::Medium);
    let class = class.unwrap_or("".into());

    let native_class = match (kind,) {
        (PageContentWidthKind::Medium,) => "max:w-[800px] mx-auto",
        (PageContentWidthKind::Small,) => "max:w-[600px] mx-auto",
        (PageContentWidthKind::Big,) => "max:w-[1200px] mx-auto",
        (PageContentWidthKind::Full,) => "max:w-full",
    };

    view! {
        <div class=format!("{class} {native_class}")>
            {children()}
        </div>
    }
}

// MARK: PAGES
#[component]
fn MainPage() -> impl IntoView {
    view! {
        <>

            <SectionDivider kind=SectionDividerKind::Small/>

            <section id="about">
                <div class="container mx-auto flex flex-col md:flex-row items-center px-4">
                    <div class="md:w-1/2 mb-8 md:mb-0">
                        // <img src="https://via.placeholder.com/600x400" class="rounded-lg shadow-lg"/>
                    </div>
                    <div class="md:w-1/2 md:pl-8 flex flex-col gap-2">
                        <h2 class="text-3xl font-bold mb-4 text-yellow-400">"Yan B."</h2>
                        <p class="">"\"What is a knight without a dragon?\""</p>
                        <p class="text-yellow-500 text-center">"— Lord Sauron"</p>
                    </div>
                </div>
            </section>

            <SectionDivider kind=SectionDividerKind::Big/>

            // <SectionDivider kind=SectionDividerKind::Small/>

            // <PageContentWidth>
            //     <section id="projects">
            //         <div class="container px-4">
            //             <h2 class="text-3xl font-bold mb-4">"Nam quis purus ut augue"</h2>
            //             <p class="max-w-2xl">
            //                 "Vivamus ornare lacus eu tortor tincidunt tincidunt. Proin interdum nisl nec erat viverra, consectetur scelerisque mi sagittis. Curabitur et scelerisque purus. Nulla at egestas urna. Nam quis purus ut augue ultrices rutrum. Sed dictum sapien sed eros egestas suscipit. Pellentesque vitae arcu urna. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Quisque ut massa massa. Donec accumsan pharetra sollicitudin."
            //             </p>
            //         </div>
            //     </section>
            // </PageContentWidth>

            // <SectionDivider kind=SectionDividerKind::Small/>

            // <PageContentWidth>
            //     <section id="projects">
            //         <div class="container px-4">
            //             <h2 class="text-3xl font-bold mb-4">"Ut tempus"</h2>
            //             <p class="max-w-2xl">
            //                 "Aenean at rhoncus mauris. Curabitur urna elit, dignissim at eros nec, rutrum blandit magna. Pellentesque eget sodales erat. Pellentesque non tellus elementum ipsum porta viverra. Sed luctus metus pharetra tristique venenatis. Vestibulum posuere efficitur molestie. Mauris volutpat, urna at bibendum tempor, metus mi porttitor lacus, eget feugiat lorem mi a leo. Etiam hendrerit ut orci quis hendrerit. Aliquam porta a magna vitae sagittis."
            //             </p>
            //         </div>
            //     </section>
            // </PageContentWidth>

            // <SectionDivider kind=SectionDividerKind::Small/>

            // <PageContentWidth>
            //     <section id="projects">
            //         <div class="container px-4">
            //             <h2 class="text-3xl font-bold mb-4">"lacus, eget feugiat lorem mi a leo"</h2>
            //             <p class=" max-w-2xl">
            //                 "Aenean at rhoncus mauris. Curabitur urna elit, dignissim at eros nec, rutrum blandit magna. Pellentesque eget sodales erat. Pellentesque non tellus elementum ipsum porta viverra. Sed luctus metus pharetra tristique venenatis. Vestibulum posuere efficitur molestie. Mauris volutpat, urna at bibendum tempor, metus mi porttitor lacus, eget feugiat lorem mi a leo. Etiam hendrerit ut orci quis hendrerit. Aliquam porta a magna vitae sagittis."
            //             </p>
            //         </div>
            //     </section>
            // </PageContentWidth>

            // <SectionDivider/>


        </>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full justify-center items-center">
            <h1>"404 - Page does not exists"</h1>
        </div>
    }
}
