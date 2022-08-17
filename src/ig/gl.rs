use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::console::log_1;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::f64::consts::PI;

use super::math::{Vec3, Matrix4};
use super::geo::{Point, Curve, Surface};
use super::Color;

pub const SEGMENT_RESOLUTION : u32 = 20; //10;
pub const TESSELLATION_RESOLUTION : u32 = 8; //10; //5; //10; //5;

pub const INSERT_POINT_ON_DEGREE1_TWISTED_SURFACE : bool = true;



#[allow(dead_code)]
pub struct GlUniformMatrix4{
    mat: Matrix4,
    name: String,
    location: Option<WebGlUniformLocation>,
}
impl GlUniformMatrix4{
    #[allow(dead_code)]
    pub fn new(m:Matrix4, s:&str)->Self{
        GlUniformMatrix4{ mat:m, name:String::from(s), location:None }
    }
    #[allow(dead_code)]
    pub fn set_matrix(&mut self, mat:&Matrix4){
        self.mat.set_with_matrix4(mat);
    }
    #[allow(dead_code)]
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
    #[allow(dead_code)]
    pub fn set_uniform(&mut self, gl:&GL){
        gl.uniform_matrix4fv_with_f32_array(Some(&self.location.as_ref().unwrap()), false, &self.mat.to_array32());
    }
}
pub struct GlUniformVec3{
    vec: Vec3,
    name: String,
    location: Option<WebGlUniformLocation>,
}
impl GlUniformVec3{
    #[allow(dead_code)]
    pub fn new(v:Vec3, s:&str)->Self{
        GlUniformVec3{ vec:v, name:String::from(s), location:None }
    }
    #[allow(dead_code)]
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
    #[allow(dead_code)]
    pub fn set_uniform(&mut self, gl:&GL){
        gl.uniform3fv_with_f32_array(Some(&self.location.as_ref().unwrap()), &self.vec.to_array32());
    }
}
pub struct GlUniformColor{
    color: Color,
    name: String,
    location: Option<WebGlUniformLocation>,
}
impl GlUniformColor{
    #[allow(dead_code)]
    pub fn new(c:Color, s:&str)->Self{
        GlUniformColor{ color:c, name:String::from(s), location:None }
    }
    #[allow(dead_code)]
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        //log_1(&JsValue::from(format!("GlUniformColor::set_location name: {}", self.name )));

        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
    #[allow(dead_code)]
    pub fn set_uniform(&mut self, gl:&GL){
        //log_1(&JsValue::from(format!("GlUniformColor::set_uniform color: {}", self.color )));

        gl.uniform4fv_with_f32_array(Some(&self.location.as_ref().unwrap()), &self.color.to_array());
    }
    #[allow(dead_code)]
    pub fn set_color(&mut self, color:&Color){
        self.color = *color;
    }
}
pub struct GlUniformInt{
    i: i32,
    name: String,
    location: Option<WebGlUniformLocation>,
}
impl GlUniformInt{
    #[allow(dead_code)]
    pub fn new(i:i32, s:&str)->Self{
        GlUniformInt{ i, name:String::from(s), location:None }
    }
    #[allow(dead_code)]
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
    #[allow(dead_code)]
    pub fn set_uniform(&mut self, gl:&GL){
        gl.uniform1i(Some(&self.location.as_ref().unwrap()), self.i);
    }
}
pub struct GlUniformFloat{
    x: f32,
    name: String,
    location: Option<WebGlUniformLocation>,
}
impl GlUniformFloat{
    #[allow(dead_code)]
    pub fn new(x:f32, s:&str)->Self{
        GlUniformFloat{ x, name:String::from(s), location:None }
    }
    #[allow(dead_code)]
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
    #[allow(dead_code)]
    pub fn set_uniform(&mut self, gl:&GL){
        gl.uniform1f(Some(&self.location.as_ref().unwrap()), self.x);
    }
}

pub struct GlAttributeVec2{
    name: String,
    location: u32,
    stride: i32,
    buffer:Option<WebGlBuffer>
}
impl GlAttributeVec2{
    #[allow(dead_code)]
    pub fn new(s:&str)->Self{
        GlAttributeVec2{ name:String::from(s), location:0, stride:2 , buffer:None }
    }
    #[allow(dead_code)]
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        let index = gl.get_attrib_location(&program, &self.name);
        if index<0{
            log_1(&JsValue::from(format!("GlAttributeVec2::set_location:ERROR parameter \"{}\" is not found", self.name)));
        }
        self.location = index as u32;
    }
    //pub fn set_buffer(&mut self, gl:&GL, buffer:&WebGlBuffer){
    #[allow(dead_code)]
    pub fn set_buffer(&mut self, gl:&GL,  data: &Vec<f32>) {
        self.create_vbo_vector(gl, data);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    #[allow(dead_code)]
    pub fn activate(&mut self, gl:&GL){
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }

    #[allow(dead_code)]
    pub fn unset_buffer(&mut self, gl:&GL){
        //gl.bind_buffer(GL::ARRAY_BUFFER, None);
        //gl.disable_vertex_attrib_array(self.location);
        gl.delete_buffer(Some(&self.buffer.as_ref().unwrap()));
    }
    //pub fn create_vbo_vector(&mut self gl: &GL, data: &Vec<f32>) -> Result<WebGlBuffer, String> {
    #[allow(dead_code)]
    pub fn create_vbo_vector(&mut self, gl: &GL, data: &Vec<f32>) {
        self.buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        unsafe {
            let f32_array = js_sys::Float32Array::view(&(*data));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::DYNAMIC_DRAW)
        }
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
}

pub struct GlAttributeVec3{
    name: String,
    location: u32,
    stride: i32,
    buffer:Option<WebGlBuffer>
}
impl GlAttributeVec3{
    #[allow(dead_code)]
    pub fn new(s:&str)->Self{
        GlAttributeVec3{ name:String::from(s), location:0, stride:3, buffer:None }
    }
    #[allow(dead_code)]
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        let index = gl.get_attrib_location(&program, &self.name);
        if index<0{
            log_1(&JsValue::from(format!("GlAttributeVec3::set_location:ERROR parameter \"{}\" is not found", self.name)));
        }
        self.location = index as u32;
    }
    //pub fn set_buffer(&mut self, gl:&GL, buffer:&WebGlBuffer){
    #[allow(dead_code)]
    pub fn set_buffer(&mut self, gl:&GL, data: &Vec<f32>){
        self.create_vbo_vector(gl, data);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    #[allow(dead_code)]
    pub fn activate(&mut self, gl:&GL){
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }

    #[allow(dead_code)]
    pub fn create_vbo_vector(&mut self, gl: &GL, data: &Vec<f32>) {
        self.buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        unsafe {
            let f32_array = js_sys::Float32Array::view(&(*data));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::DYNAMIC_DRAW)
        }
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    #[allow(dead_code)]
    pub fn unset_buffer(&mut self, gl:&GL){
        //gl.bind_buffer(GL::ARRAY_BUFFER, None);
        //gl.disable_vertex_attrib_array(self.location);
        gl.delete_buffer(Some(&self.buffer.as_ref().unwrap()));
    }

}

pub struct GlAttributeColor{
    name: String,
    location: u32,
    stride: i32,
    buffer:Option<WebGlBuffer>
}
impl GlAttributeColor{
    #[allow(dead_code)]
    pub fn new(s:&str)->Self{
        GlAttributeColor{ name:String::from(s), location:0, stride:4, buffer:None }
    }
    #[allow(dead_code)]
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        let index = gl.get_attrib_location(&program, &self.name);
        if index<0{
            log_1(&JsValue::from(format!("GlAttributeColor::set_location:ERROR parameter \"{}\" is not found", self.name)));
        }
        self.location = index as u32;
    }
    //pub fn set_buffer(&mut self, gl:&GL, buffer:&WebGlBuffer){
    #[allow(dead_code)]
    pub fn set_buffer(&mut self, gl:&GL, data: &Vec<f32>){
        self.create_vbo_vector(gl, data);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    #[allow(dead_code)]
    pub fn activate(&mut self, gl:&GL){
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    #[allow(dead_code)]
    pub fn create_vbo_vector(&mut self, gl: &GL, data: &Vec<f32>) {
        self.buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        unsafe {
            let f32_array = js_sys::Float32Array::view(&(*data));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::DYNAMIC_DRAW)
        }
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    #[allow(dead_code)]
    pub fn unset_buffer(&mut self, gl:&GL){
        //gl.bind_buffer(GL::ARRAY_BUFFER, None);
        //gl.disable_vertex_attrib_array(self.location);
        gl.delete_buffer(Some(&self.buffer.as_ref().unwrap()));
    }

}

pub struct GlIndex{
    size: i32,
    buffer:Option<WebGlBuffer>
}
impl GlIndex{
    #[allow(dead_code)]
    pub fn new()->Self{
        GlIndex{ size:0, buffer:None }
    }
    #[allow(dead_code)]
    pub fn set_buffer(&mut self, gl:&GL, data: &Vec<u16>){
        self.size = data.len() as i32;
        self.create_ibo_vector(gl, data);
    }
    #[allow(dead_code)]
    pub fn activate(&mut self, gl:&GL){
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
    }
    #[allow(dead_code)]
    pub fn create_ibo_vector(&mut self, gl: &GL, data: &Vec<u16>){
        self.buffer = gl.create_buffer();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        unsafe {
            let ui16_array = js_sys::Uint16Array::view(&(*data));
            gl.buffer_data_with_array_buffer_view( GL::ELEMENT_ARRAY_BUFFER, &ui16_array, GL::DYNAMIC_DRAW );
        }
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
    }

    #[allow(dead_code)]
    pub fn unset_buffer(&mut self, gl:&GL){
        //gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
        gl.delete_buffer(Some(&self.buffer.as_ref().unwrap()));
    }

}


pub struct GlProgram{
    vertex_shader_file:String,
    fragment_shader_file:String,
    program: Option<WebGlProgram>,
}

impl GlProgram{
    #[allow(dead_code)]
    pub fn new(vertex_shader_filename: &str, fragment_shader_filename:&str)->Self{
        GlProgram{
            vertex_shader_file:String::from(vertex_shader_filename),
            fragment_shader_file:String::from(fragment_shader_filename),
            program:None,
        }
    }
    #[allow(dead_code)]
    pub fn init(&mut self, gl:&GL) {
        //self.program = Some(gl.create_program().ok_or_else(|| String::from("Error creating program"))?);
        self.program = gl.create_program();

        let vert_shader = GlProgram::compile(&gl, GL::VERTEX_SHADER, &self.vertex_shader_file).unwrap();
        let frag_shader = GlProgram::compile(&gl, GL::FRAGMENT_SHADER, &self.fragment_shader_file).unwrap();
        gl.attach_shader(&self.program.as_ref().unwrap(), &vert_shader);
        gl.attach_shader(&self.program.as_ref().unwrap(), &frag_shader);
        gl.link_program(&self.program.as_ref().unwrap());
    }

    #[allow(dead_code)]
    pub fn use_program(&self, gl:&GL){
        if gl.get_program_parameter(&self.program.as_ref().unwrap(), GL::LINK_STATUS).as_bool().unwrap_or(false) {
            gl.use_program(Some(&self.program.as_ref().unwrap()));
        }
        else {
            //Err(gl.get_program_info_log(&self.program.as_ref().unwrap()).unwrap_or_else(|| String::from("Unknown error creating program object")));
        }
    }

    #[allow(dead_code)]
    pub fn compile(gl:&GL, shader_type: u32, source: &str)->Result<WebGlShader, String>{
        let shader = gl.create_shader(shader_type).ok_or_else(|| String::from("Error creating shader"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);
        if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false){
            Ok(shader)
        }
        else {
            Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unable to get shader info log")))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlPoint{
    pos:Vec3,
    color:Color,
    index: i32,
    size: f32,
}
impl GlPoint{
    #[allow(dead_code)]
    pub fn new(pos:Vec3)->Self{
        GlPoint{pos, color:Color::new(0.5,0.5,0.5,1.0), size:5.0, index:0}
    }

    #[allow(dead_code)]
    pub fn  from_point(pt:&Point)->Self{
        GlPoint{pos:pt.pos, color:pt.attr.color, size:pt.attr.size, index:0}
    }

    #[allow(dead_code)]
    pub fn set_index(&mut self, start_index:i32){
        self.index = start_index;
    }

    #[allow(dead_code)]
    pub fn clr(&mut self, c:Color){
        self.color = c;
    }

}

#[derive(Debug, Clone)]
pub struct GlLine{
    pos:Vec<Vec3>,
    color:Color,
    index:i32,
}
impl GlLine{
    #[allow(dead_code)]
    pub fn new(pos:Vec<Vec3>)->Self{
        GlLine{pos, color:Color::new(0.5,0.5,0.5,1.0), index:0}
    }

    #[allow(dead_code)]
    pub fn  from_curve(curve:&Curve)->Self{
        let mut pts:Vec<Vec3> = Vec::new();
        if curve.deg()==1{
            let num = curve.num();
            for i in 0..num{
                pts.push(curve.cp(i).clone());
            }
        }
        else{
            let reso = SEGMENT_RESOLUTION;
            let epnum = curve.ep_num();
            let num = (epnum-1)*(reso as usize)+1;
            #[allow(unused_variables)]
            for i in 0..num{
                pts.push(Vec3::zero());
            }
            for i in 0..epnum{
                for j in 0..reso{
                    if i<epnum-1 || j==0{
                        let pt = curve.pt(curve.u(i, (j as f64)/(reso as f64)));
                        pts[i*(reso as usize)+j as usize].set(&pt);
                    }
                }
            }
        }
        //GlLine{pos:pts, color:curve.attr.color, index:0}
        GlLine{pos:pts, color:curve.attr.color, index:0}
    }


    #[allow(dead_code)]
    pub fn clr(&mut self, c:Color){
        self.color = c;
    }

    #[allow(dead_code)]
    pub fn set_index(&mut self, start_index:i32){
        self.index = start_index;
    }

    #[allow(dead_code)]
    pub fn len(&self)->i32{ self.pos.len() as i32 }
}

#[derive(Debug, Clone)]
pub struct GlSurface{
    pos:Vec<Vec<Vec3>>,
    nml:Vec<Vec<Vec3>>,
    color:Color,
    index: i32
}

impl GlSurface{
    #[allow(dead_code)]
    pub fn  from_surface(surface:&Surface)->Self{
        let isoparm_ratio = TESSELLATION_RESOLUTION as usize;

        let mut uval:Vec<f64> = Vec::new();
        let mut vval:Vec<f64> = Vec::new();

        if surface.udeg()==1{
            let num = surface.unum();
            for i in 0..num{ uval.push(surface.u(i,0.0)); }
        }
        else{
            let epnum = surface.uep_num();
            //let num = (epnum-1)*isoparm_ratio+1;
            for i in 0..epnum{
                for j in 0..isoparm_ratio{
                    if i<epnum-1 || j==0{ uval.push(surface.u(i, j as f64 / isoparm_ratio as f64)); }
                }
            }
        }

        if surface.vdeg()==1{
            let num = surface.vnum();
            for i in 0..num{ vval.push(surface.v(i,0.0)); }
        }
        else{
            let epnum = surface.vep_num();
            //let num = (epnum-1)*isoparm_ratio+1;
            for i in 0..epnum{
                for j in 0..isoparm_ratio{
                    if i<epnum-1 || j==0 { vval.push(surface.v(i, j as f64 / isoparm_ratio as f64)); }
                }
            }
        }

        if INSERT_POINT_ON_DEGREE1_TWISTED_SURFACE && surface.udeg()==1 && surface.vdeg()==1{
            let mut uinsert : Vec<bool> = Vec::new();
            let mut vinsert : Vec<bool> = Vec::new();
            let mut any_insert = false;
            #[allow(unused_variables)]
            for i in 0..uval.len()-1{ uinsert.push(false); }
            #[allow(unused_variables)]
            for i in 0..vval.len()-1{ vinsert.push(false); }

            for i in 0..uval.len()-1{
                for j in 0..vval.len()-1{
                    if Vec3::is_flat(&surface.pt(uval[i], vval[j]),
                    &surface.pt(uval[i+1], vval[j]),
                    &surface.pt(uval[i+1], vval[j+1]),
                    &surface.pt(uval[i], vval[j+1]))
                    {
                        uinsert[i] = true;
                        vinsert[i] = true;
                        any_insert = true;
                    }
                }
            }

            if any_insert{
                let mut uval2 : Vec<f64> = Vec::new();
                for i in 0..uval.len()-1{
                    uval2.push(uval[i]);
                    if uinsert[i]{
                        for j in 0..isoparm_ratio{
                            uval2.push( ( (uval[i+1]-uval[i])*j as f64)/isoparm_ratio as f64 + uval[i]);
                        }
                    }
                }
                uval2.push(uval[uval.len()-1]);

                let mut vval2 : Vec<f64> = Vec::new();
                for i in 0..vval.len()-1{
                    vval2.push(vval[i]);
                    if vinsert[i]{
                        for j in 0..isoparm_ratio{
                            vval2.push( ( (vval[i+1]-vval[i])*j as f64)/isoparm_ratio as f64 + vval[i]);
                        }
                    }
                }
                vval2.push(vval[vval.len()-1]);

                uval = Vec::new();
                for i in 0..uval2.len(){ uval.push(uval2[i]); }
                vval = Vec::new();
                for i in 0..vval2.len(){ vval.push(vval2[i]); }
            }
        }

        let mut pos:Vec<Vec<Vec3>> = Vec::new();
        let mut nml:Vec<Vec<Vec3>> = Vec::new();
        for i in 0..uval.len(){
            let mut vpos : Vec<Vec3> = Vec::new();
            let mut vnml : Vec<Vec3> = Vec::new();
            for j in 0..vval.len(){
                let p = surface.pt(uval[i], vval[j]);
                let mut n = surface.nml(uval[i], vval[j]);
                n.unit();
                vpos.push(p);
                vnml.push(n);

                //log_1(&JsValue::from(format!("GlSurface.from: pos[{}][{}] {} ", i, j, &p )));
                //log_1(&JsValue::from(format!("GlSurface.from: nml[{}][{}] {} ", i, j, &n )));

            }
            pos.push(vpos);
            nml.push(vnml);
        }

        GlSurface{pos, nml, color:surface.attr.color, index:0}
    }

    #[allow(dead_code)]
    pub fn new(pos:Vec<Vec<Vec3>>, nml:Vec<Vec<Vec3>>)->Self{
        GlSurface{pos, nml, color:Color::new(0.5,0.5,0.5,1.0), index:0}
    }

    #[allow(dead_code)]
    pub fn set_index(&mut self, start_index:i32){
        self.index = start_index;
    }

    #[allow(dead_code)]
    pub fn clr(&mut self, c:Color){
        self.color = c;
    }

    #[allow(dead_code)]
    pub fn ulen(&self)->i32{ self.pos.len() as i32 }
    #[allow(dead_code)]
    pub fn vlen(&self)->i32{ self.pos[0].len() as i32 }
}

pub struct GlLineBuffer{
    pos: Vec<f32>,
    //buffer: GlAttributeVec3,
}
impl GlLineBuffer{
    #[allow(dead_code)]
    pub fn new()->Self{
        GlLineBuffer{pos:Vec::new()}
    }
    #[allow(dead_code)]
    pub fn add(&mut self, line:&mut GlLine){
        let idx = (self.pos.len() as i32)/3; // divided by 3 or not?
        line.set_index(idx);
        for x in &line.pos{
            self.pos.push(x.x as f32);
            self.pos.push(x.y as f32);
            self.pos.push(x.z as f32);
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.pos.clear();
    }
}

pub struct GlPointBuffer{
    pos: Vec<f32>,
}
impl GlPointBuffer{
    #[allow(dead_code)]
    pub fn new()->Self{
        GlPointBuffer{pos:Vec::new()}
    }
    #[allow(dead_code)]
    pub fn add(&mut self, pt:&mut GlPoint){
        let idx = (self.pos.len() as i32)/3;
        pt.set_index(idx);
        self.pos.push(pt.pos.x as f32);
        self.pos.push(pt.pos.y as f32);
        self.pos.push(pt.pos.z as f32);
//        log_1(&JsValue::from(format!("GlPointBuffer.add: pt {}, {}, {}", pt.pos.x, pt.pos.y, pt.pos.z )));
//        log_1(&JsValue::from(format!("GlPointBuffer.add: pt.index {}", pt.index )));
    }
    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.pos.clear();
//        log_1(&JsValue::from(format!("GlPointBuffer.clear: pos.len() = {}", self.pos.len() )));
    }
}

pub struct GlFaceBuffer{
    pos: Vec<f32>,
    nml: Vec<f32>,
}
impl GlFaceBuffer{
    #[allow(dead_code)]
    pub fn new()->Self{
        GlFaceBuffer{pos:Vec::new(), nml:Vec::new()}
    }
    #[allow(dead_code)]
    pub fn add(&mut self, surf:&mut GlSurface){
        let idx = (self.pos.len() as i32)/3; // devided by 3 or not?
        surf.set_index(idx);
        for j in 0..surf.vlen()-1{
            for i in 0..surf.ulen(){
                let p1 = &surf.pos[i as usize][j as usize];
                self.pos.push(p1.x as f32);
                self.pos.push(p1.y as f32);
                self.pos.push(p1.z as f32);
                let p2 = &surf.pos[i as usize][(j+1) as usize];
                self.pos.push(p2.x as f32);
                self.pos.push(p2.y as f32);
                self.pos.push(p2.z as f32);

                let n1 = &surf.nml[i as usize][j as usize];
                self.nml.push(n1.x as f32);
                self.nml.push(n1.y as f32);
                self.nml.push(n1.z as f32);
                let n2 = &surf.nml[i as usize][(j+1) as usize];
                self.nml.push(n2.x as f32);
                self.nml.push(n2.y as f32);
                self.nml.push(n2.z as f32);

                //log_1(&JsValue::from(format!("glFaceBuffer nml[{}][{}] = {}", i, j, n1 ))); //

            }
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.pos.clear();
        self.nml.clear();
    }
}


pub struct WebGlServer{
    width:f32,
    height:f32,
    gl: GL,
    m_matrix :Matrix4,
    v_matrix :Matrix4,
    p_matrix : Matrix4,
    mvp_matrix : GlUniformMatrix4,
    tmp_matrix : Matrix4,
    inv_matrix : GlUniformMatrix4,
    camera_pitch: f64,
    camera_yaw: f64,
    camera_rotation:bool,
    camera_rotation_speed:f64,
    zoom_ratio: f64,
    bg_colors: [Color;4],
    eye_direction : GlUniformVec3,
    light_direction : GlUniformVec3,
    ambient_color : GlUniformColor,
    flat_shading: GlUniformInt,
    point_mvp_matrix : GlUniformMatrix4,
    point_size: GlUniformFloat,
    point_color: GlUniformColor,
    line_mvp_matrix : GlUniformMatrix4,
    line_color: GlUniformColor,
    surf_color: GlUniformColor,
    bg_position_attr: GlAttributeVec2,
    bg_color_attr: GlAttributeColor,
    //bg_index: GlIndex,

    shader_program: GlProgram,
    line_shader_program: GlProgram,
    point_shader_program: GlProgram,
    bg_shader_program: GlProgram,
    //server:Option<&'a Server<'a>>,

    time: i64,

    points: Vec<Box<GlPoint>>,
    point_buffer: GlPointBuffer,
    lines: Vec<Box<GlLine>>,
    line_buffer: GlLineBuffer,
    surfs: Vec<Box<GlSurface>>,
    surf_buffer: GlFaceBuffer,
}

impl WebGlServer{
    pub fn new(width:f32, height:f32)->Self{
        WebGlServer{
            width, height,
            gl:WebGlServer::get_webgl_context(height as u32, width as u32).unwrap(),
            m_matrix: Matrix4::zero(),
            v_matrix: Matrix4::zero(),
            p_matrix: Matrix4::zero(),
            mvp_matrix: GlUniformMatrix4::new(Matrix4::zero(), "mvpMatrix"),
            tmp_matrix:Matrix4::zero(),
            inv_matrix: GlUniformMatrix4::new(Matrix4::zero(), "invMatrix"),
            camera_pitch: PI/2.0,
            camera_yaw: 0.0,
            camera_rotation: false,
            camera_rotation_speed: 1.0,
            zoom_ratio : 1.0,
            bg_colors: [Color::new(0.3,0.5,0.7,1.0), Color::new(0.3,0.5,0.7,1.0),Color::new(1.0,1.0,1.0,1.0), Color::new(0.9,0.9,0.9,1.0) ],
            eye_direction: GlUniformVec3::new(Vec3::new(0.0,0.0,15.0),"eyeDirection"),
            light_direction: GlUniformVec3::new(Vec3::new(-0.5,0.5,0.5), "lightDirection"),
            ambient_color: GlUniformColor::new(Color::new(0.2,0.2,0.2,1.0), "ambientColor"),
            flat_shading: GlUniformInt::new(0, "flatShading"),
            point_mvp_matrix: GlUniformMatrix4::new(Matrix4::zero(), "mvpMatrix"),
            point_size: GlUniformFloat::new(10.0, "pointSize"),
            point_color: GlUniformColor::new(Color::new(0.2,0.2,0.2,1.0), "color"),
            line_mvp_matrix: GlUniformMatrix4::new(Matrix4::zero(), "mvpMatrix"),
            line_color: GlUniformColor::new(Color::new(0.2,0.2,0.2,1.0), "color"),
            surf_color: GlUniformColor::new(Color::new(0.5,0.5,0.5,1.0), "color"),
            bg_position_attr: GlAttributeVec2::new("pos"),
            bg_color_attr: GlAttributeColor::new("color"),
            //bg_index: GlIndex::new(),

            shader_program: GlProgram::new(include_str!("shader/vertex.glsl"),include_str!("shader/fragment.glsl")),
            line_shader_program: GlProgram::new(include_str!("shader/vertex_line.glsl"),include_str!("shader/fragment_line.glsl")),
            point_shader_program: GlProgram::new(include_str!("shader/vertex_point.glsl"),include_str!("shader/fragment_point.glsl")),
            bg_shader_program: GlProgram::new(include_str!("shader/vertex_bg.glsl"),include_str!("shader/fragment_bg.glsl")),
            //server:None,
            time:0,

            points: Vec::new(),
            point_buffer:GlPointBuffer::new(),
            lines: Vec::new(),
            line_buffer:GlLineBuffer::new(),
            surfs: Vec::new(),
            surf_buffer:GlFaceBuffer::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_point(&mut self, mut point : Box<GlPoint>){
        self.point_buffer.add(&mut*point);
        self.points.push(point)
    }
    //pub fn add_line(&mut self, line : &mut GlLine){
    #[allow(dead_code)]
    pub fn add_line(&mut self, mut line : Box<GlLine>){
        self.line_buffer.add(&mut*line);
        self.lines.push(line)
    }
    #[allow(dead_code)]
    pub fn add_surface(&mut self, mut surf : Box<GlSurface>){
        self.surf_buffer.add(&mut*surf);
        self.surfs.push(surf)
    }

    #[allow(dead_code)]
    pub fn clear_points(&mut self){
        self.point_buffer.clear();
        self.points.clear();
//        log_1(&JsValue::from(format!("clear_points: point.len() = {}", self.points.len() ))); //
    }

    #[allow(dead_code)]
    pub fn clear_lines(&mut self){
        self.line_buffer.clear();
        self.lines.clear();
    }

    #[allow(dead_code)]
    pub fn clear_surfaces(&mut self){
        self.surf_buffer.clear();
        self.surfs.clear();
    }

    #[allow(dead_code)]
    pub fn set_zoom_ratio(&mut self, zoom:f64){
        self.zoom_ratio = zoom;
    }
    #[allow(dead_code)]
    pub fn enable_camera_rotation(&mut self, flag:bool){
        self.camera_rotation = flag;
    }
    #[allow(dead_code)]
    pub fn set_camera_rotation_speed(&mut self, speed:f64){
        self.camera_rotation_speed = speed;
    }
    #[allow(dead_code)]
    pub fn set_camera_pitch_angle(&mut self, pitch:f64){
        self.camera_pitch = pitch;
    }
    #[allow(dead_code)]
    pub fn set_camera_yaw_angle(&mut self, yaw:f64){
        self.camera_yaw = yaw;
    }

    #[allow(dead_code)]
    pub fn set_bg_color(&mut self, bgcolor:&Color){
        self.bg_colors[0].set(bgcolor);
        self.bg_colors[1].set(bgcolor);
        self.bg_colors[2].set(bgcolor);
        self.bg_colors[3].set(bgcolor);
        self.set_bg();
    }
    #[allow(dead_code)]
    pub fn set_bg_colors(&mut self, bgcolor1:&Color, bgcolor2:&Color, bgcolor3:&Color, bgcolor4:&Color){
        self.bg_colors[0].set(bgcolor1);
        self.bg_colors[1].set(bgcolor2);
        self.bg_colors[2].set(bgcolor3);
        self.bg_colors[3].set(bgcolor4);
        self.set_bg();
    }

    #[allow(dead_code)]
    pub fn set_bg(&mut self){
        let bg_geom = WebGlServer::bg_rect(self.bg_colors[0], self.bg_colors[1], self.bg_colors[2], self.bg_colors[3] );
        self.bg_position_attr.set_buffer(&self.gl, &bg_geom.0);
        self.bg_color_attr.set_buffer(&self.gl, &bg_geom.1);
    }

    #[allow(dead_code)]
    pub fn init(&mut self){

        self.bg_shader_program.init(&self.gl);
        let bg_program = self.bg_shader_program.program.as_ref().unwrap();
        self.bg_position_attr.set_location(&self.gl, &bg_program);
        self.bg_color_attr.set_location(&self.gl, &bg_program);


        self.point_shader_program.init(&self.gl);
        let point_program = self.point_shader_program.program.as_ref().unwrap();
        self.point_mvp_matrix.set_location(&self.gl,&point_program);
        self.point_color.set_location(&self.gl,&point_program);
        self.point_size.set_location(&self.gl,&point_program);


        self.line_shader_program.init(&self.gl);
        let line_program = self.line_shader_program.program.as_ref().unwrap();
        self.line_mvp_matrix.set_location(&self.gl,&line_program);
        self.line_color.set_location(&self.gl,&line_program);


        self.shader_program.init(&self.gl);
        let program = self.shader_program.program.as_ref().unwrap();
        self.surf_color.set_location(&self.gl,&program);

        self.mvp_matrix.set_location(&self.gl,&program);
        self.inv_matrix.set_location(&self.gl,&program);
        self.eye_direction.set_location(&self.gl,&program);
        self.light_direction.set_location(&self.gl,&program);
        self.ambient_color.set_location(&self.gl,&program);
        self.flat_shading.set_location(&self.gl,&program);

        //self.gl.enable(GL::DEPTH_TEST);
        //self.gl.enable(GL::CULL_FACE);
        self.gl.depth_func(GL::LEQUAL);


        self.v_matrix = Matrix4::look_at(&self.eye_direction.vec, &Vec3::new(0.0,0.0,0.0), &Vec3::new(0.0,1.0,0.0));
        self.p_matrix = Matrix4::perspective( (self.width/self.height) as f64, PI/4., 0.1, 100.0);
        self.tmp_matrix = Matrix4::new_with_matrix4(&self.v_matrix);
        self.tmp_matrix.matmul(&self.p_matrix);

        //log_1(&JsValue::from(format!("P{:?}", self.p_matrix.to_array32())));
        //log_1(&JsValue::from(format!("V{:?}", self.v_matrix.to_array32())));
        //log_1(&JsValue::from(format!("T{:?}", self.tmp_matrix.to_array32())));

        //let bg_geom = WebGlServer::bg_rect(Color::new(1.0,0.5,0.0,1.0), Color::new(1.0,0.0,0.0,1.0), Color::new(0.0,1.0,1.0,1.0), Color::new(0.5,0.5,1.0,1.0) );
        let bg_geom = WebGlServer::bg_rect(self.bg_colors[0], self.bg_colors[1], self.bg_colors[2], self.bg_colors[3] );
        self.bg_position_attr.set_buffer(&self.gl, &bg_geom.0);
        self.bg_color_attr.set_buffer(&self.gl, &bg_geom.1);
        //self.bg_index.set_buffer(&self.gl, &bg_geom.2);

    }

    pub fn draw(&mut self){
        //let count = self.time; //self.server.unwrap().time();

        //self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        //self.gl.clear_depth(1.);

        self.gl.disable(GL::DEPTH_TEST);
        self.bg_shader_program.use_program(&self.gl);

        self.bg_position_attr.activate(&self.gl);
        self.bg_color_attr.activate(&self.gl);
        //self.bg_index.activate(&self.gl);
        //self.gl.draw_elements_with_i32(GL::TRIANGLE_FAN, self.bg_index.size, GL::UNSIGNED_SHORT, 0);
        //self.gl.draw_elements_with_i32(GL::TRIANGLE_FAN, 4, GL::UNSIGNED_SHORT, 0);
        self.gl.draw_arrays(GL::TRIANGLE_FAN, 0, 4 );

        self.gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
        //self.gl.delete_buffer(Some(&bg_pos_vbo));
        //self.gl.delete_buffer(Some(&bg_clr_vbo));
        //self.gl.delete_buffer(Some(&bg_ibo));

        //self.bg_position_attr.unset_buffer(&self.gl);

        self.gl.clear_depth(1.);
        self.gl.enable(GL::DEPTH_TEST);

        //self.gl.enable(GL::BLEND); //
        //self.gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA); //


        //Webgl initialize
        //let i:f64 = (count % 360) as f64;
        let mut rad =  self.time as f64 * PI / 180. * self.camera_rotation_speed + self.camera_yaw ;
        if ! self.camera_rotation{ rad = self.camera_yaw; }

        if rad < 0.0 {
            rad += ((-rad/(2.0*PI)).floor()+1.0)*2.0*PI;
        }
        else if rad >= 2.0*PI{
            rad -= (rad/(2.0*PI)).floor()*2.0*PI;
        }
        /*
        if self.time < 500{
            self.camera_pitch = PI/2.0;
        }
        else if self.time < 1000{
            self.camera_pitch = (PI / 12.0 - PI/2.0) * (self.time - 500) as f64 / (1000.0-500.0) + PI/2.0;
        }
        else{
            self.camera_pitch = PI / 12.0;
        }
        */
        //view rotation
        //self.m_matrix = Matrix4::y_rotation(rad);
        self.m_matrix = Matrix4::x_rotation(PI/2.0);
        self.m_matrix.matmul(&Matrix4::y_rotation(rad));
        self.m_matrix.matmul(&Matrix4::x_rotation(-self.camera_pitch));
        self.m_matrix.matmul(&Matrix4::scale(self.zoom_ratio));

        self.mvp_matrix.mat = Matrix4::new_with_matrix4(&self.m_matrix);
        self.mvp_matrix.mat.matmul(&self.tmp_matrix);

        self.inv_matrix.mat = Matrix4::new_with_matrix4(&self.m_matrix);
        self.inv_matrix.mat.invert();

        // drawing points
        self.point_shader_program.use_program(&self.gl);
        let program = self.point_shader_program.program.as_ref().unwrap();

        self.point_mvp_matrix.mat = Matrix4::new_with_matrix4(&self.m_matrix);
        self.point_mvp_matrix.mat.matmul(&self.tmp_matrix);
        self.point_mvp_matrix.set_uniform(&self.gl);

        let mut point_pos_attr = GlAttributeVec3::new("position");
        point_pos_attr.set_location(&self.gl, &program);
        point_pos_attr.set_buffer(&self.gl, &self.point_buffer.pos);
        point_pos_attr.activate(&self.gl);

        for i in 0..self.points.len(){
                let pt = &self.points[i];
                self.point_color.set_color(&pt.color);
                self.point_color.set_uniform(&self.gl);
                self.point_size.x = pt.size; //5.0; //20.0;
                self.point_size.set_uniform(&self.gl);

                self.gl.draw_arrays(GL::POINTS, pt.index, 1 );
        }
        point_pos_attr.unset_buffer(&self.gl);

        // drawing lines
        self.line_shader_program.use_program(&self.gl);
        let program = self.line_shader_program.program.as_ref().unwrap();

        self.line_mvp_matrix.mat = Matrix4::new_with_matrix4(&self.m_matrix);
        self.line_mvp_matrix.mat.matmul(&self.tmp_matrix);
        self.line_mvp_matrix.set_uniform(&self.gl);

        let mut line_pos_attr = GlAttributeVec3::new("position");
        line_pos_attr.set_location(&self.gl, &program);
        line_pos_attr.set_buffer(&self.gl, &self.line_buffer.pos);

        line_pos_attr.activate(&self.gl);

        for i in 0..self.lines.len(){
            let line = &self.lines[i];
            self.line_color.set_color(&line.color);
            self.line_color.set_uniform(&self.gl);
            self.gl.draw_arrays(GL::LINE_STRIP, line.index, line.len() );
        }
        line_pos_attr.unset_buffer(&self.gl);


        // drawing surface
        self.shader_program.use_program(&self.gl);
        let surf_program = self.shader_program.program.as_ref().unwrap();
        self.mvp_matrix.set_uniform(&self.gl);
        self.inv_matrix.set_uniform(&self.gl);
        self.light_direction.set_uniform(&self.gl);
        self.eye_direction.set_uniform(&self.gl);
        self.ambient_color.set_uniform(&self.gl);
        self.flat_shading.set_uniform(&self.gl);


        let mut surf_nml_attr = GlAttributeVec3::new("normal");
        surf_nml_attr.set_location(&self.gl, &surf_program);
        surf_nml_attr.set_buffer(&self.gl, &self.surf_buffer.nml);

        let mut surf_pos_attr = GlAttributeVec3::new("position");
        surf_pos_attr.set_location(&self.gl, &surf_program);
        surf_pos_attr.set_buffer(&self.gl, &self.surf_buffer.pos);

        surf_nml_attr.activate(&self.gl);
        surf_pos_attr.activate(&self.gl);

        for i in 0..self.surfs.len(){
            let srf = &self.surfs[i];
            self.surf_color.set_color(&srf.color);
            self.surf_color.set_uniform(&self.gl);

            for j in 0..srf.vlen()-1{
                self.gl.draw_arrays(GL::TRIANGLE_STRIP, srf.index + srf.ulen()*2*j, srf.ulen()*2 );
            }
            //self.gl.draw_arrays(GL::TRIANGLE_STRIP, srf.index, srf.ulen()*(srf.vlen()-1)*2 );
        }


        for i in 0..self.surfs.len(){
            let srf = &self.surfs[i];
            //self.surf_color.set_color(&srf.color);

            self.surf_color.set_color(&Color::new(0.0,0.0,0.0,1.0));
            self.surf_color.set_uniform(&self.gl);

            //self.gl.draw_arrays(GL::TRIANGLE_STRIP, srf.index, srf.ulen()*(srf.vlen()-1)*2 );
            for j in 0..srf.ulen(){
                for k in 0..srf.vlen(){
                    self.gl.draw_arrays(GL::POINTS, srf.index+ srf.vlen()*j + k, 1 );
                }
            }
        }


        surf_nml_attr.unset_buffer(&self.gl);
        surf_pos_attr.unset_buffer(&self.gl);

        self.time+=1;
    }

    #[allow(dead_code)]
    pub fn bg_rect(clr1:Color, clr2:Color, clr3:Color, clr4:Color)->(Vec<f32>, Vec<f32>, Vec<u16>){
        let mut pos = Vec::new();
        let mut clr = Vec::new();
        let mut idx = Vec::new();
        pos.extend_from_slice(&[-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0]);
        clr.extend_from_slice(&[
            clr1.r(),clr1.g(),clr1.b(),clr1.a(),
            clr2.r(),clr2.g(),clr2.b(),clr2.a(),
            clr3.r(),clr3.g(),clr3.b(),clr3.a(),
            clr4.r(),clr4.g(),clr4.b(),clr4.a() ]);
        idx.extend_from_slice(&[0, 1, 2, 3]);
        (pos, clr, idx)
    }

    #[allow(dead_code)]
    pub fn get_webgl_context(height: u32, width: u32) -> Result<GL, String> {
        //Get WebGLContext
        let document = window().unwrap().document().unwrap();
        let canvas = document
        .get_element_by_id("canvas")
        .ok_or_else(|| String::from("canvas doesn't exist :("))?;
        let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        canvas.set_height(height);
        canvas.set_width(width);

        let gl: GL = canvas
        .get_context("webgl")
        .unwrap()
        .ok_or_else(|| String::from("webgl is not supported in this browser :("))?
        .dyn_into()
        .unwrap();

        //Initialize WebGLContext
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
//        gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
        gl.clear_depth(1.);

        Ok(gl)
    }

}
