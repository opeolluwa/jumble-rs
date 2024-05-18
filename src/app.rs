use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pkg;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let new_jumble = pkg::Library::new_jumble();
    view! {
        <main class="container">
            <div class="timer flex items-center jsutify-center rounded-full w-[100px] h-[100px] border-[10px] border-red-500 hidden ">
                15:25s
            </div>
            <div class="text-gray-400 text-3xl py-8 border-[1px] border-gray-100  text-center leading-1 bg-gray-100 rounded px-8 py-6 my-8">
                <div>{new_jumble.meaning}</div>
            </div>

            <ul class="flex gap-4 justify-center items-center mt-8 ">
                {new_jumble
                    .scrabble
                    .chars()
                    .into_iter()
                    .map(|char| {
                        view! {
                            <li class="flex text-xl flex-col items-center gap-col-4 text-gray-500 hover:text-violet-800 transition-all duration-300 bg-gray-100 hover:bg-violet-100 py-2 px-2 rounded-xl w-[50px] h-[50px] shadow cursor-pointer" onclick="alert('char')">
                                {char}
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}

            </ul>
        </main>
    }
}
