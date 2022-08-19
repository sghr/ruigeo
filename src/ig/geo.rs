use super::Attr;
use super::Color;
use super::math::Vec3;

/****************************
* constant
*****************************/

pub const TOLERANCE : f64 = 0.001;
#[allow(dead_code)]
pub const ANGLE_TOLERANCE : f64 = std::f64::consts::PI/1000.0;


pub trait Geometry{
}

pub struct Point/*<'a>*/{
//    object: Object,
    pub id: i32,
    pub pos: Vec3,
    pub attr: Attr
}

impl Point{
    #[allow(dead_code)]
    pub fn new(x:f64, y:f64, z:f64)->Self{
        Point{id:-1, pos:Vec3::new(x,y,z),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_vec3(pos:&Vec3)->Self{
        Point{id:-1, pos:Vec3::new_with_vec3(pos),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn set_id(&mut self, id:i32){
        self.id = id;
    }
    #[allow(dead_code)]
    pub fn set_clr(&mut self, c:&Color)->&mut Point{
        self.attr.color.set(c);
        self
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
    pub fn set_attr(&mut self, attr:&Attr)->&mut Point{
        self.attr.set(attr);
        self
    }

}


pub struct Curve/*<'a>*/{
//    object: Object/*<'a>*/,
    pub id: i32,
    pub curve: CurveGeo,
    pub attr: Attr

}

impl Curve{
    #[allow(dead_code)]
    pub fn new(cpts:Vec<Vec3>, degree:u8)->Self{
        Curve{ id:-1, curve:CurveGeo::new(cpts, degree),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_closed(cpts:Vec<Vec3>, degree:u8)->Self{
        Curve{ id:-1, curve:CurveGeo::new_closed(cpts, degree),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_knots(cpts:Vec<Vec3>, degree:u8, knots:Vec<f64>, ustart:f64, uend:f64)->Self{
        Curve{ id:-1, curve:CurveGeo::new_with_knots(cpts, degree, knots, ustart, uend),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_knots_and_weights(cpts:Vec<Vec3>, degree:u8, knots:Vec<f64>, weights:Vec<f64>, ustart:f64, uend:f64)->Self{
        Curve{ id:-1, curve:CurveGeo::new_with_knots_and_weights(cpts, degree, knots, weights, ustart, uend),attr:Attr::default()}
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
    pub fn set_clr(&mut self, c:&Color)->&mut Curve{
        self.attr.color.set(c);
        self
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
    pub fn set_attr(&mut self, attr:&Attr)->&mut Curve{
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
    pub fn sphere_knots()->Vec<f64>{
        Vec::from([ 0.,0.,0.,0.5,0.5,1.,1.,1.])
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
    pub attr: Attr
}


impl Surface{
    #[allow(dead_code)]
    pub fn new(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new(cpts, udegree, vdegree), attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_u_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_u_closed(cpts, udegree, vdegree),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_v_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_v_closed(cpts, udegree, vdegree),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_uv_closed(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_uv_closed(cpts, udegree, vdegree),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_knots(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8, uknots:Vec<f64>, vknots:Vec<f64>, ustart:f64, uend:f64, vstart:f64, vend:f64)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_with_knots(cpts, udegree, vdegree, uknots, vknots, ustart, uend, vstart, vend),attr:Attr::default()}
    }
    #[allow(dead_code)]
    pub fn new_with_knots_and_weights(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8, uknots:Vec<f64>, vknots:Vec<f64>, weights:Vec<Vec<f64>>, ustart:f64, uend:f64, vstart:f64, vend:f64)->Self{
        Surface{ id:-1, surface:SurfaceGeo::new_with_knots_and_weights(cpts, udegree, vdegree, uknots, vknots, weights, ustart, uend, vstart, vend),attr:Attr::default()}
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
    pub fn cylinder(pt1:&Vec3, pt2:&Vec3, radius1:f64, radius2:f64)->Self{
        let normal = pt2.dif(pt1);
        let (cpts1,weights1) = NurbsGeo::circle_cp(pt1, &normal, radius1);
        let (cpts2,weights2) = NurbsGeo::circle_cp(pt2, &normal, radius2);
        let cpts : Vec<Vec<Vec3>> = Vec::from([cpts1, cpts2]);
        let weights : Vec<Vec<f64>> = Vec::from([weights1, weights2]);
        Surface::new_with_knots_and_weights(cpts, 1, NurbsGeo::circle_deg(), NurbsGeo::create_knots(1,2,false), NurbsGeo::circle_knots(), weights, 0.0, 1.0, 0.0, 1.0)
    }

    #[allow(dead_code)]
    pub fn pipe(pt1:&Vec3, pt2:&Vec3, radius:f64)->Self{
        Surface::cylinder(pt1,pt2,radius,radius)
    }
/*
    #[allow(dead_code)]
    pub fn sphere(center:&Vec3, radius:f64)->Self{
        let tpt = center.clone().add(&Vec3::new(0.0,0.0,radius));
        let bpt = center.clone().add(&Vec3::new(0.0,0.0,-radius));
        let (cir_cpts, cir_weights) = NurbsGeo::circle_cp(center, &Vec3::new(0.0,0.0,1.0), radius);
        let cpts : Vec<Vec<Vec3>> = Vec::new();
        let weights : Vec<Vec<f64>> = Vec::new();

        for i in 0..cpts.len(){
            let cps :Vec<Vec3> = Vec::new();
            let wts :Vec<f64> = Vec::new();
            // 0
            cps.push(bpt.clone());
            if i%2==0{ wts.push(1.0); } else {wts.push( 2.0_f64.sqrt()/2.0); }
            // 1
            let cp1 = cir_cpts[i].clone().add(&Vec3::new(0.0,0.0,-radius);
            cps.push(cp1);
            wts.push(cir_weights[i] * 2.0_f64.sqrt()/2.0);
            // 2
            cps.push(cir_cpts[i].clone());
            wts.push(cir_weights[i]);
            // 3
            cps.push(cir_cpts[i].clone().add(&Vec3::new(0.0,0.0,radius)));
            wts.push(cir_weights[i] * 2.0_f64.sqrt()/2.0);
            // 4
            cps.push(tpt.clone());
            if i%2==0{ wts.push(1.0); } else {wts.push( 2.0_f64.sqrt()/2.0); }

            cpts.push(cps);
            weights.push(wts);
        }

        Surface::new_with_knots_and_weights(cpts, NurbsGeo::circle_deg(), NurbsGeo::circle_deg(), NurbsGeo::circle_knots, NurbsGeo::sphere_knots, weights, 0.0, 1.0, 0.0, 1.0)
    }
*/

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
    pub fn set_clr(&mut self, c:&Color)->&mut Surface{
        self.attr.color.set(c);
        self
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
    pub fn set_attr(&mut self, attr:&Attr)->&mut Surface{
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
    pub fn new_with_knots(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8, uknots:Vec<f64>, vknots:Vec<f64>, ustart:f64, uend:f64, vstart:f64, vend:f64)->Self{
        let mut weights : Vec<Vec<f64>> = Vec::new();
        #[allow(unused_variables)]
        for i in 0..cpts.len(){
            let mut w : Vec<f64> = Vec::new();
            for j in 0..cpts[i].len(){
                w.push(1.0);
            }
            weights.push(w);
        }
        SurfaceGeo::new_with_knots_and_weights(cpts, udegree, vdegree, uknots, vknots, weights, ustart, uend, vstart, vend)
    }

    #[allow(dead_code)]
    pub fn new_with_knots_and_weights(cpts:Vec<Vec<Vec3>>, udegree:u8, vdegree:u8, mut uknots:Vec<f64>, mut vknots:Vec<f64>, weights:Vec<Vec<f64>>, ustart:f64, uend:f64, vstart:f64, vend:f64)->Self{
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


/*
impl Object for Point{
    fn draw(&mut self){    }
//    fn interact(&self, objects:&Vec<Box<dyn Object>>){}
//    fn update(&self){}
//    fn attr(&self)->&mut Attr{
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
