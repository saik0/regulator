pub mod checkout_effector;

use crate::facts::offset::Offset;
use crate::{FactEnvelope, Stimulus, Utterance};

pub trait Effector {
    fn name(&self) -> &'static str;
    fn last_offset(&self) -> Offset;

    fn observe_utterance(&mut self, envelope: &FactEnvelope<Utterance>, offset: Offset);
    fn observe_stimulus(&mut self, envelope: &FactEnvelope<Stimulus>, offset: Offset);
}
