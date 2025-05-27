#![allow(unused)]

use pyo3::{exceptions::PyValueError, types::PyList};

fn main() {
    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn pointwise_sum(mut a: Vec<f64>, b: Vec<f64>) -> PyResult<Vec<f64>> {
        if a.len() != b.len() {
            return Err(PyValueError::new_err(
                "Input lists must have the same length",
            ));
        }

        // If 'a' is empty, 'b' must also be empty (due to the length check).
        // In this case, an empty vector is the correct result.
        if a.is_empty() {
            return Ok(Vec::new());
        }

        unsafe { pointwise_sum_simd(&mut a, &b) }

        Ok(a)
    }

    #[pymodule]
    fn pointwise_simd(py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
        m.add_function(wrap_pyfunction!(pointwise_sum, m)?)?;

        Ok(())
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn pointwise_sum_simd(a: &mut [f64], b: &[f64]) {
    use std::arch::x86_64::*;

    const WIDTH: usize = 2;

    let length = a.len();
    debug_assert_eq!(a.len(), b.len());

    let mut index = 0;

    while index + WIDTH <= length {
        // Get raw pointers to the current elements in slices 'a' and 'b'
        let ptr_a = a.as_mut_ptr().add(index); // *mut f64
        let ptr_b = b.as_ptr().add(index); // *const f64

        // Load 2 f64s from 'a' and 'b' into SIMD registers.
        // _mm_loadu_pd is used for potentially unaligned memory access.
        let simd_val_a = _mm_loadu_pd(ptr_a); // Loads [a[index], a[index+1]]
        let simd_val_b = _mm_loadu_pd(ptr_b); // Loads [b[index], b[index+1]]

        // Perform pointwise addition in SIMD register.
        let simd_sum = _mm_add_pd(simd_val_a, simd_val_b);

        // Store the 2 f64 results from SIMD register back into slice 'a'.
        // _mm_storeu_pd is used for potentially unaligned memory access.
        _mm_storeu_pd(ptr_a, simd_sum); // Stores sum into [a[index], a[index+1]]

        index += WIDTH;
    }

    while index < length {
        a[index] += b[index];

        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_pointwise_sum_simd() {
        let mut a: Vec<_> = (1..10).map(|i| i as f64).collect();
        let b: Vec<_> = (11..20).map(|i| i as f64).collect();

        unsafe { pointwise_sum_simd(&mut a, &b) };

        let expected: Vec<_> = (1..10).zip(11..20).map(|(x, y)| (x + y) as f64).collect();

        assert_eq!(a, expected);
    }
}
