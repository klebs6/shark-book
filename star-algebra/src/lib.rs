pub trait Vector<T>: Sized {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn scale(&self, scalar: T) -> Self;
}

pub trait LinearOperator<T, V: Vector<T>> {
    fn apply(&self, v: &V) -> V;
}

pub trait InnerProduct<T, V: Vector<T>> {
    fn inner_product(&self, v1: &V, v2: &V) -> T;
}

pub trait Norm<T, V: Vector<T>> {

    fn norm(&self, v: &V) -> T;
}

pub trait HilbertSpace<T, V: Vector<T>>: 
InnerProduct<T,V> + Norm<T,V> 
{
    fn add(&self, v1: &V, v2: &V) -> V;
    fn scale(&self, scalar: T, v: &V) -> V;
}

pub trait StarAlgebra<T, V: Vector<T>> {
    // The * operation
    fn star(&self) -> Self;
}

pub trait LinearStarAlgebra<T, V: Vector<T>>: LinearOperator<T, V> + StarAlgebra<T, V> {}


pub fn check_cstar_identity<T: StarAlgebra<T,V>, V: Vector<T>, H: HilbertSpace<T,V>>(op: &T, v: &V, hs: &H) -> bool
{
    let left = hs.norm(&op.star().apply(v));
    let right = hs.norm(v).powi(2);
    (left - right).abs() < std::f64::EPSILON
}

