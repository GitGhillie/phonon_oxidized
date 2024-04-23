use super::effect::DirectEffectWrapped;
use super::handle::DirectEffectHandle;
use kira::track::effect::{Effect, EffectBuilder};
use phonon::direct_effect::DirectEffectParameters;
use ringbuf::HeapRb;

const COMMAND_CAPACITY: usize = 32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DirectEffectBuilder {
    pub parameters: DirectEffectParameters,
}

impl EffectBuilder for DirectEffectBuilder {
    type Handle = DirectEffectHandle;

    fn build(self) -> (Box<dyn Effect>, Self::Handle) {
        let (command_producer, command_consumer) = HeapRb::new(COMMAND_CAPACITY).split();
        (
            Box::new(DirectEffectWrapped::new(self, command_consumer)),
            DirectEffectHandle { command_producer },
        )
    }
}
