use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::Walk;
use std::path::Path;

struct Todo<'a, 'b> {
    path: &'a Path,
    line: i32,
    column: i32,
    contents: &'b str,
}

fn walk(dir: &str, globset: GlobSet, walker: impl Fn(&Path) -> Vec<Todo>) -> Vec<Todo> {
    let mut todos = vec![];

    for result in Walk::new(dir) {
        let entry = result.unwrap();
        let path = entry.path();
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() && globset.is_match(path.to_str().unwrap()) {
            for todo in walker(path) {
                todos.push(todo);
            }
        }
    }

    todos
}

fn walker(path: &Path) -> Vec<Todo> {
    let todo = Todo {
        path,
        line: 42,
        column: 3,
        contents: "",
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
        println!("{}", todo.path.display());
    }
}
