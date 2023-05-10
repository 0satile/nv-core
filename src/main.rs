mod window;

fn main() {
    window::create_instance(
        "window_title", 
        "http://localhost:5173", 
        1024, 600
    );
}