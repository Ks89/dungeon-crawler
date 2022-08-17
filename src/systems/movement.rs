use crate::prelude::*;

#[system(for_each)]         // legion shorthand to run this system for every matching entity
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // instead of modifying the component, it's better to use commands. Legion can batch updates.
        commands.add_component(want_move.entity, want_move.destination);

        // to access to an entity outside of your query is simpler using entry_ref.
        // it returns Result.
        // get_component access to the entity component and call is_ok to check if it exists
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok()
                {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
    commands.remove(*entity);
}