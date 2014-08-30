extern crate native;
extern crate nalgebra;
extern crate ncollide = "ncollide3df32";
extern crate nphysics = "nphysics3df32";
extern crate nphysics_testbed3d;

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use nalgebra::na::{Vec3, Translation};
use nalgebra::na;
use ncollide::volumetric::Volumetric;
use ncollide::geom::{Plane, Cuboid, Compound, CompoundData, Geom};
use nphysics::world::World;
use nphysics::object::RigidBody;
use nphysics_testbed3d::Testbed;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    /*
     * World
     */
    let mut world = World::new();
    world.set_gravity(Vec3::new(0.0f32, -9.81, 0.0));

    /*
     * Planes
     */
    let rb   = RigidBody::new_static(Plane::new(Vec3::new(0.0f32, 1.0, 0.0)), 0.3, 0.6);
    let body = Rc::new(RefCell::new(rb));

    world.add_body(body.clone());

    /*
     * Cross shaped geometry
     */
    let mut cross_geoms = CompoundData::new();
    cross_geoms.push_geom(na::one(), Cuboid::new(Vec3::new(5.0f32, 0.25, 0.25)), 1.0);
    cross_geoms.push_geom(na::one(), Cuboid::new(Vec3::new(0.25f32, 5.0, 0.25)), 1.0);
    cross_geoms.push_geom(na::one(), Cuboid::new(Vec3::new(0.25f32, 0.25, 5.0)), 1.0);

    let compound = Compound::new(cross_geoms);
    let mass     = compound.mass_properties(&1.0);
    let cross    = Arc::new(box compound as Box<Geom + Send + Sync>);

    /*
     * Create the crosses 
     */
    let num     = 6;
    let rad     = 5.0;
    let shift   = (rad + 0.08) * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = 30.0 + shift / 2.0;
    let centerz = shift * (num as f32) / 2.0;

    for i in range(0u, num) {
        for j in range(0u, num) {
            for k in range(0u, num) {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery;
                let z = k as f32 * shift - centerz;

                let mut rb = RigidBody::new(cross.clone(), Some(mass), 0.3, 0.5);

                rb.append_translation(&Vec3::new(x, y, z));

                let body = Rc::new(RefCell::new(rb));

                world.add_body(body.clone());
            }
        }
    }

    /*
     * Set up the testbed.
     */
    let mut testbed = Testbed::new(world);

    testbed.look_at(Vec3::new(-30.0, 30.0, -30.0), Vec3::new(0.0, 0.0, 0.0));
    testbed.run();
}
