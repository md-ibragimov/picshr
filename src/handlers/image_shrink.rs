use caesium::parameters::CSParameters;
use caesium::compress;

pub fn handle_image_shrink((input_file_path, output_file_path, quality): (String, String, u32)) {
    let mut parameters = CSParameters::new();
    parameters.keep_metadata = true;
    parameters.jpeg.quality = quality;

    let result = compress(input_file_path, output_file_path, &parameters);
    assert!(result.is_ok());
}