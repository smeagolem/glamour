pub struct GBuf {
    id: u32,
    pos_id: u32,
    norm_id: u32,
    alb_spec_id: u32,
    attachments: [u32; 3],
    depth_id: u32,
}

impl GBuf {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::GenFramebuffers(1, &mut id));
        gl_call!(gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, id));

        // FIXME: these need to update when resizing.
        let width: u32 = 512;
        let height: u32 = 490;

        // position color buffer
        let mut pos_id: u32 = 0;
        gl_call!(gl::GenTextures(1, &mut pos_id));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, pos_id));
        GBuf::specify_texture(gl::RGBA32F, gl::FLOAT, width, height);
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST as i32
        ));
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as i32
        ));
        gl_call!(gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            pos_id,
            0
        ));

        // normal color buffer
        let mut norm_id: u32 = 0;
        gl_call!(gl::GenTextures(1, &mut norm_id));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, norm_id));
        GBuf::specify_texture(gl::RGBA32F, gl::FLOAT, width, height);
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST as i32
        ));
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as i32
        ));
        gl_call!(gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT1,
            gl::TEXTURE_2D,
            norm_id,
            0
        ));

        // albedo + specular color buffer
        let mut alb_spec_id: u32 = 0;
        gl_call!(gl::GenTextures(1, &mut alb_spec_id));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, alb_spec_id));
        GBuf::specify_texture(gl::RGBA, gl::UNSIGNED_BYTE, width, height);
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST as i32
        ));
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as i32
        ));
        gl_call!(gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT2,
            gl::TEXTURE_2D,
            alb_spec_id,
            0
        ));

        // tell OpenGL which color attachments we'll use (of this framebuffer) for rendering
        let attachments: [u32; 3] = [
            gl::COLOR_ATTACHMENT0,
            gl::COLOR_ATTACHMENT1,
            gl::COLOR_ATTACHMENT2,
        ];
        gl_call!(gl::DrawBuffers(3, attachments.as_ptr()));

        // create and attach depth buffer (renderbuffer)
        let mut depth_id: u32 = 0;
        gl_call!(gl::GenRenderbuffers(1, &mut depth_id));
        gl_call!(gl::BindRenderbuffer(gl::RENDERBUFFER, depth_id));
        gl_call!(gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH_COMPONENT,
            width as i32,
            height as i32
        ));
        gl_call!(gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::RENDERBUFFER,
            depth_id
        ));
        gl_call!(gl::BindRenderbuffer(gl::RENDERBUFFER, 0));

        // finally check if framebuffer is complete
        if gl_call!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE) {
            println!("Framebuffer not complete!");
        }
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, 0));

        GBuf {
            id,
            pos_id,
            norm_id,
            alb_spec_id,
            attachments,
            depth_id,
        }
    }

    pub fn bind(&self) {
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, self.id));
    }

    pub fn unbind(&self) {
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, 0));
    }

    fn specify_texture(
        internal_format: gl::types::GLenum,
        data_type: gl::types::GLenum,
        width: u32,
        height: u32,
    ) {
        gl_call!(gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            internal_format as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            data_type,
            std::ptr::null()
        ));
    }

    pub fn resize(&self, width: u32, height: u32) {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.pos_id));
        GBuf::specify_texture(gl::RGBA32F, gl::FLOAT, width, height);
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));

        gl_call!(gl::ActiveTexture(gl::TEXTURE1));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.norm_id));
        GBuf::specify_texture(gl::RGBA32F, gl::FLOAT, width, height);
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));

        gl_call!(gl::ActiveTexture(gl::TEXTURE2));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.alb_spec_id));
        GBuf::specify_texture(gl::RGBA, gl::UNSIGNED_BYTE, width, height);
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));

        gl_call!(gl::ActiveTexture(gl::TEXTURE0));

        gl_call!(gl::BindRenderbuffer(gl::RENDERBUFFER, self.depth_id));
        gl_call!(gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH_COMPONENT,
            width as i32,
            height as i32
        ));
        gl_call!(gl::BindRenderbuffer(gl::RENDERBUFFER, 0));
    }

    pub fn bind_bufs(&self) {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.pos_id));
        gl_call!(gl::ActiveTexture(gl::TEXTURE1));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.norm_id));
        gl_call!(gl::ActiveTexture(gl::TEXTURE2));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.alb_spec_id));

        gl_call!(gl::ActiveTexture(gl::TEXTURE0));
    }

    pub fn unbind_bufs(&self) {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
        gl_call!(gl::ActiveTexture(gl::TEXTURE1));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
        gl_call!(gl::ActiveTexture(gl::TEXTURE2));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));

        gl_call!(gl::ActiveTexture(gl::TEXTURE0));
    }
}
