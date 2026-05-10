use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_icons::Icon;

use crate::context::auth::use_auth;

#[derive(Clone)]
struct MenuItem {
    label: &'static str,
    path: &'static str,
    icon: icondata::Icon,
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
                class="flex items-center gap-2 px-3 py-2 rounded-lg text-slate-700 hover:bg-slate-100 lg:px-3 lg:py-2 lg:text-sm xl:text-base w-full lg:w-auto text-left lg:text-center"
                on:click=create_menu_item_click(item.path)
            >
                <Icon icon=item.icon width="1.2em" height="1.2em" />
                {item.label}
            </button>
        }).collect::<Vec<_>>()}
        <button
            class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-slate-100 text-blue-700 w-full lg:w-auto text-left lg:text-center"
                title="Perfil do usuário"
            >
                <Icon icon=icondata::FaCircleUserSolid width="1.2em" height="1.2em" />
                Perfil
            </button>
        <button
            class="flex items-center justify-center px-3 py-2 rounded-lg bg-red-500 text-white hover:bg-red-600 w-full lg:w-auto text-left lg:text-center"
            on:click=move |_| (logout)()
            title="Sair"
            aria-label="Sair"
        >
            <Icon icon=icondata::IoLogOut width="1.5em" height="1.5em" />
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
        MenuItem { label: "Dashboard", path: "/home", icon: icondata::FaGaugeSolid },
        MenuItem { label: "Funcionários", path: "/funcionarios", icon: icondata::FaUserGroupSolid },
        MenuItem { label: "Clientes", path: "/clientes", icon: icondata::FaUserSolid },
        MenuItem { label: "Produtos", path: "/produtos", icon: icondata::FaUtensilsSolid },
        MenuItem { label: "Comandas", path: "/comandas", icon: icondata::FaReceiptSolid },
        MenuItem { label: "Caixa", path: "/caixa", icon: icondata::FaCashRegisterSolid },
    ];

    let toggle_drawer = move |_| set_mobile_drawer_open.update(|open| *open = !*open);

    let nav_home = navigate.clone();
    let goto_home = move |_| {
        nav_home("/", Default::default());
        set_mobile_drawer_open.set(false);
    };

    let nav_desk = navigate.clone();
    let goto_login_desktop = move |_| {
        nav_desk("/login", Default::default());
        set_mobile_drawer_open.set(false);
    };
    let menu_items_desktop = menu_items.clone();
    let logout_desktop = logout.clone();

    let nav_mob = navigate.clone();
    let goto_login_mobile = move |_| {
        nav_mob("/login", Default::default());
        set_mobile_drawer_open.set(false);
    };
    let menu_items_mobile = menu_items.clone();
    let logout_mobile = logout.clone();

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
        <header class="bg-sky-300 border-b border-sky-400 shadow-sm sticky top-0 z-40">
            <div class="max-w-7xl mx-auto px-3 py-3 sm:px-4 flex items-center justify-between gap-3">
                <div class="flex min-w-0 items-center gap-3 sm:gap-4">
                    <button
                        class="lg:hidden px-3 py-2 rounded-lg bg-white/90 hover:bg-white"
                        on:click=toggle_drawer
                        aria-label="Abrir menu"
                    >
                        "☰"
                    </button>

                    <Icon icon=icondata::FaMoneyCheckSolid width="1.2em" height="1.2em" />

                    <button
                        class="truncate text-lg font-bold text-slate-900"
                        on:click=goto_home
                    >
                        "E-Comandas"
                    </button>
                    <div class="h-10 w-10 shrink-0 overflow-hidden rounded-full bg-white">
                        <img
                            class="h-full w-full object-cover"
                            src="/public/minhacarafeia.jpg?v=3"
                            alt="Minha cara"
                        />
                    </div>
                </div>

                
                <div class="hidden lg:flex items-center gap-2 xl:gap-3">
                    <Show 
                        when=is_authenticated
                        fallback=move || view! {
                            <button
                                class="px-4 py-2 rounded-lg bg-slate-900 text-white hover:bg-slate-800"
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
                class="lg:hidden bg-white border-t border-slate-200 shadow-sm"
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
