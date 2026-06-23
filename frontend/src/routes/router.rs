use leptos::prelude::*;
use leptos_router::{components::{Routes, Route}, path};

use crate::components::forms::login_form::LoginForm;
use crate::pages::{
    CaixaDashboard, ClienteForm, ClienteList, ComandaConsumoForm, ComandaForm, ComandaList,
    Dashboard, FuncionarioForm, FuncionarioList, NotFound, ProdutoForm, ProdutoList,
    ProdutoListPublic, RecebimentoComprovante, RecebimentoForm, RecebimentoList,
};
use crate::utils::user_groups::{ADMINISTRADOR, CAIXA};
use super::{PrivateRoute, RestrictedRoute};

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
      <Suspense fallback=|| view! { <div>"Carregando..."</div> }>
         <Routes fallback=|| view! { <NotFound/> }>
            <Route path=path!("/") view=move || view! { <RestrictedRoute><LoginForm/></RestrictedRoute> } />
            <Route path=path!("/login") view=move || view! { <RestrictedRoute><LoginForm/></RestrictedRoute> } />
            <Route path=path!("/produtos/publica") view=move || view! { <ProdutoListPublic/> } />
            <Route path=path!("/home") view=move || view! { <PrivateRoute><Dashboard/></PrivateRoute> } />
            <Route path=path!("/dashboard") view=move || view! { <PrivateRoute><Dashboard/></PrivateRoute> } />
            <Route path=path!("/produtos") view=move || view! { <PrivateRoute><ProdutoList/></PrivateRoute> } />
            <Route path=path!("/produtos/novo") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR]><ProdutoForm/></PrivateRoute> } />
            <Route path=path!("/produto") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR]><ProdutoForm/></PrivateRoute> } />
            <Route path=path!("/produto/:opr/:id") view=move || view! { <PrivateRoute><ProdutoForm/></PrivateRoute> } />
            <Route path=path!("/funcionarios") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR]><FuncionarioList/></PrivateRoute> } />
            <Route path=path!("/funcionarios/novo") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR]><FuncionarioForm/></PrivateRoute> } />
            <Route path=path!("/funcionario") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR]><FuncionarioForm/></PrivateRoute> } />
            <Route path=path!("/funcionario/:opr/:id") view=move || view! { <PrivateRoute><FuncionarioForm/></PrivateRoute> } />
            <Route path=path!("/clientes") view=move || view! { <PrivateRoute><ClienteList/></PrivateRoute> } />
            <Route path=path!("/clientes/novo") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR, CAIXA]><ClienteForm/></PrivateRoute> } />
            <Route path=path!("/cliente") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR, CAIXA]><ClienteForm/></PrivateRoute> } />
            <Route path=path!("/cliente/:opr/:id") view=move || view! { <PrivateRoute><ClienteForm/></PrivateRoute> } />
            <Route path=path!("/comandas") view=move || view! { <PrivateRoute><ComandaList/></PrivateRoute> } />
            <Route path=path!("/comanda") view=move || view! { <PrivateRoute><ComandaForm/></PrivateRoute> } />
            <Route path=path!("/comanda/consumo/:id") view=move || view! { <PrivateRoute><ComandaConsumoForm/></PrivateRoute> } />
            <Route path=path!("/comanda/:opr/:id") view=move || view! { <PrivateRoute><ComandaForm/></PrivateRoute> } />
            <Route path=path!("/caixa") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR, CAIXA]><CaixaDashboard/></PrivateRoute> } />
            <Route path=path!("/caixa/receber") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR, CAIXA]><RecebimentoForm/></PrivateRoute> } />
            <Route path=path!("/recebimentos") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR, CAIXA]><RecebimentoList/></PrivateRoute> } />
            <Route path=path!("/recebimento") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR, CAIXA]><RecebimentoForm/></PrivateRoute> } />
            <Route path=path!("/recebimento/:id/comprovante") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR, CAIXA]><RecebimentoComprovante/></PrivateRoute> } />
            <Route path=path!("/recebimento/:opr/:id") view=move || view! { <PrivateRoute allowed_groups=vec![ADMINISTRADOR, CAIXA]><RecebimentoForm/></PrivateRoute> } />
         </Routes>
      </Suspense>
    }
}
