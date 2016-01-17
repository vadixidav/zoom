extern crate num;
use self::num::Float;
use super::*;
use super::super::Vector;

use std::cell::UnsafeCell;

pub struct BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
    quanta: D,
    position: V,
    velocity: V,
    force: UnsafeCell<V>,
    inertia: D,
}

impl<V, D> BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
    pub fn new(quanta: D, position: V, velocity: V, inertia: D) -> Self {
        BasicParticle{
            quanta: quanta,
            position: position,
            velocity: velocity,
            force: UnsafeCell::new(V::zero()),
            inertia: inertia,
        }
    }
}

impl<V, D> Quanta<D> for BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
    fn quanta(&self) -> D {
        self.quanta
    }
}

impl<V, D> Inertia<D> for BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
    fn inertia(&self) -> D {
        self.inertia
    }
}

impl<V, D> Position<V> for BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
    fn position(&self) -> V {
        self.position
    }
}

impl <V, D> Velocity<V> for BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
    fn velocity(&self) -> V {
        self.velocity
    }
}

impl<V, D> Particle<V, D> for BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
    fn impulse(&self, vec: &V) {
        unsafe {
            *self.force.get() = *self.force.get() + *vec;
        }
    }

    fn advance(&mut self, time: D) {
        let force = unsafe {&mut *self.force.get()};
        self.velocity = self.velocity + *force / self.inertia() * time;
        self.position = self.position + self.velocity * time;
        *force = V::zero();
    }
}

impl<V, D> PhysicsParticle<V, D> for BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
}
