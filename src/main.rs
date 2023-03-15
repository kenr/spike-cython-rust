use cpython::{Python, PyResult};

fn main() {
    let gil = Python::acquire_gil();

    ask_seasnake(gil.python()).unwrap();
}

fn ask_seasnake(py: Python) -> PyResult<()> {
    // Hack to add the path for SeaSnake
    //py.run("import sys", None, None)?;
    //py.run("sys.path.insert(0, \"build\")", None, None)?;

    let sys = py.import("sys");
    let path = sys?.call(py, "path", cpython::NoArgs, None)?;

    let seasnake = py.import("seasnake")?;
    let seasnake_reply: String = seasnake.get(py, "ask_seasnake")?.extract(py)?;

    println!("I just asked the SeaSnake, it said {}", seasnake_reply);
    Ok(())
}
