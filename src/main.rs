#![feature(div_duration)]

use crate::surface::init_surface;
use crate::view::{init_view, ViewSize};
use crate::voxel::VoxelType;
use crate::window_size::{init_window_size, update_window_size};
use crate::world::{init_world, update_world, World, WorldSize};
use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use nalgebra::{Vector3, Vector4};
use palette::Srgb;
use surface::update_surface;

mod camera_3d;
mod camera_4d;
mod render_3d;
mod render_4d;
mod surface;
mod uniform_3d;
mod uniform_4d;
mod utils;
mod view;
mod voxel;
mod window_size;
mod world;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "render-4d".to_string(),
        width: 500.0,
        height: 500.0,
        ..Default::default()
    })
    .insert_resource(WorldSize(88))
    .insert_resource(ViewSize(128))
    .insert_resource(camera_3d::Camera::new(Vector3::new(4.0, 4.0, 4.0), 0.0))
    .insert_resource(camera_4d::Camera::new());
    app.add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(camera_3d::CameraPlugin)
        .add_plugin(camera_4d::CameraPlugin);
    app.add_startup_stage_after(
        StartupStage::Startup,
        "startup-surface",
        SystemStage::single_threaded(),
    )
    .add_startup_stage_after(
        "startup-surface",
        "startup-bind-groups",
        SystemStage::single_threaded(),
    )
    .add_startup_stage_after(
        "startup-bind-groups",
        "startup-pipeline",
        SystemStage::single_threaded(),
    )
    .add_startup_stage_after(
        "startup-pipeline",
        "startup-finish",
        SystemStage::single_threaded(),
    );
    app.add_startup_system(init_window_size)
        .add_startup_system_to_stage("startup-surface", init_surface)
        .add_startup_system_to_stage("startup-bind-groups", uniform_4d::init_uniforms)
        .add_startup_system_to_stage("startup-bind-groups", uniform_3d::init_uniforms)
        .add_startup_system_to_stage("startup-bind-groups", init_world)
        .add_startup_system_to_stage("startup-bind-groups", init_view)
        .add_startup_system_to_stage("startup-pipeline", render_4d::init_render_pipeline)
        .add_startup_system_to_stage("startup-pipeline", render_3d::init_render_pipeline)
        .add_startup_system_to_stage("startup-finish", init_world_data)
        .add_system(update_window_size.before("update-surface"))
        .add_system(update_surface.label("update-surface"))
        .add_system(update_world.label("update-world"))
        .add_system(
            uniform_4d::update_uniform_buffer
                .label("update-uniforms-4d")
                .after("camera-4d")
                .after("update-surface"),
        )
        .add_system(
            uniform_3d::update_uniform_buffer
                .label("update-uniforms-3d")
                .after("camera-3d")
                .after("update-surface"),
        )
        .add_system(
            render_4d::render
                .label("render-4d")
                .after("update-uniforms-4d")
                .after("update-world"),
        )
        .add_system(
            render_3d::render
                .label("render-3d")
                .after("update-uniforms-3d")
                .after("render-4d"),
        );
    app.run();
}

fn init_world_data(mut world: ResMut<World>) {
    let normal_type = world.insert_type(VoxelType::new(Srgb::new(0.212, 0.247, 0.278)));

    for i in 10..40 {
        for j in 35..60 {
            for k in 35..55 {
                for l in 10..75 {
                    world[Vector4::new(i, j, k, l)] = normal_type;
                }
            }
        }
    }
    for i in 20..70 {
        for j in 16..40 {
            for k in 16..25 {
                for l in 16..40 {
                    world[Vector4::new(i, j, k, l)] = normal_type;
                }
            }
        }
    }
}
