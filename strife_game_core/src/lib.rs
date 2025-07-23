pub mod ecs;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pos = ecs::components::Position{x: 0.0, y: 0.0};
        println!("{}", pos);
    }
}
