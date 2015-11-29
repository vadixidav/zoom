extern crate num;
use self::num::Float;
use super::vector::*;

pub trait Quanta<D>
    where D: Float
{
    //Retrieve the quanta of a physics particle
    fn quanta(&self) -> D;
}

pub trait Particle<V, D>
    where V: Vector<D>, D: Float
{
    //Get position of particle
    fn position(&self) -> V;
    //Get velocity of particle
    fn velocity(&self) -> V;
    //Accelerate particle
    fn accelerate(&mut self, vec: &V);
    //Advance particle (update position and velocity)
    fn advance(&mut self);
}

pub trait PhysicsParticle<V, D>: Particle<V, D> + Quanta<D>
    where V: Vector<D>, D: Float
{
    //A function that must be implemented by a PhysicsParticle; it reveals the resistance to change in motion
    fn inertia(&self) -> D;

    //Apply proper attraction between two physics particles based on their quanta and position
    fn gravitate(lhs: &mut Self, rhs: &mut Self) {
        //Create delta vector between the two positions
        let delta = rhs.position() - lhs.position();
        //Find the distance of the delta vector
        let distance = delta.displacement();
        //Dividing the delta vector by the distance cubed computes attractive force as per the inverse square law
        //Multiply by the two quanta of the objects to compute the final force
        let force = delta / distance.powi(3) * lhs.quanta() * rhs.quanta();

        //Accelerate lhs in the direction of force and divide it by the inertia
        let acceleration = force / lhs.inertia();
        lhs.accelerate(&acceleration);
        //Apply the inverse force to the rhs similarly
        let acceleration = -force / rhs.inertia();
        rhs.accelerate(&acceleration);
    }

    //Apply proper attraction to a single physics particle towards a location and with a magnitude
    fn gravitate_to(&mut self, location: V, magnitude: D) {
        //Create delta vector from the particle to the center of attraction
        let delta = location - self.position();
        let distance = delta.displacement();
        let acceleration = delta / distance.powi(3) * magnitude * self.quanta() / self.inertia();
        self.accelerate(&acceleration);
    }

    //Apply spring forces between two particles with specified equilibrium distance
    fn hooke(lhs: &mut Self, rhs: &mut Self, equilibrium: D) {
        let delta = rhs.position() - lhs.position();
        let force = delta.normalized() *
            //We scale such that if displacement is greater than equilibrium, the particles attract proportionally
            //Particles with a displacement smaller than equilibrium repel each other
            (delta.displacement() - equilibrium) *
            lhs.quanta() * rhs.quanta();
        let acceleration = force / lhs.inertia();
        lhs.accelerate(&acceleration);
        let acceleration = -force / rhs.inertia();
        rhs.accelerate(&acceleration);
    }

    //Apply spring forces between one particle and a position with an equilibrium and magnitude
    fn hooke_to(&mut self, location: V, magnitude: D, equilibrium: D) {
        let delta = location - self.position();
        let acceleration = delta.normalized() * (delta.displacement() - equilibrium) *
            magnitude * self.quanta() / self.inertia();
        self.accelerate(&acceleration);
    }

    //Apply lorentz forces between two PhysicsParticle objects based on quanta, position, and velocity
    fn lorentz(lhs: &mut Self, rhs: &mut Self)
        where V: CrossVector<D>
    {
        let delta = rhs.position() - lhs.position();
        let distance = delta.displacement();
        let force = rhs.velocity().cross(&lhs.velocity().cross(&delta)) * lhs.quanta() * rhs.quanta() / distance.powi(3);

        let acceleration = -force / lhs.inertia();
        lhs.accelerate(&acceleration);
        let acceleration = force / rhs.inertia();
        rhs.accelerate(&acceleration);
    }

    fn lorentz_field(&mut self, field: &V)
        where V: CrossVector<D>
    {
        let acceleration = self.velocity().cross(field) * self.quanta() / self.inertia();
        self.accelerate(&acceleration);
    }
}
