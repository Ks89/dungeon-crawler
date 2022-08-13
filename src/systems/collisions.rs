use crate::prelude::*;

// CommandBuffer is useful to remove entities

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query()
        .filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);
    let mut enemies = <(Entity, &Point)>::query()
        .filter(component::<Enemy>());
    enemies
        .iter(ecs)
        // remove &&, une form the param in input and the other applied automatically by .iter
        .filter(|(_, pos)| **pos == player_pos)// (3)
        .for_each(|(entity, _)| {// we don't need position
            commands.remove(*entity);
        });
}