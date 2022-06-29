use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::console::log_1;
use std::f64::consts::PI;
use ig::{Vec3, Color};

mod ig;


// agent 20220629_1
impl ig::Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<ig::Agent>>, manager:&mut ig::DataManager){
    }

    fn update(&mut self, manager:&mut ig::DataManager){
        let pos2 = self.pos.cp(&self.dir);
        let mut dir2 = self.dir.clone();

        //dir2.add(&Vec3{x:0.01, y:-0.01, z:0.0});
        //dir2.rot(&Vec3{x:0.0, y:0.0, z:1.0}, PI*0.89);
        //dir2.mul(1.004);

        /*
        if dir2.x > 0.0 {
            //dir2.rot(&Vec3{x:0.0, y:0.0, z:1.0}, PI*0.39);
            //dir2.mul(1.004);
            dir2.set( &Vec3{x: 0.0, y: dir2.x, z:0.0} );
            //dir2.add(&Vec3{x:0.5, y:-0.3, z:0.0});
        }
        else if dir2.y > 0.0 {
            dir2.set( &Vec3{x: -dir2.y-1.0, y: 0.0, z:0.0} );
        }
        else if dir2.x < 0.0{
            dir2.set( &Vec3{x: 0.0, y: dir2.x, z:0.0} );
        }
        else if dir2.y < 0.0{
            dir2.set( &Vec3{x: -dir2.y+1.0, y: 0.0, z:0.0} );
        }
        */

        /*
        if manager.time < 100{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/30.0 );
            dir2.mul(0.99);
        }
        else if manager.time < 200{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
            dir2.mul(1.01);
        }
        else if manager.time < 300{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
            dir2.mul(1.01);
        }
        else if manager.time < 330{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/15.0 );
            dir2.mul(1.01);
        }
        else{
            dir2.add(&Vec3{x:0.0, y:-0.05, z:0.0});
        }
        */

        /*
        if manager.time % 15 == 0 {
            dir2.neg();
            dir2.mul(1.01);
        }
        else{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/25.0 );
        }
        */

        /*
        if manager.time % 50 < 10{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/20.0 );
        }
        else if manager.time % 50 == 10{
            dir2.neg();
        }
        else if manager.time % 50 < 30{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/11.0 );
        }
        else if manager.time % 50 < 40{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, -PI/21.0 );
        }
        else if manager.time % 50 == 40{
            dir2.neg();
        }
        else{
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/15.0 );
            dir2.mul(1.01);
        }
        */

        //let r = rand::random::<f64>();
        //dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, (r-0.5)*2.0 );
        
        /*
        let r = rand::random::<f64>();
        if r < 0.025{
            dir2.neg();
            dir2.mul(1.1);
        }
        else{
            dir2.mul(0.996);
            dir2.rot( &Vec3{x:0.0,y:0.0,z:1.0}, PI/25.0 );
        }
        */


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



        let mut line = ig::Curve::line(self.pos, pos2);
        //let t = (manager.time as f64*0.01).sin() as f32;
        //line.clr(t, 0.5-t*0.5, 1.0-t, 1.0 );
        //line.clr(1.0, 1.0, 0.0, 1.0);
        //let t = manager.time as f32 * 0.001;
        let t = (manager.time as f32 * 0.02).sin() ;
        line.clr( 1.0-t*2.0, 0.3, 1.0-t*4.0, 0.4 );
        //line.clr( 1.0-t*2.0, 1.0, 1.0-t*4.0, 0.4 );
        //line.hsb(t, 1.0, 1.0, 1.0 );

        manager.add_curve(Box::new(line));

        let mut agent = ig::Agent::new_with_dir(pos2, dir2);
        agent.set_attr(&self.attr);
        manager.add_agent(Box::new(agent)) as i32;

        manager.delete_agent(self.id);
    }
}


/*
// agent type 1
impl ig::Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<ig::Agent>>, manager:&mut ig::DataManager){
    }

    fn update(&mut self, manager:&mut ig::DataManager){
        let pos2 = self.pos.cp(&self.dir);
        let mut dir2 = self.dir.clone();

        if pos2.x > 0.0{
            dir2.add(&Vec3::new(-0.0025, 0.0, 0.0));
        }
        else{
            dir2.add(&Vec3::new(0.0025, 0.0, 0.0));
        }

        let mut line = ig::Curve::line(self.pos, pos2);
        let t = (manager.time as f64*0.01).sin() as f32;
        line.clr(t, 0.5-t*0.5, 1.0-t, 1.0 );
        manager.add_curve(Box::new(line));

        let mut agent = ig::Agent::new_with_dir(pos2, dir2);
        agent.set_attr(&self.attr);
        manager.add_agent(Box::new(agent)) as i32;

        manager.delete_agent(self.id);
    }
}
*/

/*
// agent type 2
impl ig::Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<ig::Agent>>, storage:&mut ig::DataManager){
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

    fn update(&mut self, manager:&mut ig::DataManager){
        if self.time==0{
            if self.colliding{
                //web_sys::console::log_1(&JsValue::from(format!("Agent::update: Collided, delete" )));
                manager.delete_agent(self.id);
                return;
            }

            let mut pos2 = self.pos.clone();
            pos2.add(&self.dir);

            //let point = Box::new(ig::Point::new(self.pos.x, self.pos.y, self.pos.z));
            //manager.add_point(point) as i32;
            let mut line = ig::Curve::new_line(self.pos, pos2);
            line.set_attr(&self.attr);
            manager.add_curve(Box::new(line));

            if rand::random::<f64>()<0.8{
                let mut dir2 = self.dir.clone();
                let mut agent = ig::Agent::new_with_dir(pos2, dir2);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
            }
            if rand::random::<f64>() < 0.4{
                let mut dir3 = self.dir.clone();
                //dir3.rot(&Vec3::new(1.0,0.0,0.0), PI*rand::random::<f64>());
                dir3.rot(&Vec3::new(0.0,0.0,1.0), PI/3.0);
                let mut agent = ig::Agent::new_with_dir(pos2, dir3);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
            }
            if rand::random::<f64>() < 0.4{
                let mut dir3 = self.dir.clone();
                //dir3.rot(&Vec3::new(1.0,0.0,0.0), PI*rand::random::<f64>());
                dir3.rot(&Vec3::new(0.0,0.0,1.0), -PI/3.0);
                let mut agent = ig::Agent::new_with_dir(pos2, dir3);
                agent.set_attr(&self.attr);
                manager.add_agent(Box::new(agent)) as i32;
            }
        }
        self.time+=1;
    }
}
*/

/*
// agent type 3
impl ig::Agent{
    #[allow(unused_variables)]
    fn interact(&mut self, agents:&Vec<Box<ig::Agent>>, storage:&mut ig::DataManager){
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

    fn update(&mut self, manager:&mut ig::DataManager){
        if self.time==0{
            if self.colliding{
                //log_1(&JsValue::from(format!("Agent::update: Collided, delete" )));
                manager.delete_agent(self.id);
                return;
            }

            let mut pos2 = self.pos.cp(&self.dir);

            ////let point = Box::new(ig::Point::new(self.pos.x, self.pos.y, self.pos.z));
            ////manager.add_point(point) as i32;
            //let mut line = ig::Curve::new_line(self.pos, pos2);
            ////line.set_attr(&self.attr);
            //line.hsb((manager.time as f64*0.01).sin() as f32, 1.0, 1.0, 1.0 );
            //manager.add_curve(Box::new(line));

            let z1 = ((manager.time as f64*0.05).sin())*0.5 + 0.75;
            let z2 = ((manager.time as f64+1.0)*0.05).sin()*0.5 + 0.75;

            let zpos1 = self.pos.cp(&Vec3::new(0.0,0.0,z1));
            let zpos2 = pos2.cp(&Vec3::new(0.0,0.0,z2));

            let mut srf = ig::Surface::new_quad(self.pos, pos2, zpos2, zpos1);
            srf.hsb((manager.time as f64*0.01).sin() as f32, 0.5, 1.0, 0.5 );
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
                    let mut agent = ig::Agent::new_with_dir(pos2, dir2);
                    agent.set_attr(&self.attr);
                    agent.params.push(pct_right);
                    agent.params.push(pct_left);
                    manager.add_agent(Box::new(agent));
                }
                else{
                    let mut agent = ig::Agent::new_with_dir(pos2, dir2);
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
                    let mut agent = ig::Agent::new_with_dir(pos2, dir2);
                    agent.set_attr(&self.attr);
                    agent.params.push(pct_right);
                    agent.params.push(pct_left);
                    manager.add_agent(Box::new(agent));
                }
                else{
                    let mut agent = ig::Agent::new_with_dir(pos2, dir2);
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


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log_1(&JsValue::from(format!("initializing server")));

    let width: f32 = 1200.;
    let height: f32 = 800.;
    let mut server = ig::Server::new(width, height);
    server.init();
    //server.duration(600);
    server.duration(1000);
    //server.set_zoom(0.05);
    server.set_zoom(0.1);
    //server.set_zoom(1.0);
    server.set_camera_rotation_speed(0.5);
    server.enable_camera_rotation(false);
    server.set_camera_yaw(0.0);
    server.set_camera_pitch(PI/2.);
    //server.bg(&Color::new(0.0, 0.0, 0.0, 1.0));
    server.bg_colors(&Color::new(0.0, 0.0, 0.0, 1.0), &Color::new(0.0, 0.0, 0.0, 1.0), &Color::new(0.1, 0.0, 0.3, 1.0),  &Color::new(0.2, 0.2, 0.2, 1.0));
    //server.bg_colors(&Color::new(0.3, 0.5, 0.7, 1.0), &Color::new(0.3, 0.5, 0.7, 1.0), &Color::new(1.0, 1.0, 1.0, 1.0),  &Color::new(0.9, 0.9, 0.9, 1.0));

    /*
    let a = ig::Agent::new_with_dir(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.0, y:1.0, z:0.0});
    server.add_agent(Box::new(a));
    */

    // agent type 1
    let num = 40;
    let inc = PI*2.0/num as f64;
    for i in 0..num{

        let a = ig::Agent::new_with_dir(
            Vec3{x:(i as f64 * inc).cos()*1.0, y:(i as f64 * inc).sin()*1.0, z:0.0},
            Vec3{x:((i+6)as f64 *inc).cos()*1.25, y:((i+6)as f64*inc).sin()*0.25, z:0.0});
            /*
        let a = ig::Agent::new_with_dir(
            Vec3{x: i as f64 * 1.0, y: i as f64 * -1.0, z:0.0},
            Vec3{x:0.0, y:1.0, z:0.0});
            */

        server.add_agent(Box::new(a));
    }


    /*
    // agent type 2
    let mut a = ig::Agent::new_with_dir(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.0, y:0.2, z:0.0});
    a.clr(0.0,0.0,0.0,1.0);
    server.add_agent(Box::new(a));
    */
    /*
    // agent type 3
    let mut a = ig::Agent::new_with_dir(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:0.05, y:0.0, z:0.0});
    a.clr(1.0,1.0,0.0,1.0);
    a.params.push(100.0);
    a.params.push(4.5);
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
