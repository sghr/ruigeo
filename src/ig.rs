use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use std::fmt;

use std::collections::VecDeque;

pub mod math;
pub mod geo;
pub mod gl;
pub mod agent;

pub use math::{Vec3, Vec4, Matrix3, Matrix4, Orient};

pub use geo::{Point, Curve, Surface};

pub use agent::{Agent, LineAgent, Particle, OrientAgent, AgentAttr};

use gl::{WebGlServer, GlPoint, GlLine, GlSurface};



pub struct Manager{
    pub adding_data: Storage,
    pub deleting_data: Storage,
    pub time: i64,
}

impl Manager{
    #[allow(dead_code)]
    pub fn new_with_storage(storage:&Storage, time:i64)->Self{
        Manager{adding_data:Storage::new_with_storage(storage), deleting_data:Storage::new(), time}
    }
    #[allow(dead_code)]
    pub fn add_point(&mut self, point:Box<Point>)->usize{
        self.adding_data.add_point(point)
    }
    #[allow(dead_code)]
    pub fn add_curve(&mut self, curve:Box<Curve>)->usize{
        self.adding_data.add_curve(curve)
    }
    #[allow(dead_code)]
    pub fn add_surface(&mut self, surface:Box<Surface>)->usize{
        self.adding_data.add_surface(surface)
    }
    #[allow(dead_code)]
    pub fn add_agent(&mut self, agent:Box<dyn Agent>)->usize{
        self.adding_data.add_agent(agent)
    }
    #[allow(dead_code)]
    pub fn delete_point(&mut self, index:i32){
        self.deleting_data.points.add_index(index);
    }
    #[allow(dead_code)]
    pub fn delete_curve(&mut self, index:i32){
        self.deleting_data.curves.add_index(index);
    }
    #[allow(dead_code)]
    pub fn delete_surface(&mut self, index:i32){
        self.deleting_data.surfaces.add_index(index);
    }
    #[allow(dead_code)]
    //pub fn delete_agent(&mut self, index:i32){
    //    self.deleting_data.agents.add_index(index);
    //}
    pub fn delete_agent(&mut self, agent:&mut dyn Agent){
        self.deleting_data.agents.add_index(agent.attr().id);
    }

}

pub struct IndexedVec<T>{
    vec: VecDeque<T>,
    index: VecDeque<i32>,
    offset: usize
}

impl <T> IndexedVec<T>{
    #[allow(dead_code)]
    pub fn new()->Self{
        IndexedVec{
            vec:VecDeque::new(),
            index:VecDeque::new(),
            offset:0
        }
    }
    #[allow(dead_code)]
    pub fn new_with_offset(offset:usize)->Self{
        IndexedVec{
            vec:VecDeque::new(),
            index:VecDeque::new(),
            offset
        }
    }
    #[allow(dead_code)]
    pub fn len(&self)->usize{
        self.vec.len()
    }
    #[allow(dead_code)]
    pub fn index_len(&self)->usize{
        self.index.len()
    }
    pub fn get(&mut self, i:usize)->&mut T{ &mut self.vec[i] }
    pub fn idx(&self, i:usize)->i32{ self.index[i] }

    #[allow(dead_code)]
    pub fn add(&mut self, /*mut*/ data:T)->usize{
        let id = self.index.len()+self.offset;
        self.vec.push_back(data);
        self.index.push_back((self.vec.len()-1) as i32);
        id
    }
    #[allow(dead_code)]
    pub fn add_index(&mut self, index:i32)->usize{
        let id = self.index.len()+self.offset;
        self.index.push_back(index);
        id
    }

    #[allow(dead_code)]
    pub fn sort_index(&mut self){ // descending sort
        self.index.make_contiguous().sort_by(|a, b| b.cmp(&a));
    }
    #[allow(dead_code)]
    pub fn delete(&mut self, index:i32){
        if index>=0 && index < self.index.len() as i32 && self.index[index as usize] >= 0{
            self.vec.remove(self.index[index as usize] as usize);
            self.index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.index.len(){ // index of deleted object stays
                self.index[i] -= 1;
            }
        }
        else{
            log_1(&JsValue::from(format!("at delete: ERROR: index at {} is negative!", index)));
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.vec.clear();
        self.index.clear();
    }
}


pub struct Storage{
    points: IndexedVec<Box<Point>>,
    curves: IndexedVec<Box<Curve>>,
    surfaces: IndexedVec<Box<Surface>>,
    //agents: IndexedVec<Box<dyn Agent>>,
    agents: IndexedVec<Box<dyn Agent>>,
}

impl Storage{
    #[allow(dead_code)]
    pub fn new()->Self{
        Storage{
            points:IndexedVec::new(),
            curves:IndexedVec::new(),
            surfaces:IndexedVec::new(),
            agents:IndexedVec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_offset(pt_offset:usize, crv_offset:usize, srf_offset:usize, agn_offset:usize )->Self{
        Storage{
            points:IndexedVec::new_with_offset(pt_offset),
            curves:IndexedVec::new_with_offset(crv_offset),
            surfaces:IndexedVec::new_with_offset(srf_offset),
            agents:IndexedVec::new_with_offset(agn_offset),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_storage(storage:&Storage)->Self{
        Storage{
            points:IndexedVec::new_with_offset(storage.points.index_len()),
            curves:IndexedVec::new_with_offset(storage.curves.index_len()),
            surfaces:IndexedVec::new_with_offset(storage.surfaces.index_len()),
            agents:IndexedVec::new_with_offset(storage.agents.index_len()),
        }
    }

    #[allow(dead_code)]
    pub fn add_point(&mut self, point:Box<Point>)->usize{ self.points.add(point) }
    #[allow(dead_code)]
    pub fn add_curve(&mut self, curve:Box<Curve>)->usize{ self.curves.add(curve) }
    #[allow(dead_code)]
    pub fn add_surface(&mut self, surface:Box<Surface>)->usize{ self.surfaces.add(surface) }
    #[allow(dead_code)]
    pub fn add_agent(&mut self, mut agent:Box<dyn Agent>)->usize{
        let id = self.agents.index.len()+self.agents.offset;
        agent.attr().set_id(id as i32);
        self.agents.add(agent);
        id
    }

    #[allow(dead_code)]
    pub fn sort_point_index(&mut self){ self.points.sort_index(); }
    #[allow(dead_code)]
    pub fn sort_curve_index(&mut self){ self.curves.sort_index(); }
    #[allow(dead_code)]
    pub fn sort_surface_index(&mut self){ self.surfaces.sort_index(); }
    #[allow(dead_code)]
    pub fn sort_agent_index(&mut self){ self.agents.sort_index(); }

    #[allow(dead_code)]
    pub fn delete_point(&mut self, index:i32){ self.points.delete(index); }
    #[allow(dead_code)]
    pub fn delete_curve(&mut self, index:i32){ self.curves.delete(index); }
    #[allow(dead_code)]
    pub fn delete_surface(&mut self, index:i32){ self.surfaces.delete(index); }
    #[allow(dead_code)]
    pub fn delete_agent(&mut self, index:i32){ self.agents.delete(index); }

    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.points.clear();
        self.curves.clear();
        self.surfaces.clear();
        self.agents.clear();
    }

}

pub struct Server{
    storage:Storage,
    glserver: WebGlServer,
    time:i64,
    duration:i64,
    max_agent_limit:i32,
}

impl Server{
    #[allow(dead_code)]
    pub fn new(width:f32, height:f32) -> Self{
        Server{
            storage:Storage::new(),
            glserver:WebGlServer::new(width,height),
            time: 0,
            duration : -1,
            max_agent_limit:-1
        }
    }
    #[allow(dead_code)]
    pub fn add_point(&mut self, point:Box<Point>)->usize{
        self.glserver.add_point(Box::new(GlPoint::from_point(&point)));
        let id = self.storage.add_point(point);
        id
    }
    #[allow(dead_code)]
    pub fn add_curve(&mut self, curve:Box<Curve>)->usize{
        self.glserver.add_line(Box::new(GlLine::from_curve(&curve)));
        let id = self.storage.add_curve(curve);
        //curve.id = id;
        id
    }
    #[allow(dead_code)]
    pub fn add_surface(&mut self, surface:Box<Surface>)->usize{
        self.glserver.add_surface(Box::new(GlSurface::from_surface(&surface)));
        let id = self.storage.add_surface(surface);
        id
    }
    #[allow(dead_code)]
    pub fn add_agent(&mut self, agent:Box<dyn Agent>)->usize{
        if self.max_agent_limit >= 0 && self.storage.agents.len() as i32 >= self.max_agent_limit{
            return 0; // shouldn't this be -1?
        }
        let id = self.storage.add_agent(agent);
        //agent.set_id(id);
        id
    }

    #[allow(dead_code)]
    pub fn delete_data(&mut self, mut deleting_data: Storage){
        if deleting_data.points.index_len() > 0{
            deleting_data.sort_point_index();
            for i in 0..deleting_data.points.index_len(){
                self.storage.delete_point(deleting_data.points.idx(i) );
            }
            self.glserver.clear_points();
            // re-add glserver points
            for i in 0..self.storage.points.len(){
                self.glserver.add_point(Box::new(GlPoint::from_point(&self.storage.points.get(i))));
            }
        }

        if deleting_data.curves.index_len() > 0{
            deleting_data.sort_curve_index();
            for i in 0..deleting_data.curves.index_len(){
                self.storage.delete_curve(deleting_data.curves.idx(i) );
            }
            self.glserver.clear_lines();
            // re-add glserver curve
            for i in 0..self.storage.curves.len(){
                self.glserver.add_line(Box::new(GlLine::from_curve(&self.storage.curves.get(i))));
            }
        }

        if deleting_data.surfaces.index_len() > 0{
            deleting_data.sort_surface_index();
            for i in 0..deleting_data.surfaces.index_len(){
                self.storage.delete_surface(deleting_data.surfaces.idx(i) );
            }
            self.glserver.clear_surfaces();
            // re-add glserver curve
            for i in 0..self.storage.surfaces.len(){
                self.glserver.add_surface(Box::new(GlSurface::from_surface(&self.storage.surfaces.get(i))));
            }
        }

        if deleting_data.agents.index_len() > 0{
            deleting_data.sort_agent_index();
            for i in 0..deleting_data.agents.index_len(){
                self.storage.delete_agent(deleting_data.agents.idx(i) );
            }
        }
    }

    #[allow(dead_code)]
    pub fn add_data(&mut self, adding_data: Storage){
        for p in adding_data.points.vec{
            self.add_point(p);
        }
        for c in adding_data.curves.vec{
            self.add_curve(c);
        }
        for s in adding_data.surfaces.vec{
            self.add_surface(s);
        }
        for a in adding_data.agents.vec{
            self.add_agent(a);
        }
    }

    #[allow(dead_code)]
    pub fn set_active_agent_limit_num(&mut self, max_count:i32){
        self.max_agent_limit = max_count;
    }
    #[allow(dead_code)]
    pub fn set_zoom(&mut self, zoom:f64){
        self.glserver.set_zoom_ratio(zoom);
    }
    #[allow(dead_code)]
    pub fn  enable_camera_rotation(&mut self, flag:bool){
        self.glserver.enable_camera_rotation(flag);
    }
    #[allow(dead_code)]
    pub fn set_camera_rotation_speed(&mut self, speed:f64){
        self.glserver.set_camera_rotation_speed(speed);
    }
    #[allow(dead_code)]
    pub fn set_camera_pitch(&mut self, pitch:f64){
        self.glserver.set_camera_pitch_angle(pitch);
    }
    #[allow(dead_code)]
    pub fn set_camera_yaw(&mut self, yaw:f64){
        self.glserver.set_camera_yaw_angle(yaw);
    }

    #[allow(dead_code)]
    pub fn bg(&mut self, bgcolor:&Color){
        self.glserver.set_bg_color(bgcolor);
    }
    #[allow(dead_code)]
    pub fn bg_colors(&mut self, bgcolor1:&Color, bgcolor2:&Color, bgcolor3:&Color, bgcolor4:&Color){
        self.glserver.set_bg_colors(bgcolor1, bgcolor2, bgcolor3, bgcolor4);
    }


    #[allow(dead_code)]
    pub fn init(&mut self){
        self.glserver.init();
    }
    #[allow(dead_code)]
    pub fn draw(&mut self){
        self.glserver.draw();

        if self.duration < 0 || self.time < self.duration{
            let mut mgr = Manager::new_with_storage(&self.storage, self.time.clone());

            if self.storage.agents.len() > 0{
                log_1(&JsValue::from(format!("Server::agents.len()={}", self.storage.agents.len())));
            }

            if self.max_agent_limit < 0 || (self.storage.agents.len() as i32) < self.max_agent_limit {
                let mut agents_copy : Vec<Box<dyn Agent>> = Vec::new();
                for i in 0..self.storage.agents.len(){
                    //agents_copy.push(Box::new((*self.storage.agents[i]).clone()));
                    agents_copy.push(self.storage.agents.get(i).clone());
                }

                for i in 0..self.storage.agents.len(){
                    self.storage.agents.get(i).interact(&mut agents_copy, &mut mgr);
                }

                for i in 0..self.storage.agents.len(){
                    self.storage.agents.get(i).update(&mut mgr);
                }
                for i in 0..self.storage.agents.len(){
                    self.storage.agents.get(i).attr().time += 1;
                }

                self.delete_data(mgr.deleting_data);

                self.add_data(mgr.adding_data);
            }
        }
        self.time += 1;
    }


    #[allow(dead_code)]
    pub fn time(&self)->i64{
        self.time
    }

    #[allow(dead_code)]
    pub fn duration(&mut self, dur:i64){
        self.duration = dur;
    }
}


/*
pub struct GraphicServer{
    #[allow(dead_code)]
    graphics3d: Vec<Graphic>,
    glserver: WebGlServer,
    //server: Option<&'a Server<'a>>,
}
implGraphicServer{
    #[allow(dead_code)]
    pub fn new(width:f32,height:f32) -> Self{
        GraphicServer{
            graphics3d: Vec::new(),
            glserver: WebGlServer::new(width, height),
            //server:None
        }
    }
    #[allow(dead_code)]
    pub fn init(&mut self){
        //self.server = Some(serv);
        self.glserver.init();
    }
    #[allow(dead_code)]
    pub fn draw(&mut self){ self.glserver.draw(); }
}

#[allow(dead_code)]
pub struct DynamicServer{
    #[allow(dead_code)]
    dynamics: Vec<Dynamic>,
    #[allow(dead_code)]
    adding_dynamics: Vec<Dynamic>,
    #[allow(dead_code)]
    removing_dynamics: Vec<Dynamic>,
    #[allow(dead_code)]
    time: i64,
    //server: Option<&'a Server<'a>>,
}

impl DynamicServer{
    #[allow(dead_code)]
    pub fn new()->Self{
        DynamicServer{
            dynamics: Vec::new(),
            adding_dynamics: Vec::new(),
            removing_dynamics: Vec::new(),
            time: 0,
            //server:None,
        }
    }
    #[allow(dead_code)]
    pub fn init(&mut self){
        //self.server=Some(serv);
    }
    #[allow(dead_code)]
    pub fn update(&mut self){}
    #[allow(dead_code)]
    pub fn time(&self)->i64{ self.time }
}
*/





/******************
*    Graphics
******************/

#[allow(dead_code)]
pub const DEFAULT_GL_LIGHT_POSITION : [f32;4] = [0.0, 0.0, 1.0, 0.0];
#[allow(dead_code)]
pub const DEFAULT_GL_AMBIENT_LIGHT : [f32;4] = [0.4, 0.4, 0.4, 1.0];
#[allow(dead_code)]
pub const DEFAULT_GL_DIFFUSE_LIGHT : [f32;4] = [0.7, 0.7, 0.7, 1.0];
#[allow(dead_code)]
pub const DEFAULT_GL_SPECULAR_LIGHT : [f32;4] = [0.0, 0.0, 0.0, 1.0];

#[allow(dead_code)]
pub const DEFAULT_PERSPECTIVE_RATIO : f32  = 0.5;
#[allow(dead_code)]
pub const DEFAULT_AXONOMETRIC_RATIO : f32 = 1.0;
#[allow(dead_code)]
pub const DEFAULT_VIEW_DISTANCE : f32 = 500.0;
#[allow(dead_code)]
pub const DEFAULT_VIEW_DISTANCE_RATIO : f32 = 10.0;
#[allow(dead_code)]
pub const DEFAULT_NEAR_VIEW_RATIO : f32 = 0.001;
#[allow(dead_code)]
pub const DEFAULT_FAR_VIEW_RATIO : f32 = 1000.0;


#[allow(dead_code)]
pub const DEFAULT_BG_COLOR1 : Color = Color{ rgba:[1.0, 1.0, 1.0, 1.0] };
#[allow(dead_code)]
pub const DEFAULT_BG_COLOR2 : Color = Color{ rgba:[0.9, 0.9, 0.9, 1.0] };
#[allow(dead_code)]
pub const DEFAULT_BG_COLOR3 : Color = Color{ rgba:[0.3, 0.5, 0.7, 1.0] };
#[allow(dead_code)]
pub const DEFAULT_BG_COLOR4 : Color = Color{ rgba:[0.3, 0.5, 0.7, 1.0] };


#[allow(dead_code)]
pub const UPDATE_RATE : f32 = 30.0/1000.0;

pub struct View{
    pub pos:Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub axonometric: bool,
    pub screen_x: u32,
    pub screen_y: u32,
    pub screen_offset_x: u32,
    pub screen_offset_y: u32,
    pub screen_width: u32,
    pub screen_height: u32,
    pub pers_ratio: f32,
    pub axon_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub view_distance: f32,

    pub target: Vec3,
    pub rotate_around_target: bool,
    pub transform_matrix: Matrix4,
    pub transform_array: [f64;16],

    pub use_gl: bool,
    pub hide: bool,

    pub bg_color : [[Color;2];2],

}

impl View{
    #[allow(dead_code)]
    pub fn new(x:f64, y:f64, z:f64, yaw:f32, pitch:f32, roll:f32,
        screen_x:u32, screen_y:u32, screen_width:u32, screen_height:u32, axonometric:bool) -> Self{
            View{
                pos: Vec3{x,y,z},
                yaw,
                pitch,
                roll,
                axonometric,
                screen_x,
                screen_y,
                screen_offset_x:0,
                screen_offset_y:0,
                screen_width,
                screen_height,
                pers_ratio: DEFAULT_PERSPECTIVE_RATIO,
                axon_ratio: DEFAULT_AXONOMETRIC_RATIO,
                near: 100.0*DEFAULT_NEAR_VIEW_RATIO,
                far: 100.0*DEFAULT_FAR_VIEW_RATIO,
                view_distance : DEFAULT_VIEW_DISTANCE,
                target : Vec3{ x:0.0,y:0.0,z:0.0 },
                rotate_around_target: false,
                transform_matrix: Matrix4{ val: [[0.0;4];4] },
                transform_array: [0.0;16],
                use_gl: false,
                hide: false,
                bg_color: [[DEFAULT_BG_COLOR1, DEFAULT_BG_COLOR4], [DEFAULT_BG_COLOR2, DEFAULT_BG_COLOR3]]
            }
        }
}




/******************
*    Objects
******************/
//pub struct Object/*<'a>*/{
//    //server:Option<&'static/*'a*/ mut Server/*<'a>*/>,
//    server:Option<Box<Server>>,
    //graphics:Vec<&'a Graphic>,
    //dynamics:Vec<&'a Dynamic>,
//    attribute: /*&'a*/ Attr/*<'a>*/,
//}

pub trait Object{
    //fn init(&dyn self, server: &mut Server){ server.add_object(Box::new(self)); }
    fn draw(&mut self);
//    fn interact(&mut self, objects:&Vec<Box<dyn Object>>);
//    fn update(&mut self);
    //fn attr(&self)->&mut Attr;
}


//pub struct Graphic{}
//impl Graphic{}
//pub struct Dynamic{}
//impl Dynamic{}

#[derive(Debug, Clone)]
pub struct Attr{
    pub id: i32,
    pub name: String,
    //layer: Layer,
    pub layer_index: u32,
    pub color: Color,
    pub stroke: Color,
    pub size: f32,
    pub weight: f32,
    //pub mateiral: Material
    pub visible: bool,
}

impl Attr{
    #[allow(dead_code)]
    pub fn default()->Self{
        Attr{
            id:-1,name: String::from("default"), layer_index:0,
            color: Color::new(0.5,0.5,0.5,1.0), stroke: Color::new(0.5,0.5,0.5,1.0),
            size: 5.0, weight: 1.0, visible:true
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, attr:&Attr)->&mut Self{
        self.name = attr.name.clone();
        self.layer_index = attr.layer_index;
        self.color.set(&attr.color);
        self.stroke.set(&attr.stroke);
        self.size = attr.size;
        self.weight = attr.weight;
        self.visible = attr.visible;
        self
    }
    #[allow(dead_code)]
    pub fn set_id(&mut self, id:i32)->&mut Self{
        self.id = id;
        self
    }

    #[allow(dead_code)]
    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Self{
        self.color.clr(r,g,b,a);
        self.stroke.clr(r,g,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn rgba(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Self{
        self.color.rgba(r,g,b,a);
        self.stroke.rgba(r,g,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn rgb(&mut self, r:f32, g:f32, b:f32)->&mut Self{
        self.color.rgb(r,g,b);
        self.stroke.rgb(r,g,b);
        self
    }
    #[allow(dead_code)]
    pub fn hsba(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Self{
        self.color.hsba(h,s,b,a);
        self.stroke.hsba(h,s,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn hsb(&mut self, h:f32, s:f32, b:f32)->&mut Self{
        self.color.hsb(h,s,b);
        self.stroke.hsb(h,s,b);
        self
    }
}


/*
pub struct Material{
}
*/

#[derive(Debug, Clone, Copy)]
pub struct Color{
    rgba: [f32; 4]
}

impl Color{
    #[allow(dead_code)]
    pub fn new(r:f32, g:f32, b:f32, a:f32)->Self{
        Color{
            rgba:[r,g,b,a]
        }
    }

    #[allow(dead_code)]
    pub fn new_with_rgb(r:f32, g:f32, b:f32)->Self{
        Color{
            rgba:[r,g,b,1.0]
        }
    }

    #[allow(dead_code)]
    pub fn new_with_hsb(mut h:f32, s:f32, b:f32, a:f32)->Self{
        if h<0.0 { h += (-h+1.0).floor(); }
        else if h>1.0 { h -= h.floor(); }
        let frac = h*6.0 - (h*6.0).floor();
        if h*6.0 < 1.0 { return Color::new(b, b*(1.0-s*(1.0-frac)), b*(1.0-s), a); }
        if h*6.0 < 2.0 { return Color::new(b*(1.0-s*frac), b, b*(1.0-s), a); }
        if h*6.0 < 3.0 { return Color::new(b*(1.0-s), b, b*(1.0-s*(1.0-frac)), a); }
        if h*6.0 < 4.0 { return Color::new(b*(1.0-s), b*(1.0-s*frac), b, a); }
        if h*6.0 < 5.0 { return Color::new(b*(1.0-s*(1.0-frac)), b*(1.0-s), b, a); }
        return Color::new(b, b*(1.0-s), b*(1.0-s*frac), a);
    }

    #[allow(dead_code)]
    pub fn set_rgb(&mut self, r:f32, g:f32, b:f32){
        self.rgba[0] = r;
        self.rgba[1] = g;
        self.rgba[2] = b;
    }

    #[allow(dead_code)]
    pub fn set(&mut self, color:&Color){
        self.rgba[0] = color.rgba[0];
        self.rgba[1] = color.rgba[1];
        self.rgba[2] = color.rgba[2];
        self.rgba[3] = color.rgba[3];
    }

    #[allow(dead_code)]
    pub fn set_rgba(&mut self, r:f32, g:f32, b:f32, a:f32){
        self.rgba[0] = r;
        self.rgba[1] = g;
        self.rgba[2] = b;
        self.rgba[3] = a;
    }

    #[allow(dead_code)]
    pub fn set_hsb(&mut self, mut h:f32, s:f32, b:f32, a:f32){
        if h<0.0 { h += (-h+1.0).floor(); }
        else if h>1.0 { h -= h.floor(); }
        let frac = h*6.0 - (h*6.0).floor();
        if h*6.0 < 1.0 { self.set_rgba(b, b*(1.0-s*(1.0-frac)), b*(1.0-s), a); }
        else if h*6.0 < 2.0 { self.set_rgba(b*(1.0-s*frac), b, b*(1.0-s), a); }
        else if h*6.0 < 3.0 { self.set_rgba(b*(1.0-s), b, b*(1.0-s*(1.0-frac)), a); }
        else if h*6.0 < 4.0 { self.set_rgba(b*(1.0-s), b*(1.0-s*frac), b, a); }
        else if h*6.0 < 5.0 { self.set_rgba(b*(1.0-s*(1.0-frac)), b*(1.0-s), b, a); }
        else{ self.set_rgba(b, b*(1.0-s), b*(1.0-s*frac), a); }
    }

    #[allow(dead_code)]
    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32){
        self.set_rgba(r,g,b,a);
    }
    #[allow(dead_code)]
    pub fn rgba(&mut self, r:f32, g:f32, b:f32, a:f32){
        self.set_rgba(r,g,b,a);
    }
    #[allow(dead_code)]
    pub fn rgb(&mut self, r:f32, g:f32, b:f32){
        self.set_rgba(r,g,b,1.0);
    }
    #[allow(dead_code)]
    pub fn hsba(&mut self, h:f32, s:f32, b:f32, a:f32){
        self.set_hsb(h,s,b,a);
    }
    #[allow(dead_code)]
    pub fn hsb(&mut self, h:f32, s:f32, b:f32){
        self.set_hsb(h,s,b,1.0);
    }

    #[allow(dead_code)]
    pub fn red(&self)->f32{
        self.rgba[0]
    }
    #[allow(dead_code)]
    pub fn green(&self)->f32{
        self.rgba[1]
    }
    #[allow(dead_code)]
    pub fn blue(&self)->f32{
        self.rgba[2]
    }
    #[allow(dead_code)]
    pub fn alpha(&self)->f32{
        self.rgba[3]
    }
    #[allow(dead_code)]
    pub fn to_array(&self)->[f32;4]{
        self.rgba
    }

    #[allow(dead_code)]
    pub fn r(&self)->f32{ self.red() }
    #[allow(dead_code)]
    pub fn g(&self)->f32{ self.green() }
    #[allow(dead_code)]
    pub fn b(&self)->f32{ self.blue() }
    #[allow(dead_code)]
    pub fn a(&self)->f32{ self.alpha() }

}

impl fmt::Display for Color{
    #[allow(dead_code)]
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f, "({},{},{},{})", self.rgba[0], self.rgba[1], self.rgba[2],self.rgba[3])
    }
}


#[allow(dead_code)]
pub struct Layer{
//    object: Object,
//    objects: Vec<&/*'a*/ Object/*<'a>*/> // containing objects
}


/************************
* Geometry    pub fn add_object(&mut self, object:Box<dyn Object>){
        self.objects.push(object);
    }
 Objects
************************/
