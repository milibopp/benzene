#[macro_use(lift)]
extern crate carboxyl;

use std::sync::Arc;
use carboxyl::{Signal, Stream};


pub trait Driver<Input> {
    type Output;

    fn output(&self) -> Self::Output;
    fn run(&mut self, input: Input);
}

#[derive(Clone)]
pub struct Communication<Context, Event> {
    pub context: Signal<Context>,
    pub events: Stream<Event>
}

/*pub struct Component<State, Update, View> {
    pub init: State,
    pub update: Update,
    pub view: View
}*/

pub trait Component {
    type Context;
    type Action;
    type State;
    type View;

    fn init(&self) -> Self::State;

    fn update(&self, current: Self::State, Self::Action) -> Self::State {
        current
    }

    fn view(&self, Self::Context, Self::State) -> Self::View;
}

pub trait Application: Component {
    type Event;
    type Effect;

    fn intent(&self, _: Self::Context, _: Self::Event) -> Option<Self::Action> {
        None
    }

    fn effect(&self, _: Self::State, _: Self::Action) -> Option<Self::Effect> {
        None
    }
}

pub fn start<C>(app: C, inputs: Communication<C::Context, C::Event>)
    -> Communication<C::View, C::Effect>
    where C: Application + Send + Sync + 'static,
          C::Action: Clone + Send + Sync + 'static,
          C::State: Clone + Send + Sync + 'static,
          C::Context: Clone + Send + Sync + 'static,
          C::Event: Clone + Send + Sync + 'static,
          C::View: Clone + Send + Sync + 'static,
          C::Effect: Clone + Send + Sync + 'static
{
    // Stupid boilerplate!!!
    let app = Arc::new(app);
    let intent = { let app = app.clone(); move |x, y| app.intent(x, y) };
    let update = { let app = app.clone(); move |x, y| app.update(x, y) };
    let view = { let app = app.clone(); move |x, y| app.view(x, y) };
    let init = app.init();

    // Logic
    let actions = inputs.context
        .snapshot(&inputs.events, intent)
        .filter_some();
    let state = actions.fold(init, update);
    Communication {
        context: lift!(view, &inputs.context, &state),
        events: state.snapshot(&actions, move |a, b| app.effect(a, b)).filter_some()
    }
}
