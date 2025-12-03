#![no_std]
#![no_main]


use diamondfire::vec3::Vec3;


#[derive(Clone, Copy)]
pub struct Transform {
    pub translation : Vec3,
    pub rotation    : Quat,
    pub scale       : Vec3
}

#[derive(Clone, Copy)]
pub struct Quat {
    pub x : f64,
    pub y : f64,
    pub z : f64,
    pub w : f64
}
// impl Quat {
    // #[unsafe(no_mangle)]
    // pub fn from_axis_angle(axis : Vec3, angle : f64) -> Quat {
    //     let ha = angle / 2.0;
    //     let s  = ha.sin();
    //     let c  = ha.cos();
    //     Quat {
    //         x : axis.x() * s,
    //         y : axis.y() * s,
    //         z : axis.z() * s,
    //         w : c
    //     }
    // }
    #[inline]
    pub fn inverse(quat : Quat) -> Quat {
        Quat {
            x : -quat.x,
            y : -quat.y,
            z : -quat.z,
            w : quat.w
        }
    }
    pub fn to_axes(quat : Quat) -> (Vec3, Vec3, Vec3,) {
        let x2 = quat.x * 2.0;
        let y2 = quat.y * 2.0;
        let z2 = quat.z * 2.0;
        let xx = quat.x * x2;
        let xy = quat.x * y2;
        let xz = quat.x * z2;
        let yy = quat.y * y2;
        let yz = quat.y * z2;
        let zz = quat.z * z2;
        let wx = quat.w * x2;
        let wy = quat.w * y2;
        let wz = quat.w * z2;
        (
            Vec3::new(1.0 - (yy + zz), xy + wz, xz - wy),
            Vec3::new(xy - wz, 1.0 - (xx + zz), yz + wx),
            Vec3::new(xz + wy, yz - wx, 1.0 - (xx + yy))
        )
    }
// }

#[repr(transparent)]
pub struct Mat4 {
    cells : [f64; 16]
}
// impl Mat4 {
    pub fn from_transform(transform : Transform) -> Mat4 {
        let (x, y, z,) = to_axes(transform.rotation);
        Mat4 { cells : [
            x.x() * transform.scale.x(),
            y.x() * transform.scale.y(),
            z.x() * transform.scale.z(),
            transform.translation.x(),
            x.y() * transform.scale.x(),
            y.y() * transform.scale.y(),
            z.y() * transform.scale.z(),
            transform.translation.y(),
            x.z() * transform.scale.x(),
            y.z() * transform.scale.y(),
            z.z() * transform.scale.z(),
            transform.translation.z(),
            0.0, 0.0, 0.0, 1.0
        ] }
    }
// }
