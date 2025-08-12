use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures_core::{FusedStream, Future, Stream};
use futures_util::{never::Never, FutureExt, StreamExt};
use gc::Finalize;
use gloo::{events::EventListener, utils::body};
use plotters::{
    coord::Shift,
    drawing::{DrawingArea, IntoDrawingArea},
};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt::Debug,
    future::pending,
    mem,
    ops::Deref,
    pin::{pin, Pin},
    process::Output,
    rc::Rc,
    task::{Context, Poll},
};
use tokio::sync::watch::{self, Receiver, Sender};
use tokio_stream::wrappers::WatchStream;
use tracing::warn;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};
use web_sys::{
    Event, HtmlAnchorElement, HtmlBrElement, HtmlButtonElement, HtmlCanvasElement, HtmlDivElement,
    HtmlElement, HtmlFieldSetElement, HtmlFormElement, HtmlHeadingElement, HtmlHrElement,
    HtmlInputElement, HtmlLabelElement, HtmlLiElement, HtmlOptionElement, HtmlParagraphElement,
    HtmlSelectElement, HtmlSpanElement, HtmlTableCellElement, HtmlTableElement,
    HtmlTableRowElement, HtmlTextAreaElement, MutationObserver,
};

pub mod svg_backend;
use svg_backend::SVGBackend;

#[derive(Debug)]
struct ReactiveRcInner<T> {
    value: RefCell<Rc<T>>,
}

impl<T> ReactiveRcInner<T> {
    fn new(t: T) -> ReactiveRcInner<T> {
        ReactiveRcInner {
            value: RefCell::new(Rc::new(t)),
        }
    }
    fn get(&self) -> Rc<T> {
        Rc::clone(&self.value.borrow())
    }
    fn replace(&self, value: Rc<T>) -> Rc<T> {
        self.value.replace(value)
    }
}

// also have ReactiveRcMap
// with init + versioned changes in a FrozenVec
// allowing catchup etc

#[derive(Clone)]
pub struct ReactiveRc<T> {
    // inner: Rc<ReactiveRcInner<T>>,
    sender: Rc<Sender<ReactiveRcInner<T>>>,
}

impl<T> ReactiveRc<T> {
    pub fn new(t: T) -> ReactiveRc<T> {
        ReactiveRc {
            sender: Rc::new(Sender::new(ReactiveRcInner::new(t))),
        }
    }
    #[inline]
    pub fn get(&self) -> Rc<T> {
        self.sender.borrow().get()
    }
    #[inline]
    pub fn replace(&self, value: Rc<T>) -> Rc<T> {
        let old_val = self.sender.borrow().get();
        self.sender.send_modify(|existing| {
            existing.replace(value);
        });
        old_val
    }
    pub fn subscribe(&self) -> SubscriptionRc<T> {
        SubscriptionRc {
            receiver: self.sender.subscribe(),
        }
    }
}

pub struct SubscriptionRc<T> {
    receiver: Receiver<ReactiveRcInner<T>>,
}

impl<T> SubscriptionRc<T> {
    #[inline]
    pub fn get(&self) -> Rc<T> {
        self.receiver.borrow().get()
    }
    pub async fn next(&mut self) -> Option<Rc<T>> {
        self.changed().await.map(|_| self.get())
    }
    pub async fn changed(&mut self) -> Option<()> {
        self.receiver.changed().await.ok()
    }
    pub async fn pair<U>(&mut self, other: &mut SubscriptionRc<U>) -> (Rc<T>, Rc<U>) {
        let _ =
            futures_util::future::select(Box::pin(self.changed()), Box::pin(other.changed())).await;
        (self.get(), other.get())
    }
}

impl<T> Stream for SubscriptionRc<T> {
    type Item = Rc<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match pin!(self.get_mut().next()).as_mut().poll(cx) {
            Poll::Ready(val) => Poll::Ready(val),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> FusedStream for SubscriptionRc<T> {
    fn is_terminated(&self) -> bool {
        todo!()
    }
}

// impl<T> SubscriptionRc<T>
// where T: Clone + Sync + Send + 'static
// {

//     pub fn stream(&self) -> impl Stream {
//         WatchStream::new(self.receiver.clone()).map(|i|async move { i.get() })
//     }
// }

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    JsValue(wasm_bindgen::JsValue),
}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(t: wasm_bindgen::JsValue) -> Error {
        Error::JsValue(t)
    }
}

impl From<Error> for wasm_bindgen::JsValue {
    fn from(t: Error) -> wasm_bindgen::JsValue {
        match t {
            Error::JsValue(j) => j,
        }
    }
}

thread_local! {
    static DOCUMENT: web_sys::Document = web_sys::window().unwrap_throw().document().unwrap_throw();
}

pub struct AllowsText;
pub struct HasText;

pub struct AllowsId;

pub struct HasId;

pub struct Element<H, TextSet = AllowsText, IdSet = AllowsId> {
    // dom: Rc<H>,
    text_set: TextSet,
    class_set: bool,
    id_set: IdSet,
    dom: H,
    events: BTreeMap<&'static str, Closure<dyn FnMut(web_sys::Event)>>,
    // children: Vec<Rc<web_sys::HtmlElement>>,
    // id_set: bool,
    // class_set: bool,
    // has_text: bool,
}

pub fn header() -> Element<HtmlElement> {
    Element::<_, AllowsText, AllowsId>::new("header")
}
pub fn footer() -> Element<HtmlElement> {
    Element::<_, AllowsText, AllowsId>::new("footer")
}
pub fn aside() -> Element<HtmlElement> {
    Element::<_, AllowsText, AllowsId>::new("aside")
}
pub fn article() -> Element<HtmlElement> {
    Element::<_, AllowsText, AllowsId>::new("article")
}
pub fn thead() -> Element<HtmlElement> {
    Element::<_, AllowsText, AllowsId>::new("thead")
}
pub fn tbody() -> Element<HtmlElement> {
    Element::<_, AllowsText, AllowsId>::new("tbody")
}
pub fn section() -> Element<HtmlElement> {
    Element::<_, AllowsText, AllowsId>::new("section")
}

pub fn div() -> Element<HtmlDivElement> {
    Element::<_, AllowsText, AllowsId>::new("div")
}

pub fn h1() -> Element<HtmlHeadingElement> {
    Element::<_, AllowsText, AllowsId>::new("h1")
}

pub fn h2() -> Element<HtmlHeadingElement> {
    Element::<_, AllowsText, AllowsId>::new("h2")
}

pub fn h3() -> Element<HtmlHeadingElement> {
    Element::<_, AllowsText, AllowsId>::new("h3")
}

pub fn h4() -> Element<HtmlHeadingElement> {
    Element::<_, AllowsText, AllowsId>::new("h4")
}

pub fn h5() -> Element<HtmlHeadingElement> {
    Element::<_, AllowsText, AllowsId>::new("h5")
}

pub fn button() -> Element<HtmlButtonElement> {
    Element::<_, AllowsText, AllowsId>::new("button")
}

pub fn p() -> Element<HtmlParagraphElement> {
    Element::<_, AllowsText, AllowsId>::new("p")
}

pub fn a() -> Element<HtmlAnchorElement> {
    Element::<_, AllowsText, AllowsId>::new("a")
}

pub fn canvas() -> Element<HtmlCanvasElement> {
    Element::<_, AllowsText, AllowsId>::new("canvas")
}

pub fn br() -> Element<HtmlBrElement> {
    Element::<_, AllowsText, AllowsId>::new("br")
}

pub fn fieldset() -> Element<HtmlFieldSetElement> {
    Element::<_, AllowsText, AllowsId>::new("fieldset")
}

pub fn form() -> Element<HtmlFormElement> {
    Element::<_, AllowsText, AllowsId>::new("form")
}

pub fn input() -> Element<HtmlInputElement> {
    Element::<_, AllowsText, AllowsId>::new("input")
}

pub fn label() -> Element<HtmlLabelElement> {
    Element::<_, AllowsText, AllowsId>::new("label")
}

pub fn hr() -> Element<HtmlHrElement> {
    Element::<_, AllowsText, AllowsId>::new("hr")
}

pub fn li() -> Element<HtmlLiElement> {
    Element::<_, AllowsText, AllowsId>::new("li")
}

pub fn option() -> Element<HtmlOptionElement> {
    Element::<_, AllowsText, AllowsId>::new("option")
}

pub fn select() -> Element<HtmlSelectElement> {
    Element::<_, AllowsText, AllowsId>::new("select")
}

pub fn span() -> Element<HtmlSpanElement> {
    Element::<_, AllowsText, AllowsId>::new("span")
}

pub fn td() -> Element<HtmlTableCellElement> {
    Element::<_, AllowsText, AllowsId>::new("td")
}

pub fn table() -> Element<HtmlTableElement> {
    Element::<_, AllowsText, AllowsId>::new("table")
}

pub fn tr() -> Element<HtmlTableRowElement> {
    Element::<_, AllowsText, AllowsId>::new("tr")
}

pub fn th() -> Element<HtmlTableCellElement> {
    Element::<_, AllowsText, AllowsId>::new("th")
}

pub fn textarea() -> Element<HtmlTextAreaElement> {
    Element::<_, AllowsText, AllowsId>::new("textarea")
}

// maybe need another impl for H where Deref Target is Element..?
// pub fn p() -> Element<HtmlElement> {
//     Element::new("p")
// }

// enum or equivalent for Event types?

// add a type param here for eg, HtmlDivElement
// children vec could contain Rc<dyn Deref<Target=Element>> or something
// element children could also store pre-rendered children
// impl<H: Deref<Target = web_sys::HtmlElement> + JsCast + Clone + 'static> Element<H> {

impl<H, IdSet> Element<H, AllowsText, IdSet>
where
    H: AsRef<web_sys::HtmlElement> + JsCast + Clone + 'static,
{
    pub fn static_text(mut self, text: &str) -> Element<H, HasText, IdSet> {
        if !AsRef::<web_sys::HtmlElement>::as_ref(&self.dom).has_child_nodes() {
            AsRef::<web_sys::HtmlElement>::as_ref(&self.dom).set_inner_html(text);
        } else {
            //  ???!!
        };
        Element {
            text_set: HasText,
            id_set: self.id_set,
            class_set: self.class_set,
            dom: self.dom,
            events: self.events,
        }
    }
    pub fn dynamic_text<T: 'static>(
        self,
        mut subscription: SubscriptionRc<T>,
        mut handler: impl FnMut(Rc<T>) -> String + 'static,
    ) -> Element<H, HasText, IdSet> {
        let this = self.static_text(&handler(subscription.get()));

        let dom_ref = this.dom.clone();

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(val) = subscription.next().await {
                AsRef::<web_sys::HtmlElement>::as_ref(&dom_ref).set_inner_html(&handler(val));
            }
        });

        this
    }
}

impl<H, TextSet> Element<H, TextSet, AllowsId>
where
    H: AsRef<web_sys::HtmlElement> + JsCast + Clone + 'static,
{
    pub fn set_id(mut self, value: &str) -> Element<H, TextSet, HasId> {
        AsRef::<web_sys::HtmlElement>::as_ref(&self.dom).set_id(value);

        Element {
            id_set: HasId,
            text_set: self.text_set,
            class_set: self.class_set,
            dom: self.dom,
            events: self.events,
        }
    }
}

impl<H, TextSet, IdSet> Element<H, TextSet, IdSet>
where
    H: AsRef<web_sys::HtmlElement>
        + JsCast
        + Clone
        + 'static
        + AsRef<web_sys::EventTarget>
        + AsRef<web_sys::Node>,
    IdSet: 'static,
    TextSet: 'static,
{
    pub fn event<M: 'static, I: 'static>(
        mut self,
        model: Rc<M>,
        mut handler: impl FnMut(Rc<M>, Event, H) -> I + 'static,
        event: &'static str,
    ) -> Self {
        // while JS allows for multiple event listeners on the same event,
        // that shouldn't be needed here so we remove the old one, if it exists
        if let Some(existing) = self.events.remove(event) {
            // remove existing listener
            <H as AsRef<web_sys::EventTarget>>::as_ref(&self.dom)
                .remove_event_listener_with_callback(event, existing.as_ref().unchecked_ref())
                .ok();
        }
        let new_callback = Closure::wrap(Box::new({
            let model = model.clone();
            // let dom = self.dom.clone();
            move |e: web_sys::Event| {
                let dom = e.target().unwrap().unchecked_into();
                handler(model.clone(), e, dom);
            }
        }) as Box<dyn FnMut(web_sys::Event)>);

        // other option is using `into_js_value` here with WASMBIND_WEAKREF=1
        if let Err(e) = <H as AsRef<web_sys::EventTarget>>::as_ref(&self.dom)
            .add_event_listener_with_callback(event, new_callback.as_ref().unchecked_ref())
        {
            warn!("Error adding event listener: {e:?}");
        } else {
            self.events.insert(event, new_callback);
        }

        self
    }

    pub async fn mount_on_body_and_run(self) {
        let mount_point = body();

        mount_point.append_child(self.dom.as_ref()).unwrap_throw();

        pending::<()>().await;

        mem::drop(self);
    }

    // impl<H: JsCast + Clone + Debug + 'static> Element<H> {
    pub fn new(tag: &'static str) -> Element<H> {
        let base_el = DOCUMENT.with(|d| d.create_element(tag)).unwrap_throw();
        Element {
            dom: base_el.dyn_into::<H>().unwrap_throw(),
            id_set: AllowsId,
            class_set: false,
            text_set: AllowsText,
            events: BTreeMap::new(),
        }
    }
    pub fn dom_ref(&self) -> &H {
        &self.dom
    }

    pub fn dynamic_chart<T: 'static>(
        self,
        alt: impl FnOnce(&HtmlDivElement),
        mut subscription: SubscriptionRc<T>,
        mut handler: impl FnMut(Rc<T>, &mut DrawingArea<SVGBackend, Shift>) + 'static,
    ) -> Self {
        let svg_box = div().dom;
        alt(&svg_box);

        let mut backend = SVGBackend::new((640, 480), &svg_box).into_drawing_area();

        AsRef::<web_sys::HtmlElement>::as_ref(&self.dom)
            .append_child(&svg_box.unchecked_into::<web_sys::HtmlElement>())
            .unwrap_throw();

        handler(subscription.get(), &mut backend);

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(val) = subscription.next().await {
                handler(val, &mut backend);
            }
        });

        self
    }

    pub fn static_child<O, OTextSet, OIdSet>(self, el: Element<O, OTextSet, OIdSet>) -> Self
    where
        O: Deref<Target = web_sys::HtmlElement> + JsCast + Clone + 'static,
    {
        // self.dom.append_child(&Rc::new(el.dom.unchecked_into::<web_sys::HtmlElement>()));
        AsRef::<web_sys::HtmlElement>::as_ref(&self.dom)
            .append_child(&el.dom.unchecked_into::<web_sys::HtmlElement>())
            .unwrap_throw();
        // self.children
        //     .push(Rc::new(el.dom.unchecked_into::<web_sys::HtmlElement>()));
        self
    }

    pub fn dynamic_child<O, OTextSet, OIdSet, T>(
        self,
        mut subscription: SubscriptionRc<T>,
        // while this could have been Option<HtmlElement>
        // isntead encourage users to use visible: false for this
        // as otherwise the position has to be tracked elsewhere which sucks
        mut handler: impl FnMut(Rc<T>) -> Element<O, OTextSet, OIdSet> + 'static,
    ) -> Self
    where
        O: Deref<Target = web_sys::HtmlElement> + JsCast + Clone + 'static,
        T: 'static,
    {
        let this = self.static_child(handler(subscription.get()));

        let dom_ref = this.dom.clone();

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(val) = subscription.next().await {
                AsRef::<web_sys::HtmlElement>::as_ref(&dom_ref)
                    .parent_node()
                    .unwrap_throw()
                    .replace_child(
                        &handler(val).dom,
                        AsRef::<web_sys::HtmlElement>::as_ref(&dom_ref),
                    )
                    .unwrap_throw();
            }
        });

        this
    }

    pub fn dynamic_with_element<T: 'static>(
        self,
        mut subscription: SubscriptionRc<T>,
        // while this could have been Option<HtmlElement>
        // isntead encourage users to use visible: false for this
        // as otherwise the position has to be tracked elsewhere which sucks
        mut handler: impl FnMut(&H, Rc<T>) + 'static,
    ) -> Self {
        let dom_ref = self.dom.clone();
        handler(&dom_ref, subscription.get());

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(_) = subscription.next().await {
                handler(&dom_ref, subscription.get());
            }
        });

        self
    }

    // is this ever useful?
    // potentially:
    // 1. adding rows to a table
    // other?
    // if this is implemented, it might be useful to have the data structure
    // in the ReactiveRc be a initial Vec/Map and a FrozenVec/Map which stores the diffs.
    // that way the consumer here can simply track which index it is up to and apply all changes
    // ensuring no changes are ever missed
    // pub fn dynamic_children(mut self) -> Self {
    //     todo!()
    // }

    // if want to have just one global update,
    // could use some once-cell and Rc funtimes
    // to thread the Sender through here and avoid
    // users of the library having to pass it
    // but this probably doesn't make sense
    // as in that case all the event listeners and
    // handles would hang around forever!

    pub fn dynamic_attribute<T: 'static>(
        self,
        key: &'static str,
        mut subscription: SubscriptionRc<T>,
        mut handler: impl FnMut(Rc<T>) -> String + 'static,
    ) -> Self {
        let this = self.static_attribute(key, &handler(subscription.get()));

        let dom_ref = this.dom.clone();

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(val) = subscription.next().await {
                AsRef::<web_sys::HtmlElement>::as_ref(&dom_ref)
                    .set_attribute(key, &handler(val))
                    .unwrap_throw();
            }
        });

        this
    }

    pub fn static_attribute(self, key: &str, value: &str) -> Self {
        AsRef::<web_sys::HtmlElement>::as_ref(&self.dom)
            .set_attribute(key, value)
            .unwrap_throw();
        self
    }

    pub fn dynamic_class(
        self,
        class: &'static str, // is there ever a legit need to only know class name at runtime?
        mut subscription: SubscriptionRc<bool>,
    ) -> Self {
        let this = if *subscription.get() {
            self.static_class(class)
        } else {
            self
        };

        let dom_ref = this.dom.clone();

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(val) = subscription.next().await {
                let el_ref = AsRef::<web_sys::HtmlElement>::as_ref(&dom_ref);
                let current = el_ref.class_name();
                match (*val, current.contains(class)) {
                    (true, true) => {
                        // no-op
                    }
                    (false, true) => {
                        el_ref.set_class_name(&current.replace(class, ""));
                    }
                    (true, false) => {
                        el_ref.set_class_name(&format!("{current} {class}"));
                    }
                    (false, false) => {
                        // no-op
                    }
                }
            }
        });

        this
    }

    pub fn static_class(mut self, class: &'static str) -> Self {
        let el_ref = AsRef::<web_sys::HtmlElement>::as_ref(&self.dom);
        if !self.class_set {
            el_ref.set_class_name(class);
            self.class_set = true;
        } else {
            el_ref.set_class_name(&format!("{} {class}", el_ref.class_name()));
        }
        self
    }
}
