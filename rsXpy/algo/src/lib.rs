/*
Visualization of the recursion
merge([9, 3, 1, 4])
├─ merge([9, 3])
│  ├─ merge([9]) → [9]
│  ├─ merge([3]) → [3]
│  └─ merge_sorted([9], [3]) → [3, 9]
├─ merge([1, 4])
│  ├─ merge([1]) → [1]
│  ├─ merge([4]) → [4]
│  └─ merge_sorted([1], [4]) → [1, 4]
└─ merge_sorted([3, 9], [1, 4]) → [1, 3, 4, 9]
*/
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
#[pyo3(name = "merge_sort_verbose")]
fn merge_py(arr: Vec<i32>) -> Vec<i32> {
    let (_result, _log) = merge(&arr, 0, true);
    _result
}

fn merge(arr: &[i32], depth: usize, is_last: bool) -> (Vec<i32>, String) {
    let prefix = format!(
        "{}{} ",
        "│  ".repeat(depth.saturating_sub(1)),
        if depth > 0 {
            if is_last { "└─" } else { "├─" }
        } else {
            ""
        }
    );

    println!("{}merge({:?})", prefix, arr);

    if arr.len() <= 1 {
        return (arr.to_vec(), format!("{}→ {:?}\n", prefix, arr));
    }

    let mid = arr.len() / 2;

    let (left_sorted, _) = merge(&arr[..mid], depth + 1, false);
    let (right_sorted, _) = merge(&arr[mid..], depth + 1, true);

    let merged = merge_sorted(&left_sorted, &right_sorted);

    let merge_prefix = format!(
        "{}{} ",
        "│  ".repeat(depth.saturating_sub(1)),
        if depth > 0 {
            if is_last { "└─" } else { "├─" }
        } else {
            ""
        }
    );

    println!(
        "{}merge_sorted({:?}, {:?}) → {:?}",
        merge_prefix, left_sorted, right_sorted, merged
    );

    (merged, String::new())
}

fn merge_sorted(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    result
}

#[pyfunction]
#[pyo3(name = "quick_sort_verbose")]
fn quick_py(arr: Vec<i32>) -> PyResult<(Vec<i32>, String)> {
    let mut vec = arr;
    let mut log = String::new();

    if !vec.is_empty() {
        let len = vec.len() - 1;
        quick_sort(&mut vec, 0, len, &mut log);
    }

    Ok((vec, log))
}

fn quick_sort(arr: &mut [i32], low: usize, high: usize, log: &mut String) {
    if low < high {
        let pivot = partition(arr, low, high);
        log.push_str(&format!(
            "quick({:?}, {}, {})\n",
            &arr[low..=high],
            low,
            high
        ));

        if pivot > 0 {
            quick_sort(arr, low, pivot - 1, log);
        }
        quick_sort(arr, pivot + 1, high, log);
    }
}

fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;

    for j in left..right {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}




#[pymodule]
fn algo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge_py, m)?)?;
    m.add_function(wrap_pyfunction!(quick_py, m)?)?;
    Ok(())
}
