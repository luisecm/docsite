use dioxus::prelude::*;

pub fn FeaturedExamples(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "body-font w-full dark:bg-ideblack",
            div { class: "container px-6 max-w-screen-lg py-24 mx-auto",
                div { class: "flex flex-col w-full mb-10",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-4 dark:text-white",
                        "Feature-packed examples"
                    }
                }
                div { class: "flex flex-wrap -m-4",
                    FeaturedExample {
                        title: "TodoMVC",
                        subtitle: "Web",
                        description: "The classic demonstration of a web framework. Fits in one file in less than 150 lines of code.",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/todomvc",
                        img_avif: "/static/todos_cropped.avif",
                        img: "/static/todos_cropped.png",
                        img_alt: "TodoMVC"
                    }
                    FeaturedExample {
                        title: "E-Commerce Site",
                        subtitle: "Web",
                        description: "A complex website leveraging Tailwind, global state, animations, pre-rendering, and project structure.",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/ecommerce-site",
                        img_avif: "/static/marketplace_cropped.avif",
                        img: "/static/marketplace_cropped.png",
                        img_alt: "E-Commerce Site"
                    }
                    FeaturedExample {
                        title: "File Explorer",
                        subtitle: "Desktop",
                        description: "Interact with native APIs directly from your UI. Works with a simple `cargo run` and is bundle-ready.",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/file-explorer",
                        img_avif: "/static/file_explorer.avif",
                        img: "/static/file_explorer.png",
                        img_alt: "File Explorer"
                    }
                    FeaturedExample {
                        title: "WiFi Scanner",
                        subtitle: "Desktop, Mobile",
                        description: "Spawn native Tokio tasks and interact with hardware directly from your app with Dioxus coroutines.",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/wifi-scanner",
                        img_avif: "/static/scanner.avif",
                        img: "/static/scanner.png",
                        img_alt: "Wifi Scanner"
                    }
                    FeaturedExample {
                        title: "Documentation Site",
                        subtitle: "SSR",
                        description: "This very site is written in Dioxus, pre-rendered with SSR, and rehydrated with Dioxus-Web!",
                        link: "https://github.com/DioxusLabs/docsite",
                        img_avif: "/static/docsite_cropped.avif",
                        img: "/static/docsite_cropped.png",
                        img_alt: "The documentation site you're currently viewing"
                    }
                    FeaturedExample {
                        title: "JS Benchmark",
                        subtitle: "Web",
                        description: "The classic performance benchmark for web frameworks. Dioxus ranks extraordinarily high!",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/jsframework-benchmark",
                        img_avif: "/static/jsbenchmark_cropped.avif",
                        img: "/static/jsbenchmark_cropped.png",
                        img_alt: "Javascript Framework Benchmark"
                    }
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct FeaturedExampleProps<'a> {
    title: &'a str,
    subtitle: &'a str,
    description: &'a str,
    link: &'a str,
    img_avif: &'a str,
    img: &'a str,
    img_alt: &'a str,
}

fn FeaturedExample<'a>(cx: Scope<'a, FeaturedExampleProps<'a>>) -> Element {
    let FeaturedExampleProps {
        subtitle,
        title,
        description,
        link,
        img_avif,
        img,
        img_alt,
    } = cx.props;

    cx.render(rsx!{
        div { class: "lg:w-1/3 sm:w-1/2 p-4",
            a { href: "{link}",
                div { class: "flex relative",
                    picture {
                        source {
                            class: "w-full rounded-lg object-cover",
                            r#type: "image/avif",
                            "srcset": "{img_avif}"
                        }
                        img {
                            class: "absolute inset-0 w-full h-full object-cover object-center",
                            alt: "{img_alt}",
                            src: "{img}"
                        }
                    }
                    div { class: "px-8 py-10 relative z-10 w-full border-4 border-gray-200 bg-white opacity-0 hover:opacity-100",
                        h2 { class: "tracking-widest text-sm title-font font-medium text-indigo-500 mb-1",
                            "{subtitle}"
                        }
                        h1 { class: "title-font text-lg font-medium text-gray-900 mb-3",
                            "{title}"
                        }
                        p { class: "leading-relaxed", "{description}" }
                    }
                }
            }
        }
    })
}
