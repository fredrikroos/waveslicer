mod waveform;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
struct WaveForm {
    #[pyo3(get)]
    data: Vec<i32>,

    #[pyo3(get)]
    length: i32,
}

impl From<waveform::Error> for PyErr {
    fn from(_err: waveform::Error) -> PyErr {
        unimplemented!()
    }
}

#[pyfunction]
fn generate_waveform(filename: String, width: u32) -> PyResult<WaveForm> {
    let slices = waveform::generate(&filename, width)?;

    let wf = WaveForm {
        data: slices.data,
        length: slices.length.as_secs() as i32,
    };

    Ok(wf)
}

#[pymodule]
fn waveslicer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(generate_waveform))?;

    Ok(())
}
