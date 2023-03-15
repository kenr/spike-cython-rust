use std::path::Path;
use pyo3::types::PyList;
use pyo3::prelude::*;

fn main() {
    ask_seasnake().unwrap()
}

fn ask_seasnake() -> PyResult<()> {
    //let path = Path::new("/Users/kenr/dev/github/kenr/spike-cython-rust/python/build/lib.macosx-13-arm64-cpython-311");
    let path = Path::new("./python/build/lib.macosx-13-arm64-cpython-311");
    Python::with_gil(|py| {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.extract()?;
        syspath.insert(0, &path)?;
        //println!("Import path is: {:?}", syspath);

        let seasnake = py.import("seasnake")?;
        let args = ();
        let seasnake_reply: String = seasnake.call_method("ask_seasnake", args, None)?.extract()?;

        println!("I just asked the SeaSnake, it replied {}", seasnake_reply);
        Ok(())
     })
}
