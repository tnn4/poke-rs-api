use std::collections::HashMap;

#[deprecated]
pub struct Id2Pkmn {
    map: HashMap<u64, &'static str>,
}

#[deprecated]
pub struct Poke2Id {
    map: HashMap<&'static str, u64>
}

#[deprecated]
impl Poke2Id {
    pub fn new() -> Self {
        let mut m = Poke2Id {
            map: HashMap::new(),
        };
        m
    }
}

#[deprecated]
pub fn create_mappings() -> Id2Pkmn {
    let mut m = Id2Pkmn{
        map: HashMap::new(), 
    }; 
    m.map.insert(1,"bulbasaur");
    m.map.insert(6,"charizard");
    m.map.insert(25,"pikachu");
    m
}

#[deprecated]
pub fn id2pkmn(m: Id2Pkmn, id: u64) -> &'static str{
    m.map[&id]    
}

#[derive(FromPrimitive)]
pub enum PkmnEnum {
    Bulbasaur = 1,
    Ivysaur = 2,
    Venusaur = 3,
    Charmander = 4,
    Charmeleon = 5,
    Charizard = 6,
    Squirtle = 7,
    Wartortle = 8,
    Blastoise = 9,
    Pikachu = 25,
}

// INIT
pub fn init_mapping(_endpoint:&str) -> /*HashMap<String,String>*/toml::Table {
    let file_path=format!("mappings/{}2id.toml",_endpoint);
    
    #[cfg(debug_assertions)]
    println!("mapping file: {}", &file_path);
    
    let mut toml_object: toml::Table = Default::default();
    assert!(std::path::Path::new(file_path.as_str()).exists());
    
    // open toml
    //let mut file=std::fs::File::open(file_path);
    let mut contents = String::new();
    // WARNING: the variable inside if let is a totally different variable
    if let contents2=std::fs::read_to_string(file_path){
        println!("READ: success");
        // println!("{}", contents.unwrap());
        contents = contents2.unwrap();
    } else {
        panic!("[ERROR]: can't read file");
    }
    //match std::fs::read_to_string(&mut toml) {
        //Ok(string) => string,
        //Err(err) => err,
    //};
    println!("{}", contents);
    // let toml_object : HashMap<String, String> = toml::from_str(&contents).unwrap();
    
    if let toml_object2 = contents.parse::<toml::Table>().unwrap() {
        println!("[OK]:POKEMAP initialized ");
        toml_object=toml_object2;
    } else {
        panic!("[ERROR]: could not read mapping");
    }
    
    
    #[cfg(debug_assertions)]
    {
        println!("toml_object: {:?}", toml_object); // -> {}
    }
    // assert_eq!(toml_object["bulbasaur"], "1".to_string());
    // assert_eq!(toml_object["bulbasaur"], toml::Value::String("1".to_string()));

    // read it
    // then add it to dictionary
    // let mut poke2id: HashMap<String,String> = HashMap::new();
    toml_object
}

pub fn test() {
    //assert_eq!(toml_object["bulbasaur"], toml::Value::Integer(1));
    //println!("toml[bulbasaur]: {}", toml_object["bulbasaur"]);
}