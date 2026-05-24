#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

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
        .child(ImageViewer::new(("background", include_bytes!("assets/wallpaper.jpg"))))
        .child(
            rect()
                // .width(Size::percent(50.0))
                .width(Size::px(650.0))
                .height(Size::percent(65.0))
                .background((13, 5, 20, 0.8))
                .corner_radius(7.0)
                .shadow(Shadow::new().y(12.0).blur(35.0).color((0, 0, 0, 0.5)))
                .border(Border::new().alignment(BorderAlignment::Outer).fill((255, 255, 255, 0.2)).width(1.0))
                .blur(40.0)
                .center()
                .position(Position::new_absolute().top(50.0).left(50.0))
                .child(
                    // ImageViewer::new(("logo", include_bytes!("assets/logo.png"))).width()
                    rect()
                        .width(Size::px(164.0))
                        .height(Size::px(150.0))
                        .background(Color::WHITE)
                )
                .child(
                    label()
                        .text("Welcome to AzuOS")
                        .font_size(32.0)
                        .font_weight(FontWeight::MEDIUM)
                        .color(Color::WHITE)
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