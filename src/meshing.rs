use crate::voxel::Voxel;
use crate::world::{Chunk, CHUNK_SIZE};

#[derive(Default, Clone)]
pub struct SurfaceMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

pub fn mesh_chunk(chunk: &Chunk) -> SurfaceMesh {
    let mut mesh = SurfaceMesh::default();

    let n = CHUNK_SIZE as u32;
    for z in 0..n {
        for y in 0..n {
            for x in 0..n {
                let v = chunk.get(x, y, z);
                if v.is_empty() { continue; }

                if x == 0 || chunk.get(x - 1, y, z).is_empty() {
                    push_face_neg_x(&mut mesh, x, y, z);
                }
                if x + 1 >= n || chunk.get(x + 1, y, z).is_empty() {
                    push_face_pos_x(&mut mesh, x, y, z);
                }
                if y == 0 || chunk.get(x, y - 1, z).is_empty() {
                    push_face_neg_y(&mut mesh, x, y, z);
                }
                if y + 1 >= n || chunk.get(x, y + 1, z).is_empty() {
                    push_face_pos_y(&mut mesh, x, y, z);
                }
                if z == 0 || chunk.get(x, y, z - 1).is_empty() {
                    push_face_neg_z(&mut mesh, x, y, z);
                }
                if z + 1 >= n || chunk.get(x, y, z + 1).is_empty() {
                    push_face_pos_z(&mut mesh, x, y, z);
                }
            }
        }
    }

    mesh
}

fn push_quad(mesh: &mut SurfaceMesh, verts: [[f32; 3]; 4], normal: [f32; 3]) {
    let base = mesh.positions.len() as u32;
    mesh.positions.extend_from_slice(&verts);
    mesh.normals.extend_from_slice(&[normal; 4]);
    mesh.uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

fn push_face_neg_x(mesh: &mut SurfaceMesh, x: u32, y: u32, z: u32) {
    let x0 = x as f32;
    let y0 = y as f32;
    let z0 = z as f32;
    let verts = [
        [x0, y0, z0],
        [x0, y0 + 1.0, z0],
        [x0, y0 + 1.0, z0 + 1.0],
        [x0, y0, z0 + 1.0],
    ];
    push_quad(mesh, verts, [-1.0, 0.0, 0.0]);
}

fn push_face_pos_x(mesh: &mut SurfaceMesh, x: u32, y: u32, z: u32) {
    let x1 = x as f32 + 1.0;
    let y0 = y as f32;
    let z0 = z as f32;
    let verts = [
        [x1, y0, z0],
        [x1, y0, z0 + 1.0],
        [x1, y0 + 1.0, z0 + 1.0],
        [x1, y0 + 1.0, z0],
    ];
    push_quad(mesh, verts, [1.0, 0.0, 0.0]);
}

fn push_face_neg_y(mesh: &mut SurfaceMesh, x: u32, y: u32, z: u32) {
    let x0 = x as f32;
    let y0 = y as f32;
    let z0 = z as f32;
    let verts = [
        [x0, y0, z0],
        [x0, y0, z0 + 1.0],
        [x0 + 1.0, y0, z0 + 1.0],
        [x0 + 1.0, y0, z0],
    ];
    push_quad(mesh, verts, [0.0, -1.0, 0.0]);
}

fn push_face_pos_y(mesh: &mut SurfaceMesh, x: u32, y: u32, z: u32) {
    let x0 = x as f32;
    let y1 = y as f32 + 1.0;
    let z0 = z as f32;
    let verts = [
        [x0, y1, z0],
        [x0 + 1.0, y1, z0],
        [x0 + 1.0, y1, z0 + 1.0],
        [x0, y1, z0 + 1.0],
    ];
    push_quad(mesh, verts, [0.0, 1.0, 0.0]);
}

fn push_face_neg_z(mesh: &mut SurfaceMesh, x: u32, y: u32, z: u32) {
    let x0 = x as f32;
    let y0 = y as f32;
    let z0 = z as f32;
    let verts = [
        [x0, y0, z0],
        [x0 + 1.0, y0, z0],
        [x0 + 1.0, y0 + 1.0, z0],
        [x0, y0 + 1.0, z0],
    ];
    push_quad(mesh, verts, [0.0, 0.0, -1.0]);
}

fn push_face_pos_z(mesh: &mut SurfaceMesh, x: u32, y: u32, z: u32) {
    let x0 = x as f32;
    let y0 = y as f32;
    let z1 = z as f32 + 1.0;
    let verts = [
        [x0, y0, z1],
        [x0, y0 + 1.0, z1],
        [x0 + 1.0, y0 + 1.0, z1],
        [x0 + 1.0, y0, z1],
    ];
    push_quad(mesh, verts, [0.0, 0.0, 1.0]);
}
