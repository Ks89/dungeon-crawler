use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
#[read_component(Damage)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query
        .iter(ecs).next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_color(
        Point::new(1, 1),
        "HP ",
        ColorPair::new(RED, BLACK),
    );
    draw_batch.bar_horizontal(
        Point::new(4, 1),
        SCREEN_WIDTH / 3,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color(
        Point::new((SCREEN_WIDTH / 3) + 5, 1),
        format!("{} / {}",
                player_health.current,
                player_health.max
        ),
        ColorPair::new(RED, BLACK),
    );

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    draw_batch.print_color_right(
        Point::new((SCREEN_WIDTH * 2) - 1, 1),
        format!("Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 4;
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(
                Point::new(1, y + 1),
                format!("{} : {}", y - 3, &name.0),
            );
            y += 1;
        }
        );
    if y > 4 {
        draw_batch.print_color(
            Point::new(1, 3),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    // display current weapon
    let mut weapon_query = <(&Weapon, &Carried, &Damage, &Name)>::query();
    weapon_query
        .iter(ecs)
        .filter(|(_, carried, _, _)| carried.0 == player)
        .for_each(|(_, _, d, n)| {
            draw_batch.print_color(
                Point::new(1, (SCREEN_HEIGHT * 2) - 2),
                format!("Weapon: {} (attack: +{})", &n.0, &d.0),
                ColorPair::new(YELLOW, BLACK),
            );
        });

    draw_batch.submit(10000).expect("Batch error");
}