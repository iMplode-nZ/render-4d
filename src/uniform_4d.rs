use crate::camera_4d::CameraInternal;
use crate::surface::{DeviceResource, QueueResource};
use crate::world::WorldSize;
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use wgpu::*;

#[repr(C)]
#[derive(Resource, Pod, Zeroable, Clone, Copy, Debug)]
pub struct Uniforms {
    pub camera: CameraInternal,
    world_size: u32,
}

#[derive(Resource)]
pub struct UniformBuffer(pub Buffer);
#[derive(Resource)]
pub struct UniformBindGroup(pub BindGroup, pub BindGroupLayout);

pub fn init_uniforms(
    mut commands: Commands,
    device: Res<DeviceResource>,
    world_size: Res<WorldSize>,
) {
    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some("uniform-4d-buffer"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("uniform-4d-bind-group-layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("uniform-4d-bind-group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });
    commands.insert_resource(Uniforms {
        camera: Default::default(),
        world_size: world_size.0 + 2,
    });
    commands.insert_resource(UniformBuffer(buffer));
    commands.insert_resource(UniformBindGroup(bind_group, bind_group_layout));
}

pub fn update_uniform_buffer(
    uniforms: Res<Uniforms>,
    queue: Res<QueueResource>,
    buffer: ResMut<UniformBuffer>,
) {
    if uniforms.is_changed() {
        queue.write_buffer(&buffer.0, 0, bytemuck::cast_slice(&[*uniforms]));
    }
}
