// Lens undistortion is the 1st stage in the pipeline. Immediately upon
// receiving a frame from the camera, we should correct for the radial
// and tangential distortion that lenses create, which cause straight
// lines to appear curved. This involves camera calibration to identify
// internal parameters, which is done on the field using ChArUcos boards.
//
// More information can be found at the following link:
// https://docs.opencv.org/4.x/dc/dbb/tutorial_py_calibration.html
#![allow(unused_imports)]
use crate::pipeline::{CameraId, Frame};
use opencv::calib3d::undistort;
use opencv::core::{Mat, no_array};
use opencv::prelude::MatTraitConst;

// The following are internal parameters that we determined via calibration
// for camera intrinsics and extrinsics. The input camera matrix has information
// regarding things like focal length, principal point, etc. The distortion
// coefficients vector stores information for tangential and radial distortion.

// Input camera matrix for the camera placed on the left from the perspective
// of the thrower.
const CAMERA_MATRIX_LEFT: [[f64; 3]; 3] = [
    [4618.073624590161, 0.0, 1792.046572080145],
    [0.0, 4601.788617766606, 1487.4743120273447],
    [0.0, 0.0, 1.0],
];

// Input camera matrix for the camera placed on the right from the perspective
// of the thrower.
const CAMERA_MATRIX_RIGHT: [[f64; 3]; 3] = [
    [4739.099696025262, 0.0, 2015.833682673624],
    [0.0, 4739.266305590452, 1500.0654129700354],
    [0.0, 0.0, 1.0],
];

// Distortion coefficients for the camera placed on the left from the perspective
// of the thrower.
const DISTORTION_COEFFICIENTS_LEFT: [f64; 5] = [
    0.019886048681431415,
    0.0786469243777663,
    -0.003660040167248905,
    -0.01748336536479488,
    -0.26127564045849816,
];

// Distortion coefficients for the camera placed on the right from the perspective
// of the thrower.
const DISTORTION_COEFFICIENTS_RIGHT: [f64; 5] = [
    0.23224923216439672,
    -3.19926754134018,
    0.0018953080034581444,
    -0.003837005584527228,
    16.293663810964677,
];

pub fn undistortion(frame: Frame, camera_id: CameraId) -> Frame {
    // First, convert the raw bytes of the frame into an input OpenCV
    // matrix type, and create the output matrix to write to.
    let (cols, rows): (u32, u32) = frame.raw_full_resolution();
    let input_mat =
        Mat::new_rows_cols_with_data(rows as i32, cols as i32, frame.raw_bytes_full_resolution())
            .expect("Failed to create input matrix during lens undistortion.");
    let mut output_mat: Mat = Mat::default();

    // Second, get the camera intrinsics/extrinsics information which is
    // dependent on which camera this is.
    let (camera_matrix, distortion_coefficients) = match camera_id {
        CameraId::FieldLeft => (
            Mat::from_slice_2d(&CAMERA_MATRIX_LEFT).expect("Failed to get left camera matrix."),
            Mat::from_slice(&DISTORTION_COEFFICIENTS_LEFT)
                .expect("Failed to get left distortion coefficients."),
        ),
        CameraId::FieldRight => (
            Mat::from_slice_2d(&CAMERA_MATRIX_RIGHT).expect("Failed to get right camera matrix."),
            Mat::from_slice(&DISTORTION_COEFFICIENTS_RIGHT)
                .expect("Failed to get right distortion coefficients."),
        ),
    };

    // Then, perform the undistortion using OpenCV bindings.
    if let Err(err) = undistort(
        &input_mat,
        &mut output_mat,
        &camera_matrix,
        &distortion_coefficients,
        &no_array(),
    ) {
        eprintln!("Error: Failed to undistort in undistort(). Returning original frame. {err}");
        return frame;
    }

    // Finally, set the undisorted image to the result and return the frame.
    frame
        .set_undistorted_image(output_mat)
        .expect("Error: Failed to set undistorted Mat.");
    frame
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        camera::AtlasATP124SResolution,
        pipeline::test_utils::{ComputerVisionStage, generate_frame},
    };
    use rstest::rstest;

    #[rstest]
    #[case(AtlasATP124SResolution::Quarter)]
    #[case(AtlasATP124SResolution::Half)]
    #[case(AtlasATP124SResolution::Full)]
    fn test_undistortion_acts_on_frame(#[case] resolution: AtlasATP124SResolution) {
        let input_frame: Frame =
            generate_frame(200, 4372, resolution, ComputerVisionStage::Undistortion);
        let output_frame: Frame = undistortion(input_frame, CameraId::FieldLeft);

        // Check that output exists and that its dimensions match input dimensions.
        let undistorted_mat: &Mat = output_frame.undistorted_image().unwrap();
        assert_eq!(
            undistorted_mat.rows(),
            output_frame.raw_full_resolution().1 as i32
        );
        assert_eq!(
            undistorted_mat.cols(),
            output_frame.raw_full_resolution().0 as i32
        );
    }
}
