#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Todo {
    title: String,
}

#[allow(dead_code)]
enum UpdateType {
    TITLE,
    PRIORITY,
}

#[derive(Debug)]
struct DB {
    priority_set: u16,
    todo: Todo,
}

impl DB {
    /// Create a new Todo DataBase
    fn new(db: &mut Vec<DB>) -> Self {
        // Make the todo db empty/clear when it is created
        db.clear();
        Self {
            priority_set: 0,
            todo: Todo {
                title: "".to_string(),
            },
        }
    }

    /// Add Todo and index count/priority_set
    fn add(&self, todo: &Todo, db: &mut Vec<DB>, index_count: &mut u16) {
        db.push(DB {
            priority_set: *index_count,
            todo: todo.clone(),
        });
        // Increment the index count by 1 as a key for the HashMap
        *index_count += 1;
    }

    /// Doing binray search on string data type
    fn binary_search(&self, search_area: &mut Vec<DB>, target: &String) -> Option<usize> {
        let mut left = 0;
        let mut right = search_area.len();

        while left <= right {
            let mid = left + (right - left) / 2;
            match search_area[mid].todo.title.as_str().cmp(target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid,
            }
        }
        None
    }

    /// Update function either do title change in the todo or change priority in the DB
    fn update(
        &self,
        db: &mut Vec<DB>,
        which_update: UpdateType,
        target_title: &String,
        new_title: &String,
        new_priority_set: u16,
    ) {
        match which_update {
            UpdateType::TITLE => {
                let search_index = self.binary_search(db, &target_title).unwrap();
                db[search_index].todo.title = new_title.to_string();
            }
            UpdateType::PRIORITY => {
                let search_index = self.binary_search(db, &target_title).unwrap();

                let convert_usize: usize = new_priority_set.into();
                let convert_u16: u16 = search_index.try_into().unwrap();
                db[convert_usize].priority_set = convert_u16;
                db[search_index].priority_set = new_priority_set;

                db.swap(new_priority_set.into(), search_index);
            }
        }
    }

    /// Remove function just remove todo from the DB
    fn remove(&self, db: &mut Vec<DB>, todo: &Todo) {
        let search_index = self.binary_search(db, &todo.title).unwrap();
        db.remove(search_index);
    }
}

fn main() {
    // All the code below are test as to check if the program works or not

    let mut db: Vec<DB> = Vec::new();

    let t = DB::new(&mut db);
    let mut index_count: u16 = 0;

    let t1 = Todo {
        title: "Eat Dinner".to_string(),
    };

    let t2 = Todo {
        title: "Play Game".to_string(),
    };

    t.add(&t1, &mut db, &mut index_count);
    t.add(&t2, &mut db, &mut index_count);

    db.sort_by(|a, b| a.todo.title.cmp(&b.todo.title));

    for todo in &db {
        println!("Priority: {}", todo.priority_set);
        println!("Title: {}\n", todo.todo.title);
    }

    t.update(
        &mut db,
        UpdateType::PRIORITY,
        &"Eat Dinner".to_string(),
        &"Eat Lunch".to_string(),
        1,
    );

    println!("<-Updated DB->");
    for todo in &db {
        println!("Priority: {}", todo.priority_set);
        println!("Title: {}\n", todo.todo.title);
    }

    t.remove(&mut db, &t1);

    println!("<-Updated DB->");
    for todo in &db {
        println!("Priority: {}", todo.priority_set);
        println!("Title: {}\n", todo.todo.title);
    }
}
