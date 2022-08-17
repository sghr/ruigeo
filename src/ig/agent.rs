use super::DataManager;
use super::Manager;
use super::Attribute;
use super::Color;
use super::UPDATE_RATE;
use super::math::{Vec3, Orient};
use super::geo::{Point, Curve, Surface};

use std::f64::consts::PI;
use std::any::Any;


#[derive(Debug, Clone)]
pub struct AgentInfo{
    pub id: i32,
    pub color: Color,
    pub time:i64
}

impl AgentInfo{
    #[allow(dead_code)]
    pub fn new()->Self{
        AgentInfo{
            id:-1,
            color: Color::new(0.5,0.5,0.5,1.0),
            time:0
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self,info:&AgentInfo)->&mut Self{
        self.color.set(&info.color);
        //self.time = info.time;
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
        self
    }
    #[allow(dead_code)]
    pub fn rgba(&mut self, r:f32, g:f32, b:f32, a:f32)->&mut Self{
        self.color.rgba(r,g,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn rgb(&mut self, r:f32, g:f32, b:f32)->&mut Self{
        self.color.rgb(r,g,b);
        self
    }
    #[allow(dead_code)]
    pub fn hsba(&mut self, h:f32, s:f32, b:f32, a:f32)->&mut Self{
        self.color.hsba(h,s,b,a);
        self
    }
    #[allow(dead_code)]
    pub fn hsb(&mut self, h:f32, s:f32, b:f32)->&mut Self{
        self.color.hsb(h,s,b);
        self
    }

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


pub trait AgentTrait : AgentClone + AsAny{
    #[allow(dead_code)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, storage:&mut Manager);
    #[allow(dead_code)]
    fn update(&mut self, storage:&mut Manager);
    //fn set_id(&mut self, id:i32);
    //fn clone(&self)->dyn AgentTrait;
    #[allow(dead_code)]
    fn info(&mut self)->&mut AgentInfo;
    #[allow(dead_code)]
    fn time(&mut self)->i64{
        self.info().time
    }

    #[allow(dead_code)]
    fn get_clr(&mut self)->&Color{
        &self.info().color
    }

    #[allow(dead_code)]
    fn set_clr(&mut self, c:&Color){
        self.info().color.set(c);
    }
    #[allow(dead_code)]
    fn clr(&mut self, r:f32, g:f32, b:f32, a:f32){
        self.info().clr(r,g,b,a);
    }
    #[allow(dead_code)]
    fn rgba(&mut self, r:f32, g:f32, b:f32, a:f32){
        self.info().rgba(r,g,b,a);
    }
    #[allow(dead_code)]
    fn rgb(&mut self, r:f32, g:f32, b:f32){
        self.info().rgb(r,g,b);
    }
    #[allow(dead_code)]
    fn hsba(&mut self, h:f32, s:f32, b:f32, a:f32){
        self.info().hsba(h,s,b,a);
    }
    #[allow(dead_code)]
    fn hsb(&mut self, h:f32, s:f32, b:f32){
        self.info().hsb(h,s,b);
    }

}


pub trait AgentClone {
    fn clone_box(&self)->Box<dyn AgentTrait>;
}

impl<T> AgentClone for T where T:'static + AgentTrait + Clone{
    fn clone_box(&self)->Box<dyn AgentTrait>{
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AgentTrait>{
    fn clone(&self)->Box<dyn AgentTrait>{
        self.clone_box()
    }
}


pub trait AsAny{
    fn as_any(&self)->&dyn Any;
}

impl<T> AsAny for T where T:'static + AgentTrait{
    fn as_any(&self)->&dyn Any{ self as &dyn Any }
}

#[derive(Debug, Clone)]
pub struct LineAgent{
    pub pos:Vec3,
    pub dir:Vec3,
    pub info:AgentInfo,
    pub colliding:bool
}

impl AgentTrait for LineAgent{

    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }


    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){
    }

    fn update(&mut self, manager:&mut Manager){
        let pos2 = self.pos.cp(&self.dir);
        let mut dir2 = self.dir.clone();

        dir2.rot(&Vec3::new(0.0,0.0,1.0), 0.1);
        dir2.mul(0.999);


        if pos2.x > 0.0{
            dir2.add(&Vec3::new(-0.01, 0.0, 0.0));
        }
        else{
            dir2.add(&Vec3::new(0.01, 0.0, 0.0));
        }


        let mut line = Curve::line(self.pos, pos2);
        //let float_time : f64 = manager.time as f64;

        let t = (manager.time as f64*0.01).sin() as f32;
        line.clr(1.0-t, 0.5-t*0.5, t*0.5, 1.0 );
        //line.clr(0.5, 1.0, 1.0, 1.0);

        //let v = 0.01_f64.sin();

        manager.add_curve(Box::new(line));

        let mut pos3 = self.pos.clone();
        pos3.add(&Vec3::new(0.0, 0.0, 1.0));

        let mut pos4 = pos2.clone();
        pos4.add(&Vec3::new(0.0, 0.0, 1.0));

        let mut srf = Surface::quad(self.pos, pos2, pos4, pos3);
        srf.clr(1.0-t, 0.5-t*0.5, t*0.5, 1.0 );
        manager.add_surface(Box::new(srf));

        let mut cir = Curve::circle(&self.pos, &self.dir, 0.1);
        cir.clr(t, -0.5+t*0.5, 1.0-t, 1.0 );
        manager.add_curve(Box::new(cir));

        let mut pipe = Surface::pipe(&self.pos,&pos2, 0.2);
        pipe.clr(t, -0.5+t*0.5, 1.0-t, 1.0 );
        manager.add_surface(Box::new(pipe));


        let mut agent = LineAgent::new_with_dir(pos2, dir2);
        agent.info().set(&self.info);
        manager.add_agent(Box::new(agent));

        manager.delete_agent(self);
    }
}

impl LineAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3)->Self{
        LineAgent{pos, dir:Vec3::zero(),colliding:false,info:AgentInfo::new() }
    }
    #[allow(dead_code)]
    pub fn new_with_dir(pos:Vec3, dir:Vec3)->Self{
        LineAgent{pos, dir, colliding:false, info:AgentInfo::new() }
    }

}


#[derive(Debug, Clone)]
pub struct Particle{
    pub pos:Vec3,
    pub vel:Vec3,
    pub frc:Vec3,
    pub fric:f32,
    pub fixed:bool,
    pub info:AgentInfo,
}

impl AgentTrait for Particle{
    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){

        self.frc.add(&Vec3::new(0.0, 0.0, -1.0));
    }

    fn update(&mut self, manager:&mut Manager){
        if !self.fixed{
            self.vel.scale_add(UPDATE_RATE as f64, &self.frc);
            self.vel.mul(1.0-self.fric as f64);
            self.frc.set_zero();
            self.pos.scale_add(UPDATE_RATE as f64, &self.vel);
        }

        let mut point = Box::new(Point::new(self.pos.x, self.pos.y, self.pos.z));
        point.set_clr(self.get_clr());
        manager.add_point(point);


    }
}

impl Particle{
    pub fn new(pos:Vec3, vel:Vec3)->Self{
        Particle{ pos, vel, frc:Vec3::zero(), fric:0.0, fixed:false, info:AgentInfo::new()}
    }
}


#[derive(Debug, Clone)]
pub struct OrientAgent{
    pub pos:Vec3,
    pub orient:Orient,
    pub info:AgentInfo,
    pub colliding:bool
}

impl AgentTrait for OrientAgent{

    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }

    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){
        if self.time()==0 {
            //if self.orient.front().z < 0.0{
            //    self.colliding=true;
            //    return;
            //}
            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
                //if agents[i].info().id != self.info().id{
                if agents[i].info().id != self.info().id{
                    match agents[i].as_any().downcast_ref::<Self>(){
                        Some(a) => {
                            let dif = a.pos.dif(&pos2);
                            if dif.dot(&front) >= 0.0{
                                if dif.len() < l * 1.0{
                                    self.colliding=true;
                                    break;
                                }
                            }
                        }
                        None=> ()
                    }
                }
            }
        }
    }

    fn update(&mut self, manager:&mut Manager){
        if self.time() == 0{
            if self.colliding{
                manager.delete_agent(self);
                return;
            }

            let pos2 = self.pos.cp(&self.orient.front());

            let t = (manager.time as f32 * 0.08).sin()*0.5+0.5;
            let mut line = Curve::line(self.pos, pos2);
            line.clr( 1.0-t, 0.0, 1.0-t*0.5, 1.0 );
            //line.clr( 1.0, 1.0, 1.0,1.0 );
            manager.add_curve(Box::new(line));

            let pt11 = self.pos.cp(&self.orient.left().div(2.0));
            let pt12 = self.pos.cp(&self.orient.right().div(2.0));
            let pt21 = pos2.cp(&self.orient.left().div(2.0));
            let pt22 = pos2.cp(&self.orient.right().div(2.0));
            let mut surf = Surface::quad(pt11,pt12,pt22,pt21);
            surf.clr( 1.0-t, 0.0, 1.0-t*0.5, 1.0 );
            //surf.clr( rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0 );
            manager.add_surface(Box::new(surf));

            let mut child_num = 0;

            if manager.time%10 == 0{
                let pos2 = self.pos.cp(&self.orient.front());
                let mut orient2 = self.orient.clone();
                orient2.pitch(PI/2.0);
                let mut agent = OrientAgent::new_with_orient(pos2, orient2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            if manager.time%20 == 0{
                let mut pos3 = self.pos.cp(&self.orient.front().div(2.0));
                pos3.add(self.orient.right().div(2.0));
                let mut orient3 = self.orient.clone();
                orient3.yaw(PI/2.0);
                let mut agent = OrientAgent::new_with_orient(pos3, orient3);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            if manager.time%30==0{
                let mut orient2 = self.orient.clone();
                let mut agent = OrientAgent::new_with_orient(pos2, orient2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            if manager.time%20==0{
                let mut pos3 = self.pos.cp(&self.orient.front().div(2.0));
                pos3.add(self.orient.right().div(2.0));
                let mut orient3 = self.orient.clone();
                orient3.yaw(-PI/2.0);
                let mut agent = OrientAgent::new_with_orient(pos3, orient3);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            if manager.time%10!=0{
                let mut orient2 = self.orient.clone();
                //orient2.yaw(PI/50.0);
                //orient2.roll(PI/50.0);
                let mut agent = OrientAgent::new_with_orient(pos2, orient2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            //manager.delete_agent(self.id);
        }
    }
}

impl OrientAgent{
    #[allow(dead_code)]
    pub fn new_with_orient(pos:Vec3, orient:Orient)->Self{
        OrientAgent{pos, orient, colliding:false, info:AgentInfo::new() }
    }
}
