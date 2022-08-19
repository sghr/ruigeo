use super::Manager;
use super::Color;
use super::UPDATE_RATE;
use super::math::{Vec3, Orient};
use super::geo::{Point, Curve, Surface};

use std::f64::consts::PI;
use std::any::Any;

#[allow(unused_imports)]
use ig_macro::agent_fields;
#[allow(unused_imports)]
use ig_macro::agent_methods;


#[derive(Debug, Clone)]
pub struct AgentAttr{
    pub id: i32,
    pub color: Color,
    pub time:i64
}

impl AgentAttr{
    #[allow(dead_code)]
    pub fn new()->Self{
        AgentAttr{
            id:-1,
            color: Color::new(0.5,0.5,0.5,1.0),
            time:0
        }
    }
    #[allow(dead_code)]
    //pub fn set(&mut self,agent:&dyn Agent)->&mut Self{
    pub fn set(&mut self,attr:&AgentAttr)->&mut Self{
        self.color.set(&attr.color);
        //self.time = attr.time;
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


pub trait Agent : AgentClone + AsAny{
    #[allow(dead_code)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn Agent>>, storage:&mut Manager);
    #[allow(dead_code)]
    fn update(&mut self, storage:&mut Manager);
    //fn set_id(&mut self, id:i32);
    //fn clone(&self)->dyn Agent;
    #[allow(dead_code)]
    fn attr(&mut self)->&mut AgentAttr;
    #[allow(dead_code)]
    fn read_attr(&self)->&AgentAttr;

    fn set_attr(&mut self, attr:&AgentAttr){
        self.attr().set(attr);
    }
    fn copy_attr(&mut self, agent:&dyn Agent){
        self.attr().set(agent.read_attr());
    }

    #[allow(dead_code)]
    fn time(&mut self)->i64{
        self.attr().time
    }

    #[allow(dead_code)]
    fn get_clr(&mut self)->&Color{
        &self.attr().color
    }

    #[allow(dead_code)]
    fn set_clr(&mut self, c:&Color){
        self.attr().color.set(c);
    }
    #[allow(dead_code)]
    fn clr(&mut self, r:f32, g:f32, b:f32, a:f32){
        self.attr().clr(r,g,b,a);
    }
    #[allow(dead_code)]
    fn rgba(&mut self, r:f32, g:f32, b:f32, a:f32){
        self.attr().rgba(r,g,b,a);
    }
    #[allow(dead_code)]
    fn rgb(&mut self, r:f32, g:f32, b:f32){
        self.attr().rgb(r,g,b);
    }
    #[allow(dead_code)]
    fn hsba(&mut self, h:f32, s:f32, b:f32, a:f32){
        self.attr().hsba(h,s,b,a);
    }
    #[allow(dead_code)]
    fn hsb(&mut self, h:f32, s:f32, b:f32){
        self.attr().hsb(h,s,b);
    }

}


pub trait AgentClone {
    fn clone_box(&self)->Box<dyn Agent>;
}

impl<T> AgentClone for T where T:'static + Agent + Clone{
    fn clone_box(&self)->Box<dyn Agent>{
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Agent>{
    fn clone(&self)->Box<dyn Agent>{
        self.clone_box()
    }
}


pub trait AsAny{
    fn as_any(&self)->&dyn Any;
}

impl<T> AsAny for T where T:'static + Agent{
    fn as_any(&self)->&dyn Any{ self as &dyn Any }
}



#[agent_fields]
#[derive(Debug, Clone)]
pub struct LineAgent{
    pub pos:Vec3,
    pub dir:Vec3,
    pub colliding:bool
}

impl LineAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, dir:Vec3)->Self{
        LineAgent{pos, dir, colliding:false,attr:AgentAttr::new() }
    }
}

#[agent_methods]
impl Agent for LineAgent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn Agent>>, manager:&mut Manager){
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


        let mut agent = LineAgent::new(pos2, dir2);
        agent.copy_attr(self);
        manager.add_agent(Box::new(agent));

        manager.delete_agent(self);
    }
}

/*
#[agent_fields]
#[derive(Debug, Clone)]
pub struct Particle{
    pub pos:Vec3,
    pub vel:Vec3,
    pub frc:Vec3,
    pub fric:f32,
    pub fixed:bool,
}

impl Particle{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, vel:Vec3)->Self{
        Particle{ pos, vel, frc:Vec3::zero(), fric:0.0, fixed:false, attr:AgentAttr::new()}
    }
}

#[agent_methods]
impl Agent for Particle{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn Agent>>, manager:&mut Manager){

        self.frc.add(&Vec3::new(0.0, 0.0, -1.0)); // gravity example
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
*/

pub trait Particle{
  fn pos(&mut self)->&mut Vec3;
  fn vel(&mut self)->&mut Vec3;
  fn frc(&mut self)->&mut Vec3;
  fn fric(&self)->f32;
  fn fixed(&self)->bool;
  fn update_particle(&mut self){
      if !self.fixed(){
          let f = self.frc().clone();
          self.vel().scale_add(UPDATE_RATE as f64, &f);
          let fric = self.fric();
          self.vel().mul(1.0-fric as f64);
          self.frc().set_zero();
          let v = self.vel().clone();
          self.pos().scale_add(UPDATE_RATE as f64, &v);
      }
  }

}


#[agent_fields]
#[derive(Debug, Clone)]
pub struct OrientAgent{
    pub pos:Vec3,
    pub orient:Orient,
    pub colliding:bool
}

impl OrientAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, orient:Orient)->Self{
        OrientAgent{pos, orient, colliding:false, attr:AgentAttr::new() }
    }
}

#[agent_methods]
impl Agent for OrientAgent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn Agent>>, manager:&mut Manager){
        if self.time()==0 {
            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
                //if agents[i].attr().id != self.attr().id{
                if agents[i].attr().id != self.attr().id{
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

            if manager.time%10 == 0{
                let pos2 = self.pos.cp(&self.orient.front());
                let mut orient2 = self.orient.clone();
                orient2.pitch(PI/2.0);
                let mut agent = OrientAgent::new(pos2, orient2);
                agent.copy_attr(self);
                manager.add_agent(Box::new(agent));
            }

            if manager.time%20 == 0{
                let mut pos3 = self.pos.cp(&self.orient.front().div(2.0));
                pos3.add(self.orient.right().div(2.0));
                let mut orient3 = self.orient.clone();
                orient3.yaw(PI/2.0);
                let mut agent = OrientAgent::new(pos3, orient3);
                agent.copy_attr(self);
                manager.add_agent(Box::new(agent));
            }

            if manager.time%30==0{
                let orient2 = self.orient.clone();
                let mut agent = OrientAgent::new(pos2, orient2);
                agent.copy_attr(self);
                manager.add_agent(Box::new(agent));
            }

            if manager.time%20==0{
                let mut pos3 = self.pos.cp(&self.orient.front().div(2.0));
                pos3.add(self.orient.right().div(2.0));
                let mut orient3 = self.orient.clone();
                orient3.yaw(-PI/2.0);
                let mut agent = OrientAgent::new(pos3, orient3);
                agent.copy_attr(self);
                manager.add_agent(Box::new(agent));
            }

            if manager.time%10!=0{
                let /*mut*/ orient2 = self.orient.clone();
                //orient2.yaw(PI/50.0);
                //orient2.roll(PI/50.0);
                let mut agent = OrientAgent::new(pos2, orient2);
                agent.copy_attr(self);
                manager.add_agent(Box::new(agent));
            }
        }
    }
}
