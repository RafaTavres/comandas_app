use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;

use crate::{
    components::common::UserProfile,
    context::auth::use_auth,
    utils::user_groups::{read_group, ADMINISTRADOR, CAIXA},
};

#[derive(Clone)]
struct MenuItem {
    label: &'static str,
    path: &'static str,
    icon: icondata::Icon,
}

fn menu_items_for_group(group: Option<i32>) -> Vec<MenuItem> {
    let mut items = vec![MenuItem {
        label: "Dashboard",
        path: "/home",
        icon: icondata::FaGaugeSolid,
    }];

    if group == Some(ADMINISTRADOR) {
        items.push(MenuItem {
            label: "Funcionarios",
            path: "/funcionarios",
            icon: icondata::FaUserGroupSolid,
        });
    }

    items.extend([
        MenuItem {
            label: "Clientes",
            path: "/clientes",
            icon: icondata::FaUserSolid,
        },
        MenuItem {
            label: "Produtos",
            path: "/produtos",
            icon: icondata::FaUtensilsSolid,
        },
        MenuItem {
            label: "Comandas",
            path: "/comandas",
            icon: icondata::FaReceiptSolid,
        },
    ]);

    if matches!(group, Some(ADMINISTRADOR | CAIXA)) {
        items.push(MenuItem {
            label: "Caixa",
            path: "/caixa",
            icon: icondata::FaCashRegisterSolid,
        });
    }

    items
}

#[component]
fn MenuButtons(
    menu_items: Vec<MenuItem>,
    logout: std::sync::Arc<dyn Fn() + Send + Sync>,
    create_menu_item_click: std::sync::Arc<
        dyn Fn(&'static str) -> Box<dyn Fn(web_sys::MouseEvent) + 'static> + 'static,
    >,
    on_profile_click: std::sync::Arc<dyn Fn(web_sys::MouseEvent) + 'static>,
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
            title="Perfil do usuario"
            on:click=move |ev| on_profile_click(ev)
        >
            <Icon icon=icondata::FaCircleUserSolid width="1.2em" height="1.2em" />
            "Perfil"
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
    let (profile_open, set_profile_open) = signal(false);

    let logout = auth.logout.clone();
    let user = auth.user;
    let is_authenticated = move || auth.is_authenticated.get();

    let visible_menu_items = move || {
        let user = user.get();
        let group = user.as_ref().and_then(read_group);

        menu_items_for_group(group)
    };

    let toggle_drawer = move |_| {
        set_mobile_drawer_open.update(|open| *open = !*open);
        set_profile_open.set(false);
    };

    let nav_home = navigate.clone();
    let goto_home = move |_| {
        nav_home("/", Default::default());
        set_mobile_drawer_open.set(false);
        set_profile_open.set(false);
    };

    let nav_desk = navigate.clone();
    let goto_login_desktop = move |_| {
        nav_desk("/login", Default::default());
        set_mobile_drawer_open.set(false);
        set_profile_open.set(false);
    };
    let logout_desktop = logout.clone();

    let nav_mob = navigate.clone();
    let goto_login_mobile = move |_| {
        nav_mob("/login", Default::default());
        set_mobile_drawer_open.set(false);
        set_profile_open.set(false);
    };
    let logout_mobile = logout.clone();

    let nav_menu = navigate.clone();
    let create_menu_item_click = std::sync::Arc::new(move |path: &'static str| {
        let nav_click = nav_menu.clone();
        Box::new(move |_: web_sys::MouseEvent| {
            nav_click(path, Default::default());
            set_mobile_drawer_open.set(false);
            set_profile_open.set(false);
        }) as Box<dyn Fn(web_sys::MouseEvent) + 'static>
    });

    let create_click_desktop = create_menu_item_click.clone();
    let create_click_mobile = create_menu_item_click;
    let profile_click_desktop = std::sync::Arc::new(move |_: web_sys::MouseEvent| {
        set_profile_open.update(|open| *open = !*open);
    });
    let profile_click_mobile = std::sync::Arc::new(move |_: web_sys::MouseEvent| {
        set_mobile_drawer_open.set(false);
        set_profile_open.set(true);
    });

    view! {
        <header class="bg-sky-300 border-b border-sky-400 shadow-sm sticky top-0 z-40">
            <div class="relative max-w-7xl mx-auto px-3 py-3 sm:px-4 flex items-center justify-between gap-3">
                <div class="flex min-w-0 items-center gap-3 sm:gap-4">
                    <button
                        class="lg:hidden px-3 py-2 rounded-lg bg-white/90 hover:bg-white"
                        on:click=toggle_drawer
                        aria-label="Abrir menu"
                    >
                        "Menu"
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
                            menu_items=visible_menu_items()
                            logout=logout_desktop.clone()
                            create_menu_item_click=create_click_desktop.clone()
                            on_profile_click=profile_click_desktop.clone()
                        />
                    </Show>
                </div>

                <UserProfile user=user open=profile_open.into() set_open=set_profile_open />
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
                            menu_items=visible_menu_items()
                            logout=logout_mobile.clone()
                            create_menu_item_click=create_click_mobile.clone()
                            on_profile_click=profile_click_mobile.clone()
                        />
                    </Show>
                </div>
            </div>
        </header>
    }
}
