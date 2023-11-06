use crate::ShaderProgram;
use patutil::{
    csg::{Brick, BrickRef, SceneRef, BrickIdList, CSGScene},
    Size, Vecf4,
};

const BRICK_BYTE_SIZE: i32 = 34;

pub struct BrickTex {
    tex_size: Size,
    tex_id: u32,
}

impl BrickTex {
    pub fn new(total_bricks: i32) -> Result<BrickTex, String> {
        let mut tex_id: u32 = 0;
        let width = ((total_bricks as f32).sqrt()).floor() as i32;
        let height = width;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::R8UI as i32,
                width * BRICK_BYTE_SIZE,
                height,
                0,
                gl::RED_INTEGER,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(BrickTex {
            tex_size: [width as u32, height as u32].into(),
            tex_id,
        })
    }

    pub fn bind(&self, shader_program: &ShaderProgram, tex_slot: i32, scene: SceneRef) {
        shader_program.bind_uniform("uBrickTex", &tex_slot);
        shader_program.bind_uniform("uBrickTexWidth", &self.tex_size.h);

        // unsafe {
        //     gl::ActiveTexture(gl::TEXTURE0 + tex_slot as u32);
        //     gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
        // }

        // let brick1 = crate::csg::data::CSGBrick{
        //     bricktype: crate::csg::data::CSGType::Sphere,
        //     op: crate::csg::data::CSGOp::Union,
        //     next_brick: 1,
        //     pos: Vecf4::new(0.,0.,5.,0.),
        //     size: Vecf4::new(1.,1.,1.,0.),
        //     color: Vecf4::new(1.,0.,0.,1.),
        // };
        // let brick2 = crate::csg::data::CSGBrick{
        //     bricktype: crate::csg::data::CSGType::Sphere,
        //     op: crate::csg::data::CSGOp::Union,
        //     next_brick: 2,
        //     pos: Vecf4::new(3.,0.,5.,0.),
        //     size: Vecf4::new(1.,1.,1.,0.),
        //     color: Vecf4::new(1.,0.,0.,1.),
        // };
        // let brick3 = crate::csg::data::CSGBrick{
        //     bricktype: crate::csg::data::CSGType::Sphere,
        //     op: crate::csg::data::CSGOp::Union,
        //     next_brick: -1,
        //     pos: Vecf4::new(-3.,0.,5.,0.),
        //     size: Vecf4::new(1.,1.,1.,0.),
        //     color: Vecf4::new(1.,0.,0.,1.),
        // };
        // self.bind_brick(0,brick1,shader_program);
        // self.bind_brick(1,brick2,shader_program);
        // self.bind_brick(2,brick3,shader_program);

        let mut brick_added = false;
        let scene: &CSGScene = &scene.lock();
        let id_list = BrickIdList::from(scene);
        for brick in id_list.get().into_iter() {
            self.bind_brick(brick, shader_program);
            brick_added = true;
        }

        // place empty brick if no bricks
        if !brick_added {
            let empty_brick: BrickRef = Brick::default().into();
            empty_brick
                .controller()
                .set_size(Vecf4::new(0., 0., 0., 1.));
            self.bind_brick(empty_brick, shader_program);
        }

        shader_program.bind_uniform("uTotalBricks", &2i32);
    }

    fn bind_brick(&self, brickref: BrickRef, shader_program: &ShaderProgram) {
        shader_program.set_used();
        let brick = brickref.get();
        let prefix = "uBricks[".to_string() + brick.id().unwrap().to_string().as_str() + "]";
        shader_program.bind_uniform(&(prefix.clone() + ".type"), &(brick.brick_type() as u32));
        shader_program.bind_uniform(&(prefix.clone() + ".op"), &(brick.brick_op() as u32));
        shader_program.bind_uniform(&(prefix.clone() + ".rot"), &brick.rot().to_vec());
        shader_program.bind_uniform(&(prefix.clone() + ".pos"), &brick.pos());
        shader_program.bind_uniform(&(prefix.clone() + ".size"), &brick.size());
        shader_program.bind_uniform(&(prefix.clone() + ".color"), brick.color());
        shader_program.bind_uniform(
            &(prefix.clone() + ".nextP"),
            &brick
                .next_brick()
                .map_or(-1, |v| v.get().id().unwrap() as i32),
        );
        shader_program.bind_uniform(
            &(prefix + ".child_p"),
            &brick.child().map_or(-1, |v| v.get().id().unwrap() as i32),
        );
    }
    // pub fn store_brick(&self,brickdata: CSGBrick, pos: i32){

    //     let x = BRICK_BYTE_SIZE * (pos % self.tex_size[0]);
    //     let y = (pos as f32/self.tex_size[0] as f32).floor() as i32;

    //     let raw_brick = &brickdata as *const CSGBrick;

    //     unsafe{
    //         gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
    //         gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    //         gl::TexSubImage2D(
    //             gl::TEXTURE_2D,
    //             0,
    //             x,
    //             y,
    //             BRICK_BYTE_SIZE,
    //             1,
    //             gl::RED_INTEGER,
    //             gl::UNSIGNED_BYTE,
    //             raw_brick as *const c_void
    //         );
    //         gl::BindTexture(gl::TEXTURE_2D, 0);
    //     }
    // }

    // fn updateTex(data: Uint8Array){
    //     const gl = this._gl;

    //     gl::bindTexture(gl::TEXTURE_2D, this._tex);
    //     gl::pixelStorei(gl::UNPACK_ALIGNMENT, 1);
    //     gl::texSubImage2D(gl::TEXTURE_2D, 0,0,0,this._width*BRICK_BYTE_SIZE, this._height, gl::RED_INTEGER, gl::UNSIGNED_BYTE, data);
    //     gl::bindTexture(gl::TEXTURE_2D, null);
    // }
}

impl Drop for BrickTex {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.tex_id);
        }
    }
}
