use seed::{*, prelude::*, virtual_dom::node};

pub fn horizontal_group<M>(label: &str, inputs: &[node::Node<M>]) -> node::Node<M> {
    div![
        C!["field", "is-horizontal"],
        div![
            C!["field-label", "is-normal"],
            label![
                C!["label"],
                label,
            ]
        ],
        div![
            C!["field-body"],
            inputs,
        ],
    ]
}

pub fn button<F, M>(message: &str, on_input: F) -> node::Node<M> 
where 
    F: FnOnce() -> M + Clone + 'static,
    M: 'static,
{
    div![
        C!["field"],
        div![
            C!["control"],
            button![
                C!["button"],
                input_ev(Ev::Click, |_| on_input()),
                message,
            ]
        ]
    ]
}
