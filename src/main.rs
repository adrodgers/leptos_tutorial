use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        class:red=move || count() % 2 == 1
        >
            "Click me: "
            {move || count()}
        </button>
        <ProgressBar max=50 progress=count/>
        <ProgressBar progress=Signal::derive(double_count) max=50/>
        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>
    }
}

// In leptos, define props by giving additional arguments to the
// component function.
// If you have a component property that will change over time
// it should be a signal.
// Props can be set as optional.
// Props can be given a default value.

/// Docs can be added easily, show progress towards a goal.
#[component]
fn ProgressBar(
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
    /// Maximum value of the progress bar.
    #[prop(optional)]
    max: u16,
) -> impl IntoView {
    view! {
        <p>
        <progress
            max=max
            value=progress
        />
        </p>
    }
}
