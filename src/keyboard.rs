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
                    // .with_window_attributes(move |attributes, _el| {
                    //     attributes.with_fullscreen(Some(Fullscreen::Borderless(None)))
                    // }),
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
            .aspect_ratio(AspectRatio::Max)
            .child(
                rect()
                    .width(Size::px(471.2))
                    .height(Size::fill())
                    .background((13, 5, 20, 0.8))
                    .spacing(8.0)
                    .shadow(Shadow::new().y(12.0).blur(35.0).color((0, 0, 0, 0.5)))
                    .border(Border::new().alignment(BorderAlignment::Outer).fill((255, 255, 255, 0.2)).width(1.0))
                    .blur(10.0)
                    .center()
                    .padding(Gaps::new_all(25.0))
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
                            .text("Keyboard Layout")
                            .font_size(32.0)
                            .font_weight(FontWeight::MEDIUM)
                            .color(Color::WHITE)
                    )
                    .child(
                        label()
                            .text("Select your preferred keyboard layout")
                            .font_size(16.0)
                            .font_weight(FontWeight::EXTRA_LIGHT)
                            .color((255, 255, 255, 0.5))
                    )
                    .child(
                        rect()
                            .height(Size::px(20.0))
                    )
                    .child(
                        rect()
                            .width(Size::fill())
                            .height(Size::px(300.0))
                            .background(Color::WHITE)
                            // .child(
                            //     rect()
                            //     wid
                            // )
                            // .child(
                            //     rect()
                            //         // .width(Size::fill())
                            //         .width(Size::px(105.0))
                            // )
                    )
                    .child(
                        rect()
                            .height(Size::px(15.0))
                    )
                    .child(
                        label()
                            .text("Copyright @ AzuSystem 2026 - AzuOS v2026.6\nhttps://github.com/AzuSystem")
                            .font_size(10.0)
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