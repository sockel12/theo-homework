/**
 * A simple Hashmap that has a Buffer of 1009 Elements
 * Inserting a key which would resolve to a hash conflict will override the existing value.
 */
#[allow(dead_code)]
pub struct SimpleHashMap {
    table: [(u32, u32); 1009]
}
#[allow(dead_code)]
impl SimpleHashMap {
    pub fn new() -> SimpleHashMap {
        SimpleHashMap {
            table: [(0, 0); 1009]
        }
    }

    /**
     * Insert a key, value pair into the Hashmap, if the given key resolves to a hash conflict, the function returns true and overrides the key and value
     */
    pub fn insert(&mut self, key: u32, value: u32) -> bool {
        let index: u32 = key % 1009;
        if self.table[index as usize].1 != 0 {
            // Conflict
            self.table[index as usize] = (key, value);
            return true;
        }        
        self.table[index as usize] = (key, value);
        false
    }

    pub fn get(&mut self, key: u32) -> u32 {
        let index: u32 = key % 1009;
        let result: (u32, u32) = self.table[index as usize];
        return result.1;
    }

    pub fn clear(&mut self) {
        self.table = [(0, 0); 1009];
    }

    pub fn fill(&mut self, percentage: f32) {
        let table_size: u32 = self.table.len() as u32;
        let max_index: u32 = (percentage * table_size as f32) as u32;
        for i in 0..max_index {
            self.table[i as usize] = (i, 1);
        }
    }


}

#[allow(dead_code)]
pub struct AdvancedHashMap<T> {
    table: Vec<Vec<(u32, T)>>,
    table_size: u32
} 

#[allow(dead_code)]
impl<T> AdvancedHashMap<T> where T: Copy{

    pub fn new(table_size: u32) -> AdvancedHashMap<T> {
        let mut hashmap: AdvancedHashMap<T> = AdvancedHashMap { table: Vec::new(), table_size: table_size };

        hashmap.table = vec![Vec::new(); table_size as usize];
        // for _ in 0..table_size {
        //     hashmap.table.push(Vec::new());
        // }

        return hashmap;
    }
    
    /** 
     * Returns true if a conflict was found. Overrides only the value at given key
     */
    pub fn insert(&mut self, key: u32, value: T) -> bool{
        let index: usize = (key % self.table_size) as usize; 
        
        for i in 0..self.table[index].len() {
            if self.table[index][i].0 == key {
                self.table[index][i] = (key, value);
                return true;
            }
        }
        let collsion_happend: bool = self.table[index].len() > 0;        
        self.table[index].push((key, value));
        collsion_happend
    }

    pub fn get(& self, key: u32) -> T {
        let index: usize = (key % self.table_size) as usize;

        for i in 0..self.table[index].len() {
            if self.table[index][i].0 == key {
                return self.table[index][i].1;
            }
        }

        panic!("Key not found in Hashmap");
    }

    pub fn exists(& self, key: u32) -> bool {

        let index: usize = (key % self.table_size) as usize;

        for i in 0..self.table[index].len() {
            if self.table[index][i].0 == key {
                return true
            }
        }

        false
    }

    pub fn clear(&mut self) {
        self.table = Vec::new();

        for _ in 0..self.table_size {
            self.table.push(Vec::new());
        }
    }
}

pub fn get_filled_hashmap(hashmap_size: u32, mut percentage: f32) -> AdvancedHashMap<u32> {
    if percentage < 0.0{
        percentage = 0.00001;
    }
    else if percentage > 1.0 {
        percentage = 1.0
    }

    let mut hashmap: AdvancedHashMap<u32> = AdvancedHashMap::new(hashmap_size);
    let max_index: u32 = (percentage * hashmap_size as f32) as u32;
    
    for i in 0..max_index {
        hashmap.insert(i, 991);
    }

    return hashmap;
}   

#[test]
fn test_simple_hashmap() {
    let mut hashmap: SimpleHashMap = SimpleHashMap::new();
    
    assert_eq!(hashmap.get(0), 0);
    assert_eq!(hashmap.insert(0, 100), false);
    assert_eq!(hashmap.get(0), 100);
    assert_eq!(hashmap.insert(0, 101), true);
    assert_eq!(hashmap.get(0), 101);
    assert_eq!(hashmap.insert(1009, 1002), true);
    hashmap.clear();
    assert_eq!(hashmap.get(0), 0);
    assert_eq!(hashmap.insert(1009, 100), false);
    assert_eq!(hashmap.get(0), 100);
    assert_eq!(hashmap.insert(0, 0), true);
    assert_eq!(hashmap.get(0), 0);
}

#[test]
fn test_advanced_hashmap_u32() {
    let mut hashmap: AdvancedHashMap<u32> = AdvancedHashMap::new(1009);
    
    assert_eq!(hashmap.exists(0), false);
    assert_eq!(hashmap.insert(0, 1001), false);
    assert_eq!(hashmap.exists(0), true);
    assert_eq!(hashmap.get(0), 1001);
    assert_eq!(hashmap.insert(1009, 100), true);
    assert_eq!(hashmap.get(0), 1001);
    assert_eq!(hashmap.get(1009), 100);
    assert_eq!(hashmap.insert(1, 100), false);
    assert_eq!(hashmap.get(1), 100);
    assert_eq!(hashmap.insert(1, 50), true);
    assert_eq!(hashmap.get(1), 50);
    assert_eq!(hashmap.exists(1), true);
    assert_eq!(hashmap.exists(1010), false);
    hashmap.clear();
    assert_eq!(hashmap.exists(0), false);
    assert_eq!(hashmap.exists(1009), false);
    assert_eq!(hashmap.insert(0, 100), false);
    assert_eq!(hashmap.insert(1009, 200), true);
    assert_eq!(hashmap.get(0), 100);
    assert_eq!(hashmap.get(1009), 200);
}

#[test]
fn test_advanced_hashmap_string() {
    let mut hashmap: AdvancedHashMap<&str> = AdvancedHashMap::new(1009);

    assert_eq!(hashmap.exists(0), false);
    assert_eq!(hashmap.insert(0, &"Hello :D"), false);
    assert_eq!(hashmap.exists(0), true);
    assert_eq!(hashmap.get(0), "Hello :D");
    assert_eq!(hashmap.insert(1009, &"Bye :("), true);
    assert_eq!(hashmap.get(1009), "Bye :(");
    assert_eq!(hashmap.insert(0, &"Hello?"), true);
    assert_eq!(hashmap.get(0), "Hello?");
    hashmap.clear();
    assert_eq!(hashmap.exists(0), false);
    assert_eq!(hashmap.exists(1009), false);
    assert_eq!(hashmap.insert(0, &"Hey :)"), false);
    assert_eq!(hashmap.insert(0, &"Bye :("), true);
    assert_eq!(hashmap.get(0), "Bye :(");
    
}



