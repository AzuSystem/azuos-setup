#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya::winit::window::Fullscreen;

fn main() {
    // *Start* your app with a window and its root component
    launch(
        LaunchConfig::new()
            .with_default_font("Inter")
            .with_font(
                "Inter",
                Bytes::from_static(include_bytes!("assets/fonts/inter/Inter.ttf")),
            )
            .with_window(
                WindowConfig::new(app)
                    .with_title("AzuOS Installer")
                    .with_window_attributes(move |attributes, el| {
                        attributes.with_fullscreen(Some(Fullscreen::Borderless(None)))
                    }),
            )
    )
}

fn app() -> impl IntoElement {
    // Declare the *UI*
    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background((139, 134, 234))
        .center()
        .child(ImageViewer::new(("background", include_bytes!("assets/wallpaper.jpg")))
            .expanded()
            .center()
            .aspect_ratio(AspectRatio::Max)
            .child(
                rect()
                    .width(Size::percent(50.0))
                    .width(Size::px(650.0))
                    // .min_height(Size::px(300.0))
                    // .max_height(Size::px(353.5))
                    .height(Size::px(650.0))
                    .min_height(Size::px(650.0))
                    .max_height(Size::px(720.0))
                    // .height(Size::percent(70.0))
                    // .height(Size::fill_minimum())
                    
                    .background((13, 5, 20, 0.8))
                    .corner_radius(7.0)
                    .spacing(8.0)
                    .shadow(Shadow::new().y(12.0).blur(35.0).color((0, 0, 0, 0.5)))
                    .border(Border::new().alignment(BorderAlignment::Outer).fill((255, 255, 255, 0.2)).width(1.0))
                    .blur(10.0)
                    .center()
                    .padding(Gaps::new_all(20.0))
                    // .position(Position::new_absolute().top(50.0).left(50.0))
                    .child(
                        rect()
                            .width(Size::px(164.0))
                            .height(Size::px(150.0))
                            .background(Color::TRANSPARENT)
                            .child(ImageViewer::new(("logo", include_bytes!("assets/logo.png"))).width(Size::fill()).height(Size::fill()))
                    )
                    .child(
                        label()
                            .text("Welcome to AzuOS")
                            .font_size(32.0)
                            .font_weight(FontWeight::MEDIUM)
                            .color(Color::WHITE)
                    )
                    .child(
                        label()
                            .text("Please select an action to proceed with")
                            .font_size(16.0)
                            .font_weight(FontWeight::LIGHT)
                            .color((255, 255, 255, 0.5))
                    )
                    .child(
                        rect()
                            .height(Size::px(40.0))
                    )
                    .child(
                        rect()
                            .width(Size::px(493.0))
                            .height(Size::px(211.0))
                            .horizontal()
                            .main_align(Alignment::SpaceEvenly)
                            .child(
                                rect()
                                    .width(Size::px(194.0))
                                    .height(Size::fill())
                                    .background((13, 5, 20, 30))
                                    .corner_radius(7.0)
                                    .border(Border::new().alignment(BorderAlignment::Outer).fill((255, 255, 255, 0.2)).width(1.0))
                                    .shadow(Shadow::new().y(12.0).blur(35.0).color((0, 0, 0, 0.4)))
                                    .blur(10.0)
                                    .center()
                                    .spacing(1.5)
                                    .child(
                                        // svg(("download", include_bytes!("assets/icons/download.svg")))
                                        svg(include_bytes!("assets/icons/download.svg"))
                                            .width(Size::px(90.0))
                                            .height(Size::px(90.0))
                                            .padding(Gaps::new(0.0, 0.0, 4.0, 0.0))
                                    )
                                    .child(
                                        label()
                                            .text("Install")
                                            .font_size(20.0)
                                            .font_weight(FontWeight::MEDIUM)
                                            .color(Color::WHITE)
                                            // .text_shadow(Shadow::new().blur(15.0).color((255, 255, 255, 0.25)))

                                    )
                                    .child(
                                        label()
                                            .text("Get Started")
                                            .font_size(14.0)
                                            .font_weight(FontWeight::NORMAL)
                                            .color((255, 255, 255, 0.5))
                                    )
                            )
                            .child(
                                rect()
                                    // .width(Size::fill())
                                    .width(Size::px(105.0))
                            )
                            .child(
                                rect()
                                    .width(Size::px(194.0))
                                    .height(Size::fill())
                                    .background((13, 5, 20, 30))
                                    .corner_radius(7.0)
                                    .border(Border::new().alignment(BorderAlignment::Outer).fill((255, 255, 255, 0.2)).width(1.0))
                                    .shadow(Shadow::new().y(12.0).blur(35.0).color((0, 0, 0, 0.4)))
                                    .blur(10.0)
                                    .center()
                                    .spacing(1.5)
                                    .child(
                                        // svg(("download", include_bytes!("assets/icons/download.svg")))
                                        svg(include_bytes!("assets/icons/usb.svg"))
                                            .width(Size::px(90.0))
                                            .height(Size::px(90.0))
                                            .padding(Gaps::new(0.0, 0.0, 4.0, 0.0))
                                    )
                                    .child(
                                        label()
                                            .text("Live Boot")
                                            .font_size(20.0)
                                            .font_weight(FontWeight::MEDIUM)
                                            .color(Color::WHITE)
                                            // .text_shadow(Shadow::new().blur(15.0).color((255, 255, 255, 0.25)))

                                    )
                                    .child(
                                        label()
                                            .text("Try AzuOS")
                                            .font_size(14.0)
                                            .font_weight(FontWeight::NORMAL)
                                            .color((255, 255, 255, 0.5))
                                    )
                            )
                    )
                    // .child(
                    //     rect()
                    //         .height(Size::px(53.0))
                    // )
                    .child(
                        label()
                            .text("Copyright @ AzuSystem 2026 - AzuOS v2026.6\nhttps://github.com/AzuSystem")
                            .font_size(14.0)
                            .font_weight(FontWeight::LIGHT)
                            .color((255, 255, 255, 0.5))
                            .width(Size::fill())
                            .position(Position::new_absolute().bottom(0.).left(0.))
                    )
            )
            )
        // .child(
        //     rect()
        //         .width(Size::fill())
        //         .height(Size::percent(15.0))
        //         .position(Position::new_absolute().bottom(0.0))
        //         .background_linear_gradient(
        //             LinearGradient::new()
        //                 .angle(0.0)
        //                 .stop((Color::TRANSPARENT, 0.0))
        //                 .stop(((0, 0, 0), 0.5))
        //             )
        // )
        
}