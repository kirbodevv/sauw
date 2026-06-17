use crate::game::{
    GameState, ImageAssets,
    player::{Player, inventory::Inventory},
    registry::item_registry::ItemRegistry,
    ui::hud::HudBottom,
};
use bevy::prelude::*;

pub const HOTBAR_SLOT_COUNT: usize = 8;
const SLOT_SIZE: f32 = 60.0;
const ITEM_SIZE: f32 = 32.0;

const DIGIT_KEYS: [KeyCode; HOTBAR_SLOT_COUNT] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
];

#[derive(Component)]
pub struct Hotbar;

#[derive(Component)]
pub struct HotbarSlot {
    pub index: usize,
}

#[derive(Component)]
struct HotbarItemIcon {
    slot_index: usize,
}

#[derive(Component)]
struct HotbarItemCount {
    slot_index: usize,
}

#[derive(Component)]
struct HotbarSelectedMarker;

#[derive(Resource, Default)]
pub struct SelectedHotbarSlot(pub usize);

fn spawn_wrapper(cmd: &mut Commands) -> Entity {
    cmd.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::End,
        ..default()
    })
    .id()
}

fn spawn_container(cmd: &mut Commands, assets: &ImageAssets) -> Entity {
    cmd.spawn((
        Hotbar,
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::End,
            column_gap: Val::Px(2.0),
            height: Val::Px(SLOT_SIZE),
            ..default()
        },
        ImageNode {
            image: assets.ui_inventory.clone(),
            ..default()
        },
    ))
    .id()
}

fn spawn_slot(cmd: &mut Commands, assets: &ImageAssets, index: usize) -> Entity {
    let slot = cmd
        .spawn((
            HotbarSlot { index },
            Node {
                width: Val::Px(SLOT_SIZE),
                height: Val::Px(SLOT_SIZE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    let mut children = Vec::new();
    if index == 0 {
        children.push(spawn_selected_marker(cmd, assets));
    }
    children.push(spawn_item_icon(cmd, index));
    children.push(spawn_item_count(cmd, index));

    cmd.entity(slot).add_children(&children);
    slot
}

fn spawn_selected_marker(cmd: &mut Commands, assets: &ImageAssets) -> Entity {
    cmd.spawn((
        HotbarSelectedMarker,
        ImageNode {
            image: assets.ui_selected_slot.clone(),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(SLOT_SIZE),
            height: Val::Px(SLOT_SIZE),
            ..default()
        },
        ZIndex(1),
    ))
    .id()
}

fn spawn_item_icon(cmd: &mut Commands, slot_index: usize) -> Entity {
    cmd.spawn((
        HotbarItemIcon { slot_index },
        ImageNode {
            color: Color::WHITE.with_alpha(0.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(ITEM_SIZE),
            height: Val::Px(ITEM_SIZE),
            top: Val::Px((SLOT_SIZE - ITEM_SIZE) / 2.0),
            left: Val::Px((SLOT_SIZE - ITEM_SIZE) / 2.0),
            ..default()
        },
        ZIndex(2),
    ))
    .id()
}

fn spawn_item_count(cmd: &mut Commands, slot_index: usize) -> Entity {
    cmd.spawn((
        HotbarItemCount { slot_index },
        Text::new(""),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(6.0),
            right: Val::Px(6.0),
            ..default()
        },
        ZIndex(3),
    ))
    .id()
}

fn spawn_hotbar(
    mut cmd: Commands,
    assets: Res<ImageAssets>,
    hud_bottom: Single<Entity, With<HudBottom>>,
) {
    let wrapper = spawn_wrapper(&mut cmd);
    let container = spawn_container(&mut cmd, &assets);
    let slots: Vec<Entity> = (0..HOTBAR_SLOT_COUNT)
        .map(|i| spawn_slot(&mut cmd, &assets, i))
        .collect();

    cmd.entity(container).add_children(&slots);
    cmd.entity(wrapper).add_child(container);
    cmd.entity(*hud_bottom).add_child(wrapper);
}

fn update_hotbar_items(
    q_player: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    item_registry: Res<ItemRegistry>,
    mut q_icons: Query<(&HotbarItemIcon, &mut ImageNode)>,
    mut q_counts: Query<(&HotbarItemCount, &mut Text)>,
    assets: Res<ImageAssets>,
) {
    let Ok(inventory) = q_player.single() else {
        return;
    };

    for (icon, mut img) in &mut q_icons {
        match inventory
            .slots
            .get(icon.slot_index)
            .and_then(|s| s.as_ref())
        {
            Some(stack) => {
                let def = item_registry.get(stack.item);
                img.image = assets.atlas_item_texture.clone();
                img.texture_atlas = Some(TextureAtlas {
                    layout: item_registry.atlas_layout.clone(),
                    index: def.atlas_index,
                });
                img.color = Color::WHITE;
            }
            None => {
                img.image = Handle::default();
                img.texture_atlas = None;
                img.color = Color::WHITE.with_alpha(0.0);
            }
        }
    }

    for (count_comp, mut text) in &mut q_counts {
        **text = match inventory
            .slots
            .get(count_comp.slot_index)
            .and_then(|s| s.as_ref())
        {
            Some(stack) if stack.count > 1 => stack.count.to_string(),
            _ => String::new(),
        };
    }
}

fn handle_slot_selection(
    mut cmd: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedHotbarSlot>,
    assets: Res<ImageAssets>,
    q_slots: Query<(Entity, &HotbarSlot)>,
    q_markers: Query<Entity, With<HotbarSelectedMarker>>,
) {
    let new_index = DIGIT_KEYS
        .iter()
        .enumerate()
        .find_map(|(i, key)| keyboard.just_pressed(*key).then_some(i));

    let Some(new_index) = new_index else { return };
    if selected.0 == new_index {
        return;
    }
    selected.0 = new_index;

    for entity in &q_markers {
        cmd.entity(entity).despawn();
    }

    let Some((slot_entity, _)) = q_slots.iter().find(|(_, slot)| slot.index == new_index) else {
        return;
    };
    let marker = spawn_selected_marker(&mut cmd, &assets);
    cmd.entity(slot_entity).add_child(marker);
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedHotbarSlot>()
            .add_systems(OnEnter(GameState::Bootstrap), spawn_hotbar)
            .add_systems(
                Update,
                (handle_slot_selection, update_hotbar_items).run_if(in_state(GameState::Gaming)),
            );
    }
}
