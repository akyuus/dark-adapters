use bevy::prelude::App;

pub trait Registerable {
    fn register<T>(&mut self) -> &mut Self
    where
        T: GameMode;
}

pub trait GameMode {
    //      init needs to do the following:
    //      1. add necessary asset collections to a loading state
    //      2. initialize resources relating to those asset collections
    //      3. define any systems for entering, exiting, and updating this mode
    fn init(app: &mut App);
}

impl Registerable for App {
    fn register<T>(&mut self) -> &mut App
    where
        T: GameMode,
    {
        T::init(self);
        self
    }
}
