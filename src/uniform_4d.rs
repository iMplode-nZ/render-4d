use crate::camera_4d::CameraInternal;
use crate::world::WorldSize;
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use wgpu::*;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Uniforms {
    pub camera: CameraInternal,
    world_size: u32,
}

pub struct UniformBuffer(pub Buffer);
pub struct UniformBindGroup(pub BindGroup, pub BindGroupLayout);

pub fn init_uniforms(mut commands: Commands, device: Res<Device>, world_size: Res<WorldSize>) {
    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some("uniform-4d-buffer"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("uniform-4d-bind-group-layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStage::COMPUTE,
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
    queue: Res<Queue>,
    buffer: ResMut<UniformBuffer>,
) {
    if uniforms.is_changed() {
        queue.write_buffer(&buffer.0, 0, bytemuck::cast_slice(&[*uniforms]));
    }
}
