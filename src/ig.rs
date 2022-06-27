use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::console::log_1;
use wasm_bindgen::JsCast;
use web_sys::*;
use std::fmt;

/****************************
* constant
*****************************/

pub const TOLERANCE : f64 = 0.001;
pub const ANGLE_TOLERANCE : f64 = std::f64::consts::PI/1000.0;

/*
pub struct TestServer{
    objects: Vec<f64>,
    time:u64,
}
impl TestServer{
    pub fn new(width:f32, height:f32) -> Self{
        TestServer{
            objects: Vec::new(),
            time: 0
        }
    }
    pub fn init(&mut self){
    }
    pub fn draw(&mut self){
        self.time += 1;
    }
}
*/

pub struct DataManager{
    pub adding_data: DataStorage,
    pub deleting_data: DataStorage,
    pub time: u64,
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
    pub fn new_with_storage_offset(storage:&DataStorage, time:u64)->Self{
        DataManager{adding_data:DataStorage::new_with_storage_offset(storage), deleting_data:DataStorage::new(), time}
    }

    pub fn add_object(&mut self, object:Box<dyn Object>)->usize{
        self.adding_data.add_object(object)
    }
    pub fn add_point(&mut self, point:Box<Point>)->usize{
        self.adding_data.add_point(point)
    }
    pub fn add_curve(&mut self, curve:Box<Curve>)->usize{
        self.adding_data.add_curve(curve)
    }
    pub fn add_surface(&mut self, surface:Box<Surface>)->usize{
        self.adding_data.add_surface(surface)
    }
    pub fn add_agent(&mut self, agent:Box<Agent>)->usize{
        //web_sys::console::log_1(&JsValue::from(format!("DataManager::add_agent")));
        self.adding_data.add_agent(agent)
    }

    pub fn delete_object(&mut self, index:i32){
        self.deleting_data.object_index.push(index);
    }
    pub fn delete_point(&mut self, index:i32){
        self.deleting_data.point_index.push(index);
    }
    pub fn delete_point_geom(&mut self, point:Point){
        self.deleting_data.point_index.push(point.id as i32);
    }

    pub fn delete_curve(&mut self, index:i32){
        self.deleting_data.curve_index.push(index);
    }
    pub fn delete_surface(&mut self, index:i32){
        self.deleting_data.surface_index.push(index);
    }
    pub fn delete_agent(&mut self, index:i32){
        self.deleting_data.agent_index.push(index);
    }

    /*
    pub fn sort_deleting_data(&mut self){
        if self.deleting_data.object_index.len()>0{
            self.sort_deleting_object_index();
        }
        if self.deleting_data.point_index.len()>0{
            self.sort_deleting_point_index();
        }
        if self.deleting_data.curve_index.len()>0{
            self.sort_deleting_curve_index();
        }
        if self.deleting_data.surface_index.len()>0{
            self.sort_deleting_surface_index();
        }
        if self.deleting_data.agent_index.len()>0{
            self.sort_deleting_agent_index();
        }
    }

    pub fn sort_deleting_object_index(&mut self){
        // soft IDs from large to small to delete from the extend_from_slice
        self.deleting_data.object_index.sort_by(|a, b| b.cmp(&a));
    }
//    pub fn sort_deleting_points(&mut self){
//        // soft IDs from large to small to delete from the extend_from_slice
//        self.deleting_data.points.sort_by(|a, b| b.id.cmp(&a.id));
//    }

    pub fn sort_deleting_point_index(&mut self){
        // soft IDs from large to small to delete from the extend_from_slice
        self.deleting_data.point_index.sort_by(|a, b| b.cmp(&a));
    }

//    pub fn sort_deleting_curves(&mut self){
//        // soft IDs from large to small to delete from the extend_from_slice
//        self.deleting_data.curves.sort_by(|a, b| b.id.cmp(&a.id));
//    }

    pub fn sort_deleting_curve_index(&mut self){
        // soft IDs from large to small to delete from the extend_from_slice
        self.deleting_data.curve_index.sort_by(|a, b| b.cmp(&a));
    }
//    pub fn sort_deleting_surfaces(&mut self){
//        // soft IDs from large to small to delete from the extend_from_slice
//        self.deleting_data.surfaces.sort_by(|a, b| b.id.cmp(&a.id));
//    }
    pub fn sort_deleting_surface_index(&mut self){
        // soft IDs from large to small to delete from the extend_from_slice
        self.deleting_data.surface_index.sort_by(|a, b| b.cmp(&a));
    }
    pub fn sort_deleting_agent_index(&mut self){
        // soft IDs from large to small to delete from the extend_from_slice
        self.deleting_data.agent_index.sort_by(|a, b| b.cmp(&a));
    }
    */
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

    pub fn new_with_offset(objOffset:usize, ptOffset:usize, crvOffset:usize, srfOffset:usize, agnOffset:usize )->Self{
        DataStorage{
            objects:Vec::new(),
            object_index:Vec::new(),
            object_index_offset:objOffset,
            points:Vec::new(),
            point_index:Vec::new(),
            point_index_offset:ptOffset,
            curves:Vec::new(),
            curve_index:Vec::new(),
            curve_index_offset:crvOffset,
            surfaces:Vec::new(),
            surface_index:Vec::new(),
            surface_index_offset:srfOffset,
            agents:Vec::new(),
            agent_index:Vec::new(),
            agent_index_offset:agnOffset,
        }
    }

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

    pub fn object_num(&self)->usize{
        self.objects.len()
    }
    pub fn point_num(&self)->usize{
        self.points.len()
    }
    pub fn curve_num(&self)->usize{
        self.curves.len()
    }
    pub fn surface_num(&self)->usize{
        self.surfaces.len()
    }
    pub fn agent_num(&self)->usize{
        self.agents.len()
    }

    pub fn add_object(&mut self, object:Box<dyn Object>)->usize{
        self.objects.push(object);
        self.object_index.push((self.objects.len()-1) as i32);
        let id = self.object_index.len()-1+self.object_index_offset;
        //object.set_id(id);
        id
    }
    pub fn add_point(&mut self, point:Box<Point>)->usize{
        let id = self.point_index.len()+self.point_index_offset;
        //point.id = id;
        self.points.push(point);
        self.point_index.push((self.points.len()-1) as i32);
        //let id = self.point_index.len()-1+self.point_index_offset;
        id
    }
    pub fn add_curve(&mut self, curve:Box<Curve>)->usize{
        let id = self.curve_index.len()+self.curve_index_offset;
        //web_sys::console::log_1(&JsValue::from(format!("DataStorage::add_curve id = {}", id)));
        self.curves.push(curve);
        self.curve_index.push((self.curves.len()-1) as i32);
        //curve.id = id;
        id
    }
    pub fn add_surface(&mut self, surface:Box<Surface>)->usize{
        self.surfaces.push(surface);
        self.surface_index.push((self.surfaces.len()-1) as i32);
        let id = self.surface_index.len()-1+self.surface_index_offset;
        //surface.id = id;
        id
    }
    pub fn add_agent(&mut self, mut agent:Box<Agent>)->usize{
        let id = self.agent_index.len()+self.agent_index_offset;
        //web_sys::console::log_1(&JsValue::from(format!("DataStorage::add_agent id = {}", id)));

        agent.set_id(id as i32);
        self.agents.push(agent);
        self.agent_index.push((self.agents.len()-1) as i32);
        id
    }

    pub fn sort_object_index(&mut self){ // descending sort
        self.object_index.sort_by(|a, b| b.cmp(&a));
    }
    pub fn sort_point_index(&mut self){ // descending sort
        self.point_index.sort_by(|a, b| b.cmp(&a));
    }
    pub fn sort_curve_index(&mut self){ // descending sort
        self.curve_index.sort_by(|a, b| b.cmp(&a));
    }
    pub fn sort_surface_index(&mut self){ // descending sort
        self.surface_index.sort_by(|a, b| b.cmp(&a));
    }
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

    pub fn delete_object(&mut self, index:i32){
        if index>=0 && index < self.object_index.len() as i32 && self.object_index[index as usize] >= 0{
            self.objects.remove(self.object_index[index as usize] as usize);
            self.object_index[index as usize] = -1;
            for i in ((index+1) as usize)..self.object_index.len(){ // index of deleted object stays
                self.object_index[i] -= 1;
            }
        }
        else{
            web_sys::console::log_1(&JsValue::from(format!("at delete_object: ERROR: object index at {} is invalid!", index)));
        }
        // not implemented yet
        //let idx = self.objects.iter().position(|o| o.as_ref()==object.as_ref());
        //if !idx.is_none(){ self.objects.remove(idx.unwrap()); }
    }
    pub fn delete_point(&mut self, index:i32){
        if index>=0 && index < self.point_index.len() as i32 && self.point_index[index as usize] >= 0{
            self.points.remove(self.point_index[index as usize] as usize);
            self.point_index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.point_index.len(){ // index of deleted object stays
                self.point_index[i] -= 1;
            }
        }
        else{
            web_sys::console::log_1(&JsValue::from(format!("at delete_point: ERROR: point index at {} is invalid!", index)));
        }

        /*
        let idx = self.points.iter().position(|p| p.as_ref()==point.as_ref());
        if !idx.is_none(){
            self.points.remove(idx.unwrap());
        }*/
    }
    pub fn delete_curve(&mut self, index:i32){
        if index>=0 && index < self.curve_index.len() as i32 && self.curve_index[index as usize] >= 0{
            self.curves.remove(self.curve_index[index as usize] as usize);
            self.curve_index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.curve_index.len(){ // index of deleted object stays
                self.curve_index[i] -= 1;
            }
        }
        else{
            web_sys::console::log_1(&JsValue::from(format!("at delete_curve: ERROR: curve index at {} is negative!", index)));
        }
    }
    pub fn delete_surface(&mut self, index:i32){
        if index>=0 && index < self.surface_index.len() as i32 && self.surface_index[index as usize] >= 0{
            self.surfaces.remove(self.surface_index[index as usize] as usize);
            self.surface_index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.surface_index.len(){ // index of deleted object stays
                self.surface_index[i] -= 1;
            }
        }
        else{
            web_sys::console::log_1(&JsValue::from(format!("at delete_surface: ERROR: surface index at {} is negative!", index)));
        }
    }
    pub fn delete_agent(&mut self, index:i32){
        //web_sys::console::log_1(&JsValue::from(format!("at delete_agent: index at {} ", index)));

        if index>=0 && index < self.agent_index.len() as i32 && self.agent_index[index as usize] >= 0{
            self.agents.remove(self.agent_index[index as usize] as usize);
            self.agent_index[index as usize] = -1;
            for i in ((index+1)as usize) .. self.agent_index.len(){ // index of deleted object stays
                self.agent_index[i] -= 1;
            }
        }
        else{
            web_sys::console::log_1(&JsValue::from(format!("at delete_agent: ERROR: agent index at {} is negative!", index)));
        }
    }

    pub fn clear(&mut self){
        self.objects.clear();
        self.points.clear();
        self.curves.clear();
        self.surfaces.clear();
        self.agents.clear();
    }

}

pub struct Server/*<'a>*/{
    //objects: Vec<e&/*'a*/ mut Object/*<'a>*/>,
    /*
    objects: Vec<Box<dyn Object>>,
    points: Vec<Box<Point>>,
    curves: Vec<Box<Curve>>,
    surfaces: Vec<Box<Surface>>,
    agents: Vec<Box<dyn Agent>>,
    */
    storage:DataStorage,
    //objects: Vec<&impl ObjectTrait>,
    //graphic_server: GraphicServer/*<'a>*/,
    //dynamic_server: DynamicServer/*<'a>*/,
    glserver: WebGlServer/*<'a>*/,
    time:u64,
}

impl/*<'a>*/ Server/*<'a>*/{
    pub fn new(width:f32, height:f32) -> Self{
        Server{
            //objects: Vec::new(),
            storage:DataStorage::new(),
            //graphic_server: GraphicServer::new(width,height),
            //dynamic_server: DynamicServer::new(),
            glserver:WebGlServer::new(width,height),
            time: 0
        }
    }
    //pub fn add(&mut self, object:&/*'a*/ mut Object/*<'a>*/){
    pub fn add_object(&mut self, object:Box<dyn Object>)->usize{
        //self.objects.push(object);
        self.storage.add_object(object)
    }
    pub fn add_point(&mut self, point:Box<Point>)->usize{
//        web_sys::console::log_1(&JsValue::from(format!("added point {},{},{}", &point.pos.x,&point.pos.y,&point.pos.z)));
        self.glserver.add_point(Box::new(GlPoint::from_point(&point)));
        let id = self.storage.add_point(point);
        //point.id = id;
//        web_sys::console::log_1(&JsValue::from(format!("point added at {}", id)));
        id
    }
    pub fn add_curve(&mut self, curve:Box<Curve>)->usize{
        //curve.id = self.storage.curve_num();
        self.glserver.add_line(Box::new(GlLine::from_curve(&curve)));
        let id = self.storage.add_curve(curve);
        //web_sys::console::log_1(&JsValue::from(format!("Server::add_curve id = {}", id))); //

        //curve.id = id;
        id
    }
    pub fn add_surface(&mut self, surface:Box<Surface>)->usize{
        //surface.id = self.storage.surface_num();
        self.glserver.add_surface(Box::new(GlSurface::from_surface(&surface)));
        let id = self.storage.add_surface(surface);
        //surface.id = id;
        id
    }
    pub fn add_agent(&mut self, agent:Box<Agent>)->usize{
        //agent.set_id( self.storage.agent_num());
        let id = self.storage.add_agent(agent);
        //agent.set_id(id);
        //web_sys::console::log_1(&JsValue::from(format!("Server::add_agent id = {}", id)));

        id
    }

    pub fn delete_data(&mut self, mut deleting_data: DataStorage){
        if deleting_data.point_index.len() > 0{
            deleting_data.sort_point_index();
            for i in 0..deleting_data.point_index.len(){
                //web_sys::console::log_1(&JsValue::from(format!("Server::delete_data: deleting point index {}", deleting_data.point_index[i]))); //
                self.storage.delete_point(deleting_data.point_index[i] );
            }
            self.glserver.clear_points();
            // re-add glserver points
            for i in 0..self.storage.points.len(){
                self.glserver.add_point(Box::new(GlPoint::from_point(&self.storage.points[i])));
            }
        }

        if deleting_data.curve_index.len() > 0{
            deleting_data.sort_curve_index();
            for i in 0..deleting_data.curve_index.len(){
                //web_sys::console::log_1(&JsValue::from(format!("Server::delete_data: deleting curve index {}", deleting_data.curve_index[i]))); //
                self.storage.delete_curve(deleting_data.curve_index[i] );
            }
            self.glserver.clear_lines();
            // re-add glserver curve
            for i in 0..self.storage.curves.len(){
                self.glserver.add_line(Box::new(GlLine::from_curve(&self.storage.curves[i])));
            }
        }

        if deleting_data.surface_index.len() > 0{
            deleting_data.sort_surface_index();
            for i in 0..deleting_data.surface_index.len(){
                //web_sys::console::log_1(&JsValue::from(format!("Server::delete_date: deleting surface index {}", deleting_data.surface_index[i]))); //
                self.storage.delete_surface(deleting_data.surface_index[i] );
            }
            self.glserver.clear_surfaces();
            // re-add glserver curve
            for i in 0..self.storage.surfaces.len(){
                self.glserver.add_surface(Box::new(GlSurface::from_surface(&self.storage.surfaces[i])));
            }
        }

        if deleting_data.agent_index.len() > 0{
            deleting_data.sort_agent_index();
            for i in 0..deleting_data.agent_index.len(){
                //web_sys::console::log_1(&JsValue::from(format!("Server::delete_date: deleting agent index {}", deleting_data.agent_index[i]))); //
                self.storage.delete_agent(deleting_data.agent_index[i] );
            }
        }

        //web_sys::console::log_1(&JsValue::from(format!("1 adding point num{}", manager.adding_data.points.len())));

    }

    pub fn add_data(&mut self, adding_data: DataStorage){

        //web_sys::console::log_1(&JsValue::from(format!("Server::add_data"))); //

        for o in adding_data.objects{
            self.add_object(o);
        }

        //for i in 0..manager.adding_data.points.len(){
        for p in adding_data.points{
            //self.glserver.add_point(Box::new(GlPoint::from_point(&manager.adding_data.points[i)));
            //self.storage.add_point(manager.adding_data.points[i]);
            //self.glserver.add_point(Box::new(GlPoint::from_point(p)));
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


        //self.storage.add_storage(&mut manager.adding_data);
    }

    pub fn init(&/*'a*/ mut self){
        //self.graphic_server.init();
        //self.dynamic_server.init();
        self.glserver.init();
    }
    pub fn draw(&mut self){
        //self.graphic_server.draw();
        //self.dynamic_server.update();
        self.glserver.draw();

        //let mut agents = &mut self.agents;
        //for a in agents{
        //    a.update(self);
        //}
        //let mut mgr = DataManager::new();

        let mut mgr = DataManager::new_with_storage_offset(&self.storage, self.time.clone());


        if self.storage.agents.len() > 0{
            web_sys::console::log_1(&JsValue::from(format!("Server::agents.len()={}", self.storage.agents.len())));
        }
        
        let mut agentsCopy : Vec<Box<Agent>> = Vec::new();
        for i in 0..self.storage.agents.len(){
            agentsCopy.push(Box::new((*self.storage.agents[i]).clone()));
        }
        /*
        let magents = &mut self.storage.agents;

        //for i in 0..self.storage.agents.len(){
        for i in 0..magents.len(){
            //self.storage.agents[i].interact(agents, &mut mgr);
            magents[i].interact(&agents, &mut mgr);
        }
        */


        for i in 0..self.storage.agents.len(){
            for j in 0..self.storage.agents.len(){
                if i!=j{
                    //self.storage.agents[i].interact(agents[j], &mut mgr);
                    Agent::interact(&mut self.storage.agents[i], &agentsCopy, &mut mgr)
                }
            }
        }

        //web_sys::console::log_1(&JsValue::from(format!("Server: end of interact")));

        for i in 0..self.storage.agents.len(){
            self.storage.agents[i].update(&mut mgr);
        }

        //web_sys::console::log_1(&JsValue::from(format!("Server: end of update")));

        //web_sys::console::log_1(&JsValue::from(format!("0 adding point num{}", mgr.adding_data.points.len())));
        self.delete_data(mgr.deleting_data);

        //web_sys::console::log_1(&JsValue::from(format!("2 adding point num{}", mgr.adding_data.points.len())));

        self.add_data(mgr.adding_data);
        //for i in 0..mgr.adding_data.points.len(){
        //for p in mgr.adding_data.points{
            //self.glserver.add_point(Box::new(GlPoint::from_point(&manager.adding_data.points[i)));
            //self.storage.add_point(manager.adding_data.points[i]);
            //self.glserver.add_point(Box::new(GlPoint::from_point(p)));
        //    self.storage.add_point(p);
        //}


        self.time += 1;
    }


    pub fn time(&self)->u64{
        //self.dynamic_server.time()
        self.time
    }
}

pub struct GraphicServer/*<'a>*/{
    graphics3d: Vec<Graphic>,
    glserver: WebGlServer/*<'a>*/,
    //server: Option<&'a Server<'a>>,
}

impl/*<'a>*/ GraphicServer/*<'a>*/{
    pub fn new(width:f32,height:f32) -> Self{
        GraphicServer{
            graphics3d: Vec::new(),
            glserver: WebGlServer::new(width, height),
            //server:None
        }
    }
    pub fn init(&mut self/*, serv:&'a Server<'a> */){
        //self.server = Some(serv);
        self.glserver.init(/*serv*/);
    }
    pub fn draw(&mut self){
        self.glserver.draw();
    }
}

pub struct DynamicServer/*<'a>*/{
    dynamics: Vec<Dynamic>,
    adding_dynamics: Vec<Dynamic>,
    removing_dynamics: Vec<Dynamic>,
    time: u64,
    //server: Option<&'a Server<'a>>,
}

impl/*<'a>*/ DynamicServer/*<'a>*/{
    pub fn new()->Self{
        DynamicServer{
            dynamics: Vec::new(),
            adding_dynamics: Vec::new(),
            removing_dynamics: Vec::new(),
            time: 0,
            //server:None,
        }
    }
    pub fn init(&mut self/*, serv: &'a Server<'a>*/){
        //self.server=Some(serv);
    }
    pub fn update(&mut self){

    }
    pub fn time(&self)->u64{
        self.time
    }
}




pub struct GlUniformMatrix4{
    mat: Matrix4,
    name: String,
    location: Option<WebGlUniformLocation>,
}
impl GlUniformMatrix4{
    pub fn new(m:Matrix4, s:&str)->Self{
        GlUniformMatrix4{ mat:m, name:String::from(s), location:None }
    }
    pub fn set_matrix(&mut self, mat:&Matrix4){
        self.mat.set_with_matrix4(mat);
    }
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
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
    pub fn new(v:Vec3, s:&str)->Self{
        GlUniformVec3{ vec:v, name:String::from(s), location:None }
    }
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
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
    pub fn new(c:Color, s:&str)->Self{
        GlUniformColor{ color:c, name:String::from(s), location:None }
    }
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        //web_sys::console::log_1(&JsValue::from(format!("GlUniformColor::set_location name: {}", self.name )));

        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
    pub fn set_uniform(&mut self, gl:&GL){
        //web_sys::console::log_1(&JsValue::from(format!("GlUniformColor::set_uniform color: {}", self.color )));

        gl.uniform4fv_with_f32_array(Some(&self.location.as_ref().unwrap()), &self.color.to_array());
    }
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
    pub fn new(i:i32, s:&str)->Self{
        GlUniformInt{ i, name:String::from(s), location:None }
    }
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
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
    pub fn new(x:f32, s:&str)->Self{
        GlUniformFloat{ x, name:String::from(s), location:None }
    }
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        self.location = Some(gl.get_uniform_location(&program, &self.name).unwrap());
    }
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
    pub fn new(s:&str)->Self{
        GlAttributeVec2{ name:String::from(s), location:0, stride:2 , buffer:None }
    }
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        let index = gl.get_attrib_location(&program, &self.name);
        if index<0{
            web_sys::console::log_1(&JsValue::from(format!("GlAttributeVec2::set_location:ERROR parameter \"{}\" is not found", self.name)));
        }
        self.location = index as u32;
    }
    //pub fn set_buffer(&mut self, gl:&GL, buffer:&WebGlBuffer){
    pub fn set_buffer(&mut self, gl:&GL,  data: &Vec<f32>) {
        self.create_vbo_vector(gl, data);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    pub fn activate(&mut self, gl:&GL){
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }

    pub fn unset_buffer(&mut self, gl:&GL){
        //gl.bind_buffer(GL::ARRAY_BUFFER, None);
        //gl.disable_vertex_attrib_array(self.location);
        gl.delete_buffer(Some(&self.buffer.as_ref().unwrap()));
    }
    //pub fn create_vbo_vector(&mut self gl: &GL, data: &Vec<f32>) -> Result<WebGlBuffer, String> {
    pub fn create_vbo_vector(&mut self, gl: &GL, data: &Vec<f32>) {
        self.buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        unsafe {
            let f32_array = js_sys::Float32Array::view(&(*data));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::STATIC_DRAW)
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
    pub fn new(s:&str)->Self{
        GlAttributeVec3{ name:String::from(s), location:0, stride:3, buffer:None }
    }
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        let index = gl.get_attrib_location(&program, &self.name);
        if index<0{
            web_sys::console::log_1(&JsValue::from(format!("GlAttributeVec3::set_location:ERROR parameter \"{}\" is not found", self.name)));
        }
        self.location = index as u32;
    }
    //pub fn set_buffer(&mut self, gl:&GL, buffer:&WebGlBuffer){
    pub fn set_buffer(&mut self, gl:&GL, data: &Vec<f32>){
        self.create_vbo_vector(gl, data);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    pub fn activate(&mut self, gl:&GL){
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }

    pub fn create_vbo_vector(&mut self, gl: &GL, data: &Vec<f32>) {
        self.buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        unsafe {
            let f32_array = js_sys::Float32Array::view(&(*data));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::STATIC_DRAW)
        }
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
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
    pub fn new(s:&str)->Self{
        GlAttributeColor{ name:String::from(s), location:0, stride:4, buffer:None }
    }
    pub fn set_location(&mut self, gl:&GL, program: &WebGlProgram){
        let index = gl.get_attrib_location(&program, &self.name);
        if index<0{
            web_sys::console::log_1(&JsValue::from(format!("GlAttributeColor::set_location:ERROR parameter \"{}\" is not found", self.name)));
        }
        self.location = index as u32;
    }
    //pub fn set_buffer(&mut self, gl:&GL, buffer:&WebGlBuffer){
    pub fn set_buffer(&mut self, gl:&GL, data: &Vec<f32>){
        self.create_vbo_vector(gl, data);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    pub fn activate(&mut self, gl:&GL){
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_with_i32(self.location, self.stride, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
    pub fn create_vbo_vector(&mut self, gl: &GL, data: &Vec<f32>) {
        self.buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        unsafe {
            let f32_array = js_sys::Float32Array::view(&(*data));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::STATIC_DRAW)
        }
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }
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
    pub fn new()->Self{
        GlIndex{ size:0, buffer:None }
    }
    pub fn set_buffer(&mut self, gl:&GL, data: &Vec<u16>){
        self.size = data.len() as i32;
        self.create_ibo_vector(gl, data);
    }
    pub fn activate(&mut self, gl:&GL){
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
    }
    pub fn create_ibo_vector(&mut self, gl: &GL, data: &Vec<u16>){
        self.buffer = gl.create_buffer();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));
        unsafe {
            let ui16_array = js_sys::Uint16Array::view(&(*data));
            gl.buffer_data_with_array_buffer_view( GL::ELEMENT_ARRAY_BUFFER, &ui16_array, GL::STATIC_DRAW );
        }
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
    }

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
    pub fn new(vertex_shader_filename: &str, fragment_shader_filename:&str)->Self{
        GlProgram{
            vertex_shader_file:String::from(vertex_shader_filename),
            fragment_shader_file:String::from(fragment_shader_filename),
            program:None,
        }
    }
    pub fn init(&mut self, gl:&GL) {
        //self.program = Some(gl.create_program().ok_or_else(|| String::from("Error creating program"))?);
        self.program = gl.create_program();

        let vert_shader = GlProgram::compile(&gl, GL::VERTEX_SHADER, &self.vertex_shader_file).unwrap();
        let frag_shader = GlProgram::compile(&gl, GL::FRAGMENT_SHADER, &self.fragment_shader_file).unwrap();
        gl.attach_shader(&self.program.as_ref().unwrap(), &vert_shader);
        gl.attach_shader(&self.program.as_ref().unwrap(), &frag_shader);
        gl.link_program(&self.program.as_ref().unwrap());
    }

    pub fn use_program(&self, gl:&GL){
        if gl.get_program_parameter(&self.program.as_ref().unwrap(), WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            gl.use_program(Some(&self.program.as_ref().unwrap()));
        }
        else {
            //Err(gl.get_program_info_log(&self.program.as_ref().unwrap()).unwrap_or_else(|| String::from("Unknown error creating program object")));
        }
    }

    pub fn compile(gl:&GL, shader_type: u32, source: &str)->Result<WebGlShader, String>{
        let shader = gl.create_shader(shader_type).ok_or_else(|| String::from("Error creating shader"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);
        if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false){
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
    pub fn new(pos:Vec3)->Self{
        GlPoint{pos, color:Color::new(0.5,0.5,0.5,1.0), size:5.0, index:0}
    }

    pub fn  from_point(pt:&Point)->Self{
        GlPoint{pos:pt.pos, color:pt.attr.color, size:pt.attr.size, index:0}
    }

    pub fn set_index(&mut self, start_index:i32){
        self.index = start_index;
    }

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
    pub fn new(pos:Vec<Vec3>)->Self{
        GlLine{pos, color:Color::new(0.5,0.5,0.5,1.0), index:0}
    }

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


    pub fn clr(&mut self, c:Color){
        self.color = c;
    }

    pub fn set_index(&mut self, start_index:i32){
        self.index = start_index;
    }

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
    pub fn  from_surface(surface:&Surface)->Self{
        let isoparmRatio = TESSELLATION_RESOLUTION as usize;

        let mut uval:Vec<f64> = Vec::new();
        let mut vval:Vec<f64> = Vec::new();

        if surface.udeg()==1{
            let num = surface.unum();
            for i in 0..num{ uval.push(surface.u(i,0.0)); }
        }
        else{
            let epnum = surface.uep_num();
            let num = (epnum-1)*isoparmRatio+1;
            for i in 0..epnum{
                for j in 0..isoparmRatio{
                    if i<epnum-1 || j==0{ uval.push(surface.u(i, j as f64 / isoparmRatio as f64)); }
                }
            }
        }

        if surface.vdeg()==1{
            let num = surface.vnum();
            for i in 0..num{ vval.push(surface.v(i,0.0)); }
        }
        else{
            let epnum = surface.vep_num();
            let num = (epnum-1)*isoparmRatio+1;
            for i in 0..epnum{
                for j in 0..isoparmRatio{
                    if i<epnum-1 || j==0 { vval.push(surface.v(i, j as f64 / isoparmRatio as f64)); }
                }
            }
        }

        if INSERT_POINT_ON_DEGREE1_TWISTED_SURFACE && surface.udeg()==1 && surface.vdeg()==1{
            let mut uinsert : Vec<bool> = Vec::new();
            let mut vinsert : Vec<bool> = Vec::new();
            let mut any_insert = false;
            for i in 0..uval.len()-1{ uinsert.push(false); }
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
                        for j in 0..isoparmRatio{
                            uval2.push( ( (uval[i+1]-uval[i])*j as f64)/isoparmRatio as f64 + uval[i]);
                        }
                    }
                }
                uval2.push(uval[uval.len()-1]);

                let mut vval2 : Vec<f64> = Vec::new();
                for i in 0..vval.len()-1{
                    vval2.push(vval[i]);
                    if vinsert[i]{
                        for j in 0..isoparmRatio{
                            vval2.push( ( (vval[i+1]-vval[i])*j as f64)/isoparmRatio as f64 + vval[i]);
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

                //web_sys::console::log_1(&JsValue::from(format!("GlSurface.from: pos[{}][{}] {} ", i, j, &p )));
                //web_sys::console::log_1(&JsValue::from(format!("GlSurface.from: nml[{}][{}] {} ", i, j, &n )));

            }
            pos.push(vpos);
            nml.push(vnml);
        }

        //pts.push(Vec::from([Vec3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0)]));
        //pts.push(Vec::from([Vec3::new(0.0,1.0,0.0), Vec3::new(1.0,1.0,0.0)]));
        //nml.push(Vec::from([Vec3::new(0.0,0.0,1.0), Vec3::new(0.0,0.0,1.0)]));
        //nml.push(Vec::from([Vec3::new(0.0,0.0,1.0), Vec3::new(0.0,0.0,1.0)]));

        /*
        if curve.deg()==1{
            let num = curve.num();
            for i in 0..num{
                pts.push(curve.cp(i).clone());
            }
        }
        else{
            let reso = SEGMENT_RESOLUTION;
            let epnum = curve.epNum();
            let num = (epnum-1)*(reso as usize)+1;
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
        */

        GlSurface{pos, nml, color:surface.attr.color, index:0}
    }

    pub fn new(pos:Vec<Vec<Vec3>>, nml:Vec<Vec<Vec3>>)->Self{
        GlSurface{pos, nml, color:Color::new(0.5,0.5,0.5,1.0), index:0}
    }

    pub fn set_index(&mut self, start_index:i32){
        self.index = start_index;
    }

    pub fn clr(&mut self, c:Color){
        self.color = c;
    }

    pub fn ulen(&self)->i32{ self.pos.len() as i32 }
    pub fn vlen(&self)->i32{ self.pos[0].len() as i32 }
}

pub struct GlLineBuffer{
    pos: Vec<f32>,
    //buffer: GlAttributeVec3,
}
impl GlLineBuffer{
    pub fn new()->Self{
        GlLineBuffer{pos:Vec::new()}
    }
    pub fn add(&mut self, line:&mut GlLine){
        let idx = (self.pos.len() as i32)/3; // divided by 3 or not?
        line.set_index(idx);
        for x in &line.pos{
            self.pos.push(x.x as f32);
            self.pos.push(x.y as f32);
            self.pos.push(x.z as f32);
        }
    }
    pub fn clear(&mut self){
        self.pos.clear();
    }
}

pub struct GlPointBuffer{
    pos: Vec<f32>,
}
impl GlPointBuffer{
    pub fn new()->Self{
        GlPointBuffer{pos:Vec::new()}
    }
    pub fn add(&mut self, pt:&mut GlPoint){
        let idx = (self.pos.len() as i32)/3;
        pt.set_index(idx);
        self.pos.push(pt.pos.x as f32);
        self.pos.push(pt.pos.y as f32);
        self.pos.push(pt.pos.z as f32);
//        web_sys::console::log_1(&JsValue::from(format!("GlPointBuffer.add: pt {}, {}, {}", pt.pos.x, pt.pos.y, pt.pos.z )));
//        web_sys::console::log_1(&JsValue::from(format!("GlPointBuffer.add: pt.index {}", pt.index )));
    }
    pub fn clear(&mut self){
        self.pos.clear();
//        web_sys::console::log_1(&JsValue::from(format!("GlPointBuffer.clear: pos.len() = {}", self.pos.len() )));
    }
}

pub struct GlFaceBuffer{
    pos: Vec<f32>,
    nml: Vec<f32>,
}
impl GlFaceBuffer{
    pub fn new()->Self{
        GlFaceBuffer{pos:Vec::new(), nml:Vec::new()}
    }
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

                //web_sys::console::log_1(&JsValue::from(format!("glFaceBuffer nml[{}][{}] = {}", i, j, n1 ))); //

            }
        }
    }
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

    time: u64,

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

    pub fn add_point(&mut self, mut point : Box<GlPoint>){
        self.point_buffer.add(&mut*point);
        self.points.push(point)
    }
    //pub fn add_line(&mut self, line : &mut GlLine){
    pub fn add_line(&mut self, mut line : Box<GlLine>){
        self.line_buffer.add(&mut*line);
        self.lines.push(line)
    }
    pub fn add_surface(&mut self, mut surf : Box<GlSurface>){
        self.surf_buffer.add(&mut*surf);
        self.surfs.push(surf)
    }

    pub fn clear_points(&mut self){
        self.point_buffer.clear();
        self.points.clear();
//        web_sys::console::log_1(&JsValue::from(format!("clear_points: point.len() = {}", self.points.len() ))); //
    }

    pub fn clear_lines(&mut self){
        self.line_buffer.clear();
        self.lines.clear();
    }

    pub fn clear_surfaces(&mut self){
        self.surf_buffer.clear();
        self.surfs.clear();
    }

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
        self.p_matrix = Matrix4::perspective( (self.width/self.height) as f64, 45.0/*std::f64::consts::PI/4.*/, 0.1, 100.0);
        self.tmp_matrix = Matrix4::new_with_matrix4(&self.v_matrix);
        self.tmp_matrix.matmul(&self.p_matrix);

        //web_sys::console::log_1(&JsValue::from(format!("P{:?}", self.p_matrix.to_array32())));
        //web_sys::console::log_1(&JsValue::from(format!("V{:?}", self.v_matrix.to_array32())));
        //web_sys::console::log_1(&JsValue::from(format!("T{:?}", self.tmp_matrix.to_array32())));

        //let bg_geom = WebGlServer::bg_rect(Color::new(1.0,0.5,0.0,1.0), Color::new(1.0,0.0,0.0,1.0), Color::new(0.0,1.0,1.0,1.0), Color::new(0.5,0.5,1.0,1.0) );
        let bg_geom = WebGlServer::bg_rect(Color::new(0.3,0.5,0.7,1.0), Color::new(0.3,0.5,0.7,1.0),Color::new(1.0,1.0,1.0,1.0), Color::new(0.9,0.9,0.9,1.0) );
        self.bg_position_attr.set_buffer(&self.gl, &bg_geom.0);
        self.bg_color_attr.set_buffer(&self.gl, &bg_geom.1);
        //self.bg_index.set_buffer(&self.gl, &bg_geom.2);

    }

    pub fn draw(&mut self){
        let count = self.time; //self.server.unwrap().time();

        //self.gl.clear_color(1.0, 1.0, 1.0, 1.0);
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

        //Webgl initialize
        let i:f64 = (count % 360) as f64;
        let rad = i * std::f64::consts::PI / 180.;
        //let rad = std::f64::consts::PI / 2.0;
        //let rad = 0.0;

        //view rotation
        //self.m_matrix = Matrix4::y_rotation(rad);
        self.m_matrix = Matrix4::x_rotation(std::f64::consts::PI/2.0);
        self.m_matrix.matmul(&Matrix4::y_rotation(rad));
        self.m_matrix.matmul(&Matrix4::x_rotation(-std::f64::consts::PI/3.0));


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

            self.gl.draw_arrays(GL::TRIANGLE_STRIP, srf.index, srf.ulen()*(srf.vlen()-1)*2 );
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

    pub fn get_webgl_context(height: u32, width: u32) -> Result<WebGlRenderingContext, String> {
        //Get WebGLContext
        let document = window().unwrap().document().unwrap();
        let canvas = document
        .get_element_by_id("canvas")
        .ok_or_else(|| String::from("canvas doesn't exist :("))?;
        let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        canvas.set_height(height);
        canvas.set_width(width);

        let gl: WebGlRenderingContext = canvas
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


/******************
*    Graphics
******************/

pub const SEGMENT_RESOLUTION : u32 = 20; //10;
pub const TESSELLATION_RESOLUTION : u32 = 1; // 5; //10; //5;

pub const INSERT_POINT_ON_DEGREE1_TWISTED_SURFACE : bool = true;

pub const DEFAULT_GL_LIGHT_POSITION : [f32;4] = [0.0, 0.0, 1.0, 0.0];
pub const DEFAULT_GL_AMBIENT_LIGHT : [f32;4] = [0.4, 0.4, 0.4, 1.0];
pub const DEFAULT_GL_DIFFUSE_LIGHT : [f32;4] = [0.7, 0.7, 0.7, 1.0];
pub const DEFAULT_GL_SPECULAR_LIGHT : [f32;4] = [0.0, 0.0, 0.0, 1.0];

pub const DEFAULT_PERSPECTIVE_RATIO : f32  = 0.5;
pub const DEFAULT_AXONOMETRIC_RATIO : f32 = 1.0;
pub const DEFAULT_VIEW_DISTANCE : f32 = 500.0;
pub const DEFAULT_VIEW_DISTANCE_RATIO : f32 = 10.0;
pub const DEFAULT_NEAR_VIEW_RATIO : f32 = 0.001;
pub const DEFAULT_FAR_VIEW_RATIO : f32 = 1000.0;


pub const DEFAULT_BG_COLOR1 : Color = Color{ rgba:[1.0, 1.0, 1.0, 1.0] };
pub const DEFAULT_BG_COLOR2 : Color = Color{ rgba:[0.9, 0.9, 0.9, 1.0] };
pub const DEFAULT_BG_COLOR3 : Color = Color{ rgba:[0.3, 0.5, 0.7, 1.0] };
pub const DEFAULT_BG_COLOR4 : Color = Color{ rgba:[0.3, 0.5, 0.7, 1.0] };

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
//impl/*<'a>*/ Object/*<'a>*/{}
pub trait Object{
    //fn init(&dyn self, server: &mut Server){ server.add_object(Box::new(self)); }

    fn draw(&mut self);
//    fn interact(&mut self, objects:&Vec<Box<dyn Object>>);
//    fn update(&mut self);
    //fn attr(&self)->&mut Attribute;
}

#[derive(Debug, Clone)]
pub struct Agent{
    pub id:i32,
    pub pos:Vec3,
    pub vel:Vec3,
    pub frc:Vec3,
    pub fric:f64,
    pub dir:Vec3,
    pub nml:Vec3,
    pub time:i32,
    pub colliding:bool,
    pub vecs:Vec<Vec3>,
    pub params:Vec<f64>,
    pub attr:Attribute,
}


impl Agent{
    pub fn new(pos:Vec3)->Self{
        Agent{id:-1, pos, vel:Vec3::zero(), frc:Vec3::zero(), fric:0.0, dir:Vec3::zero(), nml:Vec3::zero(), time:0, colliding:false, vecs:Vec::new(), params:Vec::new(), attr:Attribute::default() }
    }
    pub fn new_with_dir(pos:Vec3, dir:Vec3)->Self{
        Agent{id:-1, pos, vel:Vec3::zero(), frc:Vec3::zero(), fric:0.0, dir, nml:Vec3::zero(), time:0, colliding:false, vecs:Vec::new(), params:Vec::new(), attr:Attribute::default() }
    }
    //fn init(&dyn self, server: &mut Server){ server.add_agent(Box::new(self)); }
    pub fn set_id(&mut self, i:i32){ self.id = i; }
    pub fn get_id(&mut self)->i32{ return self.id; }

    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Agent{
        self.attr.color.set_rgba(r,g,b,a);
        self
    }
    pub fn hsb(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Agent{
        self.attr.color.set_hsb(h,s,b,a);
        self
    }
    pub fn set_attr(&mut self, attr:&Attribute)->&mut Agent{
        self.attr.set(attr);
        self
    }

    //fn clone(&self)->Self;
    //fn test(&self)->Self{ Self{}}

    //fn interact(&mut self, agents:&Vec<&Box<dyn Agent>>, storage:&mut DataManager){}
    //fn interact(&mut self, agent: Vec<Box<Agent>>, storage:&mut DataManager){}
    //fn update(&mut self, storage:&mut DataManager){}

    //fn attr(&self)->&mut Attribute;

    //fn set_server(&mut self, server:Box<Server>);
    //fn get_server(&mut self)->&Box<Server>;

}

pub trait AgentBak{
    //fn init(&dyn self, server: &mut Server){ server.add_agent(Box::new(self)); }
    fn set_id(&mut self, i:i32){}
    fn get_id(&mut self)->i32{ return -1; }

    //fn clone(&self)->Self;
    //fn test(&self)->Self{ Self{}}

    //fn interact(&mut self, agents:&Vec<&Box<dyn Agent>>, storage:&mut DataManager){}
    fn interact(&mut self, agent: &dyn AgentBak, storage:&mut DataManager){}
    fn update(&mut self, storage:&mut DataManager){}
    //fn attr(&self)->&mut Attribute;

    //fn set_server(&mut self, server:Box<Server>);
    //fn get_server(&mut self)->&Box<Server>;

}


pub struct Graphic{

}
impl Graphic{

}
pub struct Dynamic{

}
impl Dynamic{

}

#[derive(Debug, Clone)]
pub struct Attribute/*<'a>*/{
    pub name: String,
    //layer: &'a Layer<'a>,
    pub layerIndex: u32,
    pub color: /*'a*/ Color,
    pub stroke: /*'a*/ Color,
    pub size: f32,
    pub weight: f32,
    //pub mateiral: Material
    pub visible: bool,
}

impl Attribute{
    pub fn default()->Self{
        Attribute{
            name: String::from("default"), layerIndex:0,
            color: Color::new(0.5,0.5,0.5,1.0), stroke: Color::new(0.5,0.5,0.5,1.0),
            size: 5.0, weight: 1.0, visible:true
        }
    }
    pub fn set(&mut self, attr:&Attribute){
        self.name = attr.name.clone();
        self.layerIndex = attr.layerIndex;
        self.color.set(&attr.color);
        self.stroke.set(&attr.stroke);
        self.size = attr.size;
        self.weight = attr.weight;
        self.visible = attr.visible;
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
    pub fn new(r:f32, g:f32, b:f32, a:f32)->Self{
        Color{
            rgba:[r,g,b,a]
        }
    }

    pub fn new_with_rgb(r:f32, g:f32, b:f32)->Self{
        Color{
            rgba:[r,g,b,1.0]
        }
    }

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

    pub fn set_rgb(&mut self, r:f32, g:f32, b:f32){
        self.rgba[0] = r;
        self.rgba[1] = g;
        self.rgba[2] = b;
    }

    pub fn set(&mut self, color:&Color){
        self.rgba[0] = color.rgba[0];
        self.rgba[1] = color.rgba[1];
        self.rgba[2] = color.rgba[2];
        self.rgba[3] = color.rgba[3];
    }

    pub fn set_rgba(&mut self, r:f32, g:f32, b:f32, a:f32){
        self.rgba[0] = r;
        self.rgba[1] = g;
        self.rgba[2] = b;
        self.rgba[3] = a;
    }

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

    pub fn red(&self)->f32{
        self.rgba[0]
    }
    pub fn green(&self)->f32{
        self.rgba[1]
    }
    pub fn blue(&self)->f32{
        self.rgba[2]
    }
    pub fn alpha(&self)->f32{
        self.rgba[3]
    }
    pub fn to_array(&self)->[f32;4]{
        self.rgba
    }

    pub fn r(&self)->f32{ self.red() }
    pub fn g(&self)->f32{ self.green() }
    pub fn b(&self)->f32{ self.blue() }
    pub fn a(&self)->f32{ self.alpha() }

}

impl fmt::Display for Color{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f, "({},{},{},{})", self.rgba[0], self.rgba[1], self.rgba[2],self.rgba[3])
    }
}


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

pub struct Curve/*<'a>*/{
//    object: Object/*<'a>*/,
    pub id: i32,
    pub curve: CurveGeo,
    pub attr: Attribute

}

impl Curve{
    pub fn new(cpts:Vec<Vec3>, degree:u8)->Self{
        Curve{ id:-1, curve:CurveGeo::new(cpts, degree),attr:Attribute::default()}
    }
    pub fn new_closed(cpts:Vec<Vec3>, degree:u8)->Self{
        Curve{ id:-1, curve:CurveGeo::new_closed(cpts, degree),attr:Attribute::default()}
    }
    pub fn new_with_knots(cpts:Vec<Vec3>, degree:u8, mut knots:Vec<f64>, ustart:f64, uend:f64)->Self{
        Curve{ id:-1, curve:CurveGeo::new_with_knots(cpts, degree, knots, ustart, uend),attr:Attribute::default()}
    }
    pub fn new_polyline(cpts:Vec<Vec3>)->Self{
        Curve::new(cpts, 1)
    }
    pub fn new_line(pt1:Vec3, pt2:Vec3)->Self{
        Curve::new(Vec::from([pt1,pt2]), 1)
    }

    pub fn pt(&self, u:f64)->Vec3{
        self.curve.pt(u)
    }

    pub fn tan(&self, u:f64)->Vec3{
        self.curve.tan(u)
    }

    pub fn deg(&self)->u8{
        self.curve.deg()
    }

    pub fn num(&self)->usize{
        self.curve.num()
    }

    pub fn ep_num(&self)->usize{
        self.curve.epNum()
    }

    pub fn cp(&self, i:usize)->Vec3{
        self.curve.cp(i)
    }

    pub fn u(&self, epIdx:usize, epFraction:f64)->f64{
        self.curve.u(epIdx, epFraction)
    }

    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Curve{
        self.attr.color.set_rgba(r,g,b,a);
        self
    }
    pub fn hsb(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Curve{
        self.attr.color.set_hsb(h,s,b,a);
        self
    }
    pub fn set_attr(&mut self, attr:&Attribute)->&mut Curve{
        self.attr.set(attr);
        self
    }


}


pub struct CurveGeo{
    cpts: Vec<Vec3>,
    degree: u8,
    knots: Vec<f64>,
    ustart: f64,
    uend: f64,
    weights:Vec<f64>,
    basisFunction:BSplineBasisFunction,
    derivativeFunction: BSplineBasisFunction,
}

impl CurveGeo{
    pub fn new_with_knots(cpts:Vec<Vec3>, degree:u8, mut knots:Vec<f64>, ustart:f64, uend:f64)->Self{
        if ustart != 0.0 || uend != 1.0{
            knots = NurbsGeo::normalize_knots(knots, &ustart, &uend);
        }
        let basisFunction = BSplineBasisFunction::new(degree, knots.clone());
        let mut derivativeFunction = BSplineBasisFunction::new(degree, knots.clone());
        derivativeFunction.differentiate();
        let mut weights : Vec<f64> = Vec::new();
        for i in 0..cpts.len(){ weights.push(1.0); }

        CurveGeo{
            cpts, degree, knots, ustart, uend, weights, basisFunction, derivativeFunction
        }
    }

    pub fn new(cpts:Vec<Vec3>, degree:u8)->Self{
        let knots = NurbsGeo::create_open_knots(degree, cpts.len());
        CurveGeo::new_with_knots(cpts, degree, knots, 0.0, 1.0)
    }

    pub fn new_closed(cpts:Vec<Vec3>, degree:u8)->Self{
        let cpts2 = NurbsGeo::create_closed_cp(cpts, degree);
        let knots = NurbsGeo::create_closed_knots(degree, cpts2.len());
        CurveGeo::new_with_knots(cpts2, degree, knots, 0.0, 1.0)
    }

    pub fn deg(&self)->u8{
        self.degree
    }

    pub fn num(&self)->usize{
        self.cpts.len()
    }

    pub fn epNum(&self)->usize{
        self.knots.len() - 2*(self.degree as usize)
    }

    pub fn cp(&self, i:usize)->Vec3{
        self.cpts[i]
    }

    pub fn u(&self, epIdx:usize, epFraction:f64)->f64{
        if epFraction>=0.0{
            return self.knots[epIdx+self.degree as usize] + (self.knots[epIdx+self.degree as usize+1] - self.knots[epIdx+self.degree as usize])*epFraction;
        }
        return self.knots[epIdx+self.degree as usize] + (self.knots[epIdx+self.degree as usize] - self.knots[epIdx+self.degree as usize-1])*epFraction;
    }

    pub fn pt(&self, u:f64)->Vec3{
        let index = self.basisFunction.index(u);
        let n:Vec<f64> = self.basisFunction.eval_with_index(index, u);
        let mut weight:f64 = 0.0;
        let mut retval:Vec3 = Vec3::zero();
        for i in 0..=self.degree as usize{
            let cpt:&Vec3 = &self.cpts[index-(self.degree as usize)+i];
            let w = self.weights[index-(self.degree as usize)+i];
            retval.x += cpt.x * w * n[i];
            retval.y += cpt.y * w * n[i];
            retval.z += cpt.z * w * n[i];
            weight += w * n[i];
        }
        retval.x /= weight;
        retval.y /= weight;
        retval.z /= weight;
        retval
    }

    pub fn tan(&self, u:f64)->Vec3{
        let index = self.derivativeFunction.index(u);
        let dn : Vec<f64> = self.derivativeFunction.eval_with_index(index, u);
        let n : Vec<f64> = self.basisFunction.eval_with_index(index, u);
        let mut val1: Vec3 = Vec3::zero();
        let mut val2: Vec3 = Vec3::zero();
        let mut weight1 : f64 = 0.0;
        let mut weight2 : f64 = 0.0;

        for i in 0..=self.degree as usize{
            let cpt:&Vec3 = &self.cpts[index-self.degree as usize+i];
            let w : f64 = self.weights[index-self.degree as usize+i];
            val1.x += cpt.x*w*n[i];
            val1.y += cpt.y*w*n[i];
            val1.z += cpt.z*w*n[i];
            weight1 += w*n[i];
            val2.x += cpt.x*w*dn[i];
            val2.y += cpt.y*w*dn[i];
            val2.z += cpt.z*w*dn[i];
            weight2 += w*dn[i];
        }
        val1.x *= weight2;
        val1.y *= weight2;
        val1.z *= weight2;
        val2.x *= weight1;
        val2.y *= weight1;
        val2.z *= weight1;
        weight1 *= weight1;

        Vec3::new((val2.x-val1.x)/weight1, (val2.y-val1.y)/weight1, (val2.z-val1.z)/weight1)
    }

}


pub struct NurbsGeo{}
impl NurbsGeo{
    pub fn normalize_knots(mut knots:Vec<f64>, ustart:&f64, uend:&f64)->Vec<f64>{
        for i in 0..knots.len(){
            knots[i] -= ustart;
            knots[i] /= uend-ustart;
        }
        knots
    }

    pub fn create_knots(degree:u8, num:usize, closed:bool)->Vec<f64>{
        if closed{
            return NurbsGeo::create_closed_knots(degree, num);
        }
        NurbsGeo::create_open_knots(degree,num)
    }

    pub fn create_closed_knots(degree:u8, num:usize)->Vec<f64>{
        let mut knots : Vec<f64> = Vec::new();
        let inc : f64 = 1.0/(num-(degree as usize))as f64;
        let mut k : usize = 0;
        let mut m : i32 = -(degree as i32);
        while k<=((degree as usize)+num){
            knots.push((m as f64)*inc);
            k+=1;
            m+=1;
        }
        knots
    }
    pub fn create_open_knots(degree:u8, num:usize)->Vec<f64>{
        let mut knots : Vec<f64> = Vec::new();
        let mut k : usize = 0;
        let mut m : usize = 1;
        let inc : f64 = 1.0/(num-(degree as usize))as f64;
        while k <= degree as usize{
            knots.push(0.0);
            k+=1;
        }
        while k<num {
            knots.push( (m as f64)* inc );
            k+=1;
            m+=1;
        }
        while k<=((degree as usize)+num){
            knots.push(1.0);
            k+=1;
        }
        knots
    }

    pub fn create_closed_cp(cpts:Vec<Vec3>, degree:u8)->Vec<Vec3>{
        let head_num :usize = (degree as usize -1)/2;
        let tail_num :usize = (degree as usize)/2 +1;
        let mut len = cpts.len();

        if cpts[0].eq(&cpts[cpts.len()-1]) {
            if degree==1{
                return cpts;
            }
            len-=1;
        }
        let mut cpts2 :Vec<Vec3> = Vec::new();
        for i in 0.. len + degree as usize{
            if i<head_num{
                cpts2.push(cpts[(i - head_num+len)%len].clone());
            }
            else if i < (len + degree as usize-tail_num){
                 cpts2.push(cpts[i-head_num]);
             }
            else {
                cpts2.push(cpts[(i - (len + degree as usize-tail_num))%len].clone());
            }
        }
        cpts2
    }

    pub fn create_closed_cp_in_u(cpts:Vec<Vec<Vec3>>, udeg:u8)->Vec<Vec<Vec3>>{
        let head_num :usize = (udeg as usize -1)/2;
        let tail_num :usize = (udeg as usize)/2 +1;
        let mut ulen = cpts.len();
        let mut vlen = cpts[0].len();
        let mut is_edge_closed :bool = true;
        for i in 0..vlen{
            if !is_edge_closed{ break; }
            if !cpts[0][i].eq(&cpts[ulen-1][i]){
                is_edge_closed=false;
            }
        }
        if is_edge_closed{
            if udeg==1{
                for i in 0..vlen{
                    return cpts;
                }
            }
            ulen-=1;
        }
        let mut cpts2: Vec<Vec<Vec3>> = Vec::new();
        for i in 0..ulen+udeg as usize{
            let mut cps:Vec<Vec3> = Vec::new();
            for j in 0..vlen{
                if i < head_num{ cps.push(cpts[(i-head_num+ulen)%ulen][j].clone()); }
                else if i<ulen+udeg as usize-tail_num{ cps.push(cpts[i-head_num][j]); }
                else{ cps.push(cpts[(i-(ulen+udeg as usize-tail_num))%ulen][j].clone()); }
            }
            cpts2.push(cps);
        }
        cpts2
    }

    pub fn create_closed_cp_in_v(cpts:Vec<Vec<Vec3>>, vdeg:u8)->Vec<Vec<Vec3>>{
        let head_num :usize = (vdeg as usize -1)/2;
        let tail_num :usize = (vdeg as usize)/2 +1;
        let mut ulen = cpts.len();
        let mut vlen = cpts[0].len();
        let mut is_edge_closed : bool = true;
        for i in 0..ulen{
            if !is_edge_closed{ break; }
            if !cpts[i][0].eq(&cpts[i][vlen-1]){
                is_edge_closed=false;
            }
        }
        if is_edge_closed{
            if vdeg==1{
                for i in 0..ulen{
                    return cpts;
                }
            }
            vlen-=1;
        }
        let mut cpts2: Vec<Vec<Vec3>> = Vec::new();
        for i in 0..ulen{
            let mut cps:Vec<Vec3> = Vec::new();
            for j in 0..vlen+vdeg as usize{
                if j < head_num{ cps.push(cpts[i][(j-head_num+vlen)%vlen].clone()); }
                else if j<vlen+vdeg as usize-tail_num { cps.push(cpts[i][j-head_num]); }
                else{ cps.push(cpts[i][(j-(vlen+vdeg as usize-tail_num))%vlen].clone()); }
            }
            cpts2.push(cps);
        }
        cpts2
    }


}

pub struct Surface{
    id: i32,
    surface: SurfaceGeo,
    pub attr: Attribute
}


impl Surface{
    pub fn new(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new(cpts, udegree, vdegree), attr:Attribute::default()}
    }
    pub fn new_u_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_u_closed(cpts, udegree, vdegree),attr:Attribute::default()}
    }
    pub fn new_v_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_v_closed(cpts, udegree, vdegree),attr:Attribute::default()}
    }
    pub fn new_uv_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_uv_closed(cpts, udegree, vdegree),attr:Attribute::default()}
    }
    pub fn new_with_knots(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8, mut uknots:Vec<f64>, mut vknots:Vec<f64>, ustart:f64, uend:f64, vstart:f64, vend:f64)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, ustart, uend, vstart, vend),attr:Attribute::default()}
    }
    pub fn new_quad(pt1:Vec3, pt2:Vec3, pt3:Vec3, pt4:Vec3)->Self{
        Surface::new(Vec::from([Vec::from([pt1,pt2]),Vec::from([pt4,pt3])]), 1, 1)
    }
    pub fn new_triangle(pt1:Vec3, pt2:Vec3, pt3:Vec3)->Self{
        Surface::new(Vec::from([Vec::from([pt1,pt2]),Vec::from([pt3,pt3.clone()])]), 1, 1)
    }

    pub fn pt(&self, u:f64, v:f64)->Vec3{
        self.surface.pt(u,v)
    }

    pub fn utan(&self, u:f64, v:f64)->Vec3{
        self.surface.utan(u, v)
    }

    pub fn vtan(&self, u:f64, v:f64)->Vec3{
        self.surface.vtan(u, v)
    }

    pub fn nml(&self, u:f64, v:f64)->Vec3{
        self.surface.nml(u, v)
    }


    pub fn udeg(&self)->u8{
        self.surface.udeg()
    }
    pub fn vdeg(&self)->u8{
        self.surface.vdeg()
    }

    pub fn unum(&self)->usize{
        self.surface.unum()
    }

    pub fn vnum(&self)->usize{
        self.surface.vnum()
    }

    pub fn uep_num(&self)->usize{
        self.surface.uep_num()
    }

    pub fn vep_num(&self)->usize{
        self.surface.uep_num()
    }

    pub fn cp(&self, i:usize, j:usize)->Vec3{
        self.surface.cp(i, j)
    }

    pub fn u(&self, epIdx:usize, epFraction:f64)->f64{
        self.surface.u(epIdx, epFraction)
    }

    pub fn v(&self, epIdx:usize, epFraction:f64)->f64{
        self.surface.v(epIdx, epFraction)
    }

    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Surface{
        self.attr.color.set_rgba(r,g,b,a);
        self
    }
    pub fn hsb(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Surface{
        self.attr.color.set_hsb(h,s,b,a);
        self
    }
    pub fn set_attr(&mut self, attr:&Attribute)->&mut Surface{
        self.attr.set(attr);
        self
    }

}


pub struct SurfaceGeo{
    cpts: Vec<Vec<Vec3>>,
    udegree: u8,
    vdegree: u8,
    uknots: Vec<f64>,
    vknots: Vec<f64>,
    ustart: f64,
    uend: f64,
    vstart: f64,
    vend: f64,
    weights:Vec<Vec<f64>>,
    basisFunctionU:BSplineBasisFunction,
    basisFunctionV:BSplineBasisFunction,
    derivativeFunctionU: BSplineBasisFunction,
    derivativeFunctionV: BSplineBasisFunction,
}

impl SurfaceGeo{
    pub fn new_with_knots(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8, mut uknots:Vec<f64>, mut vknots:Vec<f64>, ustart:f64, uend:f64, vstart:f64, vend:f64)->Self{
        if ustart != 0.0 || uend != 1.0{
            uknots = NurbsGeo::normalize_knots(uknots, &ustart, &uend);
        }
        if vstart != 0.0 || vend != 1.0{
            vknots = NurbsGeo::normalize_knots(vknots, &vstart, &vend);
        }
        let basisFunctionU = BSplineBasisFunction::new(udegree, uknots.clone());
        let mut derivativeFunctionU = BSplineBasisFunction::new(udegree, uknots.clone());
        derivativeFunctionU.differentiate();
        let basisFunctionV = BSplineBasisFunction::new(vdegree, vknots.clone());
        let mut derivativeFunctionV = BSplineBasisFunction::new(vdegree, vknots.clone());
        derivativeFunctionV.differentiate();

        let mut weights : Vec<Vec<f64>> = Vec::new();
        for i in 0..cpts.len(){
            let mut w : Vec<f64> = Vec::new();
            for j in 0..cpts[i].len(){
                w.push(1.0);
            }
            weights.push(w);
        }

        SurfaceGeo{
            cpts, udegree, vdegree, uknots, vknots, ustart, uend, vstart, vend, weights, basisFunctionU, basisFunctionV, derivativeFunctionU, derivativeFunctionV
        }
    }

    pub fn new(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        let uknots = NurbsGeo::create_open_knots(udegree, cpts.len());
        let vknots = NurbsGeo::create_open_knots(vdegree, cpts[0].len());
        SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, 0.0, 1.0, 0.0, 1.0)
    }

    pub fn new_uv_closed(mut cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        cpts = NurbsGeo::create_closed_cp_in_v(cpts, vdegree);
        cpts = NurbsGeo::create_closed_cp_in_u(cpts, udegree);

        let uknots = NurbsGeo::create_closed_knots(udegree, cpts.len());
        let vknots = NurbsGeo::create_closed_knots(vdegree, cpts[0].len());

        SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, 0.0, 1.0, 0.0, 1.0)
    }

    pub fn new_u_closed(mut cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        cpts = NurbsGeo::create_closed_cp_in_u(cpts, udegree);

        let uknots = NurbsGeo::create_closed_knots(udegree, cpts.len());
        let vknots = NurbsGeo::create_open_knots(vdegree, cpts[0].len());

        SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, 0.0, 1.0, 0.0, 1.0)
    }

    pub fn new_v_closed(mut cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        cpts = NurbsGeo::create_closed_cp_in_v(cpts, vdegree);

        let uknots = NurbsGeo::create_open_knots(udegree, cpts.len());
        let vknots = NurbsGeo::create_closed_knots(vdegree, cpts[0].len());

        SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, 0.0, 1.0, 0.0, 1.0)
    }



    pub fn udeg(&self)->u8{
        self.udegree
    }

    pub fn vdeg(&self)->u8{
        self.vdegree
    }

    pub fn unum(&self)->usize{
        self.cpts.len()
    }

    pub fn vnum(&self)->usize{
        self.cpts[0].len()
    }

    pub fn uep_num(&self)->usize{
        self.uknots.len() - 2*(self.udegree as usize)
    }

    pub fn vep_num(&self)->usize{
        self.vknots.len() - 2*(self.vdegree as usize)
    }

    pub fn cp(&self, i:usize, j:usize)->Vec3{
        self.cpts[i][j]
    }

    pub fn u(&self, epIdx:usize, epFraction:f64)->f64{
        if epFraction>=0.0{
            return self.uknots[epIdx+self.udegree as usize] + (self.uknots[epIdx+self.udegree as usize+1] - self.uknots[epIdx+self.udegree as usize])*epFraction;
        }
        return self.uknots[epIdx+self.udegree as usize] + (self.uknots[epIdx+self.udegree as usize] - self.uknots[epIdx+self.udegree as usize-1])*epFraction;
    }

    pub fn v(&self, epIdx:usize, epFraction:f64)->f64{
        if epFraction>=0.0{
            return self.vknots[epIdx+self.vdegree as usize] + (self.vknots[epIdx+self.vdegree as usize+1] - self.vknots[epIdx+self.vdegree as usize])*epFraction;
        }
        return self.vknots[epIdx+self.vdegree as usize] + (self.vknots[epIdx+self.vdegree as usize] - self.vknots[epIdx+self.vdegree as usize-1])*epFraction;
    }

    pub fn pt(&self, u:f64, v:f64)->Vec3{
        let uindex = self.basisFunctionU.index(u);
        let vindex = self.basisFunctionV.index(v);
        let nu:Vec<f64> = self.basisFunctionU.eval_with_index(uindex, u);
        let nv:Vec<f64> = self.basisFunctionV.eval_with_index(vindex, v);
        let mut weight:f64 = 0.0;
        let mut retval:Vec3 = Vec3::zero();
        for i in 0..=self.udegree as usize{
            for j in 0..=self.vdegree as usize{
                let cpt:&Vec3 = &self.cpts[uindex-(self.udegree as usize)+i][vindex-(self.vdegree as usize)+j];
                let w = self.weights[uindex-(self.udegree as usize)+i][vindex-(self.vdegree as usize)+j];
                retval.x += cpt.x * w * nu[i] * nv[j];
                retval.y += cpt.y * w * nu[i] * nv[j];
                retval.z += cpt.z * w * nu[i] * nv[j];
                weight += w * nu[i] * nv[j];
            }
        }
        retval.x /= weight;
        retval.y /= weight;
        retval.z /= weight;
        retval
    }

    pub fn utan(&self, u:f64, v:f64)->Vec3{
        let uindex = self.derivativeFunctionU.index(u);
        let vindex = self.derivativeFunctionV.index(v);
        let nu : Vec<f64> = self.basisFunctionU.eval_with_index(uindex, u);
        let nv : Vec<f64> = self.basisFunctionV.eval_with_index(vindex, v);
        let dnu : Vec<f64> = self.derivativeFunctionU.eval_with_index(uindex, u);
        let mut val1: Vec3 = Vec3::zero();
        let mut val2: Vec3 = Vec3::zero();
        let mut weight1 : f64 = 0.0;
        let mut weight2 : f64 = 0.0;

        for i in 0..=self.udegree as usize{
            for j in 0..=self.vdegree as usize{

                let cpt:&Vec3 = &self.cpts[uindex-self.udegree as usize+i][vindex-self.vdegree as usize+j];
                let w = self.weights[uindex-(self.udegree as usize)+i][vindex-(self.vdegree as usize)+j];

                val1.x += cpt.x*w*nu[i]*nv[j];
                val1.y += cpt.y*w*nu[i]*nv[j];
                val1.z += cpt.z*w*nu[i]*nv[j];
                weight1 += w*nu[i]*nv[j];
                val2.x += cpt.x*w*dnu[i]*nv[j];
                val2.y += cpt.y*w*dnu[i]*nv[j];
                val2.z += cpt.z*w*dnu[i]*nv[j];
                weight2 += w*dnu[i]*nv[j];
            }
        }
        val1.x *= weight2;
        val1.y *= weight2;
        val1.z *= weight2;
        val2.x *= weight1;
        val2.y *= weight1;
        val2.z *= weight1;
        weight1 *= weight1;

        Vec3::new((val2.x-val1.x)/weight1, (val2.y-val1.y)/weight1, (val2.z-val1.z)/weight1)
    }

    pub fn vtan(&self, u:f64, v:f64)->Vec3{
        let uindex = self.derivativeFunctionU.index(u);
        let vindex = self.derivativeFunctionV.index(v);
        let nu : Vec<f64> = self.basisFunctionU.eval_with_index(uindex, u);
        let nv : Vec<f64> = self.basisFunctionV.eval_with_index(vindex, v);
        let dnv : Vec<f64> = self.derivativeFunctionV.eval_with_index(vindex, v);
        let mut val1: Vec3 = Vec3::zero();
        let mut val2: Vec3 = Vec3::zero();
        let mut weight1 : f64 = 0.0;
        let mut weight2 : f64 = 0.0;

        for i in 0..=self.udegree as usize{
            for j in 0..=self.vdegree as usize{

                let cpt:&Vec3 = &self.cpts[uindex-self.udegree as usize+i][vindex-self.vdegree as usize+j];
                let w = self.weights[uindex-(self.udegree as usize)+i][vindex-(self.vdegree as usize)+j];

                val1.x += cpt.x*w*nu[i]*nv[j];
                val1.y += cpt.y*w*nu[i]*nv[j];
                val1.z += cpt.z*w*nu[i]*nv[j];
                weight1 += w*nu[i]*nv[j];
                val2.x += cpt.x*w*nu[i]*dnv[j];
                val2.y += cpt.y*w*nu[i]*dnv[j];
                val2.z += cpt.z*w*nu[i]*dnv[j];
                weight2 += w*nu[i]*dnv[j];
            }
        }
        val1.x *= weight2;
        val1.y *= weight2;
        val1.z *= weight2;
        val2.x *= weight1;
        val2.y *= weight1;
        val2.z *= weight1;
        weight1 *= weight1;

        Vec3::new((val2.x-val1.x)/weight1, (val2.y-val1.y)/weight1, (val2.z-val1.z)/weight1)
    }

    pub fn nml(&self, u:f64, v:f64)->Vec3{
        let ut = self.utan(u,v);
        let vt = self.vtan(u,v);
        if ut.len2()>0.0 && vt.len2()>0.0 {
            return ut.cross(&vt);
        }
        else if ut.len2()>0.0{ // vtan == 0
            if v>0.5{
                let vt = self.utan(u,0.0);
                return vt.cross(&ut);
            }
            else{
                let vt = self.utan(u,1.0);
                return ut.cross(&vt);
            }
        }
        else if vt.len2()>0.0{ // utan == 0
            if u>0.5{
                let ut = self.vtan(0.0, v);
                return ut.cross(&vt);
            }
            else{
                let ut = self.vtan(1.0, v);
                return vt.cross(&ut);
            }
        }
        Vec3::zero() // normal is zero
    }

}

pub struct Point/*<'a>*/{
//    object: Object,
    pub id: i32,
    pub pos: Vec3,
    pub attr: Attribute
}

impl Point{
    pub fn new(x:f64, y:f64, z:f64)->Self{
        Point{id:-1, pos:Vec3::new(x,y,z),attr:Attribute::default()}
    }
    pub fn new_with_vec3(pos:&Vec3)->Self{
        Point{id:-1, pos:Vec3::new_with_vec3(pos),attr:Attribute::default()}
    }
    pub fn set_id(&mut self, id:i32){
        self.id = id;
    }
    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Point{
        self.attr.color.set_rgba(r,g,b,a);
        self
    }
    pub fn hsb(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Point{
        self.attr.color.set_hsb(h,s,b,a);
        self
    }
    pub fn set_attr(&mut self, attr:&Attribute)->&mut Point{
        self.attr.set(attr);
        self
    }

}

/*
impl Object for Point{
    fn draw(&mut self){    }
//    fn interact(&self, objects:&Vec<Box<dyn Object>>){}
//    fn update(&self){}
//    fn attr(&self)->&mut Attribute{
//        &self.attr
//    }
}
*/


/***************************
* NURBS geometry
***************************/

#[derive(Debug, Clone)]
pub struct PolynomialFunction{
    degree: u8,
    coeff:Vec<f64>
}
impl PolynomialFunction{
    pub fn new(degree:u8, coeff:Vec<f64>)->Self{
        PolynomialFunction{degree, coeff}
    }

    pub fn new_with_function(func: &PolynomialFunction)->Self{
        PolynomialFunction{degree:func.degree, coeff:func.coeff.clone()}
    }

    pub fn eval(&self, x:f64)->f64{
        let mut retval : f64 = self.coeff[0];
        for i in 1..((self.degree+1) as usize){
            retval += self.coeff[i]*x.powi(i as i32);
        }
        retval
    }

    pub fn differentiate(&mut self){
        if self.degree==0{
            //return PolynomialFunction::new(0, Vec::from([0.0]));
            self.degree = 0;
            self.coeff = Vec::from([0.0]);
        }
        else{
            let mut coeff:Vec<f64> = Vec::new();
            for i in 0..self.degree as usize{
                coeff.push(self.coeff[i+1]*(i as f64+1.0));
            }
            //PolynomialFunction::new(self.degree-1, coeff)
            self.degree -= 1;
            self.coeff = coeff;
        }
    }

    pub fn mul(&mut self, a:f64){
        for i in 0..(self.degree+1){
            self.coeff[i as usize] *= a;
        }
    }
    pub fn add(&mut self, p: &PolynomialFunction){
        if p.degree > self.degree{
            self.degree = p.degree;
            for i in self.coeff.len()..p.coeff.len(){ //
                self.coeff.push(0.0);
            }
        }
        for i in 0..(p.degree+1) as usize{
            self.coeff[i] += p.coeff[i];
        }
    }

    pub fn mul_function(&mut self, p:&PolynomialFunction){
        let new_deg = self.degree+p.degree;
        let mut coeff:Vec<f64> = Vec::new();
        for i in 0..(new_deg+1){ coeff.push(0.0); }
        for i in 0..(self.degree+1) as usize{
            for j in 0..(p.degree+1) as usize{
                coeff[i+j] += self.coeff[i]*p.coeff[j];
            }
        }
        self.degree = new_deg;
        self.coeff = coeff;
    }

    pub fn to_string(&self)->String{
        let mut txt = String::from("PolynomialFunction{ degree: ");
        txt += &self.degree.to_string();
        txt += ", coef(";
        txt += &self.coeff.len().to_string();
        txt += ")[";
        for coef in &self.coeff{
            txt += &coef.to_string();
            txt += ",";
        }
        txt += "]";
        txt
    }

}

#[derive(Debug, Clone)]
pub struct PiecewiseFunction{
    domains:Vec<f64>,
    functions:Vec<PolynomialFunction>
}


impl PiecewiseFunction{
    pub fn new(domains:Vec<f64>, functions:Vec<PolynomialFunction>)->Self{
        PiecewiseFunction{domains, functions}
    }

    pub fn domain_index(&self, x:f64)->i32{
        if x < self.domains[0]{ return -1; }
        if x >= self.domains[self.domains.len()-1]{ return (self.domains.len()-1) as i32; }

        let mut min : usize = 0;
        let mut max : usize = self.domains.len()-1;
        let mut mid = (min+max)/2;

        while min<max && (x < self.domains[mid] || x>=self.domains[mid+1]){
            if x < self.domains[mid] { max = mid; }
            else{ min = mid; }
            mid = (min+max)/2;
        }
        mid as i32
    }

    pub fn eval(&self, x:f64)->f64{
        let idx = self.domain_index(x);
        self.functions[idx as usize].eval(x)
    }


}


pub struct BSplineBasisFunction{
    degree:u8,
    functions:Vec<Option<BSplineBasisSubFunction>>,
    knots:Vec<f64>,
}

impl BSplineBasisFunction{
    pub fn new(degree:u8, knots:Vec<f64>)->Self{
        let mut func : Vec<Option<BSplineBasisSubFunction>> = Vec::new();

        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}", degree, index, knots.len())));

        for i in 0..(knots.len()-degree as usize -1){
            func.push(Some(BSplineBasisSubFunction::new(degree,i as i32,knots.clone())));
        }
        BSplineBasisFunction{
            degree,
            functions:func,
            knots
        }
    }

    pub fn index(&self, x:f64)->usize{
        let mut min = self.degree as usize;
        let mut max = self.knots.len() -1-self.degree as usize;

        if x <= self.knots[min] { return min; }
        if x >= self.knots[max] { return max-1; }

        let mut mid = (min+max)/2;

        while min<max && (x < self.knots[mid] || x >= self.knots[mid+1]){
            if x < self.knots[mid]{
                max = mid;
            }
            else {
                min = mid;
            }
            mid = (min+max)/2;
        }
        return mid;
    }

    pub fn eval(&self, x:f64)->Vec<f64>{
        let index = self.index(x);
        self.eval_with_index(index, x)
    }

    pub fn eval_with_index(&self, index:usize, x:f64)->Vec<f64>{
        let mut val : Vec<f64> = Vec::new();
        for i in 0..(self.degree+1) as usize{
            if i+index-(self.degree as usize) >= 0 && i+index-(self.degree as usize)< self.functions.len(){
                val.push( self.functions[i+index-(self.degree as usize)].as_ref().unwrap().functions[(self.degree as usize)-i+1].as_ref().unwrap().eval(x));
            }
            else{
                val.push(0.0);
            }
        }
        val
    }

    pub fn differentiate(&mut self){
        for i in 0..self.functions.len(){
            self.functions[i].as_mut().unwrap().differentiate();
        }
    }

    pub fn to_string(&self)->String{
        let mut txt = String::from("BSplineBasisFunction{ degree: ");
        txt += &self.degree.to_string();
        txt += ", knots(";
        txt += &self.knots.len().to_string();
        txt += ")[";
        for k in &self.knots{
            txt += &k.to_string();
            txt += ",";
        }
        txt += "],\n";
        txt += "functions: [\n";
        for f in &self.functions{
            if f.is_none(){
                txt += "None,\n";
            }
            else{
                txt += &f.as_ref().unwrap().to_string();
                txt += ",\n";
            }
        }
        txt += "]\n";
        txt
    }

}


pub struct BSplineBasisSubFunction{
    domains:Vec<f64>,
    //functions:Vec<PolynomialFunction>,
    functions:Vec<Option<PolynomialFunction>>,
    degree: u8,
    index: i32,
    knots: Vec<f64>
}

impl BSplineBasisSubFunction{
    pub fn new(degree:u8, index:i32, knots:Vec<f64>)->Self{

//        web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}", degree, index, knots.len())));

        if degree==0{
            let mut dom : Vec<f64> = Vec::new();
            for i in 0..2 { dom.push(knots[(index+i) as usize]); }
            let coeff : Vec<f64> = Vec::from([1.0]);
            let f : PolynomialFunction = PolynomialFunction::new(0, coeff);

//            web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, dom {}, returns", degree, index, knots.len(), dom.len())));
            return BSplineBasisSubFunction{ domains:dom, functions:Vec::from([None, Some(f), None]), degree, index, knots }
        }

        let mut bs1 = BSplineBasisSubFunction::new(degree-1, index, knots.clone());
        let mut bs2 = BSplineBasisSubFunction::new(degree-1, index+1, knots.clone());

        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} next", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));
        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction bs1deg:{}, bs1idx:{}, bs1knots{}, bs1dom {}, bs2deg {}, bs2idx {}, bs2knots {}, bs2dom {},  next", bs1.degree, bs1.index, bs1.knots.len(), bs1.domains.len(), bs2.degree, bs2.index, bs2.knots.len(), bs2.domains.len() )));

        let coeff1:[f64;2] = [
        -knots[index as usize]/(knots[(index+degree as i32) as usize] - knots[index as usize]),
        1.0/(knots[(index+degree as i32) as usize]-knots[index as usize]) ];
        let p1 = PolynomialFunction::new(1, Vec::from(coeff1));

        let coeff2:[f64;2] = [
        knots[(index+degree as i32+1)as usize]/(knots[(index+degree as i32+1) as usize]-knots[index as usize+1]),
        -1.0/(knots[(index+degree as i32+1)as usize] - knots[index as usize +1])];
        let p2 = PolynomialFunction::new(1, Vec::from(coeff2));

        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} mul1", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));
        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction bs1dom:{}, bs2dom:{},"", degree, index, knots.len())));

        bs1.mul(&p1);

        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} mul2", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));

        bs2.mul(&p2);

        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} add", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));

        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction: BEFORE_ADD: bs1deg:{}, bs1idx:{}, bs1knots{}, bs1dom {}, bs2deg {}, bs2idx {}, bs2knots {}, bs2dom {},  next", bs1.degree, bs1.index, bs1.knots.len(), bs1.domains.len(), bs2.degree, bs2.index, bs2.knots.len(), bs2.domains.len() )));

        bs1.add(&bs2);

        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} end", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));

        //web_sys::console::log_1(&JsValue::from(format!("BSplineBasisSubFunction: end of init: bs1deg:{}", bs1.degree))); //

        BSplineBasisSubFunction{
            domains:bs1.domains,
            functions:bs1.functions,
            degree:degree,
            index:index,
            knots:knots}

            /*
            degree:bs1.degree,
            index: bs1.index,
            knots:bs1.knots }*/
    }

    pub fn new_with_function(f:&BSplineBasisSubFunction)->Self{
        let mut func : Vec<Option<PolynomialFunction>> = Vec::new();
        for i in 0..f.functions.len(){
            func.push(Some(PolynomialFunction::new_with_function(f.functions[i].as_ref().unwrap())));
        }
        BSplineBasisSubFunction{
            domains:f.domains.clone(),
            functions:func,
            degree:f.degree,
            index: f.index,
            knots:f.knots.clone() }
    }

    pub fn add(&mut self, bs: &BSplineBasisSubFunction){
        if self.index > bs.index{
            let (new_domains, new_functions) = BSplineBasisSubFunction::add_domain_and_function(self.degree, bs, self);
            self.domains = new_domains;
            self.functions = new_functions;
        }
        else{
            let (new_domains, new_functions) = BSplineBasisSubFunction::add_domain_and_function(self.degree, self, bs);
            self.domains = new_domains;
            self.functions = new_functions;
        }
    }

    pub fn add_domain_and_function(degree: u8, bs1: &BSplineBasisSubFunction, bs2: &BSplineBasisSubFunction)->(Vec<f64>, Vec<Option<PolynomialFunction>>){

        let mut new_dom:Vec<f64> = Vec::new();
        let bs1deg = bs1.degree as i32;
        let bs2deg = bs2.degree as i32;
        let bs1domlen = bs1.domains.len() as i32;
        let bs2domlen = bs2.domains.len() as i32;

        /*
        for i in 0..bs1.functions.len(){
            if bs1.functions[i].is_none(){
                web_sys::console::log_1(&JsValue::from(format!("add_domain_and_function: bs1.functions[{}] : None", i)));
            }
            else{
                web_sys::console::log_1(&JsValue::from(format!("add_domain_and_function: bs1.functions[{}] : Not None", i)));
            }
        }
        for i in 0..bs2.functions.len(){
            if bs2.functions[i].is_none(){
                web_sys::console::log_1(&JsValue::from(format!("add_domain_and_function: bs2.functions[{}] : None", i)));
            }
            else{
                web_sys::console::log_1(&JsValue::from(format!("add_domain_and_function: bs2.functions[{}] : Not None", i)));
            }
        }
        */

        //web_sys::console::log_1(&JsValue::from(format!("add_domain_and_function: 0 bs1.deg {} ,  bs1.index {}, bs2.deg {}, bs2.index {}", &bs1.degree, &bs1.index, &bs2.degree, &bs2.index )));
        //web_sys::console::log_1(&JsValue::from(format!("add_domain_and_function: bs1deg:{}, bs1idx:{}, bs1knots{}, bs1dom {}, bs2deg {}, bs2idx {}, bs2knots {}, bs2dom {},  next", bs1.degree, bs1.index, bs1.knots.len(), bs1.domains.len(), bs2.degree, bs2.index, bs2.knots.len(), bs2.domains.len() )));

        /*
        for j in 0..bs1.functions.len(){
            if bs1.functions[j].is_none(){
                web_sys::console::log_1(&JsValue::from(format!("add:: bs1.functions[{}]: None", j)));
            }
            else{
                web_sys::console::log_1(&JsValue::from(format!("add:: bs1.functions[{}]: Not None", j)));
            }
        }
        for j in 0..bs2.functions.len(){
            if bs2.functions[j].is_none(){
                web_sys::console::log_1(&JsValue::from(format!("add:: bs2.functions[{}]: None", j)));
            }
            else{
                web_sys::console::log_1(&JsValue::from(format!("add:: bs2.functions[{}]: Not None", j)));
            }
        }
        */

        //for i in 0..(bs1deg + 2 + bs2.index-bs1.index) {
        for i in 0..(degree as i32 + 2 + bs2.index-bs1.index) {
            new_dom.push(0.0);
        }
        let mut new_func:Vec<Option<PolynomialFunction>> = Vec::new();
        for i in 0..(degree as i32 + 3 + bs2.index-bs1.index){
            new_func.push(None);
        }

        let mut i : usize = 0;
        while i<bs1domlen as usize{
            new_dom[i] = bs1.domains[i];
            i+=1;
        }

//        web_sys::console::log_1(&JsValue::from(format!("bs1deg {} , bs2deg {}",bs1deg, bs2deg)));

//        web_sys::console::log_1(&JsValue::from(format!("add_domain_and_function: new_dom.len {} , bs1.dom.len {}, bs2.dom.len {}", new_dom.len(), bs1.domains.len(), bs2.domains.len())));
//        web_sys::console::log_1(&JsValue::from(format!("i {} , bs1.index {}, bs2.index {}", i, bs1.index, bs2.index)));

        while i< ((bs2domlen + bs2.index - bs1.index) as usize){

            new_dom[i] = bs2.domains[i - ((bs2.index-bs1.index) as usize)];
            i+=1;
        }

        i = 0;
        while i<((bs2.index-bs1.index) as usize){
            if bs1.functions[i].is_none() {
                new_func[i] = None;
            }
            else{
                let func = PolynomialFunction::new_with_function(&bs1.functions[i].as_ref().unwrap());
                new_func[i] = Some(func);
            }
            i+=1;
        }
        while i<bs1.functions.len(){
            if !bs1.functions[i].is_none() && !bs2.functions[(i as i32 - bs2.index + bs1.index) as usize].is_none() {
                let mut func = PolynomialFunction::new_with_function(&bs1.functions[i].as_ref().unwrap());
                func.add(&bs2.functions[(i as i32 - bs2.index + bs1.index)as usize].as_ref().unwrap());
                new_func[i] = Some(func);
            }
            else if bs1.functions[i].is_none() && !bs2.functions[(i as i32 - bs2.index + bs1.index) as usize].is_none(){
                let func = PolynomialFunction::new_with_function(&bs2.functions[(i as i32 - bs2.index + bs1.index) as usize].as_ref().unwrap());
                new_func[i] = Some(func);
            }
            else if !bs1.functions[i].is_none() && bs2.functions[(i as i32 - bs2.index + bs1.index)as usize].is_none(){
                let func = PolynomialFunction::new_with_function(&bs1.functions[i].as_ref().unwrap());
                new_func[i] = Some(func);
            }
            else{
                new_func[i] = None;
            }
            i+=1;
        }
        while i < (bs2.functions.len() as i32 + bs2.index - bs1.index) as usize{
            if bs2.functions[(i as i32 - bs2.index + bs1.index)as usize].is_none(){
                new_func[i] = None;
            }
            else{
                let func = PolynomialFunction::new_with_function(&bs2.functions[(i as i32 - bs2.index + bs1.index)as usize].as_ref().unwrap());
                new_func[i] = Some(func);
            }
            i+=1;
        }
        /*
        for j in 0..new_func.len(){
            if new_func[j].is_none(){
                web_sys::console::log_1(&JsValue::from(format!("new_func[{}]: None", j)));
            }
            else{
                web_sys::console::log_1(&JsValue::from(format!("new_func[{}]: Not None", j)));
            }
        }
        */
        (Vec::from(new_dom), Vec::from(new_func))
    }


    pub fn mul(&mut self, p:&PolynomialFunction){
        for i in 0..self.functions.len(){
            if !self.functions[i].is_none(){
                //self.functions[i].unwrap().mul_function(p);
                self.functions[i].as_mut().unwrap().mul_function(p);
            }
        }
    }

    pub fn differentiate(&mut self){
        for i in 0..self.functions.len(){
            if !self.functions[i].is_none(){
                self.functions[i].as_mut().unwrap().differentiate();
            }
        }
    }


    pub fn to_string(&self)->String{
        let mut txt = String::from("BSplineBasisSubFunction{ degree: ");
        txt += &self.degree.to_string();

        txt += ", index=";
        txt += &self.index.to_string();

        txt += ", domains(";
        txt += &self.domains.len().to_string();
        txt += ")[";
        for d in &self.domains{
            txt += &d.to_string();
            txt += ",";
        }
        txt += "],\n";

        txt += ", knots(";
        txt += &self.knots.len().to_string();
        txt += ")[";
        for k in &self.knots{
            txt += &k.to_string();
            txt += ",";
        }
        txt += "],\n";
        txt += "functions: [\n";
        for f in &self.functions{
            if f.is_none(){
                txt += "None,\n";
            }
            else{
                txt +=& f.as_ref().unwrap().to_string();
                txt += ",\n";
            }
        }
        txt += "]";
        txt
    }


}




/************************
* Vector Math
************************/
#[derive(Debug, Clone, Copy)]
pub struct Vec3{
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl Vec3{
    /*
    pub fn new() -> Self{
        Vec3{ x:0.0, y:0.0, z:0.0 }
    }
    */
    pub fn new(x:f64, y:f64, z:f64) -> Self{
        Vec3{ x, y, z }
    }

    pub fn zero() -> Self{
        Vec3{ x:0.0, y:0.0, z:0.0 }
    }

    pub fn new_with_vec3(v:&Vec3) -> Self{
        //Vec3{ x:v.x, y:v.y, z:v.z }
        Vec3{ ..*v }
    }

    pub fn clone(&self) ->Self{
        Vec3::new_with_vec3(self)
    }

    pub fn set(&mut self, v:&Vec3)->&mut Self{
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
        self
    }

    pub fn to_array(&self)->[f64;3]{
        [self.x,self.y,self.z]
    }

    pub fn to_array32(&self)->[f32;3]{
        [self.x as f32,self.y as f32,self.z as f32]
    }

    pub fn add(&mut self, v:&Vec3)->&mut Self{
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }



    pub fn sub(&mut self, v:&Vec3)->&mut Self{
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
        self
    }

    pub fn mul(&mut self, f:f64)->&mut Self{
        self.x *= f;
        self.y *= f;
        self.z *= f;
        self
    }

    pub fn div(&mut self, f:f64)->&mut Self{
        self.x /= f;
        self.y /= f;
        self.z /= f;
        self
    }

    pub fn neg(&mut self)->&mut Self{
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    pub fn dot(&self, v:&Vec3)->f64{
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v:&Vec3)->Self{
        Vec3{x: self.y*v.z-self.z*v.y, y:self.z*v.x-self.x*v.z, z:self.x*v.y-self.y*v.x}
    }

    pub fn icross(&mut self, v:&Vec3)->&mut Self{
        let x = self.y*v.z - self.z*v.y;
        let y = self.z*v.x-self.x*v.z;
        let z = self.x*v.y-self.y*v.x;
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn len2(&self) -> f64{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn len(&self)->f64{
        self.len2().sqrt()
    }

    pub fn set_len(&mut self, len:f64) ->&mut Self{
        self.mul(len/self.len())
    }
    pub fn unit(&mut self) ->&mut Self{
        let l = self.len();
        self.x /= l;
        self.y /= l;
        self.z /= l;
        self
    }

    pub fn dist2(&self, v:&Vec3)->f64{
        (self.x-v.x)*(self.x-v.x) + (self.y-v.y)*(self.y-v.y) + (self.z-v.z)*(self.z-v.z)
    }
    pub fn dist(&self, v:&Vec3)->f64{
        self.dist2(v).sqrt()
    }

    pub fn eq(&self, v:&Vec3)->bool{
        self.dist2(v) <= TOLERANCE*TOLERANCE
    }
    pub fn eq_x(&self, v:&Vec3)->bool{
        (self.x-v.x).abs() <= TOLERANCE
    }
    pub fn eq_y(&self, v:&Vec3)->bool{
        (self.y-v.y).abs() <= TOLERANCE
    }
    pub fn eq_z(&self, v:&Vec3)->bool{
        (self.z-v.z).abs() <= TOLERANCE
    }

    pub fn angle(&self, v:&Vec3)->f64{
        let len1 = self.len();
        if len1==0.0 { return 0.0; }
        let len2 = v.len();
        if len2==0.0 { return 0.0; }
        let mut cos = self.dot(v)/(len1*len2);
        if cos > 1.0 { cos = 1.0; } else if cos < -1.0 { cos=-1.0; }
        cos.acos()
    }

    pub fn angle_with_axis(&self, v:&Vec3, axis:&Vec3)->f64{
        let ang = self.angle(v);
        let crs = self.cross(v);
        if crs.dot(axis)<0.0 { return -ang; }
        ang
    }

    pub fn rot(&mut self, axis:&Vec3, angle:f64)->&mut Self{
        //null check of axis needed?
        let mut ax = axis.clone();
        ax.unit();
        let sin = angle.sin();
        let cos = angle.cos();
        let icos = 1.0-cos;

        // right-handed coordinates
        let m00 = ax.x*ax.x*icos + cos;
        let m01 = ax.x*ax.y*icos - ax.z*sin;
        let m02 = ax.x*ax.z*icos + ax.y*sin;
        let m10 = ax.y*ax.x*icos + ax.z*sin;
        let m11 = ax.y*ax.y*icos + cos;
        let m12 = ax.y*ax.z*icos - ax.x*sin;
        let m20 = ax.z*ax.x*icos - ax.y*sin;
        let m21 = ax.z*ax.y*icos + ax.x*sin;
        let m22 = ax.z*ax.z*icos + cos;

        let xt=self.x;
        let yt=self.y;
        self.x = m00*xt + m01*yt + m02*self.z;
        self.y = m10*xt + m11*yt + m12*self.z;
        self.z = m20*xt + m21*yt + m22*self.z;
        self
    }

    pub fn rot_with_center(&mut self, center:&Vec3, axis:&Vec3, angle:f64)->&mut Self{
        self.sub(center);
        self.rot(axis,angle);
        self.add(center)
    }

    pub fn rot2(&mut self, angle:f64)->&mut Self{
        self.rot(&Vec3::new(0.,0.,1.), angle)
    }

    pub fn rot2_with_center(&mut self, center:&Vec3, angle:f64)->&mut Self{
        self.sub(center);
        self.rot2(angle);
        self.add(center)
    }


    pub fn scale(&mut self, center:&Vec3, factor:f64) ->&mut Self{
        self.sub(center);
        self.mul(factor);
        self.add(center)
    }

    pub fn scale1d(&mut self, axis:&Vec3, factor:f64) ->&mut Self{
        let d = self.dot(axis)/axis.len2()*(factor-1.0);
        self.scaleAdd(d, axis)
    }

    pub fn scaleAdd(&mut self, factor:f64, v:&Vec3)->&mut Self{
        self.x += v.x*factor;
        self.y += v.y*factor;
        self.z += v.z*factor;
        self
    }

    pub fn mirror(&mut self, planeDir:&Vec3)->&mut Self{
        self.scaleAdd(self.dot(planeDir)/planeDir.len2()*-2.0, planeDir)
    }

    pub fn transform(&mut self, xvec:&Vec3, yvec:&Vec3, zvec:&Vec3) ->&mut Self{
        let tx = xvec.x*self.x + yvec.x*self.y + zvec.x*self.z;
        let ty = xvec.y*self.x + yvec.y*self.y + zvec.y*self.z;
        let tz = xvec.z*self.x + yvec.z*self.y + zvec.z*self.z;
        self.x = tx;
        self.y = ty;
        self.z = tz;
        self
    }

    pub fn transform_with_translate(&mut self, xvec:&Vec3, yvec:&Vec3, zvec:&Vec3, translate:&Vec3) ->&mut Self{
        self.transform(xvec,yvec,zvec);
        self.add(translate)
    }


    pub fn cp(&self, v:&Vec3)->Self{
        Vec3{x:self.x+v.x, y:self.y+v.y, z:self.z+v.z}
    }

    pub fn dif(&self, v:&Vec3)->Self{
        Vec3{x:self.x-v.x, y:self.y-v.y, z:self.z-v.z}
    }

    pub fn sum(&self, v:&Vec3)->Self{
        Vec3{x:self.x+v.x, y:self.y+v.y, z:self.z+v.z}
    }

    pub fn mid(&self, v:&Vec3)->Self{
        Vec3{x:(self.x+v.x)/2.0, y:(self.y+v.y)/2.0, z:(self.z+v.z)/2.0}
    }


    pub fn bisect(&self, v:&Vec3)->Self{
        let l1 = self.len();
        let l2 = v.len();
        Vec3{x:self.x/l1+v.x/l2, y:self.y/l1+v.y/l2, z:self.z/l1+v.z/l2}
    }

    pub fn intersect(line1Pt1:&Vec3, line1Pt2:&Vec3, line2Pt1:&Vec3, line2Pt2:&Vec3)->Option<Vec3>{
        if line1Pt1.eq(line2Pt1) || line1Pt1.eq(line2Pt2) { return Some(line1Pt1.clone()); }
        if line1Pt2.eq(line2Pt1) || line1Pt2.eq(line2Pt2) { return Some(line1Pt2.clone()); }

        let mut dir1 = line1Pt2.dif(line1Pt1);
        let mut dir2 = line2Pt2.dif(line2Pt1);

        let mut dif = line2Pt1.dif(line1Pt1);

        let mut op = dir1.cross(&dir2);
        let oplen = op.len();

        if oplen < TOLERANCE*TOLERANCE {
            dir1.unit();
            if dir1.mul(dif.dot(&dir1)).sub(&dif).len() > TOLERANCE{ return None; }
            return Some(line1Pt1.clone());
        }

        op.div(oplen);
        let gap = dif.dot(&op);
        if gap > TOLERANCE { return None; }

        dif.sub(op.mul(gap));

        dir1.unit();
        dir2.unit();

        let ip12 = dir1.dot(&dir2);
        let iip122 = 1.0-ip12*ip12;
        if iip122==0.0 { return None; }
        let ip1 = dif.dot(&dir1);
        let ip2 = dif.dot(&dir2);
        let ret = dir1.mul((ip1-ip2*ip12)/iip122).add(line1Pt1);
        return Some(*ret);
    }

    pub fn intersect_segment(line1Pt1:&Vec3, line1Pt2:&Vec3, line2Pt1:&Vec3, line2Pt2:&Vec3)->Option<Vec3>{
        if line1Pt1.eq(line2Pt1) || line1Pt1.eq(line2Pt2){ Some(line1Pt1.clone()); }
        if line1Pt2.eq(line2Pt1) || line1Pt2.eq(line2Pt2){ Some(line1Pt2.clone()); }

        let min1x = line1Pt1.x.min(line1Pt2.x);
        let min1y = line1Pt1.y.min(line1Pt2.y);
        let min1z = line1Pt1.z.min(line1Pt2.z);
        let max1x = line1Pt1.x.max(line1Pt2.x);
        let max1y = line1Pt1.y.max(line1Pt2.y);
        let max1z = line1Pt1.z.max(line1Pt2.z);
        let min2x = line2Pt1.x.min(line2Pt2.x);
        let min2y = line2Pt1.y.min(line2Pt2.y);
        let min2z = line2Pt1.z.min(line2Pt2.z);
        let max2x = line2Pt1.x.max(line2Pt2.x);
        let max2y = line2Pt1.y.max(line2Pt2.y);
        let max2z = line2Pt1.z.max(line2Pt2.z);

        // check bounding region
         if min1x > max2x + TOLERANCE || max1x < min2x - TOLERANCE ||
            min1y > max2y + TOLERANCE || max1y < min2y - TOLERANCE ||
            min1z > max2z + TOLERANCE || max1z < min2z - TOLERANCE { return None; }

         // judging by tolerance
         if line1Pt1.eq(&line2Pt1) { return Some(line1Pt1.clone()); }
         if line1Pt1.eq(&line2Pt2) { return Some(line1Pt1.clone()); }
         if line1Pt2.eq(&line2Pt1) { return Some(line1Pt2.clone()); }
         if line1Pt2.eq(&line2Pt2) { return Some(line1Pt2.clone()); }

         let mut dir1 = line1Pt2.dif(&line1Pt1);
         let mut dir2 = line2Pt2.dif(&line2Pt1);

         let mut dif = line2Pt1.dif(&line1Pt1);

         // optimizing by inlining & reuse variables in projectTo2Vec

         // project to a plane defined by v1 and v2
         let mut op = dir1.cross(&dir2);
         let oplen = op.len();

         if oplen < TOLERANCE*TOLERANCE { // parallel lines
             dir1.unit();
             if dir1.clone().mul(dif.dot(&dir1)).sub(&dif).len() > TOLERANCE {
                 return None; // too much gap
             }

             // now parallel and close but overwapping?
             let dif12 = line2Pt2.dif(&line1Pt1);
             let dif21 = line2Pt1.dif(&line1Pt2);
             let ip11 = dir1.dot(&dif);
             let ip12 = dir1.dot(&dif12);

             if  ip11 <= TOLERANCE && ip12 >= -TOLERANCE ||
                 ip11 >= -TOLERANCE && ip12 <= TOLERANCE { return Some(line1Pt1.clone()); }

             let ip21 = dir1.dot(&dif21);
             if ip11 >= -TOLERANCE && ip21 <= TOLERANCE ||
                ip11 <= TOLERANCE && ip21 >= -TOLERANCE { return Some(line2Pt1.clone()); }

             return None; // no overlap
         }

         op.div(oplen); // unitized
         let gap = dif.dot(&op);

         if gap > TOLERANCE{ return None; } // too much gap in vertical dir

         dif.sub(op.mul(gap));

         let len1 = dir1.len();
         let len2 = dir2.len();
         dir1.div(len1); // unitized
         dir2.div(len2); // unitized

         let ip12 = dir1.dot(&dir2);
         let iip122 = 1.0-ip12*ip12;
         if iip122==0.0 { return None; }
         let ip1 = dif.dot(&dir1);
         let ip2 = dif.dot(&dir2);

         let ilen1 = (ip1-ip2*ip12)/iip122;
         if ilen1 < -TOLERANCE || ilen1 > len1+TOLERANCE { return None; } // out of segment 1

         let ilen2 = (ip2-ip1*ip12)/iip122;
         if -ilen2 < -TOLERANCE || -ilen2 > len2+TOLERANCE { return None; } // out of segment 2

         let ret = dir1.mul(ilen1).add(&line1Pt1);
         return Some(*ret);
    }


    pub fn dist_to_plane(&self, plane_dir:&Vec3, plane_pt:&Vec3)->f64{
        let plen = plane_dir.len();
        if plen==0.0 { return self.dist(plane_pt); }
        (self.dif(plane_pt).dot(plane_dir)/plen).abs()
    }
    pub fn nml(&self, pt1:&Vec3, pt2:&Vec3)->Vec3{
        self.dif(pt1).cross(&self.dif(pt2))
    }
    pub fn is_on_plane(&self, pt1:&Vec3, pt2:&Vec3, pt3:&Vec3)->bool{
        self.is_on_plane_with_nml(&Vec3::get_normal(pt1,pt2,pt3), pt1)
    }
    pub fn is_on_plane_with_nml(&self, plane_dir:&Vec3, plane_pt:&Vec3)->bool{
        self.dist_to_plane(plane_dir, plane_pt) < TOLERANCE
    }

    pub fn is_flat(pt1:&Vec3, pt2:&Vec3, pt3:&Vec3, pt4:&Vec3)->bool{
        pt1.is_on_plane(pt2,pt3,pt4)
    }

    pub fn get_normal(pt1:&Vec3, pt2:&Vec3, pt3:&Vec3)->Vec3{
        pt1.nml(pt2,pt3)
    }
}

pub struct Vec4{
    //vec: Vec3,
    x:f64,
    y:f64,
    z:f64,
    w:f64
}


pub struct Matrix{}
impl Matrix{
    pub fn det(v11:f64, v12:f64, v21:f64, v22:f64)->f64{
        v11*v22-v12*v21
    }
}

pub struct Matrix3{
    val: [[f64;3];3],
}

impl Matrix3{
    pub fn new(v11:f64, v12:f64, v13:f64, v21:f64,v22:f64, v23:f64, v31:f64,v32:f64, v33:f64 )->Self{
        Matrix3{val:[[v11,v12,v13],[v21,v22,v23],[v31,v32,v33]]  }
    }

    pub fn new_with_matrix3(m:&Matrix3)->Self{
        Matrix3{
            val:[
            [m.val[0][0],m.val[0][1],m.val[0][2]],
            [m.val[1][0],m.val[1][1],m.val[1][2]],
            [m.val[2][0],m.val[2][1],m.val[2][2]]
            ]}
    }

    pub fn zero()->Self{
        Matrix3{
            val:[[0.0,0.0,0.0], [0.0,0.0,0.0], [0.0,0.0,0.0]]
        }
    }

    pub fn new_with_id()->Self{
        Matrix3{
            val:[[1.0,0.0,0.0], [0.0,1.0,0.0], [0.0,0.0,1.0]]
        }
    }

    pub fn set(&mut self, v11:f64, v12:f64, v13:f64, v21:f64,v22:f64, v23:f64, v31:f64,v32:f64, v33:f64 )->&mut Self{
        self.val[0][0] = v11; self.val[0][1] = v12; self.val[0][2] = v13;
        self.val[1][0] = v21; self.val[1][1] = v22; self.val[1][2] = v23;
        self.val[2][0] = v31; self.val[2][1] = v32; self.val[2][2] = v33;
        self
    }

    pub fn to_array(&self)->[f64;9]{
        [self.val[0][0],self.val[0][1],self.val[0][2],
        self.val[1][0],self.val[1][1],self.val[1][2],
        self.val[2][0],self.val[2][1],self.val[2][2]]
    }

    pub fn to_array32(&self)->[f32;9]{
        [self.val[0][0] as f32,self.val[0][1] as f32,self.val[0][2] as f32,
        self.val[1][0] as f32,self.val[1][1] as f32,self.val[1][2] as f32,
        self.val[2][0] as f32,self.val[2][1] as f32,self.val[2][2] as f32]
    }

    pub fn determinant(&self)->f64{
        self.val[0][0]*Matrix::det(self.val[1][1],self.val[1][2],self.val[2][1],self.val[2][2])+
        self.val[0][1]*Matrix::det(self.val[1][2],self.val[1][0],self.val[2][2],self.val[2][0])+
        self.val[0][2]*Matrix::det(self.val[1][0],self.val[1][1],self.val[2][0],self.val[2][1])
    }

    pub fn invert(&mut self)->&mut Self{
        let det = self.determinant();
        self.set(
            self.val[1][1]*self.val[2][2] - self.val[1][2]*self.val[2][1],
            self.val[0][2]*self.val[2][1] - self.val[0][1]*self.val[2][2],
            self.val[0][1]*self.val[1][2] - self.val[0][2]*self.val[1][1],

            self.val[1][2]*self.val[2][0] - self.val[1][0]*self.val[2][2],
            self.val[0][0]*self.val[2][2] - self.val[0][2]*self.val[2][0],
            self.val[0][2]*self.val[1][0] - self.val[0][0]*self.val[1][2],

            self.val[1][0]*self.val[2][1] - self.val[1][1]*self.val[2][0],
            self.val[0][1]*self.val[2][0] - self.val[0][0]*self.val[2][1],
            self.val[0][0]*self.val[1][1] - self.val[0][1]*self.val[1][0]
        );
        self
    }

    pub fn matmul(&mut self, m:&Matrix3)->&mut Self{
        for i in 0..3{
            let (mut v1, mut v2, mut v3) = (0.0, 0.0, 0.0);
            for j in 0..3{
                v1 += self.val[i][j] * m.val[j][0];
                v2 += self.val[i][j] * m.val[j][1];
                v3 += self.val[i][j] * m.val[j][2];
            }
            self.val[i][0] = v1;
            self.val[i][1] = v2;
            self.val[i][2] = v3;
        }
        self
    }


    pub fn vecmul(&self, v:&Vec3)->Vec3{
        Vec3{
            x:self.val[0][0]*v.x + self.val[0][1]*v.y + self.val[0][2]*v.z,
            y:self.val[1][0]*v.x + self.val[1][1]*v.y + self.val[1][2]*v.z,
            z:self.val[2][0]*v.x + self.val[2][1]*v.y + self.val[2][2]*v.z
        }
    }

    pub fn mul(&mut self, factor:f64)->&mut Self{
        for i in 0..3{ for j in 0..3{ self.val[i][j] *= factor; } }
        self
    }

    pub fn div(&mut self, factor:f64)->&mut Self{
        for i in 0..3{ for j in 0..3{ self.val[i][j] /= factor; } }
        self
    }

    pub fn set_zero(&mut self)->&mut Self{
        for i in 0..3{ for j in 0..3{ self.val[i][j] = 0.0; } }
        self
    }

    pub fn id(&mut self)->&mut Self{
        for i in 0..3{ for j in 0..3{ self.val[i][j] = if i==j {1.0 } else{ 0.0 } } }
        self
    }

    pub fn x_rotation(angle:f64)->Self{
        Matrix3::new(1.0, 0.0, 0.0,
            0.0, angle.cos(), -angle.sin(),
            0.0, angle.sin(), angle.cos())
    }
    pub fn y_rotation(angle:f64)->Self{
        Matrix3::new(angle.cos(), 0.0, angle.sin(),
            0.0, 1.0, 0.0,
            -angle.sin(), 0.0, angle.cos())
    }
    pub fn z_rotation(angle:f64)->Self{
        Matrix3::new(angle.cos(), -angle.sin(), 0.0,
            angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 1.0)
    }
    pub fn rotation(axis:&Vec3, angle:f64)->Self{
        let mut a = axis.clone();
        a.unit();
        let s = angle.sin();
        let c = angle.cos();
        let ic = 1.0-c;
        Matrix3::new(ic*a.x*a.x+c, a.x*a.y*ic-a.z*s, a.x*a.z*ic+a.y*s,
            a.x*a.y*ic+a.z*s, a.y*a.y*ic+c, a.y*a.z*ic-a.x*s,
            a.x*a.z*ic-a.y*s, a.y*a.z*ic+a.x*s, a.z*a.z*ic+c)
    }
    pub fn translate(p:&Vec3)->Self{
        Matrix3::new(1.0, 0.0, p.x,
            0.0, 1.0, p.y,
            0.0, 0.0, 1.0)
    }
}

impl fmt::Display for Vec3{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}


pub struct Matrix4{
    val: [[f64;4];4],

}

impl Matrix4{
    pub fn new(v11:f64,v12:f64,v13:f64,v14:f64, v21:f64,v22:f64, v23:f64,v24:f64, v31:f64,v32:f64, v33:f64,v34:f64, v41:f64, v42:f64, v43:f64, v44:f64 )->Self{
        Matrix4{val:[[v11,v12,v13,v14],[v21,v22,v23,v24],[v31,v32,v33,v34],[v41,v42,v43,v44]]  }
    }

    pub fn new_with_matrix4(m:&Matrix4)->Self{
        Matrix4{
            val:[
            [m.val[0][0],m.val[0][1],m.val[0][2],m.val[0][3]],
            [m.val[1][0],m.val[1][1],m.val[1][2],m.val[1][3]],
            [m.val[2][0],m.val[2][1],m.val[2][2],m.val[2][3]],
            [m.val[3][0],m.val[3][1],m.val[3][2],m.val[3][3]]
            ]}
    }

    pub fn zero()->Self{
        Matrix4{
            val:[[0.0,0.0,0.0,0.0], [0.0,0.0,0.0,0.0], [0.0,0.0,0.0,0.0], [0.0,0.0,0.0,0.0]]
        }
    }

    pub fn new_with_id()->Self{
        Matrix4{
            val:[[1.0,0.0,0.0,0.0], [0.0,1.0,0.0,0.0], [0.0,0.0,1.0,0.0], [0.0,0.0,0.0,1.0]]
        }
    }

    pub fn set(&mut self,
        v11:f64, v12:f64, v13:f64, v14:f64,
        v21:f64, v22:f64, v23:f64, v24:f64,
        v31:f64, v32:f64, v33:f64, v34:f64,
        v41:f64, v42:f64, v43:f64, v44:f64
    )->&mut Self{
        self.val[0][0] = v11; self.val[0][1] = v12; self.val[0][2] = v13; self.val[0][3] = v14;
        self.val[1][0] = v21; self.val[1][1] = v22; self.val[1][2] = v23; self.val[1][3] = v24;
        self.val[2][0] = v31; self.val[2][1] = v32; self.val[2][2] = v33; self.val[2][3] = v34;
        self.val[3][0] = v41; self.val[3][1] = v42; self.val[3][2] = v43; self.val[3][3] = v44;
        self
    }

    pub fn set_with_matrix4(&mut self, m:&Matrix4)->&mut Self{
        self.val[0][0] = m.val[0][0]; self.val[0][1] = m.val[0][1]; self.val[0][2] = m.val[0][2]; self.val[0][3] = m.val[0][3];
        self.val[1][0] = m.val[1][0]; self.val[1][1] = m.val[1][1]; self.val[1][2] = m.val[1][2]; self.val[1][3] = m.val[1][3];
        self.val[2][0] = m.val[2][0]; self.val[2][1] = m.val[2][1]; self.val[2][2] = m.val[2][2]; self.val[2][3] = m.val[2][3];
        self.val[3][0] = m.val[3][0]; self.val[3][1] = m.val[3][1]; self.val[3][2] = m.val[3][2]; self.val[3][3] = m.val[3][3];
        self
    }

    pub fn to_array(&self)->[f64;16]{
        [self.val[0][0],self.val[0][1],self.val[0][2],self.val[0][3],
        self.val[1][0],self.val[1][1],self.val[1][2],self.val[1][3],
        self.val[2][0],self.val[2][1],self.val[2][2],self.val[2][3],
        self.val[3][0],self.val[3][1],self.val[3][2],self.val[3][3]]
    }

    pub fn to_array32(&self)->[f32;16]{
        [self.val[0][0] as f32,self.val[0][1] as f32,self.val[0][2] as f32,self.val[0][3] as f32,
        self.val[1][0] as f32,self.val[1][1] as f32,self.val[1][2] as f32,self.val[1][3] as f32,
        self.val[2][0] as f32,self.val[2][1] as f32,self.val[2][2] as f32,self.val[2][3] as f32,
        self.val[3][0] as f32,self.val[3][1] as f32,self.val[3][2] as f32,self.val[3][3] as f32]
    }

    pub fn determinant(&self)->f64{
        Matrix::det(self.val[0][0],self.val[0][1],self.val[1][0],self.val[1][1])*
        Matrix::det(self.val[2][2],self.val[2][3],self.val[3][2],self.val[3][3]) +
        Matrix::det(self.val[0][0],self.val[0][2],self.val[1][0],self.val[1][2])*
        Matrix::det(self.val[2][3],self.val[2][1],self.val[3][3],self.val[3][1]) +
        Matrix::det(self.val[0][0],self.val[0][3],self.val[1][0],self.val[1][3])*
        Matrix::det(self.val[2][1],self.val[2][2],self.val[3][1],self.val[3][2]) +
        Matrix::det(self.val[0][1],self.val[0][2],self.val[1][1],self.val[1][2])*
        Matrix::det(self.val[2][0],self.val[2][3],self.val[3][0],self.val[3][3]) +
        Matrix::det(self.val[0][3],self.val[0][1],self.val[1][3],self.val[1][1])*
        Matrix::det(self.val[2][0],self.val[2][2],self.val[3][0],self.val[3][2]) +
        Matrix::det(self.val[0][2],self.val[0][3],self.val[1][2],self.val[1][3])*
        Matrix::det(self.val[2][0],self.val[2][1],self.val[3][0],self.val[3][1])
    }

    pub fn invert(&mut self)->&mut Self{
        let det = self.determinant();

        self.set(
            self.val[1][1]*Matrix::det(self.val[2][2],self.val[2][3],self.val[3][2],self.val[3][3]) +
            self.val[1][2]*Matrix::det(self.val[2][3],self.val[2][1],self.val[3][3],self.val[3][1]) +
            self.val[1][3]*Matrix::det(self.val[2][1],self.val[2][2],self.val[3][1],self.val[3][2]),

            self.val[2][1]*Matrix::det(self.val[0][2],self.val[0][3],self.val[3][2],self.val[3][3])+
            self.val[2][2]*Matrix::det(self.val[0][3],self.val[0][1],self.val[3][3],self.val[3][1])+
            self.val[2][3]*Matrix::det(self.val[0][1],self.val[0][2],self.val[3][1],self.val[3][2]),

            self.val[3][1]*Matrix::det(self.val[0][2],self.val[0][3],self.val[1][2],self.val[1][3])+
            self.val[3][2]*Matrix::det(self.val[0][3],self.val[0][1],self.val[1][3],self.val[1][1])+
            self.val[3][3]*Matrix::det(self.val[0][1],self.val[0][2],self.val[1][1],self.val[1][2]),

            self.val[0][1]*Matrix::det(self.val[1][3],self.val[1][2],self.val[2][3],self.val[2][2])+
            self.val[0][2]*Matrix::det(self.val[1][1],self.val[1][3],self.val[2][1],self.val[2][3])+
            self.val[0][3]*Matrix::det(self.val[1][2],self.val[1][1],self.val[2][2],self.val[2][1]),


            self.val[1][2]*Matrix::det(self.val[2][0],self.val[2][3],self.val[3][0],self.val[3][3]) +
            self.val[1][3]*Matrix::det(self.val[2][2],self.val[2][0],self.val[3][2],self.val[3][0]) +
            self.val[1][0]*Matrix::det(self.val[2][3],self.val[2][2],self.val[3][3],self.val[3][2]),

            self.val[2][2]*Matrix::det(self.val[0][0],self.val[0][3],self.val[3][0],self.val[3][3])+
            self.val[2][3]*Matrix::det(self.val[0][2],self.val[0][0],self.val[3][2],self.val[3][0])+
            self.val[2][0]*Matrix::det(self.val[0][3],self.val[0][2],self.val[3][3],self.val[3][2]),

            self.val[3][2]*Matrix::det(self.val[0][0],self.val[0][3],self.val[1][0],self.val[1][3])+
            self.val[3][3]*Matrix::det(self.val[0][2],self.val[0][0],self.val[1][2],self.val[1][0])+
            self.val[3][0]*Matrix::det(self.val[0][3],self.val[0][2],self.val[1][3],self.val[1][2]),

            self.val[0][2]*Matrix::det(self.val[1][3],self.val[1][0],self.val[2][3],self.val[2][0])+
            self.val[0][3]*Matrix::det(self.val[1][0],self.val[1][2],self.val[2][0],self.val[2][2])+
            self.val[0][0]*Matrix::det(self.val[1][2],self.val[1][3],self.val[2][2],self.val[2][3]),


            self.val[1][3]*Matrix::det(self.val[2][0],self.val[2][1],self.val[3][0],self.val[3][1]) +
            self.val[1][0]*Matrix::det(self.val[2][1],self.val[2][3],self.val[3][1],self.val[3][3]) +
            self.val[1][1]*Matrix::det(self.val[2][3],self.val[2][0],self.val[3][3],self.val[3][0]),

            self.val[2][3]*Matrix::det(self.val[0][0],self.val[0][1],self.val[3][0],self.val[3][1])+
            self.val[2][0]*Matrix::det(self.val[0][1],self.val[0][3],self.val[3][1],self.val[3][3])+
            self.val[2][1]*Matrix::det(self.val[0][3],self.val[0][0],self.val[3][3],self.val[3][0]),

            self.val[3][3]*Matrix::det(self.val[0][0],self.val[0][1],self.val[1][0],self.val[1][1])+
            self.val[3][0]*Matrix::det(self.val[0][1],self.val[0][3],self.val[1][1],self.val[1][3])+
            self.val[3][1]*Matrix::det(self.val[0][3],self.val[0][0],self.val[1][3],self.val[1][0]),

            self.val[0][3]*Matrix::det(self.val[1][1],self.val[1][0],self.val[2][1],self.val[2][0])+
            self.val[0][0]*Matrix::det(self.val[1][3],self.val[1][1],self.val[2][3],self.val[2][1])+
            self.val[0][1]*Matrix::det(self.val[1][0],self.val[1][3],self.val[2][0],self.val[2][3]),


            self.val[1][0]*Matrix::det(self.val[2][2],self.val[2][1],self.val[3][2],self.val[3][1]) +
            self.val[1][1]*Matrix::det(self.val[2][0],self.val[2][2],self.val[3][0],self.val[3][2]) +
            self.val[1][2]*Matrix::det(self.val[2][1],self.val[2][0],self.val[3][1],self.val[3][0]),

            self.val[2][0]*Matrix::det(self.val[0][2],self.val[0][1],self.val[3][2],self.val[3][1])+
            self.val[2][1]*Matrix::det(self.val[0][0],self.val[0][2],self.val[3][0],self.val[3][2])+
            self.val[2][2]*Matrix::det(self.val[0][1],self.val[0][0],self.val[3][1],self.val[3][0]),

            self.val[3][0]*Matrix::det(self.val[0][2],self.val[0][1],self.val[1][2],self.val[1][1])+
            self.val[3][1]*Matrix::det(self.val[0][0],self.val[0][2],self.val[1][0],self.val[1][2])+
            self.val[3][2]*Matrix::det(self.val[0][1],self.val[0][0],self.val[1][1],self.val[1][0]),

            self.val[0][0]*Matrix::det(self.val[1][1],self.val[1][2],self.val[2][1],self.val[2][2])+
            self.val[0][1]*Matrix::det(self.val[1][2],self.val[1][0],self.val[2][2],self.val[2][0])+
            self.val[0][2]*Matrix::det(self.val[1][0],self.val[1][1],self.val[2][0],self.val[2][1])
        );

        self.div(det);
        self
    }

    pub fn transpose(&mut self)->&mut Self{
        self.set(
            self.val[0][0],  self.val[1][0], self.val[2][0], self.val[3][0],
            self.val[0][1],  self.val[1][1], self.val[2][1], self.val[3][1],
            self.val[0][2],  self.val[1][2], self.val[2][2], self.val[3][2],
            self.val[0][3],  self.val[1][3], self.val[2][3], self.val[3][3],
        );
        self
    }

    pub fn matmul(&mut self, m:&Matrix4)->&mut Self{
        for i in 0..4{
            let (mut v1, mut v2, mut v3, mut v4) = (0.0, 0.0, 0.0, 0.0);
            for j in 0..4{
                v1 += self.val[i][j] * m.val[j][0];
                v2 += self.val[i][j] * m.val[j][1];
                v3 += self.val[i][j] * m.val[j][2];
                v4 += self.val[i][j] * m.val[j][3];
            }
            self.val[i][0] = v1;
            self.val[i][1] = v2;
            self.val[i][2] = v3;
            self.val[i][3] = v4;
        }
        self
    }


    pub fn vecmul(&self, v:&Vec4)->Vec4{
        Vec4{
            x:self.val[0][0]*v.x + self.val[0][1]*v.y + self.val[0][2]*v.z + self.val[0][3]*v.w,
            y:self.val[1][0]*v.x + self.val[1][1]*v.y + self.val[1][2]*v.z + self.val[1][3]*v.w,
            z:self.val[2][0]*v.x + self.val[2][1]*v.y + self.val[2][2]*v.z + self.val[2][3]*v.w,
            w:self.val[3][0]*v.x + self.val[3][1]*v.y + self.val[3][2]*v.z + self.val[3][3]*v.w,
        }
    }

    pub fn mul(&mut self, factor:f64)->&mut Self{
        for i in 0..4{ for j in 0..4{ self.val[i][j] *= factor; } }
        self
    }

    pub fn div(&mut self, factor:f64)->&mut Self{
        for i in 0..4{ for j in 0..4{ self.val[i][j] /= factor; } }
        self
    }

    pub fn set_zero(&mut self)->&mut Self{
        for i in 0..4{ for j in 0..4{ self.val[i][j] = 0.0; } }
        self
    }

    pub fn id(&mut self)->&mut Self{
        for i in 0..4{ for j in 0..4{ self.val[i][j] = if i==j {1.0 } else{ 0.0 } } }
        self
    }

    pub fn x_rotation(angle:f64)->Self{
        Matrix4::new(1.0, 0.0, 0.0, 0.0,
            0.0, angle.cos(), -angle.sin(), 0.0,
            0.0, angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn y_rotation(angle:f64)->Self{
        Matrix4::new(angle.cos(), 0.0, angle.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -angle.sin(), 0.0, angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn z_rotation(angle:f64)->Self{
        Matrix4::new(angle.cos(), -angle.sin(), 0.0, 0.0,
            angle.sin(), angle.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn rotation(axis:&Vec3, angle:f64)->Self{
        let mut a = axis.clone();
        a.unit();
        let s = angle.sin();
        let c = angle.cos();
        let ic = 1.0-c;
        Matrix4::new(ic*a.x*a.x+c, a.x*a.y*ic-a.z*s, a.x*a.z*ic+a.y*s, 0.0,
            a.x*a.y*ic+a.z*s, a.y*a.y*ic+c, a.y*a.z*ic-a.x*s, 0.0,
            a.x*a.z*ic-a.y*s, a.y*a.z*ic+a.x*s, a.z*a.z*ic+c, 0.0,
            0.0, 0.0, 0.0, 0.0)
    }
    pub fn translate(p:&Vec3)->Self{
        Matrix4::new(1.0, 0.0, 0.0, p.x,
            0.0, 1.0, 0.0, p.y,
            0.0, 0.0, 1.0, p.z,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn perspective(aspect:f64, fovy:f64, near:f64, far:f64)->Self{
        let f:f64 = 1.0/((fovy/2.0).tan());
        Matrix4::new(
            f/aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, -far/(far-near), -1.0,
            0.0, 0.0, -far*near/(far-near), 0.0)
    }

    pub fn look_at(from: &Vec3, to: &Vec3, up: &Vec3)->Self{
        let mut front = from.dif(to);
        front.unit().unit();
        let mut side = up.cross(&front);
        side.unit();
        let mut up2 = front.cross(&side);
        up2.unit();

        let d1 = -side.dot(from);
        let d2 = -up2.dot(from);
        let d3 =  -front.dot(from);

        Matrix4::new(
            side.x, up2.x, front.x, 0.0,
            side.y, up2.y, front.y, 0.0,
            side.z, up2.z, front.z, 0.0,
            d1, d2, d3, 1.0,
        )
    }

    pub fn scale(f:f64)->Self{
        Matrix4::new(f, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, f, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn transform(xvector:&Vec3, yvector:&Vec3, zvector:&Vec3, translate:&Vec3)->Self{
        Matrix4::new(xvector.x, yvector.x, zvector.x, translate.x,
            xvector.y, yvector.y, zvector.y, translate.y,
            xvector.z, yvector.z, zvector.z, translate.z,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn convert(xvec1:&Vec3, yvec1:&Vec3, zvec1:&Vec3, orig1:&Vec3,
        xvec2:&Vec3, yvec2:&Vec3, zvec2:&Vec3, orig2:&Vec3)->Self{
            let mut mat1 = Matrix4::transform(xvec1,yvec1,zvec1,orig1);
            let mut mat2 = Matrix4::transform(xvec2,yvec2,zvec2,orig2);
            mat1.invert();
            mat2.matmul(&mat1);
            Matrix4::new_with_matrix4(&mat2)
    }

}
