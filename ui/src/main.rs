use leptos::*;
use leptos::html::Input;
use leptos_router::*;
use leptos_meta::*;

/// A test API
const TEST_API: &str = "https://jsonplaceholder.typicode.com/todos/";

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// A progress bar component
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar
    #[prop(default = 100)]
    max: u16,
    /// The current progress of the bar
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max value=progress />
    }
}

/// An input component that updates a signal
#[component]
fn InputCount(
    /// The signal to update
    #[prop(into)]
    count: Signal<i32>,
    /// The setter signal to update
    #[prop(into)]
    set_count: WriteSignal<i32>,
) -> impl IntoView {
    view! {
        <input
            type="number"
            prop:value=count
            on:input=move |ev| {
                let num = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                set_count(num);
            }
        />
    }
}

/// An input that only accepts small positive numbers
#[component]
fn InputSmallNum() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<u8>());

    view! {
        <h1>"Error handling"</h1>
        <label>
            "Type a number"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || errors()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

/// A component that calls an API
#[component]
fn APICaller() -> impl IntoView {
    let (count, set_count) = create_signal(1);
    let (data, set_data) = create_signal::<Vec<String>>(vec![]);

    let async_data = create_resource(
        count,
        // every time the data changes, fetch the API
        |count| async move {
            logging::log!("fetching data");
            let url = format!("{}{}", TEST_API, count);
            reqwest::get(url).await.unwrap().text().await.unwrap()
        },
    );

    create_effect(move |_| {
        logging::log!("Updating data");
        match async_data.get() {
            None => logging::log!("No data yet"),
            Some(txt) => set_data([data(), vec![txt]].concat()),
        }
    });

    view! {
        <button
            on:click=move |_| set_count(count() + 1)
        >
            "Fetch data"
        </button>
        <p>Fetched data {count} times</p>
        <p>The effect is updating the data</p>
        <ul>
            {move || data()
                .into_iter()
                .map(|d| view! { <li>{d}</li> })
                .collect_view()
            }
        </ul>
        <p>I tried create_effect and set_data inside the resource definition.
            Only set data inside resource definition worked for updating the ui
            I was not able to get create_effect to run or get data from tje resource.
        </p>
        <p>
        But now this works! I used create_effect with the resource to update the state!
        It works because a Resource is a Signal
        </p>
        <p>
        The current resource state:
        {
            move || {
                match async_data.get() {
                    None => "Loading...".to_string(),
                    Some(txt) => txt,
                }
            }
        }
        </p>
        <p>Could I run a fetch handler using on:click?</p>

    }
}

/// A component that uses Actions to call an API
/// A component that calls an API
#[component]
fn APIAction() -> impl IntoView {

    let action = create_action(|count: &i32| {
        let count = count.clone();
        async move {
            logging::log!("fetching action data");
            let url = format!("{}{}", TEST_API, count);
            reqwest::get(url).await.unwrap().text().await.unwrap()
        }
    });

    let _submitted = action.input();
    let pending = action.pending();
    let action_data = action.value();

    let input_ref = create_node_ref::<Input>();

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let input = input_ref.get().expect("input to exist");
            action.dispatch(input.value().parse::<i32>().unwrap_or(0));
        }
        >
            <label>
                "Search for a user id: "
                <input type="number" node_ref=input_ref/>
            </label>
            <button type="submit">"Submit"</button>
        </form>
        <p>{move || pending().then(|| "Loading...")}</p>
        <p>{move || action_data().map(|txt| txt.to_string())}</p>
        <p>
        A match statement: 
        {
            move || match action_data() {
                None => "No data yet".to_string(),
                Some(txt) => txt,
            }
        }
        </p>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    create_effect(move |_| {
        logging::log!("count changed to {}", count());
    });

    let double_count = move || count() * 2;

    view! {
        <Title text="Leptos examples"/>
        <h1>Event dispatch and dynamic classes</h1>
        <button
            class:red=move || count() % 2 == 0
            on:click=move |_| {
                set_count(count() + 1);
            }
        >
            "Increment"
        </button>
        <button
            class:red=move || count() % 2 == 0
            on:click=move |_| {
                set_count(count() - 1);
            }
        >
            "Decrement"
        </button>
        <p>
            "Count " {count}
        </p>

        <h1>Components</h1>
        <h2>Progress bars</h2>
        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>

        <h1>Forms and inputs</h1>
        <h2>Controlled inputs</h2>
        <p>In this example, the props to the component are read and write signals.
        The component updates the count signal when the input changes.</p>
        <label>
            Update the count: <InputCount count=count set_count=set_count/>
        </label>

        <h1>Control Flow</h1>
        <h2>If</h2>
        <p>This example uses inline closures to conditionally render text</p>
        <p>
            "The count is: " {move || if count() % 2 == 0 { "Even" } else { "Odd" }}
        </p>
        <p>
            This example uses a match statement to better handle zero and one.
            One of the match arms uses a closure to check if the count is even.
        </p>
        <h2>Match</h2>
        <p>
        {
            move || {
                logging::log!("match statement re-running with count {}", count());
                match count() {
                    0 => "The count is zero",
                    1 => "The count is one",
                    n if (|| n % 2 == 0)() => "The count is even",
                    _ => "The count is odd",
                }
            }
        }
        </p>
        <p>
            {move || if is_prime(count()) { Some("The count is prime") } else { None } }
        </p>
        <h2>Show</h2>
        <p>To avoid re-running the closure, we can use the show component</p>
        <Show
            when=move || { count() > 5 }
            fallback=move || {
                logging::log!("show component re-running with count {}", count());
                view! {
                    <p>"The count is less than 5"</p>
                }
            }
        >
            { move || { logging::log!("show component re-running with count {}", count()); } }
            <p>"The count is greater than 5"</p>
        </Show>

        <h2>Error handling</h2>
        <InputSmallNum />

        <h2>Effects</h2>
        <p>A component that logs whenever count changes</p>
    }
}

#[component]
fn AsyncExamples() -> impl IntoView {
    view! {
        <Title text="Leptos examples - Async"/>
        <h1>Async</h1>
        <h2>Loading data with Resources</h2>

        <APICaller />

        <h2>Mutating data with Actions</h2>
        <p>Resources are for reacting to changes in signals</p>
        <p>Actions can be triggered whenever they are needed, such as a button click</p>

        <APIAction />
    }
}

#[component]
fn MetaExamples() -> impl IntoView {
    view! {
        <h1>Meta</h1>
        <p>This page demos the capabilities of the leptos_meta crate</p>
        <h2>JavaScript</h2>
        <script>
            let a = 10;
            let b = 20;
            console.log("Logging from JavaScript");

            const div = document.createElement("div");
            const content = document.createTextNode("This div was created in JavaScript");
            div.appendChild(content);
            document.body.appendChild(div);
        </script>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <ul>
                    <li><a href="/">"Home"</a></li>
                    <li><a href="/async">"Async"</a></li>
                    <li><a href="/meta">"Meta"</a></li>
                </ul>
            </nav>
            <main class="my-0 mx-auto max-w-3xl">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/async" view=AsyncExamples/>
                    <Route path="/meta" view=MetaExamples/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App/>  });
}
