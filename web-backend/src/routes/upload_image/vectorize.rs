use opencv::core::ToInputArray;
use opencv::{
    Result,
    core::{Mat, Point, Size, Vector},
    imgcodecs, imgproc,
    prelude::*,
    ximgproc,
};

#[cfg(feature = "server")]
use opencv::geometry::approx_poly_dp;
#[cfg(feature = "server")]
use opencv::geometry::arc_length;
#[cfg(not(feature = "server"))]
use opencv::imgproc::approx_poly_dp;
#[cfg(not(feature = "server"))]
use opencv::imgproc::arc_length;

// Controls the strength of the image blurring, which mostly reduces the level of detail the robot
// will try to draw. Must be an odd number. Values above 9 start to drop important details.
const BLUR_STRENGTH: i32 = 9;

// Controls the aggressiveness of the line smoothing: higher = smoother/fewer points,
// lower = more detailed. Values above around 0.0002 to 0.001 start to produce artifacts in the
// form of random straight lines crossing the image.
const SMOOTHING_EPSILON: f64 = 0.0002;

// The minimum arc perimeter in pixels. Arcs smaller than this will be discarded.
const MIN_ARC_PERIMETER: f64 = 15.0;

#[allow(clippy::missing_errors_doc)]
pub fn vectorize_image(image_bytes: &impl ToInputArray) -> Result<String> {
    let img = imgcodecs::imdecode(image_bytes, imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        return Err(opencv::Error::new(
            opencv::core::StsError,
            "Could not decode image".to_string(),
        ));
    }

    // Convert to grayscale.
    let mut gray = Mat::default();
    #[cfg(feature = "server")]
    imgproc::cvt_color(
        &img,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::default(),
    )?;
    #[cfg(not(feature = "server"))]
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let size = gray.size()?;
    let width = size.width;
    let height = size.height;

    let mut blurred = Mat::default();
    #[cfg(feature = "server")]
    imgproc::gaussian_blur(
        &gray,
        &mut blurred,
        Size::new(BLUR_STRENGTH, BLUR_STRENGTH),
        0.0,
        0.0,
        opencv::core::BORDER_DEFAULT,
        opencv::core::AlgorithmHint::default(),
    )?;
    #[cfg(not(feature = "server"))]
    imgproc::gaussian_blur(
        &gray,
        &mut blurred,
        Size::new(BLUR_STRENGTH, BLUR_STRENGTH),
        0.0,
        0.0,
        opencv::core::BORDER_DEFAULT,
    )?;

    let edges = &extract_centerline_contours(&blurred)?;

    let mut raw_contours = Vector::<Vector<Point>>::new();
    imgproc::find_contours(
        &edges,
        &mut raw_contours,
        imgproc::RETR_LIST,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )?;

    let mut smoothed_contours = Vector::<Vector<Point>>::new();

    for contour in raw_contours {
        // Filter out tiny noise contours based on arc length (perimeter).
        let perimeter = arc_length(&contour, true)?;
        if perimeter < MIN_ARC_PERIMETER {
            continue;
        }

        // Apply Douglas-Peucker approximation to smooth the curve.
        let epsilon = SMOOTHING_EPSILON * perimeter;
        let mut approx = Vector::<Point>::new();
        approx_poly_dp(&contour, &mut approx, epsilon, true)?;

        if !approx.is_empty() {
            smoothed_contours.push(approx);
        }
    }

    let optimized_paths = optimize_travel_path(
        &smoothed_contours
            .iter()
            .map(|c| c.iter().collect())
            .collect::<Vec<Vec<Point>>>(),
        Point::new(0, 0),
    );

    Ok(convert_to_command_string(&optimized_paths, width, height))
}

fn extract_centerline_contours(gray: &Mat) -> Result<Mat> {
    let mut binary = Mat::default();
    imgproc::adaptive_threshold(
        &gray,
        &mut binary,
        255.0,
        imgproc::ADAPTIVE_THRESH_GAUSSIAN_C,
        imgproc::THRESH_BINARY_INV,
        11,  // Block size (must be odd, e.g., 11, 15, 19)
        2.0, // Constant subtracted from the mean
    )?;

    let mut skeleton = Mat::default();
    ximgproc::thinning(&binary, &mut skeleton, ximgproc::THINNING_ZHANGSUEN)?;

    Ok(skeleton)
}

/// Optimizes contour order to minimize non-drawing travel time.
fn optimize_travel_path(contours: &[Vec<Point>], home_pos: Point) -> Vec<Vec<Point>> {
    if contours.is_empty() {
        return Vec::new();
    }

    let mut unvisited: Vec<Vec<Point>> = contours.to_vec();
    let mut ordered = Vec::with_capacity(contours.len());
    let mut current_pos = home_pos;

    while !unvisited.is_empty() {
        let mut best_idx = 0;
        let mut min_dist = f64::MAX;
        let mut reverse_best = false;

        // Find the closest unvisited contour (checking both start and end points).
        for (i, contour) in unvisited.iter().enumerate() {
            if contour.is_empty() {
                continue;
            }
            let start_pt = contour[0];
            let end_pt = *contour
                .last()
                .expect("Empty contours should be caught before this line");

            let dist_to_start = distance_squared(current_pos, start_pt);
            let dist_to_end = distance_squared(current_pos, end_pt);

            if dist_to_start < min_dist {
                min_dist = dist_to_start;
                best_idx = i;
                reverse_best = false;
            }
            if dist_to_end < min_dist {
                min_dist = dist_to_end;
                best_idx = i;
                reverse_best = true;
            }
        }

        let mut chosen_contour = unvisited.swap_remove(best_idx);

        if reverse_best {
            chosen_contour.reverse();
        }

        current_pos = *chosen_contour
            .last()
            .expect("Empty contours should not be chosen");
        ordered.push(chosen_contour);
    }

    ordered
}

// Helper to calculate Euclidean distance squared between two points
fn distance_squared(p1: Point, p2: Point) -> f64 {
    let dx = f64::from(p1.x - p2.x);
    let dy = f64::from(p1.y - p2.y);
    dx * dx + dy * dy
}

/// Converts the vectorized image to a command string for the robot.
/// Format: `W<width>;H<height>;<commands>`
/// Commands are semicolon-separated, with two possible formats:
/// - M<x>,<y>: Move to the specified point without drawing.
/// - L<x>,<y>: Draw a straight line to the specified point.
fn convert_to_command_string(paths: &[Vec<Point>], width: i32, height: i32) -> String {
    let mut command_string = format!("W{width};H{height};");

    for path in paths {
        for (idx, point) in path.iter().enumerate() {
            if idx == 0 {
                command_string.push_str(format!("M{},{};", point.x, point.y).as_str());
            } else {
                command_string.push_str(format!("L{},{};", point.x, point.y).as_str());
            }
        }
    }

    command_string
}
