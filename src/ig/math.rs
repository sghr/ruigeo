use std::fmt;

use super::geo::TOLERANCE;
//pub const TOLERANCE : f64 = 0.001;


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
    pub fn set_zero(&mut self)->&mut Self{
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
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


impl fmt::Display for Vec3{
    #[allow(dead_code)]
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f, "({},{},{})", self.x, self.y, self.z)
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
    pub fn det(v11:f64, v12:f64, v21:f64, v22:f64)->f64{
        v11*v22-v12*v21
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
        self.val[0][0]*Matrix3::det(self.val[1][1],self.val[1][2],self.val[2][1],self.val[2][2])+
        self.val[0][1]*Matrix3::det(self.val[1][2],self.val[1][0],self.val[2][2],self.val[2][0])+
        self.val[0][2]*Matrix3::det(self.val[1][0],self.val[1][1],self.val[2][0],self.val[2][1])
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


pub struct Matrix4{
    pub val: [[f64;4];4],

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
        Matrix3::det(self.val[0][0],self.val[0][1],self.val[1][0],self.val[1][1])*
        Matrix3::det(self.val[2][2],self.val[2][3],self.val[3][2],self.val[3][3]) +
        Matrix3::det(self.val[0][0],self.val[0][2],self.val[1][0],self.val[1][2])*
        Matrix3::det(self.val[2][3],self.val[2][1],self.val[3][3],self.val[3][1]) +
        Matrix3::det(self.val[0][0],self.val[0][3],self.val[1][0],self.val[1][3])*
        Matrix3::det(self.val[2][1],self.val[2][2],self.val[3][1],self.val[3][2]) +
        Matrix3::det(self.val[0][1],self.val[0][2],self.val[1][1],self.val[1][2])*
        Matrix3::det(self.val[2][0],self.val[2][3],self.val[3][0],self.val[3][3]) +
        Matrix3::det(self.val[0][3],self.val[0][1],self.val[1][3],self.val[1][1])*
        Matrix3::det(self.val[2][0],self.val[2][2],self.val[3][0],self.val[3][2]) +
        Matrix3::det(self.val[0][2],self.val[0][3],self.val[1][2],self.val[1][3])*
        Matrix3::det(self.val[2][0],self.val[2][1],self.val[3][0],self.val[3][1])
    }

    #[allow(dead_code)]
    pub fn invert(&mut self)->&mut Self{
        let det = self.determinant();

        self.set(
            self.val[1][1]*Matrix3::det(self.val[2][2],self.val[2][3],self.val[3][2],self.val[3][3]) +
            self.val[1][2]*Matrix3::det(self.val[2][3],self.val[2][1],self.val[3][3],self.val[3][1]) +
            self.val[1][3]*Matrix3::det(self.val[2][1],self.val[2][2],self.val[3][1],self.val[3][2]),

            self.val[2][1]*Matrix3::det(self.val[0][2],self.val[0][3],self.val[3][2],self.val[3][3])+
            self.val[2][2]*Matrix3::det(self.val[0][3],self.val[0][1],self.val[3][3],self.val[3][1])+
            self.val[2][3]*Matrix3::det(self.val[0][1],self.val[0][2],self.val[3][1],self.val[3][2]),

            self.val[3][1]*Matrix3::det(self.val[0][2],self.val[0][3],self.val[1][2],self.val[1][3])+
            self.val[3][2]*Matrix3::det(self.val[0][3],self.val[0][1],self.val[1][3],self.val[1][1])+
            self.val[3][3]*Matrix3::det(self.val[0][1],self.val[0][2],self.val[1][1],self.val[1][2]),

            self.val[0][1]*Matrix3::det(self.val[1][3],self.val[1][2],self.val[2][3],self.val[2][2])+
            self.val[0][2]*Matrix3::det(self.val[1][1],self.val[1][3],self.val[2][1],self.val[2][3])+
            self.val[0][3]*Matrix3::det(self.val[1][2],self.val[1][1],self.val[2][2],self.val[2][1]),


            self.val[1][2]*Matrix3::det(self.val[2][0],self.val[2][3],self.val[3][0],self.val[3][3]) +
            self.val[1][3]*Matrix3::det(self.val[2][2],self.val[2][0],self.val[3][2],self.val[3][0]) +
            self.val[1][0]*Matrix3::det(self.val[2][3],self.val[2][2],self.val[3][3],self.val[3][2]),

            self.val[2][2]*Matrix3::det(self.val[0][0],self.val[0][3],self.val[3][0],self.val[3][3])+
            self.val[2][3]*Matrix3::det(self.val[0][2],self.val[0][0],self.val[3][2],self.val[3][0])+
            self.val[2][0]*Matrix3::det(self.val[0][3],self.val[0][2],self.val[3][3],self.val[3][2]),

            self.val[3][2]*Matrix3::det(self.val[0][0],self.val[0][3],self.val[1][0],self.val[1][3])+
            self.val[3][3]*Matrix3::det(self.val[0][2],self.val[0][0],self.val[1][2],self.val[1][0])+
            self.val[3][0]*Matrix3::det(self.val[0][3],self.val[0][2],self.val[1][3],self.val[1][2]),

            self.val[0][2]*Matrix3::det(self.val[1][3],self.val[1][0],self.val[2][3],self.val[2][0])+
            self.val[0][3]*Matrix3::det(self.val[1][0],self.val[1][2],self.val[2][0],self.val[2][2])+
            self.val[0][0]*Matrix3::det(self.val[1][2],self.val[1][3],self.val[2][2],self.val[2][3]),


            self.val[1][3]*Matrix3::det(self.val[2][0],self.val[2][1],self.val[3][0],self.val[3][1]) +
            self.val[1][0]*Matrix3::det(self.val[2][1],self.val[2][3],self.val[3][1],self.val[3][3]) +
            self.val[1][1]*Matrix3::det(self.val[2][3],self.val[2][0],self.val[3][3],self.val[3][0]),

            self.val[2][3]*Matrix3::det(self.val[0][0],self.val[0][1],self.val[3][0],self.val[3][1])+
            self.val[2][0]*Matrix3::det(self.val[0][1],self.val[0][3],self.val[3][1],self.val[3][3])+
            self.val[2][1]*Matrix3::det(self.val[0][3],self.val[0][0],self.val[3][3],self.val[3][0]),

            self.val[3][3]*Matrix3::det(self.val[0][0],self.val[0][1],self.val[1][0],self.val[1][1])+
            self.val[3][0]*Matrix3::det(self.val[0][1],self.val[0][3],self.val[1][1],self.val[1][3])+
            self.val[3][1]*Matrix3::det(self.val[0][3],self.val[0][0],self.val[1][3],self.val[1][0]),

            self.val[0][3]*Matrix3::det(self.val[1][1],self.val[1][0],self.val[2][1],self.val[2][0])+
            self.val[0][0]*Matrix3::det(self.val[1][3],self.val[1][1],self.val[2][3],self.val[2][1])+
            self.val[0][1]*Matrix3::det(self.val[1][0],self.val[1][3],self.val[2][0],self.val[2][3]),


            self.val[1][0]*Matrix3::det(self.val[2][2],self.val[2][1],self.val[3][2],self.val[3][1]) +
            self.val[1][1]*Matrix3::det(self.val[2][0],self.val[2][2],self.val[3][0],self.val[3][2]) +
            self.val[1][2]*Matrix3::det(self.val[2][1],self.val[2][0],self.val[3][1],self.val[3][0]),

            self.val[2][0]*Matrix3::det(self.val[0][2],self.val[0][1],self.val[3][2],self.val[3][1])+
            self.val[2][1]*Matrix3::det(self.val[0][0],self.val[0][2],self.val[3][0],self.val[3][2])+
            self.val[2][2]*Matrix3::det(self.val[0][1],self.val[0][0],self.val[3][1],self.val[3][0]),

            self.val[3][0]*Matrix3::det(self.val[0][2],self.val[0][1],self.val[1][2],self.val[1][1])+
            self.val[3][1]*Matrix3::det(self.val[0][0],self.val[0][2],self.val[1][0],self.val[1][2])+
            self.val[3][2]*Matrix3::det(self.val[0][1],self.val[0][0],self.val[1][1],self.val[1][0]),

            self.val[0][0]*Matrix3::det(self.val[1][1],self.val[1][2],self.val[2][1],self.val[2][2])+
            self.val[0][1]*Matrix3::det(self.val[1][2],self.val[1][0],self.val[2][2],self.val[2][0])+
            self.val[0][2]*Matrix3::det(self.val[1][0],self.val[1][1],self.val[2][0],self.val[2][1])
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
