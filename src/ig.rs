use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::console::log_1;
use std::fmt;
use std::f64::consts::PI;

/****************************
* constant
*****************************/

pub const TOLERANCE : f64 = 0.001;
#[allow(dead_code)]
pub const ANGLE_TOLERANCE : f64 = std::f64::consts::PI/1000.0;

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
    time:i64,
    duration:i64,
    max_agent_limit:i32,
}

impl/*<'a>*/ Server/*<'a>*/{
    #[allow(dead_code)]
    pub fn new(width:f32, height:f32) -> Self{
        Server{
            //objects: Vec::new(),
            storage:DataStorage::new(),
            //graphic_server: GraphicServer::new(width,height),
            //dynamic_server: DynamicServer::new(),
            glserver:WebGlServer::new(width,height),
            time: 0,
            duration : -1,
            max_agent_limit:-1
        }
    }

    #[allow(dead_code)]
    pub fn add_object(&mut self, object:Box<dyn Object>)->usize{
        self.storage.add_object(object)
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
    pub fn add_agent(&mut self, agent:Box<Agent>)->usize{
        if self.max_agent_limit >= 0 && self.storage.agents.len() as i32 >= self.max_agent_limit{
            return 0; // shouldn't this be -1?
        }
        let id = self.storage.add_agent(agent);
        //agent.set_id(id);
        id
    }

    #[allow(dead_code)]
    pub fn delete_data(&mut self, mut deleting_data: DataStorage){
        if deleting_data.point_index.len() > 0{
            deleting_data.sort_point_index();
            for i in 0..deleting_data.point_index.len(){
                //log_1(&JsValue::from(format!("Server::delete_data: deleting point index {}", deleting_data.point_index[i]))); //
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
                //log_1(&JsValue::from(format!("Server::delete_data: deleting curve index {}", deleting_data.curve_index[i]))); //
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
                //log_1(&JsValue::from(format!("Server::delete_date: deleting surface index {}", deleting_data.surface_index[i]))); //
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
                //log_1(&JsValue::from(format!("Server::delete_date: deleting agent index {}", deleting_data.agent_index[i]))); //
                self.storage.delete_agent(deleting_data.agent_index[i] );
            }
        }

        //log_1(&JsValue::from(format!("1 adding point num{}", manager.adding_data.points.len())));

    }

    #[allow(dead_code)]
    pub fn add_data(&mut self, adding_data: DataStorage){
        //log_1(&JsValue::from(format!("Server::add_data"))); //
        for o in adding_data.objects{
            self.add_object(o);
        }
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


            if self.storage.agents.len() > 0{
                log_1(&JsValue::from(format!("Server::agents.len()={}", self.storage.agents.len())));
            }

            if self.max_agent_limit < 0 || (self.storage.agents.len() as i32) < self.max_agent_limit {
                let mut agents_copy : Vec<Box<Agent>> = Vec::new();
                for i in 0..self.storage.agents.len(){
                    agents_copy.push(Box::new((*self.storage.agents[i]).clone()));
                }

                for i in 0..self.storage.agents.len(){
                    for j in 0..self.storage.agents.len(){
                        if i!=j{
                            //self.storage.agents[i].interact(agents[j], &mut mgr);
                            Agent::interact(&mut self.storage.agents[i], &agents_copy, &mut mgr)
                        }
                    }
                }

                //log_1(&JsValue::from(format!("Server: end of interact")));

                for i in 0..self.storage.agents.len(){
                    self.storage.agents[i].update(&mut mgr);
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

pub struct GraphicServer/*<'a>*/{
    #[allow(dead_code)]
    graphics3d: Vec<Graphic>,
    glserver: WebGlServer/*<'a>*/,
    //server: Option<&'a Server<'a>>,
}

impl/*<'a>*/ GraphicServer/*<'a>*/{
    #[allow(dead_code)]
    pub fn new(width:f32,height:f32) -> Self{
        GraphicServer{
            graphics3d: Vec::new(),
            glserver: WebGlServer::new(width, height),
            //server:None
        }
    }
    #[allow(dead_code)]
    pub fn init(&mut self/*, serv:&'a Server<'a> */){
        //self.server = Some(serv);
        self.glserver.init(/*serv*/);
    }
    #[allow(dead_code)]
    pub fn draw(&mut self){
        self.glserver.draw();
    }
}

#[allow(dead_code)]
pub struct DynamicServer/*<'a>*/{
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

impl/*<'a>*/ DynamicServer/*<'a>*/{
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
    pub fn init(&mut self/*, serv: &'a Server<'a>*/){
        //self.server=Some(serv);
    }
    #[allow(dead_code)]
    pub fn update(&mut self){

    }
    #[allow(dead_code)]
    pub fn time(&self)->i64{
        self.time
    }
}





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
        if gl.get_program_parameter(&self.program.as_ref().unwrap(), WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
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
pub const TESSELLATION_RESOLUTION : u32 = 8; //10; //5; //10; //5;

pub const INSERT_POINT_ON_DEGREE1_TWISTED_SURFACE : bool = true;

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
    pub orient:Orient,
    pub time:i64,
    pub colliding:bool,
    pub vecs:Vec<Vec3>,
    pub params:Vec<f64>,
    pub attr:Attribute,
}


impl Agent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3)->Self{
        Agent{id:-1, pos, vel:Vec3::zero(), frc:Vec3::zero(), fric:0.0, dir:Vec3::zero(), nml:Vec3::zero(), orient:Orient::new(Vec3::new(1.0,0.0,0.0),Vec3::new(0.0,0.0,1.0)), time:0, colliding:false, vecs:Vec::new(), params:Vec::new(), attr:Attribute::default() }
    }
    #[allow(dead_code)]
    pub fn new_with_dir(pos:Vec3, dir:Vec3)->Self{
        Agent{id:-1, pos, vel:Vec3::zero(), frc:Vec3::zero(), fric:0.0, dir, nml:Vec3::zero(), orient:Orient::new(Vec3::new(1.0,0.0,0.0),Vec3::new(0.0,0.0,1.0)), time:0, colliding:false, vecs:Vec::new(), params:Vec::new(), attr:Attribute::default() }
    }
    #[allow(dead_code)]
    pub fn new_with_orient(pos:Vec3, orient:Orient)->Self{
        Agent{id:-1, pos, vel:Vec3::zero(), frc:Vec3::zero(), fric:0.0, dir:Vec3::zero(), nml:Vec3::zero(), orient , time:0, colliding:false, vecs:Vec::new(), params:Vec::new(), attr:Attribute::default() }
    }
    //fn init(&dyn self, server: &mut Server){ server.add_agent(Box::new(self)); }
    #[allow(dead_code)]
    pub fn set_id(&mut self, i:i32){ self.id = i; }
    #[allow(dead_code)]
    pub fn get_id(&mut self)->i32{ return self.id; }

    #[allow(dead_code)]
    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Agent{
        self.attr.color.set_rgba(r,g,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn hsb(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Agent{
        self.attr.color.set_hsb(h,s,b,a);
        self
    }
    #[allow(dead_code)]
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
            name: String::from("default"), layer_index:0,
            color: Color::new(0.5,0.5,0.5,1.0), stroke: Color::new(0.5,0.5,0.5,1.0),
            size: 5.0, weight: 1.0, visible:true
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, attr:&Attribute){
        self.name = attr.name.clone();
        self.layer_index = attr.layer_index;
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

pub struct Curve/*<'a>*/{
//    object: Object/*<'a>*/,
    pub id: i32,
    pub curve: CurveGeo,
    pub attr: Attribute

}

impl Curve{
    #[allow(dead_code)]
    pub fn new(cpts:Vec<Vec3>, degree:u8)->Self{
        Curve{ id:-1, curve:CurveGeo::new(cpts, degree),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn new_closed(cpts:Vec<Vec3>, degree:u8)->Self{
        Curve{ id:-1, curve:CurveGeo::new_closed(cpts, degree),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_knots(cpts:Vec<Vec3>, degree:u8, knots:Vec<f64>, ustart:f64, uend:f64)->Self{
        Curve{ id:-1, curve:CurveGeo::new_with_knots(cpts, degree, knots, ustart, uend),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_knots_and_weights(cpts:Vec<Vec3>, degree:u8, knots:Vec<f64>, weights:Vec<f64>, ustart:f64, uend:f64)->Self{
        Curve{ id:-1, curve:CurveGeo::new_with_knots_and_weights(cpts, degree, knots, weights, ustart, uend),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn polyline(cpts:Vec<Vec3>)->Self{
        Curve::new(cpts, 1)
    }
    #[allow(dead_code)]
    pub fn line(pt1:Vec3, pt2:Vec3)->Self{
        Curve::new(Vec::from([pt1,pt2]), 1)
    }

    #[allow(dead_code)]
    pub fn circle(center:&Vec3, normal:&Vec3, radius:f64)->Self{
        let (cpts, weights) = NurbsGeo::circle_cp(center, normal, radius);
        Curve::new_with_knots_and_weights(cpts, NurbsGeo::circle_deg(), NurbsGeo::circle_knots(), weights, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn circle_with_roll_dir(center:&Vec3, normal:&Vec3, roll_dir:&Vec3, radius:f64)->Self{
        let (cpts, weights) = NurbsGeo::circle_cp_with_roll_dir(center, normal, roll_dir, radius);
        Curve::new_with_knots_and_weights(cpts, NurbsGeo::circle_deg(), NurbsGeo::circle_knots(), weights, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn oval(center:&Vec3, xvec:&Vec3, yvec:&Vec3)->Self{
        let (cpts, weights) = NurbsGeo::oval_cp(center, xvec, yvec);
        Curve::new_with_knots_and_weights(cpts, NurbsGeo::circle_deg(), NurbsGeo::circle_knots(), weights, 0.0, 1.0)
    }



    #[allow(dead_code)]
    pub fn pt(&self, u:f64)->Vec3{
        self.curve.pt(u)
    }

    #[allow(dead_code)]
    pub fn tan(&self, u:f64)->Vec3{
        self.curve.tan(u)
    }

    #[allow(dead_code)]
    pub fn deg(&self)->u8{
        self.curve.deg()
    }
    #[allow(dead_code)]

    #[allow(dead_code)]
    pub fn num(&self)->usize{
        self.curve.num()
    }

    #[allow(dead_code)]
    pub fn ep_num(&self)->usize{
        self.curve.ep_num()
    }

    #[allow(dead_code)]
    pub fn cp(&self, i:usize)->Vec3{
        self.curve.cp(i)
    }

    #[allow(dead_code)]
    pub fn u(&self, ep_idx:usize, ep_frac:f64)->f64{
        self.curve.u(ep_idx, ep_frac)
    }

    #[allow(dead_code)]
    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Curve{
        self.attr.color.set_rgba(r,g,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn hsb(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Curve{
        self.attr.color.set_hsb(h,s,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn set_attr(&mut self, attr:&Attribute)->&mut Curve{
        self.attr.set(attr);
        self
    }


}


pub struct CurveGeo{
    cpts: Vec<Vec3>,
    degree: u8,
    knots: Vec<f64>,
    #[allow(dead_code)]
    ustart: f64,
    #[allow(dead_code)]
    uend: f64,
    weights:Vec<f64>,
    basis_function:BSplineBasisFunction,
    derivative_function: BSplineBasisFunction,
}

impl CurveGeo{
    #[allow(dead_code)]
    pub fn new_with_knots(cpts:Vec<Vec3>, degree:u8, knots:Vec<f64>, ustart:f64, uend:f64)->Self{
        let mut default_weights : Vec<f64> = Vec::new();
        #[allow(unused_variables)]
        for i in 0..cpts.len(){ default_weights.push(1.0); }
        CurveGeo::new_with_knots_and_weights(cpts,degree,knots,default_weights,ustart,uend)
    }

    #[allow(dead_code)]
    pub fn new_with_knots_and_weights(cpts:Vec<Vec3>, degree:u8, mut knots:Vec<f64>, weights:Vec<f64>, ustart:f64, uend:f64)->Self{
        if ustart != 0.0 || uend != 1.0{
            knots = NurbsGeo::normalize_knots(knots, &ustart, &uend);
        }
        let basis_function = BSplineBasisFunction::new(degree, knots.clone());
        let mut derivative_function = BSplineBasisFunction::new(degree, knots.clone());
        derivative_function.differentiate();

        CurveGeo{
            cpts, degree, knots, ustart, uend, weights, basis_function, derivative_function
        }
    }

    #[allow(dead_code)]
    pub fn new(cpts:Vec<Vec3>, degree:u8)->Self{
        let knots = NurbsGeo::create_open_knots(degree, cpts.len());
        CurveGeo::new_with_knots(cpts, degree, knots, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn new_closed(cpts:Vec<Vec3>, degree:u8)->Self{
        let cpts2 = NurbsGeo::create_closed_cp(cpts, degree);
        let knots = NurbsGeo::create_closed_knots(degree, cpts2.len());
        CurveGeo::new_with_knots(cpts2, degree, knots, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn deg(&self)->u8{
        self.degree
    }

    #[allow(dead_code)]
    pub fn num(&self)->usize{
        self.cpts.len()
    }

    #[allow(dead_code)]
    pub fn ep_num(&self)->usize{
        self.knots.len() - 2*(self.degree as usize)
    }

    #[allow(dead_code)]
    pub fn cp(&self, i:usize)->Vec3{
        self.cpts[i]
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn u(&self, ep_idx:usize, ep_frac:f64)->f64{
        if ep_frac>=0.0{
            return self.knots[ep_idx+self.degree as usize] + (self.knots[ep_idx+self.degree as usize+1] - self.knots[ep_idx+self.degree as usize])*ep_frac;
        }
        return self.knots[ep_idx+self.degree as usize] + (self.knots[ep_idx+self.degree as usize] - self.knots[ep_idx+self.degree as usize-1])*ep_frac;
    }

    #[allow(dead_code)]
    pub fn pt(&self, u:f64)->Vec3{
        let index = self.basis_function.index(u);
        let n:Vec<f64> = self.basis_function.eval_with_index(index, u);
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

    #[allow(dead_code)]
    pub fn tan(&self, u:f64)->Vec3{
        let index = self.derivative_function.index(u);
        let dn : Vec<f64> = self.derivative_function.eval_with_index(index, u);
        let n : Vec<f64> = self.basis_function.eval_with_index(index, u);
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

    #[allow(dead_code)]
    pub fn normalize_knots(mut knots:Vec<f64>, ustart:&f64, uend:&f64)->Vec<f64>{
        for i in 0..knots.len(){
            knots[i] -= ustart;
            knots[i] /= uend-ustart;
        }
        knots
    }

    #[allow(dead_code)]
    pub fn create_knots(degree:u8, num:usize, closed:bool)->Vec<f64>{
        if closed{
            return NurbsGeo::create_closed_knots(degree, num);
        }
        NurbsGeo::create_open_knots(degree,num)
    }

    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn create_closed_cp_in_u(cpts:Vec<Vec<Vec3>>, udeg:u8)->Vec<Vec<Vec3>>{
        let head_num :usize = (udeg as usize -1)/2;
        let tail_num :usize = (udeg as usize)/2 +1;
        let mut ulen = cpts.len();
        let vlen = cpts[0].len();
        let mut is_edge_closed :bool = true;
        for i in 0..vlen{
            if !is_edge_closed{ break; }
            if !cpts[0][i].eq(&cpts[ulen-1][i]){
                is_edge_closed=false;
            }
        }
        if is_edge_closed{
            if udeg==1{
                return cpts;
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

    #[allow(dead_code)]
    pub fn create_closed_cp_in_v(cpts:Vec<Vec<Vec3>>, vdeg:u8)->Vec<Vec<Vec3>>{
        let head_num :usize = (vdeg as usize -1)/2;
        let tail_num :usize = (vdeg as usize)/2 +1;
        let ulen = cpts.len();
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
                return cpts;
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

    #[allow(dead_code)]
    pub fn circle_knots()->Vec<f64>{
        Vec::from([0.,0.,0.,0.25,0.25,0.5,0.5,0.75,0.75,1.,1.,1. ])
    }

    #[allow(dead_code)]
    pub fn circle_deg()->u8{ 2 }

    #[allow(dead_code)]
    pub fn circle_cp_with_xy_radius(center:&Vec3, normal:&Vec3, roll_dir:&Vec3, xradius:f64, yradius:f64)->(Vec<Vec3>, Vec<f64>){
        let mut roll_dir2 = roll_dir.clone();
        if roll_dir2.len2() == 0.0{
            roll_dir2 = Vec3::new(0.0,0.0,1.0);
        }
        else if normal.cross(&roll_dir2).len2() == 0.0{
            if roll_dir2.cross(&Vec3::new(0.0,0.0,1.0)).len2() == 0.0{
                roll_dir2 = Vec3::new(1.0,0.0,0.0);
            }
            else{
                roll_dir2 = Vec3::new(0.0,0.0,1.0);
            }
        }

        let mut yvec = normal.cross(&roll_dir2);
        let mut xvec = yvec.cross(normal);
        xvec.set_len(xradius);
        yvec.set_len(yradius);
        NurbsGeo::oval_cp(center, &xvec, &yvec)
    }

    #[allow(dead_code)]
    pub fn circle_cp_with_roll_dir(center:&Vec3, normal:&Vec3, roll_dir:&Vec3, radius:f64)->(Vec<Vec3>, Vec<f64>){
        NurbsGeo::circle_cp_with_xy_radius(center,normal,roll_dir,radius,radius)
    }

    #[allow(dead_code)]
    pub fn circle_cp(center:&Vec3, normal:&Vec3, radius:f64)->(Vec<Vec3>, Vec<f64>){
        let default_roll_dir = Vec3::new(1.0,0.0,0.0);
        NurbsGeo::circle_cp_with_xy_radius(center,normal,&default_roll_dir,radius,radius)
    }

    #[allow(dead_code)]
    pub fn oval_cp(center:&Vec3, xvec:&Vec3, yvec:&Vec3)->(Vec<Vec3>, Vec<f64>){
        let mut cpts : Vec<Vec3> = Vec::new();
        let mut weights : Vec<f64> = Vec::new();
        for i in 0..9{
            cpts.push(center.clone());
            if i%2 == 0{
                weights.push(1.0);
            }
            else{
                weights.push(2.0_f64.sqrt()/2.0);
            }
        }
        cpts[0].add(&xvec);
        cpts[1].add(&xvec).add(&yvec);
        cpts[2].add(&yvec);
        cpts[3].sub(&xvec).add(&yvec);
        cpts[4].sub(&xvec);
        cpts[5].sub(&xvec).sub(&yvec);
        cpts[6].sub(&yvec);
        cpts[7].add(&xvec).sub(&yvec);
        cpts[8].add(&xvec);
        (cpts, weights)
    }

}

pub struct Surface{
    #[allow(dead_code)]
    id: i32,
    surface: SurfaceGeo,
    pub attr: Attribute
}


impl Surface{
    #[allow(dead_code)]
    pub fn new(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new(cpts, udegree, vdegree), attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn new_u_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_u_closed(cpts, udegree, vdegree),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn new_v_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_v_closed(cpts, udegree, vdegree),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn new_uv_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_uv_closed(cpts, udegree, vdegree),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_knots(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8, uknots:Vec<f64>, vknots:Vec<f64>, ustart:f64, uend:f64, vstart:f64, vend:f64)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, ustart, uend, vstart, vend),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn quad(pt1:Vec3, pt2:Vec3, pt3:Vec3, pt4:Vec3)->Self{
        Surface::new(Vec::from([Vec::from([pt1,pt2]),Vec::from([pt4,pt3])]), 1, 1)
    }

    #[allow(dead_code)]
    pub fn triangle(pt1:Vec3, pt2:Vec3, pt3:Vec3)->Self{
        Surface::new(Vec::from([Vec::from([pt1,pt2]),Vec::from([pt3,pt3.clone()])]), 1, 1)
    }

    #[allow(dead_code)]
    pub fn square_pipe(pt1:Vec3, pt2:Vec3, size:f64)->Self{

        let dir = pt2.dif(&pt1);
        let mut t1 = dir.cross(&Vec3::new(0.0,0.0,1.0));
        if t1.len2()<TOLERANCE {
            t1.set(&dir.cross(&Vec3::new(1.0,0.0,0.0)));
        }
        let mut t2 = dir.cross(&t1);
        t1.set_len(size/2.0);
        t2.set_len(size/2.0);
        let pts : Vec<Vec<Vec3>> = vec![
        vec![ *pt1.dif(&t1).sub(&t2), *pt1.dif(&t1).add(&t2), *pt1.cp(&t1).add(&t2), *pt1.dif(&t1).add(&t1)],
        vec![ *pt2.dif(&t1).sub(&t2), *pt2.dif(&t1).add(&t2), *pt2.cp(&t1).add(&t2), *pt2.dif(&t1).add(&t1)]];
        Surface::new(pts, 1, 1)
    }

    #[allow(dead_code)]
    pub fn pt(&self, u:f64, v:f64)->Vec3{
        self.surface.pt(u,v)
    }

    #[allow(dead_code)]
    pub fn utan(&self, u:f64, v:f64)->Vec3{
        self.surface.utan(u, v)
    }

    #[allow(dead_code)]
    pub fn vtan(&self, u:f64, v:f64)->Vec3{
        self.surface.vtan(u, v)
    }

    #[allow(dead_code)]
    pub fn nml(&self, u:f64, v:f64)->Vec3{
        self.surface.nml(u, v)
    }

        #[allow(dead_code)]
    pub fn off_pt(&self, u:f64, v:f64, n:f64)->Vec3{
        self.surface.off_pt(u,v,n)
    }


    #[allow(dead_code)]
    pub fn udeg(&self)->u8{
        self.surface.udeg()
    }
    #[allow(dead_code)]
    pub fn vdeg(&self)->u8{
        self.surface.vdeg()
    }

    #[allow(dead_code)]
    pub fn unum(&self)->usize{
        self.surface.unum()
    }
    #[allow(dead_code)]
    pub fn vnum(&self)->usize{
        self.surface.vnum()
    }

    #[allow(dead_code)]
    pub fn uep_num(&self)->usize{
        self.surface.uep_num()
    }

    #[allow(dead_code)]
    pub fn vep_num(&self)->usize{
        self.surface.vep_num()
    }

    #[allow(dead_code)]
    pub fn cp(&self, i:usize, j:usize)->Vec3{
        self.surface.cp(i, j)
    }

    #[allow(dead_code)]
    pub fn u(&self, ep_idx:usize, ep_frac:f64)->f64{
        self.surface.u(ep_idx, ep_frac)
    }

    #[allow(dead_code)]
    pub fn v(&self, ep_idx:usize, ep_frac:f64)->f64{
        self.surface.v(ep_idx, ep_frac)
    }


    #[allow(dead_code)]
    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Surface{
        self.attr.color.set_rgba(r,g,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn hsb(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Surface{
        self.attr.color.set_hsb(h,s,b,a);
        self
    }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    ustart: f64,
    #[allow(dead_code)]
    uend: f64,
    #[allow(dead_code)]
    vstart: f64,
    #[allow(dead_code)]
    vend: f64,
    weights:Vec<Vec<f64>>,
    basis_function_u:BSplineBasisFunction,
    basis_function_v:BSplineBasisFunction,
    derivative_function_u: BSplineBasisFunction,
    derivative_function_v: BSplineBasisFunction,
}

impl SurfaceGeo{
    #[allow(dead_code)]
    pub fn new_with_knots(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8, mut uknots:Vec<f64>, mut vknots:Vec<f64>, ustart:f64, uend:f64, vstart:f64, vend:f64)->Self{
        if ustart != 0.0 || uend != 1.0{
            uknots = NurbsGeo::normalize_knots(uknots, &ustart, &uend);
        }
        if vstart != 0.0 || vend != 1.0{
            vknots = NurbsGeo::normalize_knots(vknots, &vstart, &vend);
        }
        let basis_function_u = BSplineBasisFunction::new(udegree, uknots.clone());
        let mut derivative_function_u = BSplineBasisFunction::new(udegree, uknots.clone());
        derivative_function_u.differentiate();
        let basis_function_v = BSplineBasisFunction::new(vdegree, vknots.clone());
        let mut derivative_function_v = BSplineBasisFunction::new(vdegree, vknots.clone());
        derivative_function_v.differentiate();

        let mut weights : Vec<Vec<f64>> = Vec::new();
        #[allow(unused_variables)]
        for i in 0..cpts.len(){
            let mut w : Vec<f64> = Vec::new();
            for j in 0..cpts[i].len(){
                w.push(1.0);
            }
            weights.push(w);
        }

        SurfaceGeo{
            cpts, udegree, vdegree, uknots, vknots, ustart, uend, vstart, vend, weights, basis_function_u, basis_function_v, derivative_function_u, derivative_function_v
        }
    }

    #[allow(dead_code)]
    pub fn new(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        let uknots = NurbsGeo::create_open_knots(udegree, cpts.len());
        let vknots = NurbsGeo::create_open_knots(vdegree, cpts[0].len());
        SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, 0.0, 1.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn new_uv_closed(mut cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        cpts = NurbsGeo::create_closed_cp_in_v(cpts, vdegree);
        cpts = NurbsGeo::create_closed_cp_in_u(cpts, udegree);

        let uknots = NurbsGeo::create_closed_knots(udegree, cpts.len());
        let vknots = NurbsGeo::create_closed_knots(vdegree, cpts[0].len());

        SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, 0.0, 1.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn new_u_closed(mut cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        cpts = NurbsGeo::create_closed_cp_in_u(cpts, udegree);

        let uknots = NurbsGeo::create_closed_knots(udegree, cpts.len());
        let vknots = NurbsGeo::create_open_knots(vdegree, cpts[0].len());

        SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, 0.0, 1.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn new_v_closed(mut cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        cpts = NurbsGeo::create_closed_cp_in_v(cpts, vdegree);

        let uknots = NurbsGeo::create_open_knots(udegree, cpts.len());
        let vknots = NurbsGeo::create_closed_knots(vdegree, cpts[0].len());

        SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, 0.0, 1.0, 0.0, 1.0)
    }



    #[allow(dead_code)]
    pub fn udeg(&self)->u8{
        self.udegree
    }

    #[allow(dead_code)]
    pub fn vdeg(&self)->u8{
        self.vdegree
    }

    #[allow(dead_code)]
    pub fn unum(&self)->usize{
        self.cpts.len()
    }

    #[allow(dead_code)]
    pub fn vnum(&self)->usize{
        self.cpts[0].len()
    }

    #[allow(dead_code)]
    pub fn uep_num(&self)->usize{
        self.uknots.len() - 2*(self.udegree as usize)
    }

    #[allow(dead_code)]
    pub fn vep_num(&self)->usize{
        self.vknots.len() - 2*(self.vdegree as usize)
    }

    #[allow(dead_code)]
    pub fn cp(&self, i:usize, j:usize)->Vec3{
        self.cpts[i][j]
    }

    #[allow(dead_code)]
    pub fn u(&self, ep_idx:usize, ep_frac:f64)->f64{
        if ep_frac>=0.0{
            return self.uknots[ep_idx+self.udegree as usize] + (self.uknots[ep_idx+self.udegree as usize+1] - self.uknots[ep_idx+self.udegree as usize])*ep_frac;
        }
        return self.uknots[ep_idx+self.udegree as usize] + (self.uknots[ep_idx+self.udegree as usize] - self.uknots[ep_idx+self.udegree as usize-1])*ep_frac;
    }

    #[allow(dead_code)]
    pub fn v(&self, ep_idx:usize, ep_frac:f64)->f64{
        if ep_frac>=0.0{
            return self.vknots[ep_idx+self.vdegree as usize] + (self.vknots[ep_idx+self.vdegree as usize+1] - self.vknots[ep_idx+self.vdegree as usize])*ep_frac;
        }
        return self.vknots[ep_idx+self.vdegree as usize] + (self.vknots[ep_idx+self.vdegree as usize] - self.vknots[ep_idx+self.vdegree as usize-1])*ep_frac;
    }

    #[allow(dead_code)]
    pub fn pt(&self, u:f64, v:f64)->Vec3{
        let uindex = self.basis_function_u.index(u);
        let vindex = self.basis_function_v.index(v);
        let nu:Vec<f64> = self.basis_function_u.eval_with_index(uindex, u);
        let nv:Vec<f64> = self.basis_function_v.eval_with_index(vindex, v);
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

    #[allow(dead_code)]
    pub fn utan(&self, u:f64, v:f64)->Vec3{
        let uindex = self.derivative_function_u.index(u);
        let vindex = self.derivative_function_v.index(v);
        let nu : Vec<f64> = self.basis_function_u.eval_with_index(uindex, u);
        let nv : Vec<f64> = self.basis_function_v.eval_with_index(vindex, v);
        let dnu : Vec<f64> = self.derivative_function_u.eval_with_index(uindex, u);
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

    #[allow(dead_code)]
    pub fn vtan(&self, u:f64, v:f64)->Vec3{
        let uindex = self.derivative_function_u.index(u);
        let vindex = self.derivative_function_v.index(v);
        let nu : Vec<f64> = self.basis_function_u.eval_with_index(uindex, u);
        let nv : Vec<f64> = self.basis_function_v.eval_with_index(vindex, v);
        let dnv : Vec<f64> = self.derivative_function_v.eval_with_index(vindex, v);
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn off_pt(&self, u:f64, v:f64, n:f64)->Vec3{
        let mut pt = self.pt(u,v);
        let mut nml = self.nml(u,v);
        pt.add(nml.set_len(n));
        pt
    }

}

pub struct Point/*<'a>*/{
//    object: Object,
    pub id: i32,
    pub pos: Vec3,
    pub attr: Attribute
}

impl Point{
    #[allow(dead_code)]
    pub fn new(x:f64, y:f64, z:f64)->Self{
        Point{id:-1, pos:Vec3::new(x,y,z),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_vec3(pos:&Vec3)->Self{
        Point{id:-1, pos:Vec3::new_with_vec3(pos),attr:Attribute::default()}
    }
    #[allow(dead_code)]
    pub fn set_id(&mut self, id:i32){
        self.id = id;
    }
    #[allow(dead_code)]
    pub fn clr(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Point{
        self.attr.color.set_rgba(r,g,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn hsb(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Point{
        self.attr.color.set_hsb(h,s,b,a);
        self
    }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(degree:u8, coeff:Vec<f64>)->Self{
        PolynomialFunction{degree, coeff}
    }

    #[allow(dead_code)]
    pub fn new_with_function(func: &PolynomialFunction)->Self{
        PolynomialFunction{degree:func.degree, coeff:func.coeff.clone()}
    }

    #[allow(dead_code)]
    pub fn eval(&self, x:f64)->f64{
        let mut retval : f64 = self.coeff[0];
        for i in 1..((self.degree+1) as usize){
            retval += self.coeff[i]*x.powi(i as i32);
        }
        retval
    }

    #[allow(dead_code)]
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


    #[allow(dead_code)]
    pub fn mul(&mut self, a:f64){
        for i in 0..(self.degree+1){
            self.coeff[i as usize] *= a;
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, p: &PolynomialFunction){
        if p.degree > self.degree{
            self.degree = p.degree;
            #[allow(unused_variables)]
            for i in self.coeff.len()..p.coeff.len(){ //
                self.coeff.push(0.0);
            }
        }
        for i in 0..(p.degree+1) as usize{
            self.coeff[i] += p.coeff[i];
        }
    }
    #[allow(dead_code)]
    pub fn mul_function(&mut self, p:&PolynomialFunction){
        let new_deg = self.degree+p.degree;
        let mut coeff:Vec<f64> = Vec::new();
        #[allow(unused_variables)]
        for i in 0..(new_deg+1){ coeff.push(0.0); }
        for i in 0..(self.degree+1) as usize{
            for j in 0..(p.degree+1) as usize{
                coeff[i+j] += self.coeff[i]*p.coeff[j];
            }
        }
        self.degree = new_deg;
        self.coeff = coeff;
    }

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(domains:Vec<f64>, functions:Vec<PolynomialFunction>)->Self{
        PiecewiseFunction{domains, functions}
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(degree:u8, knots:Vec<f64>)->Self{
        let mut func : Vec<Option<BSplineBasisSubFunction>> = Vec::new();

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}", degree, index, knots.len())));

        for i in 0..(knots.len()-degree as usize -1){
            func.push(Some(BSplineBasisSubFunction::new(degree,i as i32,knots.clone())));
        }
        BSplineBasisFunction{
            degree,
            functions:func,
            knots
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn eval(&self, x:f64)->Vec<f64>{
        let index = self.index(x);
        self.eval_with_index(index, x)
    }

    #[allow(dead_code)]
    pub fn eval_with_index(&self, index:usize, x:f64)->Vec<f64>{
        let mut val : Vec<f64> = Vec::new();
        for i in 0..(self.degree+1) as i32{
            if i+index as i32-(self.degree as i32) >= 0 && i+index as i32 -(self.degree as i32)< self.functions.len() as i32{
                val.push( self.functions[i as usize +index-(self.degree as usize)].as_ref().unwrap().functions[(self.degree as usize)- i as usize + 1].as_ref().unwrap().eval(x));
            }
            else{
                val.push(0.0);
            }
        }
        val
    }

    #[allow(dead_code)]
    pub fn differentiate(&mut self){
        for i in 0..self.functions.len(){
            self.functions[i].as_mut().unwrap().differentiate();
        }
    }

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(degree:u8, index:i32, knots:Vec<f64>)->Self{

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}", degree, index, knots.len())));

        if degree==0{
            let mut dom : Vec<f64> = Vec::new();
            for i in 0..2 { dom.push(knots[(index+i) as usize]); }
            let coeff : Vec<f64> = Vec::from([1.0]);
            let f : PolynomialFunction = PolynomialFunction::new(0, coeff);

            //log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, dom {}, returns", degree, index, knots.len(), dom.len())));
            return BSplineBasisSubFunction{ domains:dom, functions:Vec::from([None, Some(f), None]), degree, index, knots }
        }

        let mut bs1 = BSplineBasisSubFunction::new(degree-1, index, knots.clone());
        let mut bs2 = BSplineBasisSubFunction::new(degree-1, index+1, knots.clone());

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} next", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));
        //log_1(&JsValue::from(format!("BSplineBasisSubFunction bs1deg:{}, bs1idx:{}, bs1knots{}, bs1dom {}, bs2deg {}, bs2idx {}, bs2knots {}, bs2dom {},  next", bs1.degree, bs1.index, bs1.knots.len(), bs1.domains.len(), bs2.degree, bs2.index, bs2.knots.len(), bs2.domains.len() )));

        let coeff1:[f64;2] = [
        -knots[index as usize]/(knots[(index+degree as i32) as usize] - knots[index as usize]),
        1.0/(knots[(index+degree as i32) as usize]-knots[index as usize]) ];
        let p1 = PolynomialFunction::new(1, Vec::from(coeff1));

        let coeff2:[f64;2] = [
        knots[(index+degree as i32+1)as usize]/(knots[(index+degree as i32+1) as usize]-knots[index as usize+1]),
        -1.0/(knots[(index+degree as i32+1)as usize] - knots[index as usize +1])];
        let p2 = PolynomialFunction::new(1, Vec::from(coeff2));

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} mul1", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));
        //log_1(&JsValue::from(format!("BSplineBasisSubFunction bs1dom:{}, bs2dom:{},"", degree, index, knots.len())));

        bs1.mul(&p1);

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} mul2", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));

        bs2.mul(&p2);

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} add", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction: BEFORE_ADD: bs1deg:{}, bs1idx:{}, bs1knots{}, bs1dom {}, bs2deg {}, bs2idx {}, bs2knots {}, bs2dom {},  next", bs1.degree, bs1.index, bs1.knots.len(), bs1.domains.len(), bs2.degree, bs2.index, bs2.knots.len(), bs2.domains.len() )));

        bs1.add(&bs2);

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction deg:{}, idx:{}, knots{}, bs1dom {}, bs2dom {} end", degree, index, knots.len(), bs1.domains.len(), bs2.domains.len() )));

        //log_1(&JsValue::from(format!("BSplineBasisSubFunction: end of init: bs1deg:{}", bs1.degree))); //

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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn add_domain_and_function(degree: u8, bs1: &BSplineBasisSubFunction, bs2: &BSplineBasisSubFunction)->(Vec<f64>, Vec<Option<PolynomialFunction>>){

        let mut new_dom:Vec<f64> = Vec::new();
        //let bs1deg = bs1.degree as i32;
        //let bs2deg = bs2.degree as i32;
        let bs1domlen = bs1.domains.len() as i32;
        let bs2domlen = bs2.domains.len() as i32;

        /*
        for i in 0..bs1.functions.len(){
            if bs1.functions[i].is_none(){
                log_1(&JsValue::from(format!("add_domain_and_function: bs1.functions[{}] : None", i)));
            }
            else{
                log_1(&JsValue::from(format!("add_domain_and_function: bs1.functions[{}] : Not None", i)));
            }
        }
        for i in 0..bs2.functions.len(){
            if bs2.functions[i].is_none(){
                log_1(&JsValue::from(format!("add_domain_and_function: bs2.functions[{}] : None", i)));
            }
            else{
                log_1(&JsValue::from(format!("add_domain_and_function: bs2.functions[{}] : Not None", i)));
            }
        }
        */

        //log_1(&JsValue::from(format!("add_domain_and_function: 0 bs1.deg {} ,  bs1.index {}, bs2.deg {}, bs2.index {}", &bs1.degree, &bs1.index, &bs2.degree, &bs2.index )));
        //log_1(&JsValue::from(format!("add_domain_and_function: bs1deg:{}, bs1idx:{}, bs1knots{}, bs1dom {}, bs2deg {}, bs2idx {}, bs2knots {}, bs2dom {},  next", bs1.degree, bs1.index, bs1.knots.len(), bs1.domains.len(), bs2.degree, bs2.index, bs2.knots.len(), bs2.domains.len() )));

        /*
        for j in 0..bs1.functions.len(){
            if bs1.functions[j].is_none(){
                log_1(&JsValue::from(format!("add:: bs1.functions[{}]: None", j)));
            }
            else{
                log_1(&JsValue::from(format!("add:: bs1.functions[{}]: Not None", j)));
            }
        }
        for j in 0..bs2.functions.len(){
            if bs2.functions[j].is_none(){
                log_1(&JsValue::from(format!("add:: bs2.functions[{}]: None", j)));
            }
            else{
                log_1(&JsValue::from(format!("add:: bs2.functions[{}]: Not None", j)));
            }
        }
        */

        #[allow(unused_variables)]
        for i in 0..(degree as i32 + 2 + bs2.index-bs1.index) {
            new_dom.push(0.0);
        }
        let mut new_func:Vec<Option<PolynomialFunction>> = Vec::new();

        #[allow(unused_variables)]
        for i in 0..(degree as i32 + 3 + bs2.index-bs1.index){
            new_func.push(None);
        }

        let mut i : usize = 0;
        while i<bs1domlen as usize{
            new_dom[i] = bs1.domains[i];
            i+=1;
        }

//        log_1(&JsValue::from(format!("bs1deg {} , bs2deg {}",bs1deg, bs2deg)));

//        log_1(&JsValue::from(format!("add_domain_and_function: new_dom.len {} , bs1.dom.len {}, bs2.dom.len {}", new_dom.len(), bs1.domains.len(), bs2.domains.len())));
//        log_1(&JsValue::from(format!("i {} , bs1.index {}, bs2.index {}", i, bs1.index, bs2.index)));

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
                log_1(&JsValue::from(format!("new_func[{}]: None", j)));
            }
            else{
                log_1(&JsValue::from(format!("new_func[{}]: Not None", j)));
            }
        }
        */
        (Vec::from(new_dom), Vec::from(new_func))
    }


    #[allow(dead_code)]
    pub fn mul(&mut self, p:&PolynomialFunction){
        for i in 0..self.functions.len(){
            if !self.functions[i].is_none(){
                //self.functions[i].unwrap().mul_function(p);
                self.functions[i].as_mut().unwrap().mul_function(p);
            }
        }
    }

    #[allow(dead_code)]
    pub fn differentiate(&mut self){
        for i in 0..self.functions.len(){
            if !self.functions[i].is_none(){
                self.functions[i].as_mut().unwrap().differentiate();
            }
        }
    }

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(x:f64, y:f64, z:f64) -> Self{
        Vec3{ x, y, z }
    }

    #[allow(dead_code)]
    pub fn zero() -> Self{
        Vec3{ x:0.0, y:0.0, z:0.0 }
    }

    #[allow(dead_code)]
    pub fn new_with_vec3(v:&Vec3) -> Self{
        //Vec3{ x:v.x, y:v.y, z:v.z }
        Vec3{ ..*v }
    }

    #[allow(dead_code)]
    pub fn clone(&self) ->Self{
        Vec3::new_with_vec3(self)
    }

    #[allow(dead_code)]
    pub fn set(&mut self, v:&Vec3)->&mut Self{
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
        self
    }

    #[allow(dead_code)]
    pub fn to_array(&self)->[f64;3]{
        [self.x,self.y,self.z]
    }

    #[allow(dead_code)]
    pub fn to_array32(&self)->[f32;3]{
        [self.x as f32,self.y as f32,self.z as f32]
    }

    #[allow(dead_code)]
    pub fn add(&mut self, v:&Vec3)->&mut Self{
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }



    #[allow(dead_code)]
    pub fn sub(&mut self, v:&Vec3)->&mut Self{
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
        self
    }

    #[allow(dead_code)]
    pub fn mul(&mut self, f:f64)->&mut Self{
        self.x *= f;
        self.y *= f;
        self.z *= f;
        self
    }

    #[allow(dead_code)]
    pub fn div(&mut self, f:f64)->&mut Self{
        self.x /= f;
        self.y /= f;
        self.z /= f;
        self
    }

    #[allow(dead_code)]
    pub fn neg(&mut self)->&mut Self{
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    #[allow(dead_code)]
    pub fn dot(&self, v:&Vec3)->f64{
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    #[allow(dead_code)]
    pub fn cross(&self, v:&Vec3)->Self{
        Vec3{x: self.y*v.z-self.z*v.y, y:self.z*v.x-self.x*v.z, z:self.x*v.y-self.y*v.x}
    }

    #[allow(dead_code)]
    pub fn icross(&mut self, v:&Vec3)->&mut Self{
        let x = self.y*v.z - self.z*v.y;
        let y = self.z*v.x-self.x*v.z;
        let z = self.x*v.y-self.y*v.x;
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    #[allow(dead_code)]
    pub fn len2(&self) -> f64{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    #[allow(dead_code)]
    pub fn len(&self)->f64{
        self.len2().sqrt()
    }

    #[allow(dead_code)]
    pub fn set_len(&mut self, len:f64) ->&mut Self{
        self.mul(len/self.len())
    }
    #[allow(dead_code)]
    pub fn unit(&mut self) ->&mut Self{
        let l = self.len();
        self.x /= l;
        self.y /= l;
        self.z /= l;
        self
    }

    #[allow(dead_code)]
    pub fn dist2(&self, v:&Vec3)->f64{
        (self.x-v.x)*(self.x-v.x) + (self.y-v.y)*(self.y-v.y) + (self.z-v.z)*(self.z-v.z)
    }
    #[allow(dead_code)]
    pub fn dist(&self, v:&Vec3)->f64{
        self.dist2(v).sqrt()
    }

    #[allow(dead_code)]
    pub fn eq(&self, v:&Vec3)->bool{
        self.dist2(v) <= TOLERANCE*TOLERANCE
    }
    #[allow(dead_code)]
    pub fn eq_x(&self, v:&Vec3)->bool{
        (self.x-v.x).abs() <= TOLERANCE
    }
    #[allow(dead_code)]
    pub fn eq_y(&self, v:&Vec3)->bool{
        (self.y-v.y).abs() <= TOLERANCE
    }
    #[allow(dead_code)]
    pub fn eq_z(&self, v:&Vec3)->bool{
        (self.z-v.z).abs() <= TOLERANCE
    }

    #[allow(dead_code)]
    pub fn angle(&self, v:&Vec3)->f64{
        let len1 = self.len();
        if len1==0.0 { return 0.0; }
        let len2 = v.len();
        if len2==0.0 { return 0.0; }
        let mut cos = self.dot(v)/(len1*len2);
        if cos > 1.0 { cos = 1.0; } else if cos < -1.0 { cos=-1.0; }
        cos.acos()
    }

    #[allow(dead_code)]
    pub fn angle_with_axis(&self, v:&Vec3, axis:&Vec3)->f64{
        let ang = self.angle(v);
        let crs = self.cross(v);
        if crs.dot(axis)<0.0 { return -ang; }
        ang
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn rot_with_center(&mut self, center:&Vec3, axis:&Vec3, angle:f64)->&mut Self{
        self.sub(center);
        self.rot(axis,angle);
        self.add(center)
    }

    #[allow(dead_code)]
    pub fn rot2(&mut self, angle:f64)->&mut Self{
        self.rot(&Vec3::new(0.,0.,1.), angle)
    }

    #[allow(dead_code)]
    pub fn rot2_with_center(&mut self, center:&Vec3, angle:f64)->&mut Self{
        self.sub(center);
        self.rot2(angle);
        self.add(center)
    }


    #[allow(dead_code)]
    pub fn scale(&mut self, center:&Vec3, factor:f64) ->&mut Self{
        self.sub(center);
        self.mul(factor);
        self.add(center)
    }

    #[allow(dead_code)]
    pub fn scale1d(&mut self, axis:&Vec3, factor:f64) ->&mut Self{
        let d = self.dot(axis)/axis.len2()*(factor-1.0);
        self.scale_add(d, axis)
    }

    #[allow(dead_code)]
    pub fn scale_add(&mut self, factor:f64, v:&Vec3)->&mut Self{
        self.x += v.x*factor;
        self.y += v.y*factor;
        self.z += v.z*factor;
        self
    }
    #[allow(dead_code)]
    pub fn mirror(&mut self, plane_dir:&Vec3)->&mut Self{
        self.scale_add(self.dot(plane_dir)/plane_dir.len2()*-2.0, plane_dir)
    }
    #[allow(dead_code)]
    pub fn transform(&mut self, xvec:&Vec3, yvec:&Vec3, zvec:&Vec3) ->&mut Self{
        let tx = xvec.x*self.x + yvec.x*self.y + zvec.x*self.z;
        let ty = xvec.y*self.x + yvec.y*self.y + zvec.y*self.z;
        let tz = xvec.z*self.x + yvec.z*self.y + zvec.z*self.z;
        self.x = tx;
        self.y = ty;
        self.z = tz;
        self
    }

    #[allow(dead_code)]
    pub fn transform_with_translate(&mut self, xvec:&Vec3, yvec:&Vec3, zvec:&Vec3, translate:&Vec3) ->&mut Self{
        self.transform(xvec,yvec,zvec);
        self.add(translate)
    }


    #[allow(dead_code)]
    pub fn cp(&self, v:&Vec3)->Self{
        Vec3{x:self.x+v.x, y:self.y+v.y, z:self.z+v.z}
    }

    #[allow(dead_code)]
    pub fn dif(&self, v:&Vec3)->Self{
        Vec3{x:self.x-v.x, y:self.y-v.y, z:self.z-v.z}
    }

    #[allow(dead_code)]
    pub fn sum(&self, v:&Vec3)->Self{
        Vec3{x:self.x+v.x, y:self.y+v.y, z:self.z+v.z}
    }

    #[allow(dead_code)]
    pub fn mid(&self, v:&Vec3)->Self{
        Vec3{x:(self.x+v.x)/2.0, y:(self.y+v.y)/2.0, z:(self.z+v.z)/2.0}
    }


    #[allow(dead_code)]
    pub fn bisect(&self, v:&Vec3)->Self{
        let l1 = self.len();
        let l2 = v.len();
        Vec3{x:self.x/l1+v.x/l2, y:self.y/l1+v.y/l2, z:self.z/l1+v.z/l2}
    }

    #[allow(dead_code)]
    pub fn intersect(line1pt1:&Vec3, line1pt2:&Vec3, line2pt1:&Vec3, line2pt2:&Vec3)->Option<Vec3>{
        if line1pt1.eq(line2pt1) || line1pt1.eq(line2pt2) { return Some(line1pt1.clone()); }
        if line1pt2.eq(line2pt1) || line1pt2.eq(line2pt2) { return Some(line1pt2.clone()); }

        let mut dir1 = line1pt2.dif(line1pt1);
        let mut dir2 = line2pt2.dif(line2pt1);

        let mut dif = line2pt1.dif(line1pt1);

        let mut op = dir1.cross(&dir2);
        let oplen = op.len();

        if oplen < TOLERANCE*TOLERANCE {
            dir1.unit();
            if dir1.mul(dif.dot(&dir1)).sub(&dif).len() > TOLERANCE{ return None; }
            return Some(line1pt1.clone());
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
        let ret = dir1.mul((ip1-ip2*ip12)/iip122).add(line1pt1);
        return Some(*ret);
    }

    #[allow(dead_code)]
    pub fn intersect_segment(line1pt1:&Vec3, line1pt2:&Vec3, line2pt1:&Vec3, line2pt2:&Vec3)->Option<Vec3>{
        if line1pt1.eq(line2pt1) || line1pt1.eq(line2pt2){ Some(line1pt1.clone()); }
        if line1pt2.eq(line2pt1) || line1pt2.eq(line2pt2){ Some(line1pt2.clone()); }

        let min1x = line1pt1.x.min(line1pt2.x);
        let min1y = line1pt1.y.min(line1pt2.y);
        let min1z = line1pt1.z.min(line1pt2.z);
        let max1x = line1pt1.x.max(line1pt2.x);
        let max1y = line1pt1.y.max(line1pt2.y);
        let max1z = line1pt1.z.max(line1pt2.z);
        let min2x = line2pt1.x.min(line2pt2.x);
        let min2y = line2pt1.y.min(line2pt2.y);
        let min2z = line2pt1.z.min(line2pt2.z);
        let max2x = line2pt1.x.max(line2pt2.x);
        let max2y = line2pt1.y.max(line2pt2.y);
        let max2z = line2pt1.z.max(line2pt2.z);

        // check bounding region
         if min1x > max2x + TOLERANCE || max1x < min2x - TOLERANCE ||
            min1y > max2y + TOLERANCE || max1y < min2y - TOLERANCE ||
            min1z > max2z + TOLERANCE || max1z < min2z - TOLERANCE { return None; }

         // judging by tolerance
         if line1pt1.eq(&line2pt1) { return Some(line1pt1.clone()); }
         if line1pt1.eq(&line2pt2) { return Some(line1pt1.clone()); }
         if line1pt2.eq(&line2pt1) { return Some(line1pt2.clone()); }
         if line1pt2.eq(&line2pt2) { return Some(line1pt2.clone()); }

         let mut dir1 = line1pt2.dif(&line1pt1);
         let mut dir2 = line2pt2.dif(&line2pt1);

         let mut dif = line2pt1.dif(&line1pt1);

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
             let dif12 = line2pt2.dif(&line1pt1);
             let dif21 = line2pt1.dif(&line1pt2);
             let ip11 = dir1.dot(&dif);
             let ip12 = dir1.dot(&dif12);

             if  ip11 <= TOLERANCE && ip12 >= -TOLERANCE ||
                 ip11 >= -TOLERANCE && ip12 <= TOLERANCE { return Some(line1pt1.clone()); }

             let ip21 = dir1.dot(&dif21);
             if ip11 >= -TOLERANCE && ip21 <= TOLERANCE ||
                ip11 <= TOLERANCE && ip21 >= -TOLERANCE { return Some(line2pt1.clone()); }

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

         let ret = dir1.mul(ilen1).add(&line1pt1);
         return Some(*ret);
    }


    #[allow(dead_code)]
    pub fn dist_to_plane(&self, plane_dir:&Vec3, plane_pt:&Vec3)->f64{
        let plen = plane_dir.len();
        if plen==0.0 { return self.dist(plane_pt); }
        (self.dif(plane_pt).dot(plane_dir)/plen).abs()
    }
    #[allow(dead_code)]
    pub fn nml(&self, pt1:&Vec3, pt2:&Vec3)->Vec3{
        self.dif(pt1).cross(&self.dif(pt2))
    }
    #[allow(dead_code)]
    pub fn is_on_plane(&self, pt1:&Vec3, pt2:&Vec3, pt3:&Vec3)->bool{
        self.is_on_plane_with_nml(&Vec3::get_normal(pt1,pt2,pt3), pt1)
    }
    #[allow(dead_code)]
    pub fn is_on_plane_with_nml(&self, plane_dir:&Vec3, plane_pt:&Vec3)->bool{
        self.dist_to_plane(plane_dir, plane_pt) < TOLERANCE
    }

    #[allow(dead_code)]
    pub fn is_flat(pt1:&Vec3, pt2:&Vec3, pt3:&Vec3, pt4:&Vec3)->bool{
        pt1.is_on_plane(pt2,pt3,pt4)
    }

    #[allow(dead_code)]
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

#[derive(Debug, Clone, Copy)]
pub struct Orient{
    dir:Vec3,
    nml:Vec3,
    righthand:bool,
}

impl Orient{
    #[allow(dead_code)]
    pub fn new(dir:Vec3, nml:Vec3)->Self{
        Orient{dir, nml, righthand:true}
    }
    #[allow(dead_code)]
    pub fn clone(&self)->Self{
        Orient{dir:self.dir.clone(), nml:self.nml.clone(), righthand:self.righthand.clone()}
    }
    #[allow(dead_code)]
    pub fn front(&self)->Vec3{
        self.dir.clone()
    }
    #[allow(dead_code)]
    pub fn back(&self)->Vec3{
        *self.dir.clone().neg()
    }
    #[allow(dead_code)]
    pub fn up(&self)->Vec3{
        self.nml.clone()
    }
    #[allow(dead_code)]
    pub fn down(&self)->Vec3{
        self.nml.clone()
    }
    #[allow(dead_code)]
    pub fn right(&self)->Vec3{
        if self.righthand{
            return self.dir.cross(&self.nml);
        }
        self.nml.cross(&self.dir)
    }
    #[allow(dead_code)]
    pub fn left(&self)->Vec3{
        if self.righthand{
            return self.nml.cross(&self.dir);
        }
        self.dir.cross(&self.nml)
    }
    #[allow(dead_code)]
    pub fn rot(&mut self, a:f64)->&mut Self{
        self.dir.rot(&self.nml, a);
        self
    }
    #[allow(dead_code)]
    pub fn yaw(&mut self, a:f64)->&mut Self{
        self.dir.rot(&self.nml, a);
        self
    }
    #[allow(dead_code)]
    pub fn pitch(&mut self, a:f64)->&mut Self{
        let ax = self.dir.cross(&self.nml);
        self.dir.rot(&ax, a);
        self.nml.rot(&ax, a);
        self
    }
    #[allow(dead_code)]
    pub fn roll(&mut self, a:f64)->&mut Self{
        self.nml.rot(&self.dir, a);
        self
    }
    #[allow(dead_code)]
    pub fn flip(&mut self)->&mut Self{
        self.dir.neg();
        self.righthand = !self.righthand;
        self
    }
    #[allow(dead_code)]
    pub fn flip_nml(&mut self)->&mut Self{
        self.nml.neg();
        self.righthand = !self.righthand;
        self
    }
    #[allow(dead_code)]
    pub fn flip_side(&mut self)->&mut Self{
        self.righthand = !self.righthand;
        self
    }
    #[allow(dead_code)]
    pub fn mul(&mut self, f:f64)->&mut Self{
        self.dir.mul(f);
        self
    }
    #[allow(dead_code)]
    pub fn div(&mut self, f:f64)->&mut Self{
        self.dir.div(f);
        self
    }



}



pub struct Matrix{}
impl Matrix{
    #[allow(dead_code)]
    pub fn det(v11:f64, v12:f64, v21:f64, v22:f64)->f64{
        v11*v22-v12*v21
    }
}

pub struct Matrix3{
    val: [[f64;3];3],
}

impl Matrix3{
    #[allow(dead_code)]
    pub fn new(v11:f64, v12:f64, v13:f64, v21:f64,v22:f64, v23:f64, v31:f64,v32:f64, v33:f64 )->Self{
        Matrix3{val:[[v11,v12,v13],[v21,v22,v23],[v31,v32,v33]]  }
    }

    #[allow(dead_code)]
    pub fn new_with_matrix3(m:&Matrix3)->Self{
        Matrix3{
            val:[
            [m.val[0][0],m.val[0][1],m.val[0][2]],
            [m.val[1][0],m.val[1][1],m.val[1][2]],
            [m.val[2][0],m.val[2][1],m.val[2][2]]
            ]}
    }

    #[allow(dead_code)]
    pub fn zero()->Self{
        Matrix3{
            val:[[0.0,0.0,0.0], [0.0,0.0,0.0], [0.0,0.0,0.0]]
        }
    }

    #[allow(dead_code)]
    pub fn new_with_id()->Self{
        Matrix3{
            val:[[1.0,0.0,0.0], [0.0,1.0,0.0], [0.0,0.0,1.0]]
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, v11:f64, v12:f64, v13:f64, v21:f64,v22:f64, v23:f64, v31:f64,v32:f64, v33:f64 )->&mut Self{
        self.val[0][0] = v11; self.val[0][1] = v12; self.val[0][2] = v13;
        self.val[1][0] = v21; self.val[1][1] = v22; self.val[1][2] = v23;
        self.val[2][0] = v31; self.val[2][1] = v32; self.val[2][2] = v33;
        self
    }

    #[allow(dead_code)]
    pub fn to_array(&self)->[f64;9]{
        [self.val[0][0],self.val[0][1],self.val[0][2],
        self.val[1][0],self.val[1][1],self.val[1][2],
        self.val[2][0],self.val[2][1],self.val[2][2]]
    }

    #[allow(dead_code)]
    pub fn to_array32(&self)->[f32;9]{
        [self.val[0][0] as f32,self.val[0][1] as f32,self.val[0][2] as f32,
        self.val[1][0] as f32,self.val[1][1] as f32,self.val[1][2] as f32,
        self.val[2][0] as f32,self.val[2][1] as f32,self.val[2][2] as f32]
    }

    #[allow(dead_code)]
    pub fn determinant(&self)->f64{
        self.val[0][0]*Matrix::det(self.val[1][1],self.val[1][2],self.val[2][1],self.val[2][2])+
        self.val[0][1]*Matrix::det(self.val[1][2],self.val[1][0],self.val[2][2],self.val[2][0])+
        self.val[0][2]*Matrix::det(self.val[1][0],self.val[1][1],self.val[2][0],self.val[2][1])
    }

    #[allow(dead_code)]
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
        self.div(det);
        self
    }

    #[allow(dead_code)]
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


    #[allow(dead_code)]
    pub fn vecmul(&self, v:&Vec3)->Vec3{
        Vec3{
            x:self.val[0][0]*v.x + self.val[0][1]*v.y + self.val[0][2]*v.z,
            y:self.val[1][0]*v.x + self.val[1][1]*v.y + self.val[1][2]*v.z,
            z:self.val[2][0]*v.x + self.val[2][1]*v.y + self.val[2][2]*v.z
        }
    }

    #[allow(dead_code)]
    pub fn mul(&mut self, factor:f64)->&mut Self{
        for i in 0..3{ for j in 0..3{ self.val[i][j] *= factor; } }
        self
    }

    #[allow(dead_code)]
    pub fn div(&mut self, factor:f64)->&mut Self{
        for i in 0..3{ for j in 0..3{ self.val[i][j] /= factor; } }
        self
    }

    #[allow(dead_code)]
    pub fn set_zero(&mut self)->&mut Self{
        for i in 0..3{ for j in 0..3{ self.val[i][j] = 0.0; } }
        self
    }

    #[allow(dead_code)]
    pub fn id(&mut self)->&mut Self{
        for i in 0..3{ for j in 0..3{ self.val[i][j] = if i==j {1.0 } else{ 0.0 } } }
        self
    }

    #[allow(dead_code)]
    pub fn x_rotation(angle:f64)->Self{
        Matrix3::new(1.0, 0.0, 0.0,
            0.0, angle.cos(), -angle.sin(),
            0.0, angle.sin(), angle.cos())
    }
    #[allow(dead_code)]
    pub fn y_rotation(angle:f64)->Self{
        Matrix3::new(angle.cos(), 0.0, angle.sin(),
            0.0, 1.0, 0.0,
            -angle.sin(), 0.0, angle.cos())
    }
    #[allow(dead_code)]
    pub fn z_rotation(angle:f64)->Self{
        Matrix3::new(angle.cos(), -angle.sin(), 0.0,
            angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 1.0)
    }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn translate(p:&Vec3)->Self{
        Matrix3::new(1.0, 0.0, p.x,
            0.0, 1.0, p.y,
            0.0, 0.0, 1.0)
    }
}

impl fmt::Display for Vec3{
    #[allow(dead_code)]
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}


pub struct Matrix4{
    val: [[f64;4];4],

}

impl Matrix4{
    #[allow(dead_code)]
    pub fn new(v11:f64,v12:f64,v13:f64,v14:f64, v21:f64,v22:f64, v23:f64,v24:f64, v31:f64,v32:f64, v33:f64,v34:f64, v41:f64, v42:f64, v43:f64, v44:f64 )->Self{
        Matrix4{val:[[v11,v12,v13,v14],[v21,v22,v23,v24],[v31,v32,v33,v34],[v41,v42,v43,v44]]  }
    }

    #[allow(dead_code)]
    pub fn new_with_matrix4(m:&Matrix4)->Self{
        Matrix4{
            val:[
            [m.val[0][0],m.val[0][1],m.val[0][2],m.val[0][3]],
            [m.val[1][0],m.val[1][1],m.val[1][2],m.val[1][3]],
            [m.val[2][0],m.val[2][1],m.val[2][2],m.val[2][3]],
            [m.val[3][0],m.val[3][1],m.val[3][2],m.val[3][3]]
            ]}
    }

    #[allow(dead_code)]
    pub fn zero()->Self{
        Matrix4{
            val:[[0.0,0.0,0.0,0.0], [0.0,0.0,0.0,0.0], [0.0,0.0,0.0,0.0], [0.0,0.0,0.0,0.0]]
        }
    }

    #[allow(dead_code)]
    pub fn new_with_id()->Self{
        Matrix4{
            val:[[1.0,0.0,0.0,0.0], [0.0,1.0,0.0,0.0], [0.0,0.0,1.0,0.0], [0.0,0.0,0.0,1.0]]
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn set_with_matrix4(&mut self, m:&Matrix4)->&mut Self{
        self.val[0][0] = m.val[0][0]; self.val[0][1] = m.val[0][1]; self.val[0][2] = m.val[0][2]; self.val[0][3] = m.val[0][3];
        self.val[1][0] = m.val[1][0]; self.val[1][1] = m.val[1][1]; self.val[1][2] = m.val[1][2]; self.val[1][3] = m.val[1][3];
        self.val[2][0] = m.val[2][0]; self.val[2][1] = m.val[2][1]; self.val[2][2] = m.val[2][2]; self.val[2][3] = m.val[2][3];
        self.val[3][0] = m.val[3][0]; self.val[3][1] = m.val[3][1]; self.val[3][2] = m.val[3][2]; self.val[3][3] = m.val[3][3];
        self
    }

    #[allow(dead_code)]
    pub fn to_array(&self)->[f64;16]{
        [self.val[0][0],self.val[0][1],self.val[0][2],self.val[0][3],
        self.val[1][0],self.val[1][1],self.val[1][2],self.val[1][3],
        self.val[2][0],self.val[2][1],self.val[2][2],self.val[2][3],
        self.val[3][0],self.val[3][1],self.val[3][2],self.val[3][3]]
    }

    #[allow(dead_code)]
    pub fn to_array32(&self)->[f32;16]{
        [self.val[0][0] as f32,self.val[0][1] as f32,self.val[0][2] as f32,self.val[0][3] as f32,
        self.val[1][0] as f32,self.val[1][1] as f32,self.val[1][2] as f32,self.val[1][3] as f32,
        self.val[2][0] as f32,self.val[2][1] as f32,self.val[2][2] as f32,self.val[2][3] as f32,
        self.val[3][0] as f32,self.val[3][1] as f32,self.val[3][2] as f32,self.val[3][3] as f32]
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn transpose(&mut self)->&mut Self{
        self.set(
            self.val[0][0],  self.val[1][0], self.val[2][0], self.val[3][0],
            self.val[0][1],  self.val[1][1], self.val[2][1], self.val[3][1],
            self.val[0][2],  self.val[1][2], self.val[2][2], self.val[3][2],
            self.val[0][3],  self.val[1][3], self.val[2][3], self.val[3][3],
        );
        self
    }

    #[allow(dead_code)]
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


    #[allow(dead_code)]
    pub fn vecmul(&self, v:&Vec4)->Vec4{
        Vec4{
            x:self.val[0][0]*v.x + self.val[0][1]*v.y + self.val[0][2]*v.z + self.val[0][3]*v.w,
            y:self.val[1][0]*v.x + self.val[1][1]*v.y + self.val[1][2]*v.z + self.val[1][3]*v.w,
            z:self.val[2][0]*v.x + self.val[2][1]*v.y + self.val[2][2]*v.z + self.val[2][3]*v.w,
            w:self.val[3][0]*v.x + self.val[3][1]*v.y + self.val[3][2]*v.z + self.val[3][3]*v.w,
        }
    }

    #[allow(dead_code)]
    pub fn mul(&mut self, factor:f64)->&mut Self{
        for i in 0..4{ for j in 0..4{ self.val[i][j] *= factor; } }
        self
    }

    #[allow(dead_code)]
    pub fn div(&mut self, factor:f64)->&mut Self{
        for i in 0..4{ for j in 0..4{ self.val[i][j] /= factor; } }
        self
    }

    #[allow(dead_code)]
    pub fn set_zero(&mut self)->&mut Self{
        for i in 0..4{ for j in 0..4{ self.val[i][j] = 0.0; } }
        self
    }

    #[allow(dead_code)]
    pub fn id(&mut self)->&mut Self{
        for i in 0..4{ for j in 0..4{ self.val[i][j] = if i==j {1.0 } else{ 0.0 } } }
        self
    }

    #[allow(dead_code)]
    pub fn x_rotation(angle:f64)->Self{
        Matrix4::new(1.0, 0.0, 0.0, 0.0,
            0.0, angle.cos(), -angle.sin(), 0.0,
            0.0, angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn y_rotation(angle:f64)->Self{
        Matrix4::new(angle.cos(), 0.0, angle.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -angle.sin(), 0.0, angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn z_rotation(angle:f64)->Self{
        Matrix4::new(angle.cos(), -angle.sin(), 0.0, 0.0,
            angle.sin(), angle.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn translate(p:&Vec3)->Self{
        Matrix4::new(1.0, 0.0, 0.0, p.x,
            0.0, 1.0, 0.0, p.y,
            0.0, 0.0, 1.0, p.z,
            0.0, 0.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn perspective(aspect:f64, fovy:f64, near:f64, far:f64)->Self{
        let f:f64 = 1.0/((fovy/2.0).tan());
        Matrix4::new(
            f/aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, -far/(far-near), -1.0,
            0.0, 0.0, -far*near/(far-near), 0.0)
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn scale(f:f64)->Self{
        Matrix4::new(f, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, f, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn transform(xvector:&Vec3, yvector:&Vec3, zvector:&Vec3, translate:&Vec3)->Self{
        Matrix4::new(xvector.x, yvector.x, zvector.x, translate.x,
            xvector.y, yvector.y, zvector.y, translate.y,
            xvector.z, yvector.z, zvector.z, translate.z,
            0.0, 0.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn convert(xvec1:&Vec3, yvec1:&Vec3, zvec1:&Vec3, orig1:&Vec3,
        xvec2:&Vec3, yvec2:&Vec3, zvec2:&Vec3, orig2:&Vec3)->Self{
            let mut mat1 = Matrix4::transform(xvec1,yvec1,zvec1,orig1);
            let mut mat2 = Matrix4::transform(xvec2,yvec2,zvec2,orig2);
            mat1.invert();
            mat2.matmul(&mat1);
            Matrix4::new_with_matrix4(&mat2)
    }

}
