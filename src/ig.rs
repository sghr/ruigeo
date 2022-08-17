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

pub use agent::{Agent, AgentTrait, LineAgent, Particle, OrientAgent, AgentInfo};

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
    pub fn add_agent(&mut self, agent:Box<dyn AgentTrait>)->usize{
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
    pub fn delete_agent(&mut self, agent:&mut dyn AgentTrait){
        self.deleting_data.agents.add_index(agent.info().id);
    }

}

pub struct DataManager{
    pub adding_data: DataStorage,
    pub deleting_data: DataStorage,
    pub time: i64,
}

impl DataManager{
    /*
    pub fn new()->Self{
        DataManager{adding_data:DataStorage::new(), deleting_data:DataStorage::new(),time:0}
    }
    pub fn new_with_offset(objOffset:usize, ptOffset:usize, crvOffset:usize, srfOffset:usize, agnOffset:usize )->Self{
        DataManager{adding_data:DataStorage::new_with_offset(objOffset, ptOffset, crvOffset, srfOffset, agnOffset), deleting_data:DataStorage::new()}
    }
    */
    #[allow(dead_code)]
    pub fn new_with_storage_offset(storage:&DataStorage, time:i64)->Self{
        DataManager{adding_data:DataStorage::new_with_storage_offset(storage), deleting_data:DataStorage::new(), time}
    }

    #[allow(dead_code)]
    pub fn add_object(&mut self, object:Box<dyn Object>)->usize{
        self.adding_data.add_object(object)
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
    pub fn add_agent(&mut self, agent:Box<Agent>)->usize{
        self.adding_data.add_agent(agent)
    }

    #[allow(dead_code)]
    pub fn delete_object(&mut self, index:i32){
        self.deleting_data.object_index.push(index);
    }
    #[allow(dead_code)]
    pub fn delete_point(&mut self, index:i32){
        self.deleting_data.point_index.push(index);
    }
    #[allow(dead_code)]
    pub fn delete_point_geom(&mut self, point:Point){
        self.deleting_data.point_index.push(point.id as i32);
    }

    #[allow(dead_code)]
    pub fn delete_curve(&mut self, index:i32){
        self.deleting_data.curve_index.push(index);
    }
    #[allow(dead_code)]
    pub fn delete_surface(&mut self, index:i32){
        self.deleting_data.surface_index.push(index);
    }
    #[allow(dead_code)]
    pub fn delete_agent(&mut self, index:i32){
        self.deleting_data.agent_index.push(index);
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
    pub fn add(&mut self, mut data:T)->usize{
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
    //agents: IndexedVec<Box<dyn AgentTrait>>,
    agents: IndexedVec<Box<dyn AgentTrait>>,
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
    pub fn add_agent(&mut self, mut agent:Box<dyn AgentTrait>)->usize{
        let id = self.agents.index.len()+self.agents.offset;
        agent.info().set_id(id as i32);
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




pub struct DataStorage{
    objects: Vec<Box<dyn Object>>,
    object_index: Vec<i32>,
    object_index_offset: usize,
    points: Vec<Box<Point>>,
    point_index: Vec<i32>,
    point_index_offset: usize,
    curves: Vec<Box<Curve>>,
    curve_index: Vec<i32>,
    curve_index_offset: usize,
    surfaces: Vec<Box<Surface>>,
    surface_index: Vec<i32>,
    surface_index_offset: usize,
    agents: Vec<Box<Agent>>,
    agent_index: Vec<i32>,
    agent_index_offset: usize,
}

impl DataStorage{
    #[allow(dead_code)]
    pub fn new()->Self{
        DataStorage{
            objects:Vec::new(),
            object_index:Vec::new(),
            object_index_offset:0,
            points:Vec::new(),
            point_index:Vec::new(),
            point_index_offset:0,
            curves:Vec::new(),
            curve_index:Vec::new(),
            curve_index_offset:0,
            surfaces:Vec::new(),
            surface_index:Vec::new(),
            surface_index_offset:0,
            agents:Vec::new(),
            agent_index:Vec::new(),
            agent_index_offset:0,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_offset(obj_offset:usize, pt_offset:usize, crv_offset:usize, srf_offset:usize, agn_offset:usize )->Self{
        DataStorage{
            objects:Vec::new(),
            object_index:Vec::new(),
            object_index_offset:obj_offset,
            points:Vec::new(),
            point_index:Vec::new(),
            point_index_offset:pt_offset,
            curves:Vec::new(),
            curve_index:Vec::new(),
            curve_index_offset:crv_offset,
            surfaces:Vec::new(),
            surface_index:Vec::new(),
            surface_index_offset:srf_offset,
            agents:Vec::new(),
            agent_index:Vec::new(),
            agent_index_offset:agn_offset,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_storage_offset(storage:&DataStorage)->Self{
        DataStorage{
            objects:Vec::new(),
            object_index:Vec::new(),
            object_index_offset:storage.object_index.len(),
            points:Vec::new(),
            point_index:Vec::new(),
            point_index_offset:storage.point_index.len(),
            curves:Vec::new(),
            curve_index:Vec::new(),
            curve_index_offset:storage.curve_index.len(),
            surfaces:Vec::new(),
            surface_index:Vec::new(),
            surface_index_offset:storage.surface_index.len(),
            agents:Vec::new(),
            agent_index:Vec::new(),
            agent_index_offset:storage.agent_index.len()
        }
    }

    #[allow(dead_code)]
    pub fn object_num(&self)->usize{
        self.objects.len()
    }
    #[allow(dead_code)]
    pub fn point_num(&self)->usize{
        self.points.len()
    }
    #[allow(dead_code)]
    pub fn curve_num(&self)->usize{
        self.curves.len()
    }
    #[allow(dead_code)]
    pub fn surface_num(&self)->usize{
        self.surfaces.len()
    }
    #[allow(dead_code)]
    pub fn agent_num(&self)->usize{
        self.agents.len()
    }

    #[allow(dead_code)]
    pub fn add_object(&mut self, object:Box<dyn Object>)->usize{
        self.objects.push(object);
        self.object_index.push((self.objects.len()-1) as i32);
        let id = self.object_index.len()-1+self.object_index_offset;
        //object.set_id(id);
        id
    }
    #[allow(dead_code)]
    pub fn add_point(&mut self, point:Box<Point>)->usize{
        let id = self.point_index.len()+self.point_index_offset;
        //point.id = id;
        self.points.push(point);
        self.point_index.push((self.points.len()-1) as i32);
        //let id = self.point_index.len()-1+self.point_index_offset;
        id
    }
    #[allow(dead_code)]
    pub fn add_curve(&mut self, curve:Box<Curve>)->usize{
        let id = self.curve_index.len()+self.curve_index_offset;
        //log_1(&JsValue::from(format!("DataStorage::add_curve id = {}", id)));
        self.curves.push(curve);
        self.curve_index.push((self.curves.len()-1) as i32);
        //curve.id = id;
        id
    }
    #[allow(dead_code)]
    pub fn add_surface(&mut self, surface:Box<Surface>)->usize{
        self.surfaces.push(surface);
        self.surface_index.push((self.surfaces.len()-1) as i32);
        let id = self.surface_index.len()-1+self.surface_index_offset;
        //surface.id = id;
        id
    }
    #[allow(dead_code)]
    pub fn add_agent(&mut self, mut agent:Box<Agent>)->usize{
        let id = self.agent_index.len()+self.agent_index_offset;
        //log_1(&JsValue::from(format!("DataStorage::add_agent id = {}", id)));

        agent.set_id(id as i32);
        self.agents.push(agent);
        self.agent_index.push((self.agents.len()-1) as i32);
        id
    }

    #[allow(dead_code)]
    pub fn sort_object_index(&mut self){ // descending sort
        self.object_index.sort_by(|a, b| b.cmp(&a));
    }
    #[allow(dead_code)]
    pub fn sort_point_index(&mut self){ // descending sort
        self.point_index.sort_by(|a, b| b.cmp(&a));
    }
    #[allow(dead_code)]
    pub fn sort_curve_index(&mut self){ // descending sort
        self.curve_index.sort_by(|a, b| b.cmp(&a));
    }
    #[allow(dead_code)]
    pub fn sort_surface_index(&mut self){ // descending sort
        self.surface_index.sort_by(|a, b| b.cmp(&a));
    }
    #[allow(dead_code)]
    pub fn sort_agent_index(&mut self){ // descending sort
        self.agent_index.sort_by(|a, b| b.cmp(&a));
    }


    /*
    pub fn add_storage(&mut self, storage:&mut DataStorage){
        self.objects.append(&mut storage.objects);
        self.points.append(&mut storage.points);
        self.curves.append(&mut storage.curves);
        self.surfaces.append(&mut storage.surfaces);
        self.agents.append(&mut storage.agents);
    }
    */

    #[allow(dead_code)]
    pub fn delete_object(&mut self, index:i32){
        if index>=0 && index < self.object_index.len() as i32 && self.object_index[index as usize] >= 0{
            self.objects.remove(self.object_index[index as usize] as usize);
            self.object_index[index as usize] = -1;
            for i in ((index+1) as usize)..self.object_index.len(){ // index of deleted object stays
                self.object_index[i] -= 1;
            }
        }
        else{
            log_1(&JsValue::from(format!("at delete_object: ERROR: object index at {} is invalid!", index)));
        }
        // not implemented yet
        //let idx = self.objects.iter().position(|o| o.as_ref()==object.as_ref());
        //if !idx.is_none(){ self.objects.remove(idx.unwrap()); }
    }
    #[allow(dead_code)]
    pub fn delete_point(&mut self, index:i32){
        if index>=0 && index < self.point_index.len() as i32 && self.point_index[index as usize] >= 0{
            self.points.remove(self.point_index[index as usize] as usize);
            self.point_index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.point_index.len(){ // index of deleted object stays
                self.point_index[i] -= 1;
            }
        }
        else{
            log_1(&JsValue::from(format!("at delete_point: ERROR: point index at {} is invalid!", index)));
        }

        /*
        let idx = self.points.iter().position(|p| p.as_ref()==point.as_ref());
        if !idx.is_none(){
            self.points.remove(idx.unwrap());
        }*/
    }
    #[allow(dead_code)]
    pub fn delete_curve(&mut self, index:i32){
        if index>=0 && index < self.curve_index.len() as i32 && self.curve_index[index as usize] >= 0{
            self.curves.remove(self.curve_index[index as usize] as usize);
            self.curve_index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.curve_index.len(){ // index of deleted object stays
                self.curve_index[i] -= 1;
            }
        }
        else{
            log_1(&JsValue::from(format!("at delete_curve: ERROR: curve index at {} is negative!", index)));
        }
    }
    #[allow(dead_code)]
    pub fn delete_surface(&mut self, index:i32){
        if index>=0 && index < self.surface_index.len() as i32 && self.surface_index[index as usize] >= 0{
            self.surfaces.remove(self.surface_index[index as usize] as usize);
            self.surface_index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.surface_index.len(){ // index of deleted object stays
                self.surface_index[i] -= 1;
            }
        }
        else{
            log_1(&JsValue::from(format!("at delete_surface: ERROR: surface index at {} is negative!", index)));
        }
    }
    #[allow(dead_code)]
    pub fn delete_agent(&mut self, index:i32){
        //log_1(&JsValue::from(format!("at delete_agent: index at {} ", index)));

        if index>=0 && index < self.agent_index.len() as i32 && self.agent_index[index as usize] >= 0{
            self.agents.remove(self.agent_index[index as usize] as usize);
            self.agent_index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.agent_index.len(){ // index of deleted object stays
                self.agent_index[i] -= 1;
            }
        }
        else{
            log_1(&JsValue::from(format!("at delete_agent: ERROR: agent index at {} is negative!", index)));
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.objects.clear();
        self.points.clear();
        self.curves.clear();
        self.surfaces.clear();
        self.agents.clear();
    }

}

pub struct Server{
    storage:DataStorage,
    //storage:Storage,

    //graphic_server: GraphicServer/*<'a>*/,
    //dynamic_server: DynamicServer/*<'a>*/,
    glserver: WebGlServer/*<'a>*/,
    time:i64,
    duration:i64,
    max_agent_limit:i32,
}

impl Server{
    #[allow(dead_code)]
    pub fn new(width:f32, height:f32) -> Self{
        Server{
            storage:DataStorage::new(),
            //storage:Storage::new(),
            glserver:WebGlServer::new(width,height),
            time: 0,
            duration : -1,
            max_agent_limit:-1
        }
    }
    /*
    #[allow(dead_code)]
    pub fn add_object(&mut self, object:Box<dyn Object>)->usize{
        self.storage.add_object(object)
    }
    */
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
    pub fn add_agent(&mut self, agent:Box<Agent>)->usize{
    //pub fn add_agent(&mut self, agent:Box<dyn AgentTrait>)->usize{
        if self.max_agent_limit >= 0 && self.storage.agents.len() as i32 >= self.max_agent_limit{
            return 0; // shouldn't this be -1?
        }
        let id = self.storage.add_agent(agent);
        //agent.set_id(id);
        id
    }

    #[allow(dead_code)]
    pub fn delete_data(&mut self, mut deleting_data: DataStorage){
    //pub fn delete_data(&mut self, mut deleting_data: Storage){
        if deleting_data.point_index.len() > 0{
        //if deleting_data.points.index_len() > 0{
            deleting_data.sort_point_index();
            for i in 0..deleting_data.point_index.len(){
            //for i in 0..deleting_data.points.index_len(){
                //log_1(&JsValue::from(format!("Server::delete_data: deleting point index {}", deleting_data.point_index[i]))); //
                self.storage.delete_point(deleting_data.point_index[i] );
                //self.storage.delete_point(deleting_data.points.idx(i) );
            }
            self.glserver.clear_points();
            // re-add glserver points
            for i in 0..self.storage.points.len(){
                self.glserver.add_point(Box::new(GlPoint::from_point(&self.storage.points[i])));
                //self.glserver.add_point(Box::new(GlPoint::from_point(&self.storage.points.get(i))));
            }
        }

        if deleting_data.curve_index.len() > 0{
        //if deleting_data.curves.index_len() > 0{
            deleting_data.sort_curve_index();
            for i in 0..deleting_data.curve_index.len(){
            //for i in 0..deleting_data.curves.index_len(){
                //log_1(&JsValue::from(format!("Server::delete_data: deleting curve index {}", deleting_data.curve_index[i]))); //
                self.storage.delete_curve(deleting_data.curve_index[i] );
                //self.storage.delete_curve(deleting_data.curves.idx(i) );
            }
            self.glserver.clear_lines();
            // re-add glserver curve
            for i in 0..self.storage.curves.len(){
                self.glserver.add_line(Box::new(GlLine::from_curve(&self.storage.curves[i])));
                //self.glserver.add_line(Box::new(GlLine::from_curve(&self.storage.curves.get(i))));
            }
        }

        if deleting_data.surface_index.len() > 0{
        //if deleting_data.surfaces.index_len() > 0{
            deleting_data.sort_surface_index();
            for i in 0..deleting_data.surface_index.len(){
            //for i in 0..deleting_data.surfaces.index_len(){
                //log_1(&JsValue::from(format!("Server::delete_date: deleting surface index {}", deleting_data.surface_index[i]))); //
                self.storage.delete_surface(deleting_data.surface_index[i] );
                //self.storage.delete_surface(deleting_data.surfaces.idx(i) );
            }
            self.glserver.clear_surfaces();
            // re-add glserver curve
            for i in 0..self.storage.surfaces.len(){
                self.glserver.add_surface(Box::new(GlSurface::from_surface(&self.storage.surfaces[i])));
                //self.glserver.add_surface(Box::new(GlSurface::from_surface(&self.storage.surfaces.get(i))));
            }
        }

        if deleting_data.agent_index.len() > 0{
        //if deleting_data.agents.index_len() > 0{
            deleting_data.sort_agent_index();
            for i in 0..deleting_data.agent_index.len(){
            //for i in 0..deleting_data.agents.index_len(){
                //log_1(&JsValue::from(format!("Server::delete_date: deleting agent index {}", deleting_data.agent_index[i]))); //
                self.storage.delete_agent(deleting_data.agent_index[i] );
                //self.storage.delete_agent(deleting_data.agents.idx(i) );
            }
        }

        //log_1(&JsValue::from(format!("1 adding point num{}", manager.adding_data.points.len())));

    }

    #[allow(dead_code)]
    pub fn add_data(&mut self, adding_data: DataStorage){
    //pub fn add_data(&mut self, adding_data: Storage){
        //log_1(&JsValue::from(format!("Server::add_data"))); //
        //for o in adding_data.objects{
        //    self.add_object(o);
        //}
        for p in adding_data.points{
            self.add_point(p);
        }
        for c in adding_data.curves{
            self.add_curve(c);
        }
        for s in adding_data.surfaces{
            self.add_surface(s);
        }
        for a in adding_data.agents{
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
    pub fn init(&/*'a*/ mut self){
        //self.graphic_server.init();
        //self.dynamic_server.init();
        self.glserver.init();
    }
    #[allow(dead_code)]
    pub fn draw(&mut self){
        //self.graphic_server.draw();
        //self.dynamic_server.update();
        self.glserver.draw();

        //let mut agents = &mut self.agents;
        //for a in agents{
        //    a.update(self);
        //}
        //let mut mgr = DataManager::new();

        if self.duration < 0 || self.time < self.duration{

            let mut mgr = DataManager::new_with_storage_offset(&self.storage, self.time.clone());
            //let mut mgr = Manager::new_with_storage(&self.storage, self.time.clone());

            if self.storage.agents.len() > 0{
                log_1(&JsValue::from(format!("Server::agents.len()={}", self.storage.agents.len())));
            }

            if self.max_agent_limit < 0 || (self.storage.agents.len() as i32) < self.max_agent_limit {
                let mut agents_copy : Vec<Box<Agent>> = Vec::new();
                //let mut agents_copy : Vec<Box<dyn AgentTrait>> = Vec::new();
                for i in 0..self.storage.agents.len(){
                    agents_copy.push(Box::new((*self.storage.agents[i]).clone()));
                    //agents_copy.push(Box::new(*self.storage.agents.get(i)));
                }

                for i in 0..self.storage.agents.len(){
                    //for j in 0..self.storage.agents.len(){
                    //    if i!=j{
                            //self.storage.agents[i].interact(agents[j], &mut mgr);
                            Agent::interact(&mut self.storage.agents[i], &agents_copy, &mut mgr)
                            //AgentTrait::interact(&mut self.storage.agents.get(i), &agents_copy, &mut mgr)
                    //   }
                    //}
                }

                //log_1(&JsValue::from(format!("Server: end of interact")));

                for i in 0..self.storage.agents.len(){
                    self.storage.agents[i].update(&mut mgr);
                    //self.storage.agents.get(i).update(&mut mgr);
                }

                //log_1(&JsValue::from(format!("Server: end of update")));

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


pub struct Server2{
    storage:Storage,
    glserver: WebGlServer,
    time:i64,
    duration:i64,
    max_agent_limit:i32,
}

impl Server2{
    #[allow(dead_code)]
    pub fn new(width:f32, height:f32) -> Self{
        Server2{
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
    pub fn add_agent(&mut self, agent:Box<dyn AgentTrait>)->usize{
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
    pub fn init(&/*'a*/ mut self){
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
                let mut agents_copy : Vec<Box<dyn AgentTrait>> = Vec::new();
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
                    self.storage.agents.get(i).info().time += 1;
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
//    attribute: /*&'a*/ Attribute/*<'a>*/,
//}

pub trait Object{
    //fn init(&dyn self, server: &mut Server){ server.add_object(Box::new(self)); }
    fn draw(&mut self);
//    fn interact(&mut self, objects:&Vec<Box<dyn Object>>);
//    fn update(&mut self);
    //fn attr(&self)->&mut Attribute;
}


//pub struct Graphic{}
//impl Graphic{}
//pub struct Dynamic{}
//impl Dynamic{}

#[derive(Debug, Clone)]
pub struct Attribute/*<'a>*/{
    pub id: i32,
    pub name: String,
    //layer: &'a Layer<'a>,
    pub layer_index: u32,
    pub color: /*'a*/ Color,
    pub stroke: /*'a*/ Color,
    pub size: f32,
    pub weight: f32,
    //pub mateiral: Material
    pub visible: bool,
}

impl Attribute{
    #[allow(dead_code)]
    pub fn default()->Self{
        Attribute{
            id:-1,name: String::from("default"), layer_index:0,
            color: Color::new(0.5,0.5,0.5,1.0), stroke: Color::new(0.5,0.5,0.5,1.0),
            size: 5.0, weight: 1.0, visible:true
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, attr:&Attribute)->&mut Self{
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
pub struct Layer/*<'a>*/{
//    object: Object,
//    objects: Vec<&/*'a*/ Object/*<'a>*/> // containing objects
}


/************************
* Geometry    pub fn add_object(&mut self, object:Box<dyn Object>){
        self.objects.push(object);
    }
 Objects
************************/
