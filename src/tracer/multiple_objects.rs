use std::cell::RefCell;
use std::rc::Rc;
use color::RGBColor;
use math::Ray;
use tracer::Tracer;
use world::World;

pub struct MultipleObjects {
    world: Rc<RefCell<World>>,
}

impl MultipleObjects {
    pub fn new(w: Rc<RefCell<World>>) -> MultipleObjects {
        MultipleObjects {
            world: w,
        }
    }
}

impl Tracer for MultipleObjects {
    fn trace_ray(&self, ray: &Ray) -> Option<RGBColor> {
        let mut result = None;
        let shade_record = self.world.borrow().hit_objects(ray);
        match shade_record {
            Some(s) => {
                if s.hit_an_object() {
                    result = Some(s.color());
                }
            },
            None => { }
        }
        result
    }
}
