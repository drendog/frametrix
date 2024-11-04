use crate::device::{LED_MATRIX_HEIGHT, LED_MATRIX_WIDTH};
use slint::{Model, ModelRc, VecModel};

pub fn create_initial_matrix() -> Vec<Vec<bool>> {
    vec![vec![false; LED_MATRIX_WIDTH]; LED_MATRIX_HEIGHT]
}

pub fn create_matrix_model(matrix: &[Vec<bool>]) -> ModelRc<ModelRc<bool>> {
    ModelRc::new(VecModel::from(
        matrix
            .iter()
            .map(|row| ModelRc::new(VecModel::from(row.clone())))
            .collect::<Vec<_>>(),
    ))
}

pub fn convert_model_to_array(matrix_model: &ModelRc<ModelRc<bool>>) -> [[u8; 34]; 9] {
    let mut matrix_array = [[0; 34]; 9];
    for (x, row) in matrix_model.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            matrix_array[y][x] = if value { 0xFF } else { 0x00 };
        }
    }
    matrix_array
}
