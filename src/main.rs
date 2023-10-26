use leptos::{*, leptos_dom::logging::*};

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let color = create_memo(move |_| if count.get() % 2 == 0 { "red" } else { "green" });
    let custom_html =
        create_memo(move |_| format!("<p>This HTML will be {} injected.</p>", color.get()));



        
    view! {
        <div>
            <button
                on:click=move |_e| set_count.update(|n| *n += 1)
                class=move || format!("text-2xl {}", color.get())
                style:color=color
                data=color
            >
                "Click me: "
                {count}
            </button>
            <ProgressBar progress=count />
            <div inner_html=custom_html />
        </div>
    }
}


#[component]
fn ProgressBar(
    progress: ReadSignal<i32>
) -> impl IntoView {
    
    create_effect(|_| {
        console_log("nihao222")
    });

    view! {
        <progress
            max="50"
            // now this works
            value=progress
            on:change= |_| console_log("progress change")
        />
    }
}
