///!Contains traits and methods for the operation of particle physics

pub mod basic_particle;
pub use self::basic_particle::*;

extern crate num;
use self::num::Float;
use super::vector::*;

///An object that has quanta
pub trait Quanta<D> {
    //Retrieve the quanta of a physics particle.
    fn quanta(&self) -> D;
}

///An object that has inertia
pub trait Inertia<D> {
    //Retrieve the inertia of a physics particle.
    fn inertia(&self) -> D;
}

///An object that has location
pub trait Position<V> {
    //Get position of particle.
    fn position(&self) -> V;
}

///An object that has velocity
pub trait Velocity<V> {
    //Get velocity of particle.
    fn velocity(&self) -> V;
}

///An object that has a simple particle motion interface
pub trait Particle<V, D>: Position<V> + Velocity<V> + Inertia<D> {
    ///Apply force to particle, but it isn't moved forward in time until advance is called.
    ///This requires interior mutability and unsafe declarations to fulfil.
    fn impulse(&self, vec: &V);
    ///Advance particle forward in time (update position and velocity from net force).
    fn advance(&mut self, time: D);
}

///Any particle that implements the necessary traits gains access to all of the capabilities of PhysicsParticle.
pub trait PhysicsParticle<V, D>: Particle<V, D> + Quanta<D> + Inertia<D>
    where V: Vector<D>, D: Float
{
    ///Convert a PhysicsParticle into a "basic particle" that implements PhysicsParticle but has the minimum members.
    ///Use this to implement data structures that need to create intermediary particles.
    fn basic_form(&self) -> BasicParticle<V, D> {
        BasicParticle::new(self.quanta(), self.position(), self.velocity(), self.inertia())
    }

    ///Apply drag forces to a particle.
    fn drag(&self, magnitude: D) {
        let force = -self.velocity() * magnitude;
        self.impulse(&force);
    }

    ///Apply proper attraction to a single physics particle towards a location and with a magnitude.
    fn gravitate_to<T: ?Sized>(&self, center: &T, magnitude: D)
        where T: Quanta<D> + Position<V>
    {
        //Create delta vector from the particle to the center of attraction.
        let delta = center.position() - self.position();
        let distance = delta.displacement();
        if distance.is_normal() {
            let force = delta / distance.powi(3) * magnitude * self.quanta() * center.quanta();
            self.impulse(&force);
        }
    }

    ///This works the same as gravitate_radius_squared and gravitate_to.
    fn gravitate_radius_to<T: ?Sized>(&self, center: &T, magnitude: D)
        where T: Quanta<D> + Position<V> + Ball<D>
    {
        //Create delta vector from the particle to the center of attraction.
        let delta = center.position() - self.position();
        let distance_squared = delta.displacement_squared();
        if distance_squared.is_normal() {
            let force = delta * magnitude * self.quanta() * center.quanta() /
                if distance_squared > center.radius().powi(2) {
                    distance_squared.sqrt().powi(3)
                } else {
                    center.radius().powi(2)
                };
            self.impulse(&force);
        }
    }

    ///Apply spring forces between one particle and a virtual particle that is unaffected.
    fn hooke_to<T: ?Sized>(&self, center: &T, magnitude: D)
        where T: Quanta<D> + Position<V>
    {
        let delta = center.position() - self.position();
        let force = delta * magnitude * self.quanta() * center.quanta();
        self.impulse(&force);
    }

    ///Apply spring forces between one particle and a virtual particle that is unaffected.
    fn hooke_equilibrium_to<T: ?Sized>(&self, center: &T, equilibrium: D, magnitude: D)
        where T: Quanta<D> + Position<V>
    {
        let delta = center.position() - self.position();
        let displace = delta.displacement();
        if displace.is_normal() {
            let force = delta / displace * (displace - equilibrium) *
                magnitude * self.quanta() * center.quanta();
            self.impulse(&force);
        }
    }

    ///Apply lorentz force to a particle in a field given by a vector with the magnitude and direction of the field.
    fn lorentz_field(&self, field: &V)
        where V: CrossVector
    {
        let force = V::cross(&self.velocity(), field) * self.quanta() / self.inertia();
        self.impulse(&force);
    }

    ///Apply the lorentz force on a virtual particle that is unaffected.
    fn lorentz_to<T: ?Sized>(&self, center: &T, magnitude: D)
        where V: CrossVector, T: Quanta<D> + Position<V> + Velocity<V>
    {
        let delta = center.position() - self.position();
        let distance_squared = delta.displacement_squared();
        if distance_squared.is_normal() {
            let force = V::cross(&V::cross(&self.velocity(), &delta), &center.velocity()) * magnitude *
                self.quanta() * center.quanta() / distance_squared.sqrt().powi(3);

            self.impulse(&force);
        }
    }

    ///Apply the lorentz force on a virtual particle that is unaffected.
    fn lorentz_radius_to<T: ?Sized>(&self, center: &T, magnitude: D)
        where V: CrossVector, T: Quanta<D> + Position<V> + Velocity<V> + Ball<D>
    {
        let delta = center.position() - self.position();
        let distance_squared = delta.displacement_squared();
        if distance_squared.is_normal() {
            let force = V::cross(&V::cross(&self.velocity(), &delta), &center.velocity()) * magnitude *
                self.quanta() * center.quanta() /
                if distance_squared > center.radius().powi(2) {
                    distance_squared.sqrt().powi(3)
                } else {
                    center.radius().powi(2)
                };

            self.impulse(&force);
        }
    }
}

///Apply proper attraction between two physics particles based on their quanta and position.
pub fn gravitate<V, D, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D>, D: Float
{
    //Create delta vector between the two positions.
    let delta = rhs.position() - lhs.position();
    //Find the distance of the delta vector.
    let distance = delta.displacement();
    if distance.is_normal() {
        //Dividing the delta vector by the distance cubed computes attractive force as per the inverse square law.
        //The extra degree normalizes the direction vector to a unit vector.
        let force = delta * magnitude / distance.powi(3) *
        //Multiply by the two quanta of the objects to compute the final force.
        lhs.quanta() * rhs.quanta();

        //Force lhs in the direction of force.
        lhs.impulse(&force);
        //Apply the inverse force to the rhs similarly.
        rhs.impulse(&-force);
    }
}

///Use a special comp_delta closure to compute the delta from the first to the second param
pub fn gravitate_delta<V, D, F, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D, comp_delta: F)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D>, D: Float, F: FnOnce(V, V) -> V
{
    //Create delta vector between the two positions.
    let delta = comp_delta(lhs.position(), rhs.position());
    //Find the distance of the delta vector.
    let distance = delta.displacement();
    if distance.is_normal() {
        //Dividing the delta vector by the distance cubed computes attractive force as per the inverse square law.
        //The extra degree normalizes the direction vector to a unit vector.
        let force = delta * magnitude / distance.powi(3) *
        //Multiply by the two quanta of the objects to compute the final force.
        lhs.quanta() * rhs.quanta();

        //Force lhs in the direction of force.
        lhs.impulse(&force);
        //Apply the inverse force to the rhs similarly.
        rhs.impulse(&-force);
    }
}

#[test]
fn gravitate_test() {
    type P = BasicParticle<Cartesian3<f64>, f64>;
    let mut a = P::default();
    let mut b = P::default();
    gravitate(&a, &b, 1.0);
    a.advance(1.0);
    b.advance(1.0);
}

///If gravitating at a distance greater than the net radius, then gravitational interaction is applied as if the
///particles are point particles. If the distance is less than the radius, then the interaction happens as if the
///gravitational quanta (mass) is evenly distributed and gravitational flux is used instead, which causes the
///interaction to become proportional to the radius, meaning that as the radius approaches zero, so does the force.
pub fn gravitate_radius<V, D, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D)
    where T1: PhysicsParticle<V, D> + Ball<D>, T2: PhysicsParticle<V, D> + Ball<D>, V: Vector<D>, D: Float
{
    let delta = rhs.position() - lhs.position();
    let distance_squared = delta.displacement_squared();
    if distance_squared.is_normal() {
        let radius_squared = (lhs.radius() + rhs.radius()).powi(2);
        //Force is the only thing that changes from normal gravitation.
        let force = delta * magnitude * lhs.quanta() * rhs.quanta() / if distance_squared > radius_squared {
            distance_squared.sqrt().powi(3)
        } else {
            radius_squared
        };
        lhs.impulse(&force);
        rhs.impulse(&-force);
    }
}

///Same as gravitate_radius, but uses a special comp_delta closure to compute the delta between the particles
pub fn gravitate_radius_delta<V, D, F, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D, comp_delta: F)
    where T1: PhysicsParticle<V, D> + Ball<D>, T2: PhysicsParticle<V, D> + Ball<D>, V: Vector<D>, D: Float,
    F: FnOnce(V, V) -> V
{
    //Create delta vector between the two positions.
    let delta = comp_delta(lhs.position(), rhs.position());
    let distance_squared = delta.displacement_squared();
    if distance_squared.is_normal() {
        let radius_squared = (lhs.radius() + rhs.radius()).powi(2);
        //Force is the only thing that changes from normal gravitation.
        let force = delta * magnitude * lhs.quanta() * rhs.quanta() / if distance_squared > radius_squared {
            distance_squared.sqrt().powi(3)
        } else {
            radius_squared
        };
        lhs.impulse(&force);
        rhs.impulse(&-force);
    }
}

///This is the same as the radius function, but the sum of the radii squared is passed separately to avoid overhead.
pub fn gravitate_radius_squared<V, D, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, radius_squared: D, magnitude: D)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D>, D: Float
{
    let delta = rhs.position() - lhs.position();
    let distance_squared = delta.displacement_squared();
    if distance_squared.is_normal() {
        //Force is the only thing that changes from normal gravitation.
        let force = delta * magnitude * lhs.quanta() * rhs.quanta() / if distance_squared > radius_squared {
            distance_squared.sqrt().powi(3)
        } else {
            radius_squared
        };
        lhs.impulse(&force);
        rhs.impulse(&-force);
    }
}

///Same as gravitate_radius_squared, but uses a special comp_delta closure to compute the delta between the particles
pub fn gravitate_radius_squared_delta<V, D, F, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, radius_squared: D,
    magnitude: D, comp_delta: F)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D>, D: Float, F: FnOnce(V, V) -> V
{
    //Create delta vector between the two positions.
    let delta = comp_delta(lhs.position(), rhs.position());
    let distance_squared = delta.displacement_squared();
    if distance_squared.is_normal() {
        //Force is the only thing that changes from normal gravitation.
        let force = delta * magnitude * lhs.quanta() * rhs.quanta() / if distance_squared > radius_squared {
            distance_squared.sqrt().powi(3)
        } else {
            radius_squared
        };
        lhs.impulse(&force);
        rhs.impulse(&-force);
    }
}

///Apply spring forces between two particles.
pub fn hooke<V, D, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D>, D: Float
{
    let delta = rhs.position() - lhs.position();
    let force = delta * magnitude * lhs.quanta() * rhs.quanta();
    lhs.impulse(&force);
    rhs.impulse(&-force);
}

///Same as hooke, but uses a comp_delta closure to compute the distance from the first to second parameter
pub fn hooke_delta<V, D, F, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D, comp_delta: F)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D>, D: Float, F: FnOnce(V, V) -> V
{
    //Create delta vector between the two positions.
    let delta = comp_delta(lhs.position(), rhs.position());
    let force = delta * magnitude * lhs.quanta() * rhs.quanta();
    lhs.impulse(&force);
    rhs.impulse(&-force);
}

#[test]
fn hooke_test() {
    type P = BasicParticle<Cartesian3<f64>, f64>;
    let mut a = P::default();
    let mut b = P::default();
    hooke(&a, &b, 1.0);
    a.advance(1.0);
    b.advance(1.0);
}

///Apply spring forces between two particles with specified equilibrium distance.
pub fn hooke_equilibrium<V, D, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, equilibrium: D, magnitude: D)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D>, D: Float
{
    let delta = rhs.position() - lhs.position();
    let displace = delta.displacement();
    if displace.is_normal() {
        let force = delta / displace * magnitude *
            //We scale such that if displacement is greater than equilibrium, the particles attract proportionally.
            //Particles with a displacement smaller than equilibrium repel each other.
            (displace - equilibrium) *
            lhs.quanta() * rhs.quanta();
        lhs.impulse(&force);
        rhs.impulse(&-force);
    }
}

///Same as hooke_equilibrium, but uses a comp_delta closure to compute the distance from the first to second parameter
pub fn hooke_equilibrium_delta<V, D, F, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, equilibrium: D, magnitude: D,
    comp_delta: F)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D>, D: Float, F: FnOnce(V, V) -> V
{
    //Create delta vector between the two positions.
    let delta = comp_delta(lhs.position(), rhs.position());
    let displace = delta.displacement();
    if displace.is_normal() {
        let force = delta / displace * magnitude *
            //We scale such that if displacement is greater than equilibrium, the particles attract proportionally.
            //Particles with a displacement smaller than equilibrium repel each other.
            (displace - equilibrium) *
            lhs.quanta() * rhs.quanta();
        lhs.impulse(&force);
        rhs.impulse(&-force);
    }
}

///Apply lorentz forces between two PhysicsParticle objects based on quanta, position, and velocity.
pub fn lorentz<V, D, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D> + CrossVector, D: Float
{
    let delta = rhs.position() - lhs.position();
    let distance = delta.displacement();
    if distance.is_normal() {
        let force = V::cross(&rhs.velocity(), &V::cross(&lhs.velocity(), &delta)) *
            lhs.quanta() * rhs.quanta() / distance.powi(3) * magnitude;

        lhs.impulse(&-force);
        rhs.impulse(&force);
    }
}

///Same as lorentz, but uses a comp_delta closure to compute the distance from the first to second parameter
pub fn lorentz_delta<V, D, F, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D, comp_delta: F)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D> + CrossVector, D: Float,
    F: FnOnce(V, V) -> V
{
    //Create delta vector between the two positions.
    let delta = comp_delta(lhs.position(), rhs.position());
    let distance = delta.displacement();
    if distance.is_normal() {
        let force = V::cross(&rhs.velocity(), &V::cross(&lhs.velocity(), &delta)) *
            lhs.quanta() * rhs.quanta() / distance.powi(3) * magnitude;

        lhs.impulse(&-force);
        rhs.impulse(&force);
    }
}

#[test]
fn lorentz_test() {
    type P = BasicParticle<Cartesian3<f64>, f64>;
    let mut a = P::default();
    let mut b = P::default();
    lorentz(&a, &b, 1.0);
    a.advance(1.0);
    b.advance(1.0);
}

///Apply lorentz forces between two PhysicsParticle + Ball objects assuming uniformly distributed quanta.
pub fn lorentz_radius<V, D, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D)
    where T1: PhysicsParticle<V, D> + Ball<D>, T2: PhysicsParticle<V, D> + Ball<D>, V: Vector<D> + CrossVector, D: Float
{
    let delta = rhs.position() - lhs.position();
    let distance_squared = delta.displacement_squared();
    if distance_squared.is_normal() {
        let radius_squared = (lhs.radius() + rhs.radius()).powi(2);
        let force = V::cross(&rhs.velocity(), &V::cross(&lhs.velocity(), &delta)) * magnitude *
            lhs.quanta() * rhs.quanta() /
            if distance_squared > radius_squared {
                distance_squared.sqrt().powi(3)
            } else {
                radius_squared
            };

        lhs.impulse(&-force);
        rhs.impulse(&force);
    }
}

///Same as lorentz_radius, but uses a comp_delta closure to compute the distance from the first to second parameter
pub fn lorentz_radius_delta<V, D, F, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, magnitude: D, comp_delta: F)
    where T1: PhysicsParticle<V, D> + Ball<D>, T2: PhysicsParticle<V, D> + Ball<D>, V: Vector<D> + CrossVector,
    D: Float, F: FnOnce(V, V) -> V
{
    //Create delta vector between the two positions.
    let delta = comp_delta(lhs.position(), rhs.position());
    let distance_squared = delta.displacement_squared();
    if distance_squared.is_normal() {
        let radius_squared = (lhs.radius() + rhs.radius()).powi(2);
        let force = V::cross(&rhs.velocity(), &V::cross(&lhs.velocity(), &delta)) * magnitude *
            lhs.quanta() * rhs.quanta() /
            if distance_squared > radius_squared {
                distance_squared.sqrt().powi(3)
            } else {
                radius_squared
            };

        lhs.impulse(&-force);
        rhs.impulse(&force);
    }
}

///Apply lorentz forces between two PhysicsParticle objects using a precomputed net radius.
pub fn lorentz_radius_squared<V, D, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, radius_squared: D, magnitude: D)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D> + CrossVector, D: Float
{
    let delta = rhs.position() - lhs.position();
    let distance_squared = delta.displacement_squared();
    if distance_squared.is_normal() {
        let force = V::cross(&rhs.velocity(), &V::cross(&lhs.velocity(), &delta)) * magnitude *
            lhs.quanta() * rhs.quanta() /
            if distance_squared > radius_squared {
                distance_squared.sqrt().powi(3)
            } else {
                radius_squared
            };

        lhs.impulse(&-force);
        rhs.impulse(&force);
    }
}

///Same as lorentz_radius_squared, but uses a comp_delta closure to compute the distance between the particles
pub fn lorentz_radius_squared_delta<V, D, F, T1: ?Sized, T2: ?Sized>(lhs: &T1, rhs: &T2, radius_squared: D,
    magnitude: D, comp_delta: F)
    where T1: PhysicsParticle<V, D>, T2: PhysicsParticle<V, D>, V: Vector<D> + CrossVector, D: Float,
    F: FnOnce(V, V) -> V
{
    //Create delta vector between the two positions.
    let delta = comp_delta(lhs.position(), rhs.position());
    let distance_squared = delta.displacement_squared();
    if distance_squared.is_normal() {
        let force = V::cross(&rhs.velocity(), &V::cross(&lhs.velocity(), &delta)) * magnitude *
            lhs.quanta() * rhs.quanta() /
            if distance_squared > radius_squared {
                distance_squared.sqrt().powi(3)
            } else {
                radius_squared
            };

        lhs.impulse(&-force);
        rhs.impulse(&force);
    }
}
