use crate::{AddMessageChannelAppExtensions, ProcedureResultMessage, StdbPlugin};
use bevy::prelude::World;
use spacetimedb_sdk::__codegen as spacetime_codegen;
use std::sync::mpsc::{Sender, channel};

/// Trait for making a procedure registerable into the bevy application.
pub trait RegisterableProcedureMessage<
    C: spacetime_codegen::DbConnection<Module = M> + spacetimedb_sdk::DbContext,
    M: spacetime_codegen::SpacetimeModule<DbConnection = C>,
> where
    Self: Sized,
{
    /// The function that should define the stdb callback behaviour, and send a bevy message through sender.
    fn set_stdb_callback(procedures: &C::Procedures, sender: Sender<ProcedureResultMessage<Self>>);
}

impl<
    C: spacetime_codegen::DbConnection<Module = M> + spacetimedb_sdk::DbContext,
    M: spacetime_codegen::SpacetimeModule<DbConnection = C>,
> StdbPlugin<C, M>
{
    /// Registers a procedure message <E> for the bevy application.
    pub fn add_procedure<E: RegisterableProcedureMessage<C, M> + Send + Sync + 'static>(
        self,
    ) -> Self {
        // This callback manages the registration of the message.
        let register_fn = move |world: &mut World, procedures: &C::Procedures| {
            let (send, recv) = channel::<ProcedureResultMessage<E>>();
            world.add_message_channel(recv);
            E::set_stdb_callback(procedures, send);
        };

        // The register_fn will get called once the connection is built.
        self.procedure_registers
            .lock()
            .unwrap()
            .push(Box::new(register_fn));

        self
    }
}
