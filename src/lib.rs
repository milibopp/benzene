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

fn actions<C>(app: Arc<C>, inputs: &Communication<C::Context, C::Event>)
    -> Stream<C::Action>
    where C: Application + Send + Sync + 'static,
          C::Action: Clone + Send + Sync + 'static,
          C::Context: Clone + Send + Sync + 'static,
          C::Event: Clone + Send + Sync + 'static
{
    inputs.context
        .snapshot(&inputs.events, move |x, y| app.intent(x, y))
        .filter_some()
}

fn state<C>(app: Arc<C>, actions: &Stream<C::Action>) -> Signal<C::State>
    where C: Component + Send + Sync + 'static,
          C::Action: Clone + Send + Sync + 'static,
          C::State: Clone + Send + Sync + 'static
{
    actions.fold(app.init(), move |x, y| app.update(x, y))
}

fn view<C>(app: Arc<C>, context: &Signal<C::Context>, state: &Signal<C::State>)
    -> Signal<C::View>
    where C: Component + Send + Sync + 'static,
          C::State: Clone + Send + Sync + 'static,
          C::Context: Clone + Send + Sync + 'static,
          C::View: Clone + Send + Sync + 'static
{
    lift!(move |x, y| app.view(x, y), context, state)
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
    let app = Arc::new(app);
    let actions = actions(app.clone(), &inputs);
    let state = state(app.clone(), &actions);
    Communication {
        context: view(app.clone(), &inputs.context, &state),
        events: state.snapshot(&actions, move |a, b| app.effect(a, b)).filter_some()
    }
}
