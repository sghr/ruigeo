use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::console::log_1;
use std::f64::consts::PI;
use std::collections::VecDeque;
#[allow(unused_imports)]

mod ig;

use ig::{Agent, Color,Orient, Vec3, Point, Curve, Surface, Server, DataManager, Manager, Server2, LineAgent, Particle, OrientAgent, AgentTrait, AgentInfo};


// agent 20220630: orthogonal
impl Agent{

    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<Agent>>, storage:&mut DataManager){
        if self.time==0 {
            //if self.orient.front().z < 0.0{
            //    self.colliding=true;
            //    return;
            //}
            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
                if agents[i].id != self.id{
                    let dif = agents[i].pos.dif(&pos2);
                    if dif.dot(&front) >= 0.0{
                        if dif.len() < l * 1.0{
                            self.colliding=true;
                            break;
                        }
                    }
                }
                else{
                    break;
                }
            }
        }
    }

    fn update(&mut self, manager:&mut DataManager){
        if self.time == 0{
            if self.colliding{
                manager.delete_agent(self.id);
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
                let mut agent = Agent::new_with_orient(pos2, orient2);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }

            if manager.time%20 == 0{
                let mut pos3 = self.pos.cp(&self.orient.front().div(2.0));
                pos3.add(self.orient.right().div(2.0));
                let mut orient3 = self.orient.clone();
                orient3.yaw(PI/2.0);
                let mut agent = Agent::new_with_orient(pos3, orient3);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }

            if manager.time%30==0{
                let mut orient2 = self.orient.clone();
                let mut agent = Agent::new_with_orient(pos2, orient2);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }

            if manager.time%20==0{
                let mut pos3 = self.pos.cp(&self.orient.front().div(2.0));
                pos3.add(self.orient.right().div(2.0));
                let mut orient3 = self.orient.clone();
                orient3.yaw(-PI/2.0);
                let mut agent = Agent::new_with_orient(pos3, orient3);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }

            if manager.time%10!=0{
                let mut orient2 = self.orient.clone();
                //orient2.yaw(PI/50.0);
                orient2.roll(PI/50.0);
                let mut agent = Agent::new_with_orient(pos2, orient2);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }

            //manager.delete_agent(self.id);
        }
        self.time+=1;
    }
}


#[derive(Debug, Clone)]
pub struct OrthoAgent{
    pub pos:Vec3,
    pub orient:Orient,
    pub info:AgentInfo,
    pub colliding:bool
}

impl AgentTrait for OrthoAgent{

    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }

    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){
        if self.time()==0 {
            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
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
                let mut agent = OrthoAgent::new(pos2, orient2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            if manager.time%20 == 0{
                let mut pos3 = self.pos.cp(&self.orient.front().div(2.0));
                pos3.add(self.orient.right().div(2.0));
                let mut orient3 = self.orient.clone();
                orient3.yaw(PI/2.0);
                let mut agent = OrthoAgent::new(pos3, orient3);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            if manager.time%30==0{
                let mut orient2 = self.orient.clone();
                let mut agent = OrthoAgent::new(pos2, orient2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            if manager.time%20==0{
                let mut pos3 = self.pos.cp(&self.orient.front().div(2.0));
                pos3.add(self.orient.right().div(2.0));
                let mut orient3 = self.orient.clone();
                orient3.yaw(-PI/2.0);
                let mut agent = OrthoAgent::new(pos3, orient3);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }

            if manager.time%10!=0{
                let mut orient2 = self.orient.clone();
                //orient2.yaw(PI/50.0);
                //orient2.roll(PI/50.0);
                let mut agent = OrthoAgent::new(pos2, orient2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
                child_num += 1;
            }
        }
    }
}

impl OrthoAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, orient:Orient)->Self{
        OrthoAgent{pos, orient, colliding:false, info:AgentInfo::new() }
    }
}


#[derive(Debug, Clone)]
pub struct RandomOrthoAgent{
    pub pos:Vec3,
    pub orient:Orient,
    pub info:AgentInfo,
    pub colliding:bool
}

impl AgentTrait for RandomOrthoAgent{

    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }

    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){
        if self.time()==0 {
            if self.orient.front().z < 0.0{
                self.colliding=true;
                return;
            }
            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
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

            let t = (manager.time as f32 * 0.02).sin() ;
            let mut line = Curve::line(self.pos, pos2);
            //line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
            line.clr( 1.0, 0.5, 1.0,1.0 );
            manager.add_curve(Box::new(line));

            let pt11 = self.pos.cp(&self.orient.left().div(2.0));
            let pt12 = self.pos.cp(&self.orient.right().div(2.0));
            let pt21 = pos2.cp(&self.orient.left().div(2.0));
            let pt22 = pos2.cp(&self.orient.right().div(2.0));
            let mut surf = Surface::quad(pt11,pt12,pt22,pt21);
            //surf.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 1.0 );
            surf.clr( rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0 );
            manager.add_surface(Box::new(surf));

            let mut child_num = 0;

            if manager.time%4 == 0{
                if rand::random::<f64>() < 0.2{ // up
                    let pos2 = self.pos.cp(&self.orient.front());
                    let mut orient2 = self.orient.clone();
                    orient2.pitch(PI/2.0);
                    let mut agent = RandomOrthoAgent::new(pos2, orient2);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }

                if rand::random::<f64>() < 0.2{ // down
                    let pos2 = self.pos.cp(&self.orient.front());
                    let mut orient2 = self.orient.clone();
                    orient2.pitch(-PI/2.0);
                    let mut agent = RandomOrthoAgent::new(pos2, orient2);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }

                if rand::random::<f64>() < 0.2{ // right
                    let mut pos2 = self.pos.cp(&self.orient.front().div(2.0));
                    pos2.add(self.orient.right().div(2.0));
                    let mut orient2 = self.orient.clone();
                    orient2.yaw(-PI/2.0);
                    let mut agent = RandomOrthoAgent::new(pos2, orient2);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }

                if rand::random::<f64>() < 0.2{ // left
                    let mut pos2 = self.pos.cp(&self.orient.front().div(2.0));
                    pos2.add(self.orient.left().div(2.0));
                    let mut orient2 = self.orient.clone();
                    orient2.yaw(PI/2.0);
                    let mut agent = RandomOrthoAgent::new(pos2, orient2);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }
            }
            if rand::random::<f64>() < 0.6 || child_num == 0{ // front
                let mut orient2 = self.orient.clone();
                let mut agent = RandomOrthoAgent::new(pos2, orient2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }
        }
    }
}

impl RandomOrthoAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, orient:Orient)->Self{
        RandomOrthoAgent{pos, orient, colliding:false, info:AgentInfo::new() }
    }
}


/*
// agent 20220630: random orthogonal
impl Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<Agent>>, storage:&mut DataManager){
        if self.time==0 {
            if self.orient.front().z < 0.0{
                self.colliding=true;
                return;
            }

            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
                if agents[i].id != self.id{
                    let dif = agents[i].pos.dif(&pos2);
                    if dif.dot(&front) >= 0.0{
                        if dif.len() < l * 1.0{
                            self.colliding=true;
                            break;
                        }
                    }
                    else if agents[i].orient.front().dot(&self.orient.front()) > 0.0 && agents[i].pos.eq(&self.pos){
                        self.colliding=true;
                        break;
                    }
                }
                else{
                    break;
                }
            }
        }
    }

    fn update(&mut self, manager:&mut DataManager){
        if self.time == 0{
            if self.colliding{
                manager.delete_agent(self.id);
                return;
            }

            let pos2 = self.pos.cp(&self.orient.front());

            let t = (manager.time as f32 * 0.02).sin() ;
            let mut line = Curve::line(self.pos, pos2);
            //line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
            line.clr( 1.0, 0.5, 1.0,1.0 );
            manager.add_curve(Box::new(line));

            let pt11 = self.pos.cp(&self.orient.left().div(2.0));
            let pt12 = self.pos.cp(&self.orient.right().div(2.0));
            let pt21 = pos2.cp(&self.orient.left().div(2.0));
            let pt22 = pos2.cp(&self.orient.right().div(2.0));
            let mut surf = Surface::quad(pt11,pt12,pt22,pt21);
            //surf.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 1.0 );
            surf.clr( rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0 );
            manager.add_surface(Box::new(surf));

            let mut child_num = 0;

            if manager.time%4 == 0{
                if rand::random::<f64>() < 0.2{ // up
                    let pos2 = self.pos.cp(&self.orient.front());
                    let mut orient2 = self.orient.clone();
                    orient2.pitch(PI/2.0);
                    let mut agent = Agent::new_with_orient(pos2, orient2);
                    agent.set_attr(&self.attr);
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }

                if rand::random::<f64>() < 0.2{ // down
                    let pos2 = self.pos.cp(&self.orient.front());
                    let mut orient2 = self.orient.clone();
                    orient2.pitch(-PI/2.0);
                    let mut agent = Agent::new_with_orient(pos2, orient2);
                    agent.set_attr(&self.attr);
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }

                if rand::random::<f64>() < 0.2{ // right
                    let mut pos2 = self.pos.cp(&self.orient.front().div(2.0));
                    pos2.add(self.orient.right().div(2.0));
                    let mut orient2 = self.orient.clone();
                    orient2.yaw(-PI/2.0);
                    let mut agent = Agent::new_with_orient(pos2, orient2);
                    agent.set_attr(&self.attr);
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }

                if rand::random::<f64>() < 0.2{ // left
                    let mut pos2 = self.pos.cp(&self.orient.front().div(2.0));
                    pos2.add(self.orient.left().div(2.0));
                    let mut orient2 = self.orient.clone();
                    orient2.yaw(PI/2.0);
                    let mut agent = Agent::new_with_orient(pos2, orient2);
                    agent.set_attr(&self.attr);
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }
            }
            if rand::random::<f64>() < 0.6 || child_num == 0{ // front
                let mut orient2 = self.orient.clone();
                let mut agent = Agent::new_with_orient(pos2, orient2);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }

            //manager.delete_agent(self.id);
        }
        self.time+=1;
    }
}
*/

#[derive(Debug, Clone)]
pub struct SpiralBranchAgent{
    pub pos:Vec3,
    pub orient:Orient,
    pub info:AgentInfo,
    pub colliding:bool
}

impl AgentTrait for SpiralBranchAgent{

    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }

    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){

        if self.time()==0 {
            if self.orient.front().z < 0.0{
                self.colliding=true;
                return;
            }

            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
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

            let t = (manager.time as f32 * 0.05).sin() ;
            let mut line = Curve::line(self.pos, pos2);
            //line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
            line.clr( 1.0, 1.0, 1.0,1.0 );
            manager.add_curve(Box::new(line));

            let pt11 = self.pos.cp(&self.orient.left().div(2.0));
            let pt12 = self.pos.cp(&self.orient.right().div(2.0));
            let pt21 = pos2.cp(&self.orient.left().div(2.0));
            let pt22 = pos2.cp(&self.orient.right().div(2.0));
            let mut surf = Surface::quad(pt11,pt12,pt22,pt21);
            surf.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 1.0 );
            //surf.clr( rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0 );
            manager.add_surface(Box::new(surf));

            let mut child_num = 0;

            if manager.time%10==0{
                if rand::random::<f64>() < 0.2{ // up
                    let pos2 = self.pos.cp(&self.orient.front());
                    let mut orient2 = self.orient.clone();
                    orient2.pitch(PI/6.0);
                    let mut agent = SpiralBranchAgent::new(pos2, orient2);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }

                if rand::random::<f64>() < 0.2{ // down
                    let pos2 = self.pos.cp(&self.orient.front());
                    let mut orient2 = self.orient.clone();
                    orient2.pitch(-PI/6.0);
                    let mut agent = SpiralBranchAgent::new(pos2, orient2);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }
            }

            if rand::random::<f64>() < 0.9 || child_num == 0{ // front
                let mut orient2 = self.orient.clone();
                orient2.yaw(0.1);
                orient2.roll(0.2);
                let mut agent = SpiralBranchAgent::new(pos2, orient2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }
        }
    }
}

impl SpiralBranchAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, orient:Orient)->Self{
        SpiralBranchAgent{pos, orient, colliding:false, info:AgentInfo::new() }
    }
}


/*
// agent 20220630: spiral branch
impl Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<Agent>>, storage:&mut DataManager){
        if self.time==0 {
            //if self.orient.front().z < 0.0{
            //    self.colliding=true;
            //    return;
            //}
            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
                if agents[i].id != self.id{
                    let dif = agents[i].pos.dif(&pos2);
                    if dif.dot(&front) >= 0.0{
                        if dif.len() < l * 1.0{
                            self.colliding=true;
                            break;
                        }
                    }
                }
                else{
                    break;
                }
            }
        }
    }

    fn update(&mut self, manager:&mut DataManager){
        if self.time == 0{
            if self.colliding{
                manager.delete_agent(self.id);
                return;
            }

            let pos2 = self.pos.cp(&self.orient.front());

            let t = (manager.time as f32 * 0.05).sin() ;
            let mut line = Curve::line(self.pos, pos2);
            //line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
            line.clr( 1.0, 1.0, 1.0,1.0 );
            manager.add_curve(Box::new(line));

            let pt11 = self.pos.cp(&self.orient.left().div(2.0));
            let pt12 = self.pos.cp(&self.orient.right().div(2.0));
            let pt21 = pos2.cp(&self.orient.left().div(2.0));
            let pt22 = pos2.cp(&self.orient.right().div(2.0));
            let mut surf = Surface::quad(pt11,pt12,pt22,pt21);
            surf.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 1.0 );
            //surf.clr( rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0 );
            manager.add_surface(Box::new(surf));

            let mut child_num = 0;

            if manager.time%10==0{
                if rand::random::<f64>() < 0.2{ // up
                    let pos2 = self.pos.cp(&self.orient.front());
                    let mut orient2 = self.orient.clone();
                    orient2.pitch(PI/6.0);
                    let mut agent = Agent::new_with_orient(pos2, orient2);
                    agent.set_attr(&self.attr);
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }

                if rand::random::<f64>() < 0.2{ // down
                    let pos2 = self.pos.cp(&self.orient.front());
                    let mut orient2 = self.orient.clone();
                    orient2.pitch(-PI/6.0);
                    let mut agent = Agent::new_with_orient(pos2, orient2);
                    agent.set_attr(&self.attr);
                    manager.add_agent(Box::new(agent)) as i32;
                    child_num += 1;
                }
            }

            if rand::random::<f64>() < 0.9 || child_num == 0{ // front
                let mut orient2 = self.orient.clone();
                orient2.yaw(0.1);
                orient2.roll(0.2);
                let mut agent = Agent::new_with_orient(pos2, orient2);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
                child_num += 1;
            }

            //manager.delete_agent(self.id);
        }
        self.time+=1;
    }
}
*/



#[derive(Debug, Clone)]
pub struct SimpleOrientAgent{
    pub pos:Vec3,
    pub orient:Orient,
    pub info:AgentInfo,
    pub colliding:bool
}

impl AgentTrait for SimpleOrientAgent{

    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }

    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){

        if self.time()==0 {
            if self.orient.front().z < 0.0{
                self.colliding=true;
                return;
            }

            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
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

            let t = (manager.time as f32 * 0.05).sin() ;
            let mut line = Curve::line(self.pos, pos2);
            //line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
            line.clr( 1.0, 1.0, 1.0,1.0 );
            manager.add_curve(Box::new(line));

            let pt11 = self.pos.cp(&self.orient.left().div(2.0));
            let pt12 = self.pos.cp(&self.orient.right().div(2.0));
            let pt21 = pos2.cp(&self.orient.left().div(2.0));
            let pt22 = pos2.cp(&self.orient.right().div(2.0));
            let mut surf = Surface::quad(pt11,pt12,pt22,pt21);
            surf.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 1.0 );
            //surf.clr( rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0 );
            manager.add_surface(Box::new(surf));

            let mut orient2 = self.orient.clone();
            //orient2.yaw(0.1);
            //orient2.roll(0.2);
            let mut agent = SimpleOrientAgent::new(pos2, orient2);
            agent.info().set(&self.info());
            manager.add_agent(Box::new(agent));
        }
    }
}

impl SimpleOrientAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, orient:Orient)->Self{
        SimpleOrientAgent{pos, orient, colliding:false, info:AgentInfo::new() }
    }
}


/*
// agent 20220630: orientation agent base
impl Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<Agent>>, storage:&mut DataManager){
        if self.time==0 {
            //if self.orient.front().z < 0.0{
            //    self.colliding=true;
            //    return;
            //}
            self.colliding = false;
            let front = self.orient.front();
            let l = front.len();
            let pos2 = self.pos.cp(&front);
            for i in 0..agents.len(){
                if agents[i].id != self.id{
                    let dif = agents[i].pos.dif(&pos2);
                    if dif.dot(&front) >= 0.0{
                        if dif.len() < l * 1.0{
                            self.colliding=true;
                            break;
                        }
                    }
                }
                else{
                    break;
                }
            }
        }
    }

    fn update(&mut self, manager:&mut DataManager){
        if self.time == 0{
            if self.colliding{
                manager.delete_agent(self.id);
                return;
            }

            let pos2 = self.pos.cp(&self.orient.front());

            let t = (manager.time as f32 * 0.05).sin() ;
            let mut line = Curve::line(self.pos, pos2);
            //line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
            line.clr( 1.0, 1.0, 1.0,1.0 );
            manager.add_curve(Box::new(line));

            let pt11 = self.pos.cp(&self.orient.left().div(2.0));
            let pt12 = self.pos.cp(&self.orient.right().div(2.0));
            let pt21 = pos2.cp(&self.orient.left().div(2.0));
            let pt22 = pos2.cp(&self.orient.right().div(2.0));
            let mut surf = Surface::quad(pt11,pt12,pt22,pt21);
            surf.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 1.0 );
            //surf.clr( rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0 );
            manager.add_surface(Box::new(surf));

            let mut orient2 = self.orient.clone();
            //orient2.yaw(0.1);
            //orient2.roll(0.2);
            let mut agent = Agent::new_with_orient(pos2, orient2);
            agent.set_attr(&self.attr);
            manager.add_agent(Box::new(agent)) as i32;

            //manager.delete_agent(self.id);
        }
        self.time+=1;
    }
}
*/

#[allow(dead_code)]
pub fn random_percent(percent:f64)->bool{
    rand::random::<f64>() * 100.0 < percent
}

#[derive(Debug, Clone)]
pub struct MyLineAgent2{
    pub pos:Vec3,
    pub dir:Vec3,
    pub info:AgentInfo,
}



impl AgentTrait for MyLineAgent2{

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

        //dir2.add(&Vec3{x:0.01, y:-0.01, z:0.0});
        //dir2.rot(&Vec3{x:0.0, y:0.0, z:1.0}, PI*0.89);
        //dir2.mul(1.004);


        //if dir2.x > 0.0 {
        //    //dir2.rot(&Vec3{x:0.0, y:0.0, z:1.0}, PI*0.39);
        //    //dir2.mul(1.004);
        //    dir2.set( &Vec3{x: 0.0, y: dir2.x, z:0.0} );
        //    //dir2.add(&Vec3{x:0.5, y:-0.3, z:0.0});
        //}
        //else if dir2.y > 0.0 {
        //    dir2.set( &Vec3{x: -dir2.y-1.0, y: 0.0, z:0.0} );
        //}
        //else if dir2.x < 0.0{
        //    dir2.set( &Vec3{x: 0.0, y: dir2.x, z:0.0} );
        //}
        //else if dir2.y < 0.0{
        //    dir2.set( &Vec3{x: -dir2.y+1.0, y: 0.0, z:0.0} );
        //}



        //if manager.time < 100{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/30.0 );
        //    dir2.mul(0.99);
        //}
        //else if manager.time < 200{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
        //    dir2.mul(1.01);
        //}
        //else if manager.time < 300{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
        //    dir2.mul(1.01);
        //}
        //else if manager.time < 330{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
        //    dir2.mul(1.01);
        //}
        //else{
        //    dir2.add(&Vec3{x:0.0, y:-0.05, z:0.0});
        //}

        //if manager.time % 15 == 0 {
        //    dir2.neg();
        //    dir2.mul(1.01);
        //}
        //else{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/25.0 );
        //}

        //if manager.time % 50 < 10{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/20.0 );
        //}
        //else if manager.time % 50 == 10{
        //    dir2.neg();
        //}
        //else if manager.time % 50 < 30{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/11.0 );
        //}
        //else if manager.time % 50 < 40{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/21.0 );
        //}
        //else if manager.time % 50 == 40{
        //    dir2.neg();
        //}
        //else{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/15.0 );
        //    dir2.mul(1.01);
        //}


        //let r = rand::random::<f64>();
        //dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, (r-0.5)*2.0 );


        //let r = rand::random::<f64>();
        //if r < 0.025{
        //    dir2.neg();
        //    dir2.mul(1.1);
        //}
        //else{
        //    dir2.mul(0.996);
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/25.0 );
        //}


        if manager.time % 60 < 20{
            //dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/20.0 );
        }
        else if manager.time % 60 == 20{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/3.0 );
        }
        else if manager.time % 60 < 41{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/3.0 );
            dir2.mul(1.05);
        }
        else if manager.time % 60 < 50{
            dir2.mul(0.99);
        }
        else if manager.time % 60 == 50{
            dir2.neg();
            dir2.mul(0.40);
        }
        else{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/17.0 );
            dir2.mul(1.01);
        }

        let mut line = Curve::line(self.pos, pos2);
        //let t = (manager.time as f64*0.01).sin() as f32;
        //line.clr(t, 0.5-t*0.5, 1.0-t, 1.0 );
        //line.clr(1.0, 1.0, 0.0, 1.0);
        //let t = manager.time as f32 * 0.001;
        let t = (manager.time as f32 * 0.02).sin() ;
        line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.5 );
        //line.clr( 1.0-t*2.0, 1.0, 1.0-t*4.0, 0.4 );
        //line.hsb(t, 1.0, 1.0, 1.0 );
        //line.clr(0.0, 0.0, 0.0, 1.0 );
        manager.add_curve(Box::new(line));

        let l = self.dir.len()*0.5;
        let mut srf = Surface::quad(self.pos, pos2, pos2.cp(&Vec3::new(0.0,0.0,l)), self.pos.cp(&Vec3::new(0.0,0.0,l)) );
        srf.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
        manager.add_surface(Box::new(srf));

        let mut agent = MyLineAgent2::new(pos2, dir2);
        agent.info().set(&self.info());
        manager.add_agent(Box::new(agent));

        manager.delete_agent(self);
    }
}

impl MyLineAgent2{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, dir:Vec3)->Self{
        MyLineAgent2{pos, dir, info:AgentInfo::new() }
    }
}



/*
// agent 20220629
impl Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<Agent>>, manager:&mut DataManager){
    }

    fn update(&mut self, manager:&mut DataManager){
        let pos2 = self.pos.cp(&self.dir);
        let mut dir2 = self.dir.clone();

        //dir2.add(&Vec3{x:0.01, y:-0.01, z:0.0});
        //dir2.rot(&Vec3{x:0.0, y:0.0, z:1.0}, PI*0.89);
        //dir2.mul(1.004);


        //if dir2.x > 0.0 {
        //    //dir2.rot(&Vec3{x:0.0, y:0.0, z:1.0}, PI*0.39);
        //    //dir2.mul(1.004);
        //    dir2.set( &Vec3{x: 0.0, y: dir2.x, z:0.0} );
        //    //dir2.add(&Vec3{x:0.5, y:-0.3, z:0.0});
        //}
        //else if dir2.y > 0.0 {
        //    dir2.set( &Vec3{x: -dir2.y-1.0, y: 0.0, z:0.0} );
        //}
        //else if dir2.x < 0.0{
        //    dir2.set( &Vec3{x: 0.0, y: dir2.x, z:0.0} );
        //}
        //else if dir2.y < 0.0{
        //    dir2.set( &Vec3{x: -dir2.y+1.0, y: 0.0, z:0.0} );
        //}



        //if manager.time < 100{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/30.0 );
        //    dir2.mul(0.99);
        //}
        //else if manager.time < 200{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
        //    dir2.mul(1.01);
        //}
        //else if manager.time < 300{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
        //    dir2.mul(1.01);
        //}
        //else if manager.time < 330{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
        //    dir2.mul(1.01);
        //}
        //else{
        //    dir2.add(&Vec3{x:0.0, y:-0.05, z:0.0});
        //}



        //if manager.time % 15 == 0 {
        //    dir2.neg();
        //    dir2.mul(1.01);
        //}
        //else{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/25.0 );
        //}



        //if manager.time % 50 < 10{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/20.0 );
        //}
        //else if manager.time % 50 == 10{
        //    dir2.neg();
        //}
        //else if manager.time % 50 < 30{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/11.0 );
        //}
        //else if manager.time % 50 < 40{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/21.0 );
        //}
        //else if manager.time % 50 == 40{
        //    dir2.neg();
        //}
        //else{
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/15.0 );
        //    dir2.mul(1.01);
        //}


        //let r = rand::random::<f64>();
        //dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, (r-0.5)*2.0 );


        //let r = rand::random::<f64>();
        //if r < 0.025{
        //    dir2.neg();
        //    dir2.mul(1.1);
        //}
        //else{
        //    dir2.mul(0.996);
        //    dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/25.0 );
        //}


        if manager.time % 60 < 20{
            //dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/20.0 );
        }
        else if manager.time % 60 == 20{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/3.0 );
        }
        else if manager.time % 60 < 41{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/3.0 );
            dir2.mul(1.05);
        }
        else if manager.time % 60 < 50{
            dir2.mul(0.99);
        }
        else if manager.time % 60 == 50{
            dir2.neg();
            dir2.mul(0.40);
        }
        else{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/17.0 );
            dir2.mul(1.01);
        }



        let mut line = Curve::line(self.pos, pos2);
        //let t = (manager.time as f64*0.01).sin() as f32;
        //line.clr(t, 0.5-t*0.5, 1.0-t, 1.0 );
        //line.clr(1.0, 1.0, 0.0, 1.0);
        //let t = manager.time as f32 * 0.001;
        let t = (manager.time as f32 * 0.02).sin() ;
        line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.5 );
        //line.clr( 1.0-t*2.0, 1.0, 1.0-t*4.0, 0.4 );
        //line.hsb(t, 1.0, 1.0, 1.0 );
        //line.clr(0.0, 0.0, 0.0, 1.0 );
        manager.add_curve(Box::new(line));

        let l = self.dir.len()*0.5;
        let mut srf = Surface::quad(self.pos, pos2, pos2.cp(&Vec3::new(0.0,0.0,l)), self.pos.cp(&Vec3::new(0.0,0.0,l)) );
        srf.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
        manager.add_surface(Box::new(srf));

        let mut agent = Agent::new_with_dir(pos2, dir2);
        agent.set_attr(&self.attr);
        manager.add_agent(Box::new(agent)) as i32;

        manager.delete_agent(self.id);
    }
}
*/




#[derive(Debug, Clone)]
pub struct MyLineAgent{
    pub pos:Vec3,
    pub dir:Vec3,
    pub info:AgentInfo,
}

impl AgentTrait for MyLineAgent{

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

        //dir2.rot(&Vec3::new(0.0,0.0,1.0), 0.1);
        //dir2.mul(0.999);

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
        manager.add_curve(Box::new(line));

        //let mut pos3 = self.pos.clone();
        //pos3.add(&Vec3::new(0.0, 0.0, 1.0));
        //let mut pos4 = pos2.clone();
        //pos4.add(&Vec3::new(0.0, 0.0, 1.0));
        //let mut srf = Surface::quad(self.pos, pos2, pos4, pos3);
        //srf.clr(1.0-t, 0.5-t*0.5, t*0.5, 1.0 );
        //manager.add_surface(Box::new(srf));

        //let mut cir = Curve::circle(&self.pos, &self.dir, 0.1);
        //cir.clr(t, -0.5+t*0.5, 1.0-t, 1.0 );
        //manager.add_curve(Box::new(cir));

        //let mut pipe = Surface::pipe(&self.pos,&pos2, 0.2);
        //pipe.clr(t, -0.5+t*0.5, 1.0-t, 1.0 );
        //manager.add_surface(Box::new(pipe));

        let mut agent = MyLineAgent::new(pos2, dir2);
        agent.info().set(&self.info());
        manager.add_agent(Box::new(agent));

        manager.delete_agent(self);
    }
}

impl MyLineAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, dir:Vec3)->Self{
        MyLineAgent{pos, dir, info:AgentInfo::new() }
    }
}



/*
// agent type 1
impl Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<Agent>>, manager:&mut DataManager){
    }

    fn update(&mut self, manager:&mut DataManager){
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

        //let mut cir = Curve::circle(&self.pos, &self.dir, 0.1);
        //cir.clr(t, -0.5+t*0.5, 1.0-t, 1.0 );
        //manager.add_curve(Box::new(cir));

        //let mut pipe = Surface::pipe(&self.pos,&pos2, 0.2);
        //pipe.clr(t, -0.5+t*0.5, 1.0-t, 1.0 );
        //manager.add_surface(Box::new(pipe));



        let mut agent = Agent::new_with_dir(pos2, dir2);
        agent.set_attr(&self.attr);
        manager.add_agent(Box::new(agent)) as i32;

        manager.delete_agent(self.id);
    }
}
*/



#[derive(Debug, Clone)]
pub struct MyBranchAgent{
    pub pos:Vec3,
    pub dir:Vec3,
    pub info:AgentInfo,
    pub colliding:bool,
}

impl AgentTrait for MyBranchAgent{

    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }

    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){
        if self.time()==0 {
            self.colliding = false;
            let l = self.dir.len();
            let pos2 = self.pos.cp(&self.dir);
            for i in 0..agents.len(){
                if agents[i].info().id != self.info().id{
                    match agents[i].as_any().downcast_ref::<Self>(){
                        Some(a) => {
                            let apos2 = a.pos.cp(&a.dir);
                            if (apos2.x - pos2.x ).abs() <= l{
                                if (apos2.y - pos2.y).abs() <= l{
                                    if (apos2.z - pos2.z).abs() <= l{
                                        let dist = apos2.dist(&pos2);
                                        if dist < l*0.99{
                                            self.colliding=true;
                                            break;
                                        }
                                    }
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
        if self.time()==0{
            if self.colliding{
                //web_sys::console::log_1(&JsValue::from(format!("Agent::update: Collided, delete" )));
                manager.delete_agent(self);
                return;
            }

            let mut pos2 = self.pos.clone();
            pos2.add(&self.dir);

            //let point = Box::new(Point::new(self.pos.x, self.pos.y, self.pos.z));
            //manager.add_point(point) as i32;
            let mut line = Curve::line(self.pos, pos2);
            //line.set_attr(&self.attr);
            //line.clr(1.0,1.0,1.0,1.0);
            line.clr(0.0,0.0,0.0,1.0);
            manager.add_curve(Box::new(line));

            if rand::random::<f64>()<0.8{
                let mut dir2 = self.dir.clone();
                let mut agent = MyBranchAgent::new(pos2, dir2);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
            }
            if rand::random::<f64>() < 0.4{
                let mut dir3 = self.dir.clone();
                //dir3.rot(&Vec3::new(1.0,0.0,0.0), PI*rand::random::<f64>());
                dir3.rot(&Vec3::new(0.0,0.0,1.0), PI/3.0);
                let mut agent = MyBranchAgent::new(pos2, dir3);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
            }
            if rand::random::<f64>() < 0.4{
                let mut dir3 = self.dir.clone();
                //dir3.rot(&Vec3::new(1.0,0.0,0.0), PI*rand::random::<f64>());
                dir3.rot(&Vec3::new(0.0,0.0,1.0), -PI/3.0);
                let mut agent = MyBranchAgent::new(pos2, dir3);
                agent.info().set(&self.info());
                manager.add_agent(Box::new(agent));
            }
        }
    }
}

impl MyBranchAgent{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, dir:Vec3)->Self{
        MyBranchAgent{pos, dir, colliding:false, info:AgentInfo::new() }
    }
}



/*
// agent type 2
impl Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<Agent>>, storage:&mut DataManager){
        if self.time==0 {
            self.colliding = false;
            let l = self.dir.len();
            let pos2 = self.pos.cp(&self.dir);
            for i in 0..agents.len(){
                if agents[i].id != self.id{
                    let apos2 = agents[i].pos.cp(&agents[i].dir);
                    if (apos2.x - pos2.x ).abs() <= l{
                        if (apos2.y - pos2.y).abs() <= l{
                            if (apos2.z - pos2.z).abs() <= l{
                                let dist = apos2.dist(&pos2);
                                if dist < l*0.99{
                                    self.colliding=true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self, manager:&mut DataManager){
        if self.time==0{
            if self.colliding{
                //web_sys::console::log_1(&JsValue::from(format!("Agent::update: Collided, delete" )));
                manager.delete_agent(self.id);
                return;
            }

            let mut pos2 = self.pos.clone();
            pos2.add(&self.dir);

            //let point = Box::new(Point::new(self.pos.x, self.pos.y, self.pos.z));
            //manager.add_point(point) as i32;
            let mut line = Curve::line(self.pos, pos2);
            //line.set_attr(&self.attr);
            //line.clr(1.0,1.0,1.0,1.0);
            line.clr(0.0,0.0,0.0,1.0);
            manager.add_curve(Box::new(line));

            if rand::random::<f64>()<0.8{
                let mut dir2 = self.dir.clone();
                let mut agent = Agent::new_with_dir(pos2, dir2);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
            }
            if rand::random::<f64>() < 0.4{
                let mut dir3 = self.dir.clone();
                //dir3.rot(&Vec3::new(1.0,0.0,0.0), PI*rand::random::<f64>());
                dir3.rot(&Vec3::new(0.0,0.0,1.0), PI/3.0);
                let mut agent = Agent::new_with_dir(pos2, dir3);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
            }
            if rand::random::<f64>() < 0.4{
                let mut dir3 = self.dir.clone();
                //dir3.rot(&Vec3::new(1.0,0.0,0.0), PI*rand::random::<f64>());
                dir3.rot(&Vec3::new(0.0,0.0,1.0), -PI/3.0);
                let mut agent = Agent::new_with_dir(pos2, dir3);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
            }
        }
        self.time+=1;
    }
}
*/


#[derive(Debug, Clone)]
pub struct MyBranchAgent2{
    pos:Vec3,
    dir:Vec3,
    pct_l:f64,
    pct_r:f64,
    colliding:bool,
    info:AgentInfo,
}

impl AgentTrait for MyBranchAgent2{

    #[allow(unused_variables)]
    fn info(&mut self)->&mut AgentInfo{
        &mut self.info
    }

    #[allow(unused_variables)]
    fn interact(&mut self, agents:&mut Vec<Box<dyn AgentTrait>>, manager:&mut Manager){
        if self.time()==0 {
            self.colliding = false;
            let pos2 = self.pos.cp(&self.dir);
            for i in 0..agents.len(){
                if agents[i].info().id != self.info().id{

                    match agents[i].as_any().downcast_ref::<Self>(){
                        Some(a) => {
                            let apos2 = a.pos.cp(&a.dir);

                            if !apos2.eq(&self.pos) && !a.pos.eq(&self.pos) {
                                let itxn =  Vec3::intersect_segment(&a.pos, &apos2, &self.pos, &pos2);
                                if !itxn.is_none(){
                                    self.colliding=true;
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
        if self.time()==0{
            if self.colliding{
                manager.delete_agent(self);
                return;
            }

            let pos2 = self.pos.cp(&self.dir);

            ////let point = Box::new(Point::new(self.pos.x, self.pos.y, self.pos.z));
            ////manager.add_point(point) as i32;
            //let mut line = Curve::line(self.pos, pos2);
            ////line.set_attr(&self.attr);
            //line.hsb((manager.time as f64*0.01).sin() as f32, 1.0, 1.0, 1.0 );
            //manager.add_curve(Box::new(line));

            let len = self.dir.len();

            let z1 = (((manager.time as f64*0.05).sin())*0.5 + 0.5)*10.0+1.0;
            let z2 = (((manager.time as f64+1.0)*0.05).sin()*0.5 + 0.5)*10.0+1.0;

            let zpos1 = self.pos.cp(&Vec3::new(0.0,0.0,z1));
            let zpos2 = pos2.cp(&Vec3::new(0.0,0.0,z2));

            let mut srf = Surface::quad(self.pos, pos2, zpos2, zpos1);
            //srf.hsb(((manager.time as f64*0.01).sin()*0.5+0.5) as f32, 1.0, 1.0, 1.0 );
            srf.hsb( (manager.time % 100) as f32/100.0, 1.0, 1.0, 1.0 );
            manager.add_surface(Box::new(srf));

            let branch_left = rand::random::<f64>()*100.0 < self.pct_l;
            let branch_right = rand::random::<f64>()*100.0 < self.pct_r;

            let mut left_len = len;
            let mut right_len = len;
            left_len *= 0.995;
            right_len *= 0.995;

            if branch_left && branch_right {
                if rand::random::<f64>()*100.0 < 50.0{
                    if self.pct_l < self.pct_r { left_len *= 0.9; }
                    else{ right_len *= 0.9; }
                }
                else if rand::random::<f64>()*100.0 < 6.0{
                    if self.pct_l < self.pct_r { left_len *= 0.4; }
                    else{ right_len *= 4.0; }
                }
                else if rand::random::<f64>()*100.0 < 5.0{
                    if self.pct_l < self.pct_r { left_len *= 4.0; }
                    else{ right_len *= 4.0; }
                }
            }

            if branch_left{
                let mut dir2 = self.dir.clone();
                dir2.set_len(left_len);
                dir2.rot2(PI/30.0);

                if branch_right && self.pct_r > self.pct_l{
                    let mut agent = MyBranchAgent2::new(pos2, dir2, self.pct_r, self.pct_l);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent));
                }
                else{
                    let mut agent = MyBranchAgent2::new(pos2, dir2, self.pct_l, self.pct_r);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent));
                }
            }

            if branch_right{
                let mut dir2 = self.dir.clone();
                dir2.set_len(right_len);
                dir2.rot2(-PI/30.0);

                if branch_left && self.pct_r < self.pct_l{
                    let mut agent = MyBranchAgent2::new(pos2, dir2, self.pct_r, self.pct_l);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent));
                }
                else{
                    let mut agent = MyBranchAgent2::new(pos2, dir2, self.pct_l, self.pct_r);
                    agent.info().set(&self.info());
                    manager.add_agent(Box::new(agent));
                }
            }
        }
    }
}


impl MyBranchAgent2{
    #[allow(dead_code)]
    pub fn new(pos:Vec3, dir:Vec3, pct_l:f64, pct_r:f64)->Self{
        MyBranchAgent2{pos, dir, pct_l, pct_r, colliding:false, info:AgentInfo::new() }
    }
}


/*
// agent type 3
impl Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<Agent>>, storage:&mut DataManager){
        if self.time==0 {
            self.colliding = false;
            let pos2 = self.pos.cp(&self.dir);
            for i in 0..agents.len(){
                if agents[i].id != self.id{
                    let apos2 = agents[i].pos.cp(&agents[i].dir);
                    if !apos2.eq(&self.pos) && !agents[i].pos.eq(&self.pos) {
                        let itxn =  Vec3::intersect_segment(&agents[i].pos, &apos2, &self.pos, &pos2 );
                        if !itxn.is_none(){
                            if self.vecs.len()==0{
                                self.vecs.push(itxn.unwrap());
                            }
                            else{
                                self.vecs[0].set(&itxn.unwrap());
                                self.colliding=true;
                            }
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self, manager:&mut DataManager){
        if self.time==0{
            if self.colliding{
                //log_1(&JsValue::from(format!("Agent::update: Collided, delete" )));
                manager.delete_agent(self.id);
                return;
            }

            let pos2 = self.pos.cp(&self.dir);

            ////let point = Box::new(Point::new(self.pos.x, self.pos.y, self.pos.z));
            ////manager.add_point(point) as i32;
            //let mut line = Curve::line(self.pos, pos2);
            ////line.set_attr(&self.attr);
            //line.hsb((manager.time as f64*0.01).sin() as f32, 1.0, 1.0, 1.0 );
            //manager.add_curve(Box::new(line));

            let z1 = (((manager.time as f64*0.05).sin())*0.5 + 0.5)*10.0+1.0;
            let z2 = (((manager.time as f64+1.0)*0.05).sin()*0.5 + 0.5)*10.0+1.0;

            let zpos1 = self.pos.cp(&Vec3::new(0.0,0.0,z1));
            let zpos2 = pos2.cp(&Vec3::new(0.0,0.0,z2));

            let mut srf = Surface::quad(self.pos, pos2, zpos2, zpos1);
            srf.hsb((manager.time as f64*0.01).sin() as f32, 1.0, 1.0, 0.7 );
            manager.add_surface(Box::new(srf));


            let pct_left = self.params[0];
            let pct_right= self.params[1];

            let branch_left = rand::random::<f64>()*100.0 < pct_left;
            let branch_right = rand::random::<f64>()*100.0 < pct_right;

            let mut left_len = self.dir.len();
            let mut right_len = self.dir.len();
            left_len *= 0.995;
            right_len *= 0.995;

            if branch_left && branch_right {
                if rand::random::<f64>()*100.0 < 50.0{
                    if pct_left < pct_right { left_len *= 0.9; }
                    else{ right_len *= 0.9; }
                }
                else if rand::random::<f64>()*100.0 < 6.0{
                    if pct_left < pct_right { left_len *= 0.4; }
                    else{ right_len *= 4.0; }
                }
                else if rand::random::<f64>()*100.0 < 5.0{
                    if pct_left < pct_right { left_len *= 4.0; }
                    else{ right_len *= 4.0; }
                }
            }

            if branch_left{
                let mut dir2 = self.dir.clone();
                dir2.set_len(left_len);
                dir2.rot2(PI/30.0);

                if branch_right && pct_right > pct_left{
                    let mut agent = Agent::new_with_dir(pos2, dir2);
                    agent.set_attr(&self.attr);
                    agent.params.push(pct_right);
                    agent.params.push(pct_left);
                    manager.add_agent(Box::new(agent));
                }
                else{
                    let mut agent = Agent::new_with_dir(pos2, dir2);
                    agent.set_attr(&self.attr);
                    agent.params.push(pct_left);
                    agent.params.push(pct_right);
                    manager.add_agent(Box::new(agent));
                }
            }

            if branch_right{
                let mut dir2 = self.dir.clone();
                dir2.set_len(right_len);
                dir2.rot2(-PI/30.0);

                if branch_left && pct_right < pct_left{
                    let mut agent = Agent::new_with_dir(pos2, dir2);
                    agent.set_attr(&self.attr);
                    agent.params.push(pct_right);
                    agent.params.push(pct_left);
                    manager.add_agent(Box::new(agent));
                }
                else{
                    let mut agent = Agent::new_with_dir(pos2, dir2);
                    agent.set_attr(&self.attr);
                    agent.params.push(pct_left);
                    agent.params.push(pct_right);
                    manager.add_agent(Box::new(agent));
                }
            }
        }
        self.time+=1;
    }
}
*/


pub trait AbstractAgent : AgentClone{
    fn interact(&mut self, agents:&VecDeque<Box<dyn AbstractAgent>>);
    fn update(&mut self);
    //fn clone_ref(&self)->&Self;
}

pub trait AgentClone {
    fn clone_box(&self)->Box<dyn AbstractAgent>;
}

impl<T> AgentClone for T where T:'static + AbstractAgent + Clone, {
    fn clone_box(&self)->Box<dyn AbstractAgent>{
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AbstractAgent>{
    fn clone(&self)->Box<dyn AbstractAgent>{
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Agent1{  }
impl AbstractAgent for Agent1{
    fn interact(&mut self, agents:&VecDeque<Box<dyn AbstractAgent>>){}
    fn update(&mut self){
    }
}

#[derive(Clone)]
pub struct Agent2{  }
impl AbstractAgent for Agent2{
    fn interact(&mut self, agents:&VecDeque<Box<dyn AbstractAgent>>){}
    fn update(&mut self){
    }
}
pub struct MyServer{
    agents:VecDeque<Box<dyn AbstractAgent>>,
}
impl MyServer{
    fn add(&mut self, agent: Box<dyn AbstractAgent>){
        self.agents.push_back(agent);
    }
    fn update(&mut self){

        let mut copy : VecDeque<Box<dyn AbstractAgent>> = VecDeque::new();


        for agent in &mut self.agents{
            copy.push_back(agent.clone_box());
        //for i in 0..self.agents.len(){
            //self.storage.agents[i].interact(agents[j], &mut mgr);
            //Agent::interact(&mut self.storage.agents[i], &agents_copy, &mut mgr)
            //AgentTrait::interact(&mut self.storage.agents.get(i), &agents_copy, &mut mgr)
        //    agent.interact(self.agents);
        }
        for agent in &mut self.agents{
            agent.interact(&copy);
        }


        //for i in 0..self.agents.len(){
        for agent in &mut self.agents{
            agent.update();
        }
    }
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log_1(&JsValue::from(format!("initializing server")));

    let width: f32 = 1200.;
    let height: f32 = 800.;
    //let mut server = Server::new(width, height);
    let mut server = Server2::new(width, height);
    server.init();
    server.duration(2000);
    //server.duration(600);
    //server.duration(100);
    server.set_active_agent_limit_num(100000);
    server.set_zoom(0.1);
    //server.set_zoom(0.2);
    //server.set_zoom(1.0);
    //server.set_camera_rotation_speed(0.5);
    server.enable_camera_rotation(true);
    //server.enable_camera_rotation(false);
    //server.set_camera_yaw(0.0);
    //server.set_camera_pitch(PI/12.);
    server.set_camera_pitch(PI/4.);
    //server.set_camera_pitch(PI/2.); // xy plane
    //server.bg(&Color::new(0.0, 0.0, 0.0, 1.0));
    //server.bg_colors(&Color::new(0.0, 0.0, 0.0, 1.0), &Color::new(0.0, 0.0, 0.0, 1.0), &Color::new(0.1, 0.0, 0.3, 1.0),  &Color::new(0.2, 0.2, 0.2, 1.0));
    //server.bg_colors(&Color::new(0.3, 0.5, 0.7, 1.0), &Color::new(0.3, 0.5, 0.7, 1.0), &Color::new(1.0, 1.0, 1.0, 1.0),  &Color::new(0.9, 0.9, 0.9, 1.0));

/*

    let oa = OrthoAgent::new(Vec3{x:0.0, y:0.0, z:0.0}, Orient::new(*Vec3{x:1.0, y:0.0, z:1.0}.unit(), *Vec3{x:0.0,y:-1.0,z:0.0}.unit()));
    server.add_agent(Box::new(oa));

    let ra = RandomOrthoAgent::new(Vec3{x:0.0, y:0.0, z:-15.0}, Orient::new(Vec3{x:1.0, y:0.0, z:0.0}, Vec3{x:0.0,y:0.0,z:1.0}));
    server.add_agent(Box::new(ra));

    let sa = SpiralBranchAgent::new(Vec3{x:0.0, y:0.0, z:0.0}, Orient::new(Vec3{x:0.0, y:0.0, z:1.0}, Vec3{x:1.0,y:0.0,z:0.0}));
    server.add_agent(Box::new(sa));

    let soa = SimpleOrientAgent::new(Vec3{x:0.0, y:0.0, z:0.0}, Orient::new(Vec3{x:0.0, y:0.0, z:1.0}, Vec3{x:1.0,y:0.0,z:0.0}));
    server.add_agent(Box::new(soa));
*/
    //let mla2 = MyLineAgent2::new(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.0, y:1.0, z:0.1});
    //server.add_agent(Box::new(mla2));

/*
    let num = 40;
    let inc = PI*2.0/num as f64;
    for i in 0..num{
        let mla = MyLineAgent::new(
            Vec3{x:(i as f64 * inc).cos()*4.0, y:(i as f64 * inc).sin()*4.0, z:0.0},
            Vec3{x:((i+6)as f64 *inc).cos()*0.5, y:((i+6)as f64*inc).sin()*0.1, z:0.0});

        //let a = Agent::new_with_dir(
        //    Vec3{x: i as f64 * 1.0, y: i as f64 * -1.0, z:0.0},
        //    Vec3{x:0.0, y:1.0, z:0.0});

        server.add_agent(Box::new(mla));
    }
*/

/*
    let mut mba = MyBranchAgent::new(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.0, y:0.2, z:0.0});
    mba.clr(0.0,0.0,0.0,1.0);
    server.add_agent(Box::new(mba));
*/

/*
    let agent : LineAgent = LineAgent::new_with_dir(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.05));
    server.add_agent(Box::new(agent));
*/


    let mut mba2 = MyBranchAgent2::new(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.5, y:0.0, z:0.0}, 100.0, 4.5);
    mba2.clr(1.0,1.0,0.0,1.0);
    server.add_agent(Box::new(mba2));

/*
    for i in 0..10 {
        let mut p = Vec3::new(10.0,0.0,0.0);
        let mut d = Vec3::new(5.0,0.0,i as f64);

        p.rot(&Vec3::new(0.0,0.0,1.0), i as f64 * 0.3);
        d.rot(&Vec3::new(0.0,0.0,1.0), i as f64 * 0.3);

        let mut particle : Particle = Particle::new(p, d);
        particle.info.clr(1.0, 0.0, i as f32 * 0.1, 1.0);
        server.add_agent(Box::new(particle));
    }
*/

    // agent type 1

    //let vec : Vec3 = Vec3{ x: 0.0, y:0.0, z:0.0 };
    //let vec2 : Vec3 = Vec3::new(0.0, 0.0, 0.0);

    //let agent : Agent = Agent::new_with_dir(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.05));
    //server.add_agent(Box::new(agent));

    /*
    let num = 40;
    let inc = PI*2.0/num as f64;
    for i in 0..num{

        let a = Agent::new_with_dir(
            Vec3{x:(i as f64 * inc).cos()*4.0, y:(i as f64 * inc).sin()*4.0, z:0.0},
            Vec3{x:((i+6)as f64 *inc).cos()*0.5, y:((i+6)as f64*inc).sin()*0.1, z:0.0});

        //let a = Agent::new_with_dir(
        //    Vec3{x: i as f64 * 1.0, y: i as f64 * -1.0, z:0.0},
        //    Vec3{x:0.0, y:1.0, z:0.0});

        server.add_agent(Box::new(a));
    }
    */

    /*
    // agent type 2
    let mut a = Agent::new_with_dir(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.0, y:0.2, z:0.0});
    a.clr(0.0,0.0,0.0,1.0);
    server.add_agent(Box::new(a));
    */

    /*
    // agent type 3
    let mut a = Agent::new_with_dir(Vec3{x:0.0, y:0.0, z:-5.0}, Vec3{x:1.0, y:0.0, z:0.0});
    a.clr(1.0,1.0,0.0,1.0);
    a.params.push(100.0);
    a.params.push(4.5);
    server.add_agent(Box::new(a));
    */

    /*
    // agent 20220629
    let a = Agent::new_with_dir(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.0, y:1.0, z:0.1});
    //let a = Agent::new_with_dir(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.0, y:1.0, z:0.0});
    server.add_agent(Box::new(a));
    */

    /*
    // agent 20220630: orientation agent base
    // agent 20220630: spiral branch
    let a = Agent::new_with_orient(Vec3{x:0.0, y:0.0, z:0.0}, Orient::new(Vec3{x:0.0, y:0.0, z:1.0}, Vec3{x:1.0,y:0.0,z:0.0}));
    server.add_agent(Box::new(a));
    */

    /*
    // agent 20220630: random orthogonal
    let a = Agent::new_with_orient(Vec3{x:0.0, y:0.0, z:-15.0}, Orient::new(Vec3{x:1.0, y:0.0, z:0.0}, Vec3{x:0.0,y:0.0,z:1.0}));
    //let a = Agent::new_with_orient(Vec3{x:0.0, y:0.0, z:0.0}, Orient::new(*Vec3{x:1.0, y:1.0, z:1.0}.unit(), *Vec3{x:1.0,y:-1.0,z:0.0}.unit()));
    server.add_agent(Box::new(a));
    */

    /*
    // agent 20220630: orthogonal
    //let a = Agent::new_with_orient(Vec3{x:0.0, y:0.0, z:0.0}, Orient::new(Vec3{x:0.0, y:0.0, z:1.0}, Vec3{x:0.0,y:1.0,z:0.0}));
    let a = Agent::new_with_orient(Vec3{x:0.0, y:0.0, z:0.0}, Orient::new(*Vec3{x:1.0, y:0.0, z:1.0}.unit(), *Vec3{x:0.0,y:-1.0,z:0.0}.unit()));
    server.add_agent(Box::new(a));
    */




    //call once per animation frame
    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        server.draw();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().unwrap().request_animation_frame(f.as_ref().unchecked_ref()).expect("should register 'requestAnimationFrame'");
}
