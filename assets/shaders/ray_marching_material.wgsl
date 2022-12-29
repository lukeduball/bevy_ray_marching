struct Camera {
    position: vec3<f32>,
    forward: vec3<f32>,
    horizontal: vec3<f32>,
    vertical: vec3<f32>,
    aspect_ratio: f32,
};

struct Globals {
    // The time since startup in seconds
    // Wraps to 0 after 1 hour.
    time: f32,
    // The delta time since the previous frame in seconds
    delta_time: f32,
    // Frame count since the start of the app.
    // It wraps to zero when it reaches the maximum value of a u32.
    frame_count: u32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _wasm_padding: f32
#endif
}

@group(0) @binding(1)
var<uniform> globals: Globals;

@group(1) @binding(0)
var<uniform> camera: Camera;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_coords: vec2<f32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4(vertex.position, 1.0);
    out.uv_coords = vertex.uv_coords * 2.0 - 1.0;
    out.uv_coords.x *= camera.aspect_ratio;
    return out;
}

struct FragmentIn {
    @location(0) uv_coords: vec2<f32>,
}

fn get_distance_from_sphere(current_position: vec3<f32>, sphere_center: vec3<f32>, radius: f32) -> f32 {
    return length(current_position - sphere_center) - radius;
}

fn get_distance_from_world(current_position: vec3<f32>) -> f32 {
    var displacement = sin(5.0 * current_position.x + globals.time) * sin(5.0 * current_position.y + globals.time) * sin(5.0 * current_position.z + globals.time) * 0.25;
    var sphere_distance = get_distance_from_sphere(current_position, vec3<f32>(0.0, 0.0, 0.0), 1.0);

    //Add other shapes in this area

    return sphere_distance + displacement;
}

//Calculate the normal for any shape by calculating the gradient
// We calculate the gradient by taking a small offset in each unit direction and find the difference
fn calculate_normal(current_position: vec3<f32>) -> vec3<f32> {
    var SMALL_STEP = vec2<f32>(0.001, 0.0);

    var gradient_x = get_distance_from_world(current_position + SMALL_STEP.xyy) - get_distance_from_world(current_position - SMALL_STEP.xyy);
    var gradient_y = get_distance_from_world(current_position + SMALL_STEP.yxy) - get_distance_from_world(current_position - SMALL_STEP.yxy);
    var gradient_z = get_distance_from_world(current_position + SMALL_STEP.yyx) - get_distance_from_world(current_position - SMALL_STEP.yyx);

    return normalize(vec3<f32>(gradient_x, gradient_y, gradient_z));
}

fn ray_march(ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> vec3<f32> {
    var total_distance_traveled = 0.0;
    var NUMBER_OF_STEPS = 128;
    var MINIMUM_HIT_DISTANCE = 0.001;
    var MAXIMUM_TRAVEL_DISTANCE = 1000.0;

    for(var i = 0; i < NUMBER_OF_STEPS; i++) {
        var current_position = ray_origin + total_distance_traveled * ray_direction;

        var distance_to_closest = get_distance_from_world(current_position);

        if(distance_to_closest < MINIMUM_HIT_DISTANCE) {
            var normal = calculate_normal(current_position);

            var light_position = vec3<f32>(2.0, -5.0, -3.0);

            var direction_to_light = normalize(current_position - light_position);

            var diffuse_intensity = max(0.0, dot(normal, direction_to_light));

            return vec3<f32>(1.0, 0.0, 0.0) * diffuse_intensity;
        }

        if(total_distance_traveled > MAXIMUM_TRAVEL_DISTANCE) {
            //No hit has occured, break out of the loop
            break;
        }

        total_distance_traveled += distance_to_closest;
    } 

    //A miss has occured so return a background color
    return vec3<f32>(0.0, 0.0, 0.0);
}

@fragment
fn fragment(in: FragmentIn) -> @location(0) vec4<f32> {
    var camera_origin = camera.position;
    var ray_origin = camera_origin + camera.forward * 1.0 + (in.uv_coords.x * camera.horizontal) + (in.uv_coords.y * camera.vertical);
    var ray_direction = normalize(ray_origin - camera_origin);

    var color = ray_march(ray_origin, ray_direction);

    return vec4(color.x, color.y, color.z, 1.0);
}