use bevy::prelude::App;

pub trait RegisterTarget {
    fn register<T>(&mut self) -> &mut Self
    where
        T: Registerable;
}

pub trait Registerable {
    //      init needs to do the following:
    //      1. add necessary asset collections to a loading state
    //      2. initialize resources relating to those asset collections
    //      3. add any necessary systems
    // TODO: I'M STUPID JUST MOVE EVERYTHING TO PLUGINS (ADAPTERS-35)
    fn init(app: &mut App);
}

impl RegisterTarget for App {
    fn register<T>(&mut self) -> &mut App
    where
        T: Registerable,
    {
        T::init(self);
        self
    }
}
