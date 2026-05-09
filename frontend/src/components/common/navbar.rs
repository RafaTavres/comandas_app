use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::context::auth::use_auth;

#[derive(Clone)]
struct MenuItem {
    label: &'static str,
    path: &'static str,
}

#[component]
fn MenuButtons(
    menu_items: Vec<MenuItem>,
    logout: std::sync::Arc<dyn Fn() + Send + Sync>,
    create_menu_item_click: std::sync::Arc<dyn Fn(&'static str) -> Box<dyn Fn(web_sys::MouseEvent) + 'static> + 'static>,
) -> impl IntoView {
    view! {
        {menu_items.into_iter().map(|item| view! {
            <button
                class="px-3 py-2 rounded-lg text-slate-700 hover:bg-slate-100 sm:px-3 sm:py-2 sm:text-base w-full sm:w-auto text-left sm:text-center"
                on:click=create_menu_item_click(item.path)
            >
                {item.label}
            </button>
        }).collect::<Vec<_>>()}
        <button
            class="px-3 py-2 rounded-lg bg-red-500 text-white hover:bg-red-600 w-full sm:w-auto text-left sm:text-center"
            on:click=move |_| (logout)()
        >
            "Sair"
        </button>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let (mobile_drawer_open, set_mobile_drawer_open) = signal(false);

    let logout = auth.logout.clone();
    let is_authenticated = move || auth.is_authenticated.get();

    let menu_items = vec![
        MenuItem { label: "Dashboard", path: "/home" },
        MenuItem { label: "Funcionários", path: "/funcionarios" },
        MenuItem { label: "Clientes", path: "/clientes" },
        MenuItem { label: "Produtos", path: "/produtos" },
        MenuItem { label: "Comandas", path: "/comandas" },
        MenuItem { label: "Caixa", path: "/caixa" },
    ];

    let toggle_drawer = move |_| set_mobile_drawer_open.update(|open| *open = !*open);

    // --- INSTÂNCIA 1: Botão Logo ---
    let nav_home = navigate.clone();
    let goto_home = move |_| {
        nav_home("/", Default::default());
        set_mobile_drawer_open.set(false);
    };

    // --- INSTÂNCIA 2: Menu Desktop ---
    let nav_desk = navigate.clone();
    let goto_login_desktop = move |_| {
        nav_desk("/login", Default::default());
        set_mobile_drawer_open.set(false);
    };
    let menu_items_desktop = menu_items.clone();
    let logout_desktop = logout.clone();

    // --- INSTÂNCIA 3: Menu Mobile ---
    let nav_mob = navigate.clone();
    let goto_login_mobile = move |_| {
        nav_mob("/login", Default::default());
        set_mobile_drawer_open.set(false);
    };
    let menu_items_mobile = menu_items.clone();
    let logout_mobile = logout.clone();

    // --- INSTÂNCIA 4: Cliques do Menu ---
    let nav_menu = navigate.clone();
    let create_menu_item_click = std::sync::Arc::new(move |path: &'static str| {
        let nav_click = nav_menu.clone();
        Box::new(move |_: web_sys::MouseEvent| {
            nav_click(path, Default::default());
            set_mobile_drawer_open.set(false);
        }) as Box<dyn Fn(web_sys::MouseEvent) + 'static>
    });

    let create_click_desktop = create_menu_item_click.clone();
    let create_click_mobile = create_menu_item_click;

    view! {
        <header class="bg-white border-b border-slate-200 sticky top-0 z-40">
            <div class="max-w-7xl mx-auto px-4 py-3 flex items-center justify-between gap-3">
                <div class="flex items-center gap-4">
                    <button
                        class="sm:hidden px-3 py-2 rounded-lg bg-slate-100 hover:bg-slate-200"
                        on:click=toggle_drawer
                        aria-label="Abrir menu"
                    >
                        "☰"
                    </button>

                    <button
                        class="text-lg font-bold text-slate-900"
                        on:click=goto_home
                    >
                        "Comandas do Zé"
                    </button>
                </div>

                // === MENU DESKTOP ===
                <div class="hidden sm:flex items-center gap-3">
                    <Show 
                        when=is_authenticated
                        fallback=move || view! {
                            <button
                                class="px-4 py-2 rounded-lg bg-amber-500 text-white hover:bg-amber-600"
                                on:click=goto_login_desktop.clone()
                            >
                                "Login"
                            </button>
                        }
                    >
                        <MenuButtons
                            menu_items=menu_items_desktop.clone()
                            logout=logout_desktop.clone()
                            create_menu_item_click=create_click_desktop.clone()
                        />
                    </Show>
                </div>
            </div>

            <div 
                class="sm:hidden bg-white border-t border-slate-200"
                class=("hidden", move || !mobile_drawer_open.get()) 
            >
                <div class="flex flex-col gap-1 p-4">
                    <Show 
                        when=is_authenticated
                        fallback=move || view! {
                            <button
                                class="w-full text-left px-4 py-3 rounded-lg text-slate-700 hover:bg-slate-100"
                                on:click=goto_login_mobile.clone()
                            >
                                "Login"
                            </button>
                        }
                    >
                        <MenuButtons
                            menu_items=menu_items_mobile.clone()
                            logout=logout_mobile.clone()
                            create_menu_item_click=create_click_mobile.clone()
                        />
                    </Show>
                </div>
            </div>
        </header>
    }
}