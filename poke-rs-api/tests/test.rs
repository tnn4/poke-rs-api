

// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test
// not when you run `cargo build`
#[cfg(test)] 
mod tests {
    use std::fs;
    use toml;
    use poke_rs_api::pkmn::mapper::create_mappings;

    #[test]
    fn id2pkmn(){
        let m = create_mappings();
        let pikachu_id = 25;
        let pikachu_str = "pikachu";
        assert_eq!(poke_rs_api::pkmn::mapper::id2pkmn(m, pikachu_id),pikachu_str)
    }
}