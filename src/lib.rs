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

pub struct Component<State, Update, View, Effect> {
    pub init: State,
    pub update: Update,
    pub view: View,
    pub effect: Effect
}

pub fn interpret<Context, Event, Action, Intent>(
        inputs: Communication<Context, Event>, intent: Intent)
        -> Communication<Context, Action>
    where Context: Clone + Send + Sync + 'static,
          Event: Clone + Send + Sync + 'static,
          Action: Clone + Send + Sync + 'static,
          Intent: Fn(Context, Event) -> Option<Action> + Send + Sync + 'static
{
    let events = inputs.context
        .snapshot(&inputs.events, intent)
        .filter_some();
    Communication {
        context: inputs.context,
        events: events
    }
}

pub fn start<Context, Event, State, Update, View, Effect, ViewOut, EffectOut>(
        app: Component<State, Update, View, Effect>,
        inputs: Communication<Context, Event>)
        -> Communication<ViewOut, EffectOut>
    where Event: Clone + Send + Sync + 'static,
          State: Clone + Send + Sync + 'static,
          Context: Clone + Send + Sync + 'static,
          ViewOut: Clone + Send + Sync + 'static,
          EffectOut: Clone + Send + Sync + 'static,
          Update: Fn(State, Event) -> State + Send + Sync + 'static,
          View: Fn(Context, State) -> ViewOut + Send + Sync + 'static,
          Effect: Fn(State, Event) -> Option<EffectOut> + Send + Sync + 'static
{
    let Component { init, update, view, effect } = app;
    let state = inputs.events.fold(init, update);
    Communication {
        context: lift!(view, &inputs.context, &state),
        events: state.snapshot(&inputs.events, effect).filter_some()
    }
}
