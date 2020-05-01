extern crate gl;

use gl::types::*;
use glm::*;
use std::vec::Vec;
use tobj::*;

pub struct Vertex {
    nor: Vec3,
    pos: Vec3,
    tex: Vec2,
}

pub struct Mesh {
    draw_count: i32,
    vao: u32,
    vao_buffer: [u32; 4],
}

pub struct IndexedModel {
    positions: Vec<f32>,
    normals: Vec<f32>,
    textures: Vec<f32>,
    indices: Vec<u32>,
}

pub fn draw(mesh: &mut Mesh) {
    unsafe {
        gl::BindVertexArray(mesh.vao);
        gl::DrawElementsBaseVertex(
            gl::TRIANGLES,
            mesh.draw_count,
            gl::UNSIGNED_INT,
            0 as *mut std::ffi::c_void,
            0,
        );
        gl::BindVertexArray(0);
    }
}

pub fn delete_mesh(mesh: &Mesh) {
    unsafe {
        gl::DeleteBuffers(4, &mesh.vao);
        gl::DeleteVertexArrays(1, &mesh.vao);
    }
}

pub fn init_mesh(model: &mut IndexedModel) -> Mesh {
    let mut mesh = Mesh {
        draw_count: 0,
        vao: 0,
        vao_buffer: [0, 0, 0, 0],
    };
    unsafe {
        mesh.draw_count = model.indices.len() as i32;
        gl::GenVertexArrays(1, &mut mesh.vao);
        gl::BindVertexArray(mesh.vao);

        gl::GenBuffers(4, mesh.vao_buffer.as_mut_ptr());
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vao_buffer[0]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            model.positions.len() as isize * std::mem::size_of::<u32>() as isize,
            model.positions.as_mut_ptr() as *mut std::ffi::c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *mut std::ffi::c_void);

        gl::GenBuffers(3, mesh.vao_buffer.as_mut_ptr());
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vao_buffer[1]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            model.textures.len() as isize * std::mem::size_of::<u32>() as isize,
            model.textures.as_mut_ptr() as *mut std::ffi::c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, 0 as *mut std::ffi::c_void);

        gl::GenBuffers(3, mesh.vao_buffer.as_mut_ptr());
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vao_buffer[2]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            model.normals.len() as isize * std::mem::size_of::<u32>() as isize,
            model.normals.as_mut_ptr() as *mut std::ffi::c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, 0 as *mut std::ffi::c_void);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.vao_buffer[3]);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            model.indices.len() as isize * std::mem::size_of::<u32>() as isize,
            model.indices.as_mut_ptr() as *mut std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindVertexArray(0);
    }
    mesh
}

pub fn create_mesh_from_file(file: &str) -> IndexedModel {
    let mut model = IndexedModel {
        positions: Vec::with_capacity(1),
        normals: Vec::with_capacity(1),
        textures: Vec::with_capacity(1),
        indices: Vec::with_capacity(1),
    };

    let path = std::path::Path::new(file);

    std::println!("{}", path.display());

    let loaded_obj = tobj::load_obj(path);

    assert!(loaded_obj.is_ok());

    std::println!("{}", file);

    let (models, _) = loaded_obj.unwrap();

    for (i, m) in models.iter().enumerate() {
        let loaded_mesh = &m.mesh;

        for f in 0..loaded_mesh.positions.len() {
            model.positions.push(loaded_mesh.positions[f]);
        }

        for f in 0..loaded_mesh.normals.len() {
            model.normals.push(loaded_mesh.normals[f]);
        }
        for f in 0..loaded_mesh.texcoords.len() {
            model.textures.push(loaded_mesh.texcoords[f]);
        }

        for f in 0..loaded_mesh.indices.len() {
            model.indices.push(loaded_mesh.indices[f]);
        }
    }
    model
}

pub fn create_mesh(
    vertices: &[Vertex],
    vertices_len: usize,
    indices: &[u32],
    indices_len: usize,
) -> IndexedModel {
    let mut model = IndexedModel {
        positions: Vec::with_capacity(vertices_len),
        normals: Vec::with_capacity(vertices_len),
        textures: Vec::with_capacity(vertices_len),
        indices: Vec::with_capacity(indices_len),
    };

    for i in 0..vertices_len {
        model.positions.push(vertices[i].pos.x);
        model.positions.push(vertices[i].pos.y);
        model.positions.push(vertices[i].pos.z);
        model.textures.push(vertices[i].tex.x);
        model.textures.push(vertices[i].tex.y);
        model.normals.push(vertices[i].nor.x);
        model.normals.push(vertices[i].nor.y);
        model.normals.push(vertices[i].nor.z);
    }

    for i in 0..indices_len {
        model.indices.push(indices[i]);
    }
    model
}
