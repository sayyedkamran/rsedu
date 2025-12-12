use leptos::prelude::*;

mod components;
mod pages;

use pages::home::Home;

fn main() {
    // Set up better error messages in console
    console_error_panic_hook::set_once();

    // Initialize logging
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging");

    log::info!("🎨 Starting rsEdu Frontend");

    // Mount the app to the <body> tag
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            <div class="header">
                <h1>"🎓 rsEdu"</h1>
                <p>"School Management System - Built with Rust"</p>
            </div>
            
            <Home/>
        </div>
    }
}

