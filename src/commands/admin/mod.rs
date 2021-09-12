use crate::commands::CommandHelpers;
use crate::game_object::GameObjectRef;
use crate::objects::Realm;
use crate::player_output::PlayerOutput;

mod enter_room_command;

pub use enter_room_command::enter_room;

pub fn wrap_admin_command<F>(
    f: F,
) -> Box<dyn Fn(&mut Realm, GameObjectRef, CommandHelpers) -> Vec<PlayerOutput>>
where
    F: Fn(&mut Realm, GameObjectRef, CommandHelpers) -> Result<Vec<PlayerOutput>, &'static str>
        + 'static,
{
    Box::new(
        move |realm, player_ref, helpers| match realm.player(player_ref) {
            Some(player) if player.is_admin() => match f(realm, player_ref, helpers) {
                Ok(output) => output,
                Err(message) => {
                    let mut output: Vec<PlayerOutput> = Vec::new();
                    push_output_str!(output, player_ref, message);
                    output
                }
            },
            _ => {
                let mut output: Vec<PlayerOutput> = Vec::new();
                push_output_str!(output, player_ref, "You are not an admin.");
                output
            }
        },
    )
}
