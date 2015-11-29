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
    fn position(&self) -> V;
    fn velocity(&self) -> V;
    fn accelerate(&mut self, vec: V);
}

pub trait PhysicsParticle<V, D>: Particle<V, D> + Quanta<D>
    where V: Vector<D>, D: Float
{
    //A function that must be implemented by a PhysicsParticle; it reveals the resistance to change in motion
    fn inertia(&self) -> D;

    //Apply proper attraction between two physics particles based on their quanta and position
    fn attract(&mut self, rhs: &mut Self) {
        //Create delta vector between the two positions
        let delta = rhs.position() - self.position();
        //Find the distance of the delta vector
        let distance = delta.displacement();
        //Dividing the delta vector by the distance cubed computes attractive force as per the inverse square law
        //Multiply by the two quanta of the objects to compute the final force
        let force = delta / distance.powi(3) * self.quanta() * rhs.quanta();

        //Accelerate self in the direction of force and divide it by the inertia
        let acceleration = force / self.inertia();
        self.accelerate(acceleration);
        //Apply the inverse force to the rhs
        let acceleration = -force / rhs.inertia();
        rhs.accelerate(acceleration);
    }

    //Apply proper attraction to a single physics particle towards a location and with a magnitude
    fn attract_to(&mut self, location: V, magnitude: D) {
        //Create delta vector from the particle to the center of attraction
        let delta = location - self.position();
        let distance = delta.displacement();
        let acceleration = delta / distance.powi(3) * magnitude * self.quanta() / self.inertia();
        self.accelerate(acceleration);
    }

    //Apply spring forces between two particles with specified equilibrium distance
    fn hooke(&mut self, rhs: &mut Self, equilibrium: D) {
        let delta = rhs.position() - self.position();
        let force = delta.normalized() * (delta.displacement() - equilibrium) * self.quanta() * rhs.quanta();
        let acceleration = force / self.inertia();
        self.accelerate(acceleration);
        let acceleration = -force / rhs.inertia();
        rhs.accelerate(acceleration);
    }

    //Apply spring forces between one particle and a position with an equilibrium and magnitude
    fn hooke_to(&mut self, location: V, magnitude: D, equilibrium: D) {
        let delta = location - self.position();
        let acceleration = delta.normalized() * (delta.displacement() - equilibrium) *
            magnitude * self.quanta() / self.inertia();
        self.accelerate(acceleration);
    }
    /*
    //Apply lorentz forces between two PhysicsParticle objects based on quanta, position, and velocity
    fn lorentz(&mut self, rhs: &mut Self) {
        let delta = rhs.position() - self.position();

    }
    //Apply lorentz forces on one object based on a field at the particle's location
    fn lorentz(&mut self, field: V);*/
}
