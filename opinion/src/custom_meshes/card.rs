use std::ops::Index;

use bevy::{asset::RenderAssetUsages, math::{vec3, Vec2}, mesh::{Indices, Mesh, PrimitiveTopology}};
// #[derive(Clone, Copy)]
pub struct UvRect {
    pub tl : Vec2,
    pub tr : Vec2,
    pub bl : Vec2,
    pub br : Vec2
}

impl From<(u16,u16,u16,u16)> for UvRect {
    fn from(value: (u16,u16,u16,u16)) -> Self {
	let (columns, rows, column_number, row_number) = value;
	assert!(column_number < columns, "column number ({}) exceeded columns ({})",column_number, columns);
	assert!(row_number < rows, "row number ({}) exceeded rows ({})",row_number, rows);
	
	let x = columns as f32;
	let y = rows as f32;
	let c = column_number as f32;
	let r = row_number as f32;
	let c_ = c+1.;
	let r_ = r+1.;
	UvRect {
	    tl: Vec2{x: c/x, y: r/y},
	    tr: Vec2{x: c_/x, y: r/y},
	    bl: Vec2{x: c/x, y: r_/y},
	    br: Vec2{x: c_/x, y: r_/y}
	}
    }
}

pub enum CubeSide {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back
}



pub struct UV1x3(pub [UvRect; 3]);
impl Default for UV1x3 {
    fn default() -> Self {
	UV1x3([
	    UvRect::from((1,3,0,0)),
	    UvRect::from((1,3,0,1)),
	    UvRect::from((1,3,0,2))
		]
	)
    }
}

impl Index<CubeSide> for UV1x3 {
    type Output = UvRect;
    fn index(&self, index: CubeSide) -> &Self::Output {
	return match index {
	    CubeSide::Top	=> &self.0[0],
	    CubeSide::Bottom	=> &self.0[1],
	    CubeSide::Front	=> &self.0[2],
	    CubeSide::Back	=> &self.0[2],
	    CubeSide::Left	=> &self.0[2],
	    CubeSide::Right	=> &self.0[2]
	}
    }
}





pub fn card(x: f32, y: f32, z: f32, uv: impl Index<CubeSide, Output = UvRect>) -> Mesh {
    // uv map 0-0.25 front face, 0.25-.5 back face, .5-.75 edges 
    let half_size = vec3(x / 2.0, y / 2.0, z / 2.0);
    let min = -half_size;
    let max = half_size;

    
    // Suppose Z-up right hand
    let vertices = &[
        // top
        ([min.x, max.y, max.z], [ 0.0, 0.0, 1.0], uv[CubeSide::Top].tl),
        ([min.x, min.y, max.z], [ 0.0, 0.0, 1.0], uv[CubeSide::Top].bl),
        ([max.x, min.y, max.z], [ 0.0, 0.0, 1.0], uv[CubeSide::Top].br),
        ([max.x, max.y, max.z], [ 0.0, 0.0, 1.0], uv[CubeSide::Top].tr),
        // bottom		  
	([max.x, max.y, min.z], [ 0.0, 0.0,-1.0], uv[CubeSide::Bottom].tl),
        ([max.x, min.y, min.z], [ 0.0, 0.0,-1.0], uv[CubeSide::Bottom].bl),
        ([min.x, min.y, min.z], [ 0.0, 0.0,-1.0], uv[CubeSide::Bottom].br),
        ([min.x, max.y, min.z], [ 0.0, 0.0,-1.0], uv[CubeSide::Bottom].tr),
        // Front		  
        ([min.x, min.y, max.z], [ 0.0,-1.0, 0.0], uv[CubeSide::Front].tl),
        ([min.x, min.y, min.z], [ 0.0,-1.0, 0.0], uv[CubeSide::Front].bl),
        ([max.x, min.y, min.z], [ 0.0,-1.0, 0.0], uv[CubeSide::Front].br),
        ([max.x, min.y, max.z], [ 0.0,-1.0, 0.0], uv[CubeSide::Front].tr),
        // Back			  
        ([max.x, max.y, max.z], [ 0.0, 1.0, 0.0], uv[CubeSide::Back].tl),
        ([max.x, max.y, min.z], [ 0.0, 1.0, 0.0], uv[CubeSide::Back].bl),
        ([min.x, max.y, min.z], [ 0.0, 1.0, 0.0], uv[CubeSide::Back].br),
        ([min.x, max.y, max.z], [ 0.0, 1.0, 0.0], uv[CubeSide::Back].tr),
        // left			  
        ([min.x, max.y, max.z], [-1.0, 0.0, 0.0], uv[CubeSide::Left].tl),
        ([min.x, max.y, min.z], [-1.0, 0.0, 0.0], uv[CubeSide::Left].bl),
        ([min.x, min.y, min.z], [-1.0, 0.0, 0.0], uv[CubeSide::Left].br),
        ([min.x, min.y, max.z], [-1.0, 0.0, 0.0], uv[CubeSide::Left].tr),
	// right		  
        ([max.x, min.y, max.z], [ 1.0, 0.0, 0.0], uv[CubeSide::Right].tl),
        ([max.x, min.y, min.z], [ 1.0, 0.0, 0.0], uv[CubeSide::Right].bl),
        ([max.x, max.y, min.z], [ 1.0, 0.0, 0.0], uv[CubeSide::Right].br),
        ([max.x, max.y, max.z], [ 1.0, 0.0, 0.0], uv[CubeSide::Right].tr),
	
    ];

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    #[rustfmt::skip]
    let indices = Indices::U32(vec![
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // front
        12, 13, 14, 14, 15, 12, // back
        16, 17, 18, 18, 19, 16, // left
        20, 21, 22, 22, 23, 20, // right
    ]);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(indices)
}
