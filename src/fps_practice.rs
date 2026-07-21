use std::f32::consts::FRAC_PI_2;

use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*,
};


#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
pub struct MoveVelocity{
    horizontal: f32,
    vartical: f32
}

impl Default for MoveVelocity{
    fn default() -> Self{
        Self {
            horizontal: 1.5, 
            vartical: 1.0 
        }
    }
}

pub fn spawn_player_view(
    mut commands: Commands,
){
    commands.spawn((
        Player,
        CameraSensitivity::default(),
        MoveVelocity::default(),
        Camera3d::default(),
        Transform::from_xyz(0.0,9., 10.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        Projection::from(PerspectiveProjection{
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
    ));
}

pub fn mouse_look(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn move_body(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, & MoveVelocity), With<Player>>,
) {
    let (mut transform, move_velocity) = player.into_inner();
    let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
    let mut forward = *transform.forward();
    forward.y = 0_f32;
    let mut right   = *transform.right(); 
    right.y = 0_f32;

    // 
    // awの値を0にした後にxとzで正規化する
    // 

    if keyboard_input.pressed(KeyCode::KeyW){
        transform.translation += forward * move_velocity.vartical;
    }
    if keyboard_input.pressed(KeyCode::KeyA){
        transform.translation -= right * move_velocity.vartical;
    }
    if keyboard_input.pressed(KeyCode::KeyS){
        transform.translation -=  forward * move_velocity.vartical;
    }
    if keyboard_input.pressed(KeyCode::KeyD){
        transform.translation += right * move_velocity.vartical;
    }
}

