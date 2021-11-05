use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

//圆的公式:(x - C_x)^2 + (y - C_y) ^2 + (z - C_z)^2 = r^2
//光线在t时刻所看到的点 P(t) = A + tb A是原点 t 是时间 b 是方向向量
//把P(t)带入圆的方程 写成矩阵形式 (P(t) -C) ⋅ (P(t) -C) = r^2
// 进一步化简 => (A + tb -C) ⋅ (A + tb -C) => t^2b ⋅ b + 2tb  ⋅ (A - C) + (A - C) ⋅(A - C) - r^2 = 0
//上面的式子 是个 未知数为t的2次方程 可以用求根公式 计算 b^2 - 4ac > 0
// 如果有大于零 说明 圆与 射线有2个交点 如果 = 0 说明有一个交点 如果小于0 说明 没交点
// -b +- sqrt(b^2 - 4ac) / 2a
// b的系数是2 把b 替换为 h 所以 h = b / 2
// 所以 √b^2 - 4ac =>   √2^2(h)^2 - 2^2*ac => 2√h^2 - ac 可以和分母 2a 可以约掉 2
//最后 -h +- sqrt(h^2 - 2ac) / a
pub(crate) fn hit_sphere(center: Point3, radius:f64, ray:Ray) -> f64{
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b =  Vec3::dot(oc,ray.direction());
    let c = oc.length_squared()- radius * radius;
    let discriminant = half_b * half_b -   a  * c;
    if discriminant < 0.0 {
        return - 1.0;
    }
    return (-half_b - discriminant.sqrt()) / a ;
}
