use bevy::{asset::RenderAssetUsages, math::vec3, mesh::{Indices, Mesh, PrimitiveTopology}};


pub fn card(x: f32, y: f32, z: f32) -> Mesh {
    // uv map 0-0.25 front face, 0.25-.5 back face, .5-.75 edges 
    let half_size = vec3(x / 2.0, y / 2.0, z / 2.0);
    let min = -half_size;
    let max = half_size;

    // Suppose Z-up right hand
    let vertices = &[
        // Front
        ([min.x, max.y, max.z], [0.0, 0.0, 1.0], [0.0, 0.0]),
        ([max.x, max.y, max.z], [0.0, 0.0, 1.0], [1.0, 0.0]),
        ([max.x, min.y, max.z], [0.0, 0.0, 1.0], [1.0, 0.25]),
        ([min.x, min.y, max.z], [0.0, 0.0, 1.0], [0.0, 0.25]),
        // back
	([min.x, max.y, min.z], [0.0, 0.0, -1.0], [0.0, 0.25]),
        ([max.x, max.y, min.z], [0.0, 0.0, -1.0], [1.0, 0.25]),
        ([max.x, min.y, min.z], [0.0, 0.0, -1.0], [1.0, 0.5]),
        ([min.x, min.y, min.z], [0.0, 0.0, -1.0], [0.0, 0.5]),
        // Top
        ([min.x, max.y, min.z, ], [0.0, 1.0, 0.0], [0.0, 0.50]),
        ([max.x, max.y, min.z, ], [0.0, 1.0, 0.0], [1.0, 0.50]),
        ([max.x, max.y, max.z, ], [0.0, 1.0, 0.0], [1.0, 0.75]),
        ([min.x, max.y, max.z, ], [0.0, 1.0, 0.0], [0.0, 0.75]),
        // Bottom
        ([max.x, min.y, min.z, ], [0.0, -1.0, 0.0], [0.0, 0.50]),
        ([min.x, min.y, min.z, ], [0.0, -1.0, 0.0], [1.0, 0.50]),
        ([min.x, min.y, max.z, ], [0.0, -1.0, 0.0], [1.0, 0.75]),
        ([max.x, min.y, max.z, ], [0.0, -1.0, 0.0], [0.0, 0.75]),
        // left
        ([min.x, min.y, min.z, ], [-1.0, 0.0, 0.0], [0.0, 0.50]),
        ([min.x, max.y, min.z, ], [-1.0, 0.0, 0.0], [1.0, 0.50]),
        ([min.x, max.y, max.z, ], [-1.0, 0.0, 0.0], [1.0, 0.75]),
        ([min.x, min.y, max.z, ], [-1.0, 0.0, 0.0], [0.0, 0.75]),
	// right
        ([max.x, max.y, min.z, ], [1.0, 0.0, 0.0], [0.0, 0.50]),
        ([max.x, min.y, min.z, ], [1.0, 0.0, 0.0], [1.0, 0.50]),
        ([max.x, min.y, max.z, ], [1.0, 0.0, 0.0], [1.0, 0.75]),
        ([max.x, max.y, max.z, ], [1.0, 0.0, 0.0], [0.0, 0.75]),
	
    ];

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    #[rustfmt::skip]
    let indices = Indices::U32(vec![
        2, 1, 0, 0, 3, 2, // front
        4, 5, 6, 6, 7, 4, // back
        10, 9, 8, 8, 11, 10, // top
        14, 13, 12, 12, 15, 14, // bottom
        18, 17, 16, 16, 19, 18, // left
        22, 21, 20, 20, 23, 22, // right
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
