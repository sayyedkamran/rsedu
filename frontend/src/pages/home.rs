use leptos::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Deserialize)]
struct HealthResponse {
    status: String,
    message: String,
    version: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ApiInfo {
    name: String,
    version: String,
    description: String,
}

#[component]
pub fn Home() -> impl IntoView {
    let (health_status, set_health_status) = create_signal(None::<HealthResponse>);
    let (health_loading, set_health_loading) = create_signal(false);
    let (health_error, set_health_error) = create_signal(None::<String>);

    let (api_info, set_api_info) = create_signal(None::<ApiInfo>);
    let (api_loading, set_api_loading) = create_signal(false);
    let (api_error, set_api_error) = create_signal(None::<String>);

    let check_health = move || {
        spawn_local(async move {
            set_health_loading.set(true);
            set_health_error.set(None);

            let result = Request::get("http://localhost:3000/health")
                .send()
                .await;

            match result {
                Ok(response) => {
                    match response.json::<HealthResponse>().await {
                        Ok(data) => {
                            set_health_status.set(Some(data));
                            set_health_loading.set(false);
                        }
                        Err(e) => {
                            set_health_error.set(Some(format!("Failed to parse: {}", e)));
                            set_health_loading.set(false);
                        }
                    }
                }
                Err(e) => {
                    set_health_error.set(Some(format!("Failed to connect: {}", e)));
                    set_health_loading.set(false);
                }
            }
        });
    };

    let get_api_info = move || {
        spawn_local(async move {
            set_api_loading.set(true);
            set_api_error.set(None);

            let result = Request::get("http://localhost:3000/api/v1/info")
                .send()
                .await;

            match result {
                Ok(response) => {
                    match response.json::<ApiInfo>().await {
                        Ok(data) => {
                            set_api_info.set(Some(data));
                            set_api_loading.set(false);
                        }
                        Err(e) => {
                            set_api_error.set(Some(format!("Failed to parse: {}", e)));
                            set_api_loading.set(false);
                        }
                    }
                }
                Err(e) => {
                    set_api_error.set(Some(format!("Failed to connect: {}", e)));
                    set_api_loading.set(false);
                }
            }
        });
    };

    view! {
        <div>
            <div class="card">
                <h2>"🏥 Backend Health Check"</h2>
                <button 
                    on:click=move |_| check_health()
                    disabled=move || health_loading.get()
                >
                    {move || if health_loading.get() { 
                        "⏳ Checking..." 
                    } else { 
                        "Check Backend Status" 
                    }}
                </button>

                {move || {
                    if let Some(status) = health_status.get() {
                        view! {
                            <div class="status success">
                                <div class="info-grid">
                                    <div class="info-item">
                                        <strong>{"Status".to_string()}</strong>
                                        <span>{status.status}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{"Message".to_string()}</strong>
                                        <span>{status.message}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{"Version".to_string()}</strong>
                                        <span>{status.version}</span>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else if let Some(err) = health_error.get() {
                        view! {
                            <div class="status error">
                                <div class="info-grid">
                                    <div class="info-item">
                                        <strong>{"Error".to_string()}</strong>
                                        <span>{err}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{String::new()}</strong>
                                        <span>{String::new()}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{String::new()}</strong>
                                        <span>{String::new()}</span>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="status">
                                <div class="info-grid">
                                    <div class="info-item">
                                        <strong>{"Info".to_string()}</strong>
                                        <span>{"Click the button to test backend connection".to_string()}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{String::new()}</strong>
                                        <span>{String::new()}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{String::new()}</strong>
                                        <span>{String::new()}</span>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    }
                }}
            </div>

            <div class="card">
                <h2>"📡 API Information"</h2>
                <button 
                    on:click=move |_| get_api_info()
                    disabled=move || api_loading.get()
                >
                    {move || if api_loading.get() { 
                        "⏳ Loading..." 
                    } else { 
                        "Get API Info" 
                    }}
                </button>

                {move || {
                    if let Some(info) = api_info.get() {
                        view! {
                            <div class="status success">
                                <div class="info-grid">
                                    <div class="info-item">
                                        <strong>{"Name".to_string()}</strong>
                                        <span>{info.name}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{"Version".to_string()}</strong>
                                        <span>{info.version}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{"Description".to_string()}</strong>
                                        <span>{info.description}</span>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else if let Some(err) = api_error.get() {
                        view! {
                            <div class="status error">
                                <div class="info-grid">
                                    <div class="info-item">
                                        <strong>{"Error".to_string()}</strong>
                                        <span>{err}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{String::new()}</strong>
                                        <span>{String::new()}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{String::new()}</strong>
                                        <span>{String::new()}</span>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="status">
                                <div class="info-grid">
                                    <div class="info-item">
                                        <strong>{"Info".to_string()}</strong>
                                        <span>{"Click the button to fetch API information".to_string()}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{String::new()}</strong>
                                        <span>{String::new()}</span>
                                    </div>
                                    <div class="info-item">
                                        <strong>{String::new()}</strong>
                                        <span>{String::new()}</span>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}