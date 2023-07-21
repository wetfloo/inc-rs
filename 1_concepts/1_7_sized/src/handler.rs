use crate::prerequisite::{Command, CommandHandler, User, UserRepository};

impl<Key, Cmd: Command> CommandHandler<Key, Cmd> for User {
    type Context = dyn UserRepository<Key, Self>;
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &Cmd, ctx: &Self::Context) -> Self::Result {
        Ok(())
    }
}

impl<C: ?Sized> Command for CreateUser {
    type Context = C;

    fn run(ctx: &Self::Context) {
        todo!()
    }
}

pub struct CreateUser;

#[derive(Debug)]
pub struct UserError;
