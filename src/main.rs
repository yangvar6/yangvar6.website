//! # A website
//! that just exists for now, no flexy / fancy things etc.
//!
//! - [x] bootstrap
//! - [x] [`thaw`] ui
//! - [x] vibecode old site to new one with some AI
//! - [x] simple css
//! - [x] complex tailwind
//! - [ ] fork [`thaw`] ui into `/crates/thaw` and adapt to me if needed
//! - [ ] glsl backgroung
//! - [x] link tree replacement with all links that are out there
//! - [ ] push to github pages

#![allow(non_snake_case)]
#![feature(cold_path)]

use leptos::prelude::*;
use thaw::*;

mod circles;
mod consts;
mod glsl;

/// WASM environment btw... (finally)
///
/// ## During development run next things in separete terminals:
/// - watch tailwind updates
///     - tailwindcss -i style/input.css -o style/output.css --optimize -m --watch
/// - watch website updates
///     - trunk serve --open
fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <thaw::ConfigProvider>
                <App />
            </thaw::ConfigProvider>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    let clicked_log = move |_| {
        leptos::logging::log!("clicked thaw button");
    };

    view! {
        <>

            <ButtonGroup >
                <Button
                    on:click=clicked_log>
                    "Button!"
                </Button>
                <Button
                    on:click=clicked_log>
                    "Button!"
                </Button>
                <Button
                    class="bg-red-500 hover:bg-red-600 text-white"
                    on:click=move |_| {
                        leptos::logging::log!("clicked special thaw button");
                    }>
                    "Special Button!"
                </Button>
            </ButtonGroup>


            <Flex vertical=true>
                <Button>"1"</Button>
                <Button>"2"</Button>
                <Button>"3"</Button>
            </Flex>

            <section
                class="flex flex-row w-full justify-center h-fit">
                <div class="flex flex-col md:flex-row w-[90%] md:w-1/2 h-fit justify-between md:gap-[42px] gap-[6px] py-6">
                    <div class="flex flex-row self-end md:self-start gap-x-4">
                        <a href=consts::socials::INSTAGRAM>"INSTAGRAM"</a>
                        <a href=consts::socials::MAILTO>"Gmail"</a>
                        <a href=consts::socials::GITHUB>"Github"</a>
                        <a href=consts::socials::TIKTOK>"TIKTOK"</a>
                        // <a href=consts::socials::TELEGRAM>"Telegram"</a>
                        <a href=consts::socials::XDOTCOM>"XDOTCOM"</a>
                    </div>
                    <div class="self-end">
                        <a >"Linkzz"</a>
                    </div>
                    <div class="self-end">
                        <h1 class="jumbotron">"Hello, jumbotron!"</h1>
                    </div>
                </div>
            </section>

            <section
                class="flex flex-col md:flex-row w-full h-fit max-w-[1200px] gap-[5px] md:gap-[60px]">
                <div class="md:hidden flex h-[90px]"></div>
                    <div class="flex flex-col w-full justify-center items-center p-[42px]">
                    <circles::Patterns amount=10 radius_base=200 radius_max=800 />
                </div>
                <div class="md:hidden flex h-[60px]"></div>
                <div class="flex flex-col w-full justify-center items-center gap-[28px]">
                    <div class="flex flex-col w-[90%] max-w-[550px] min-w-[0px] md:min-w-[440px] justify-center items-center gap-[28px]">
                        <div class="flex flex-col w-full gap-[6px]">
                            <a >
                                <h1 class="text-[48px] font-extrabold">"Yan B."</h1>
                            </a>
                            <h1 class="text-[38px] text-left text-yellow-600">"Do lorem"</h1>
                            <h1 class="text-[38px] text-right text-yellow-600">"Ipsum"</h1>
                        </div>
                        <div class="flex flex-col w-full gap-[6px]">
                            <div class="flex flex-col justify-center items-center">
                                <div class="w-[80%] flex flex-col gap-2">
                                    <p class="bg-gray-900/80 p-2">
                                        "🤗 I'm a ...... "
                                        <span class="p_acc">"............"</span>
                                        " ....................."
                                    </p>
                                    <p class="bg-gray-900/80 p-2">
                                        "🤗 ...... ..... ... ....... .......... ........... ...... . "
                                        <span class="p_acc">"..... ....."</span>
                                        ", "
                                        <span class="p_acc">"... .."</span>
                                        " ................. "
                                        <span class="p_acc">"....... ..... ........."</span>
                                        " .......... ....... ....... "
                                    </p>
                                    <p class="bg-gray-900/80 p-2">
                                        "🤗 "
                                        <span class="p_acc">
                                            "....."
                                        </span>
                                        " ...... . "
                                        <span class="p_acc">
                                            "... .... ..."
                                        </span>
                                        " ... .. .................. ........ ..... ...... ............ ............ ........... "
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="flex flex-row w-full h-fit justify-center py-6">
                <p class="w-[90%] md:w-1/2 text-center opacity-70">
                    "This website was built by me over a weekend using Leptos, TailwindCSS, Rust, and WebGLGLGLGLGLGL."
                </p>
            </section>

            <section class="z-[-10000000] fixed inset-0 bg-gray-900">
                <glsl::BackgroundCanvas />
            </section>
        </>
    }
}
