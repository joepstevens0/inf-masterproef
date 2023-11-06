use glrender::GLMesh;
use patutil::mesh::MeshRef;



pub fn create_cone_mesh(base_width: f32, tip_width: f32) -> MeshRef {
    let mut vertices = vec![];

    const TOTAL_HEIGHT: f32 = 1.;
    const TOTAL_CORNERS: u32 = 10;
    const TOTAL_CIRCLES: u32 = 10;
    const CIRCLE_HEIGHT:f32 = TOTAL_HEIGHT/TOTAL_CIRCLES as f32;

    for edge in 0..TOTAL_CORNERS {
        let k1: f32 = (edge as f32 / TOTAL_CORNERS as f32) * 2. * std::f32::consts::PI;
        let x_l_b = base_width * k1.cos();
        let z_l_b = base_width * k1.sin();
        let y_b = 0.;

        let k2: f32 =
            (((edge + 1) % TOTAL_CORNERS) as f32 / TOTAL_CORNERS as f32) * 2. * std::f32::consts::PI;
        let x_r_b = base_width * k2.cos();
        let z_r_b = base_width * k2.sin();

        // push pos
        vertices.append(&mut vec![x_l_b, y_b, z_l_b]); // bottom left
        // push normal
        vertices.append(&mut vec![x_l_b, y_b, z_l_b]);
        // push texcoord
        vertices.append(&mut vec![0., 0.]);

        vertices.append(&mut vec![0., 0., 0.]); // top left
                // push normal
        vertices.append(&mut vec![0., -1., 0.]);
        // push texcoord
        vertices.append(&mut vec![0., 1.]);

        vertices.append(&mut vec![0., 0., 0.]); // top  right
                // push normal
        vertices.append(&mut vec![0., -1., 0.]);
        // push texcoord
        vertices.append(&mut vec![1., 1.]);

        vertices.append(&mut vec![x_l_b, y_b, z_l_b]); // bottom left
                // push normal
        vertices.append(&mut vec![x_l_b, y_b, z_l_b]);
        // push texcoord
        vertices.append(&mut vec![0., 0.]);

        vertices.append(&mut vec![0., 0., 0.]); // top right
                // push normal
        vertices.append(&mut vec![0., -1., 0.]);
        // push texcoord
        vertices.append(&mut vec![1., 1.]);

        vertices.append(&mut vec![x_r_b, y_b, z_r_b]); // bottom right
                // push normal
        vertices.append(&mut vec![x_r_b, y_b, z_r_b]);
        // push texcoord
        vertices.append(&mut vec![1., 0.]);
    }

    for ty in 0..TOTAL_CIRCLES {
        let y_pos: f32 = ty as f32*CIRCLE_HEIGHT;
        for edge in 0..TOTAL_CORNERS {
            let width_bottom = base_width - (ty as f32/TOTAL_CIRCLES as f32).powf(1.)*(base_width - tip_width);
            let width_top = base_width - ((ty as f32 + 1.)/TOTAL_CIRCLES as f32).powf(1.)*(base_width - tip_width);

            let k1: f32 = (edge as f32 / TOTAL_CORNERS as f32) * 2. * std::f32::consts::PI;
            let x_l_b = width_bottom * k1.cos();
            let z_l_b = width_bottom * k1.sin();
            let x_l_t = width_top * k1.cos();
            let z_l_t = width_top * k1.sin();
            let y_b = y_pos;

            let k2: f32 =
                (((edge + 1) % TOTAL_CORNERS) as f32 / TOTAL_CORNERS as f32) * 2. * std::f32::consts::PI;
            let x_r_b = width_bottom * k2.cos();
            let z_r_b = width_bottom * k2.sin();
            let x_r_t = width_top * k2.cos();
            let z_r_t = width_top * k2.sin();
            let y_t = y_pos + CIRCLE_HEIGHT;

            // push pos
            vertices.append(&mut vec![x_l_b, y_b, z_l_b]); // bottom left
                                                           // push normal
            vertices.append(&mut vec![x_l_b, y_b, z_l_b]);
            // push texcoord
            vertices.append(&mut vec![0., 0.]);

            vertices.append(&mut vec![x_l_t, y_t, z_l_t]); // top left
                                                           // push normal
            vertices.append(&mut vec![x_l_t, y_t, z_l_t]);
            // push texcoord
            vertices.append(&mut vec![0., 1.]);

            vertices.append(&mut vec![x_r_t, y_t, z_r_t]); // top  right
                                                           // push normal
            vertices.append(&mut vec![x_r_t, y_t, z_r_t]);
            // push texcoord
            vertices.append(&mut vec![1., 1.]);

            vertices.append(&mut vec![x_l_b, y_b, z_l_b]); // bottom left
                                                           // push normal
            vertices.append(&mut vec![x_l_b, y_b, z_l_b]);
            // push texcoord
            vertices.append(&mut vec![0., 0.]);

            vertices.append(&mut vec![x_r_t, y_t, z_r_t]); // top right
                                                           // push normal
            vertices.append(&mut vec![x_r_t, y_t, z_r_t]);
            // push texcoord
            vertices.append(&mut vec![1., 1.]);

            vertices.append(&mut vec![x_r_b, y_b, z_r_b]); // bottom right
                                                           // push normal
            vertices.append(&mut vec![x_r_b, y_b, z_r_b]);
            // push texcoord
            vertices.append(&mut vec![1., 0.]);
        }
    }

    for edge in 0..TOTAL_CORNERS {
        let k1: f32 = (edge as f32 / TOTAL_CORNERS as f32) * 2. * std::f32::consts::PI;
        let x_l_b = tip_width * k1.cos();
        let z_l_b = tip_width * k1.sin();
        let y_b = TOTAL_CIRCLES as f32*CIRCLE_HEIGHT;

        let k2: f32 =
            (((edge + 1) % TOTAL_CORNERS) as f32 / TOTAL_CORNERS as f32) * 2. * std::f32::consts::PI;
        let x_r_b = tip_width * k2.cos();
        let z_r_b = tip_width * k2.sin();

        let y_t = TOTAL_CIRCLES as f32*CIRCLE_HEIGHT;
        // push pos
        vertices.append(&mut vec![x_l_b, y_b, z_l_b]); // bottom left
        // push normal
        vertices.append(&mut vec![x_l_b, y_b, z_l_b]);
        // push texcoord
        vertices.append(&mut vec![0., 0.]);

        vertices.append(&mut vec![0., y_t, 0.]); // top left
                // push normal
        vertices.append(&mut vec![0., 1., 0.]);
        // push texcoord
        vertices.append(&mut vec![0., 1.]);

        vertices.append(&mut vec![0., y_t, 0.]); // top  right
        // push normal
        vertices.append(&mut vec![0., 1., 0.]);
        // push texcoord
        vertices.append(&mut vec![1., 1.]);

        vertices.append(&mut vec![x_l_b, y_b, z_l_b]); // bottom left
        // push normal
        vertices.append(&mut vec![x_l_b, y_b, z_l_b]);
        // push texcoord
        vertices.append(&mut vec![0., 0.]);

        vertices.append(&mut vec![0., y_t, 0.]); // top right
        // push normal
        vertices.append(&mut vec![0., 1., 0.]);
        // push texcoord
        vertices.append(&mut vec![1., 1.]);

        vertices.append(&mut vec![x_r_b, y_b, z_r_b]); // bottom right
                // push normal
        vertices.append(&mut vec![x_r_b, y_b, z_r_b]);
        // push texcoord
        vertices.append(&mut vec![1., 0.]);
    }

    let mesh = GLMesh::from_vertices(vertices);
    return mesh.into();
}

pub fn create_plane_mesh() -> MeshRef {
    GLMesh::new_plane().into()
}