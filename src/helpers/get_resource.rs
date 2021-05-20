use bbecs::get_resource;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;

pub fn get_string(world: &World, name: &str) -> String {
    let resource: &String;
    get_resource!(resource, world, name);
    resource.to_owned()
}

pub fn get_f32(world: &World, name: &str) -> f32 {
    let resource: &f32;
    get_resource!(resource, world, name);
    *resource
}
