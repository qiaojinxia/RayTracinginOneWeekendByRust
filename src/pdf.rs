use crate::vec3::Vec3;

trait Pdf{
    fn pdf_value() -> f64;
    fn generate() -> Vec3;
}

struct cosine_pdf{

}