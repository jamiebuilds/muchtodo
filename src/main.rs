use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::Walk;
use std::path::PathBuf;

struct Todo {
    path: PathBuf,
    line: i32,
    column: i32,
}

fn walk(dir: &str, globset: GlobSet, walker: impl Fn(PathBuf) -> Vec<Todo>) -> Vec<Todo> {
    let mut todos = vec![];

    for result in Walk::new(dir) {
        let entry = result.unwrap();
        let path = entry.path().to_path_buf();
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() && globset.is_match(path.to_str().unwrap()) {
            for todo in walker(path) {
                todos.push(todo);
            }
        }
    }

    todos
}

fn walker(path: PathBuf) -> Vec<Todo> {
    let todo = Todo {
        path,
        line: 0,
        column: 0,
    };
    let todos = vec![todo];
    todos
}

fn to_globset(globs: Vec<&str>) -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for glob in globs {
        builder.add(Glob::new(glob).unwrap());
    }
    builder.build().unwrap()
}

fn main() {
    let entry = "./";
    let globs = to_globset(vec!["*"]);
    let todos = walk(entry, globs, walker);

    for todo in todos {
        println!("{}:{}:{}", todo.path.display(), todo.line, todo.column);
    }
}
