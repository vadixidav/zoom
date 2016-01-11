extern crate num;
use self::num::Float;
use super::*;
use super::super::Vector;

pub struct BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
    quanta: D,
    position: V,
    velocity: V,
    acceleration: V,
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
            acceleration: V::zero(),
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
    fn accelerate(&mut self, vec: &V) {
        self.acceleration = self.acceleration + *vec;
    }

    fn advance(&mut self, time: D) {
        self.velocity = self.velocity + self.acceleration * time;
        self.position = self.position + self.velocity * time;
        self.acceleration = V::zero();
    }
}

impl<V, D> PhysicsParticle<V, D> for BasicParticle<V, D>
    where V: Vector<D>, D: Float
{
}
