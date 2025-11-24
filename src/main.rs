mod handlers;

use handlers::image_shrink::handle_image_shrink;
use handlers::input_params::get_input_params;

fn main() {
    handle_image_shrink(get_input_params());
}

