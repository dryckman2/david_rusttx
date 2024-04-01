use crate::hittables::hittable::{HitRecord, Hittable};
use crate::hittables::hittable_list::HittableList;
use crate::math_structures::aabb::Aabb;
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::rtweekend::random_int_bounded;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_list(list: &HittableList) -> BvhNode {
        Self::from(&list.objects, 0, list.objects.len())
    }

    pub fn from(src_objects: &Vec<Box<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let left;
        let right;
        let bbox;
        let mut objects = (*src_objects).clone(); // Create a modifiable array of the source scene objects
        let axis = random_int_bounded(0, 2);
        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };
        let object_span = end - start;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                } else {
                    left = objects[start + 1].clone();
                    right = objects[start].clone();
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = start + object_span / 2;
                left = Box::new(BvhNode::from(&objects, start, mid)) as Box<dyn Hittable>;
                right = Box::new(BvhNode::from(&objects, mid, end)) as Box<dyn Hittable>;
            }
        }

        bbox = Aabb::from_aabbs(&left.bounding_box(), &right.bounding_box());
        BvhNode { left, right, bbox }
    }
    pub fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis_index: i64) -> Ordering {
        f64::total_cmp(
            &a.bounding_box().axis(axis_index).min,
            &b.bounding_box().axis(axis_index).min,
        )
    }
    pub fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec = None;
        match self.bbox.hit(r) {
            None => {
                return None;
            }
            Some(_) => {}
        }

        match self.left.hit(r, &ray_t) {
            None => {}
            Some(x) => rec = Some(x),
        }
        match self.right.hit(r, &ray_t) {
            None => {}
            Some(x) => rec = Some(x),
        }

        return rec;
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::from((*self).clone())
    }
}
//class BvhNode : public hittable {
//   public:

//
//     bool hit(const ray& r, interval ray_t, hit_record& rec) const override {
//         if (!box.hit(r, ray_t))
//             return false;
//
//         bool hit_left = left->hit(r, ray_t, rec);
//         bool hit_right = right->hit(r, interval(ray_t.min, hit_left ? rec.t : ray_t.max), rec);
//
//         return hit_left || hit_right;
//     }
//
//     aabb bounding_box() const override { return bbox; }
//
//   private:
//     shared_ptr<hittable> left;
//     shared_ptr<hittable> right;
//     aabb bbox;
// };
