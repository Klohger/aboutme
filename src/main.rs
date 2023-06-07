use three_d::core::{
    degrees, radians, vec3, Camera, ClearState, Color, Context, Mat4, Program, RenderStates,
    VertexBuffer,
};
use three_d::window::{FrameOutput, Window, WindowSettings};
use three_d::ElementBuffer;

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings::default()).unwrap();

    // Get the graphics context from the window
    let context: Context = window.gl();

    // Define triangle vertices and colors
    let positions = VertexBuffer::new_with_data(
        &context,
        &[
            vec3(-0.5, 0.5, 0.0),  // top left
            vec3(-0.5, -0.5, 0.0), // bottom left
            vec3(0.5, -0.5, 0.0),  // bottom right
            vec3(0.5, 0.5, 0.0),   // top right
        ],
    );
    let indices = ElementBuffer::new_with_data(&context, &[0u8, 1, 2, 2, 3, 0]);
    let program = Program::from_source(
        &context,
        include_str!("triangle.vert"),
        include_str!("triangle.frag"),
    )
    .unwrap();

    window.render_loop(move |frame_input| {
        let aspect = frame_input.viewport.width as f32 / frame_input.viewport.height as f32;
        let inv_aspect = frame_input.viewport.height as f32 / frame_input.viewport.width as f32;
        frame_input
            .screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.1, 0.1, 0.1, 1.0, 1.0))
            .write(|| {
                let time = frame_input.accumulated_time as f32;
                program.use_uniform(
                    "model",
                    Mat4::from_angle_y(radians(time * 0.005))
                        * Mat4::from_nonuniform_scale(
                            if aspect > inv_aspect { inv_aspect } else { 1.0 },
                            if aspect > inv_aspect { aspect } else { 1.0 },
                            1.0,
                        ),
                );
                program.use_vertex_attribute("position", &positions);
                program.draw_elements(RenderStates::default(), frame_input.viewport, &indices);
            });

        FrameOutput::default()
    });
}
