//! A component wrapping a `<button>` tag that changes the route.
use crate::{
    agent::{RouteAgentDispatcher, RouteRequest},
    route::Route,
};
use yew::prelude::*;

use super::{Msg, Props};
use crate::RouterState;
use yew::virtual_dom::VNode;

/// Changes the route when clicked.
#[derive(Debug)]
pub struct RouterButton<T: for<'de> RouterState<'de>> {
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<T>,
    props: Props<T>,
}

impl<T: for<'de> RouterState<'de>> Component for RouterButton<T> {
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        RouterButton {
            link,
            router,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                let route = Route {
                    route: self.props.link.clone(),
                    state: self.props.state.clone(),
                };
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        let cb = |x| self.link.callback(x);
        html! {
            <button
                class=self.props.classes.clone(),
                onclick=cb(|_| Msg::Clicked),
                disabled=self.props.disabled,
            >
                {&self.props.text}
            </button>
        }
    }
}
