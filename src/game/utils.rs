use numpy::ndarray::Array1;

pub fn indices_to_bitmap(indices: Vec<usize>, total_size: usize) -> Vec<u8> {
    let mut bitmap = vec![0; total_size];
    for idx in indices {
        bitmap[idx] = 1;
    }
    bitmap
}

pub fn indices_to_bitmap_as_array1(indices: Vec<usize>, total_size: usize) -> Array1<u8> {
    let mut bitmap = Array1::zeros(total_size);
    for idx in indices {
        bitmap[idx] = 1;
    }
    bitmap
}
