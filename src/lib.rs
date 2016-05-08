#[macro_use(lift)]
extern crate carboxyl;

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

pub struct Component<State, Update, View> {
    pub init: State,
    pub update: Update,
    pub view: View
}

pub struct Application<Component, Intent, Effect> {
    pub component: Component,
    pub intent: Intent,
    pub effect: Effect
}

pub fn start<State, Action, Update, View, Intent, Effect, Context, Event, ViewOut, EffectOut>(
        app: Application<Component<State, Update, View>, Intent, Effect>,
        inputs: Communication<Context, Event>)
        -> Communication<ViewOut, EffectOut>
    where Action: Clone + Send + Sync + 'static,
          State: Clone + Send + Sync + 'static,
          Context: Clone + Send + Sync + 'static,
          Event: Clone + Send + Sync + 'static,
          ViewOut: Clone + Send + Sync + 'static,
          EffectOut: Clone + Send + Sync + 'static,
          Update: Fn(State, Action) -> State + Send + Sync + 'static,
          View: Fn(Context, State) -> ViewOut + Send + Sync + 'static,
          Intent: Fn(Context, Event) -> Option<Action> + Send + Sync + 'static,
          Effect: Fn(State, Action) -> Option<EffectOut> + Send + Sync + 'static
{
    let Application {
        component: Component { init, update, view },
        intent, effect } = app;
    let actions = inputs.context
        .snapshot(&inputs.events, intent)
        .filter_some();
    let state = actions.fold(init, update);
    Communication {
        context: lift!(view, &inputs.context, &state),
        events: state.snapshot(&actions, effect).filter_some()
    }
}
