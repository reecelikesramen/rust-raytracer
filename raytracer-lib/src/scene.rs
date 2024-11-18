use crate::{
    camera::{OrthographicCamera, PerspectiveCamera},
    color,
    geometry::{Cuboid, Mesh, Shape, Sphere, Triangle, BVH},
    light::{AmbientLight, Light, PointLight},
    prelude::*,
    shader::{
        BlinnPhongShader, GGXMirrorShader, LambertianShader, NormalShader, PerfectMirrorShader,
        Shader,
    },
};
use nalgebra::Vector3;
use serde::{de, Deserialize, Deserializer};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    str::FromStr,
    sync::Arc,
};

fn parse_vec3<FloatType>(s: &str) -> Result<Vector3<FloatType>, String>
where
    FloatType: Into<f64> + Copy + FromStr,
    <FloatType as FromStr>::Err: std::fmt::Display,
{
    let numbers: Result<Vec<FloatType>, _> =
        s.split_whitespace().map(FloatType::from_str).collect();

    match numbers {
        Ok(nums) if nums.len() == 3 => Ok(Vector3::new(nums[0], nums[1], nums[2])),
        Ok(_) => Err("expected exactly 3 space-separated numbers".to_string()),
        Err(e) => Err(format!("failed to parse number: {}", e)),
    }
}

fn deserialize_vec3<'de, D, FloatType>(deserializer: D) -> Result<Vector3<FloatType>, D::Error>
where
    D: Deserializer<'de>,
    FloatType: Into<f64> + Copy + FromStr,
    <FloatType as FromStr>::Err: std::fmt::Display,
{
    let s: String = String::deserialize(deserializer)?;
    parse_vec3(&s).map_err(de::Error::custom)
}

fn deserialize_optional_vec3<'de, D, FloatType>(
    deserializer: D,
) -> Result<Option<Vector3<FloatType>>, D::Error>
where
    D: Deserializer<'de>,
    FloatType: Into<f64> + Copy + FromStr,
    <FloatType as FromStr>::Err: std::fmt::Display,
{
    // First deserialize to Option<String>
    let opt: Option<String> = Option::deserialize(deserializer)?;

    match opt {
        Some(s) => parse_vec3(&s).map(Some).map_err(de::Error::custom),
        None => Ok(None),
    }
}

#[derive(Deserialize, Debug)]
struct SceneFile {
    scene: SceneData,
}

#[derive(Deserialize, Debug)]
struct SceneData {
    camera: Vec<CameraData>,
    light: Vec<LightData>,
    shader: Vec<ShaderData>,
    shape: Vec<ShapeData>,
    #[serde(
        rename = "_bgColor",
        default,
        deserialize_with = "deserialize_optional_vec3"
    )]
    background_color: Option<Color>,
}

#[derive(Deserialize, Debug)]
struct CameraData {
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "imagePlaneWidth", default)]
    image_plane_width: Option<Real>,
    #[serde(flatten)]
    camera_type: CameraType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "_type")]
enum CameraType {
    #[serde(rename = "perspective")]
    Perspective(PerspectiveCameraData),
    #[serde(rename = "orthographic")]
    Orthographic(OrthographicCameraData),
}

#[derive(Deserialize, Debug)]
struct PerspectiveCameraData {
    #[serde(deserialize_with = "deserialize_vec3")]
    position: Vec3,
    #[serde(
        rename = "viewDir",
        default,
        deserialize_with = "deserialize_optional_vec3"
    )]
    view_dir: Option<Vec3>,
    #[serde(
        rename = "lookatPoint",
        default,
        deserialize_with = "deserialize_optional_vec3"
    )]
    lookat_point: Option<Vec3>,
    #[serde(rename = "focalLength")]
    focal_length: Real,
}

#[derive(Deserialize, Debug)]
struct OrthographicCameraData {
    #[serde(deserialize_with = "deserialize_vec3")]
    position: Vec3,
    #[serde(
        rename = "viewDir",
        default,
        deserialize_with = "deserialize_optional_vec3"
    )]
    view_dir: Option<Vec3>,
    #[serde(
        rename = "lookatPoint",
        default,
        deserialize_with = "deserialize_optional_vec3"
    )]
    lookat_point: Option<Vec3>,
}

#[derive(Deserialize, Debug)]
struct LightData {
    #[serde(flatten)]
    light_type: LightType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "_type")]
enum LightType {
    #[serde(rename = "point")]
    PointLight(PointLightData),
    #[serde(rename = "ambient")]
    AmbientLight(AmbientLightData),
}

#[derive(Deserialize, Debug)]
struct PointLightData {
    #[serde(deserialize_with = "deserialize_vec3")]
    position: Vec3,
    #[serde(deserialize_with = "deserialize_vec3")]
    intensity: Color,
}

#[derive(Deserialize, Debug)]
struct AmbientLightData {
    #[serde(deserialize_with = "deserialize_vec3")]
    intensity: Color,
}

#[derive(Deserialize, Debug)]
struct ShaderData {
    #[serde(rename = "_name")]
    name: String,
    #[serde(flatten)]
    shader: ShaderType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "_type")]
enum ShaderType {
    #[serde(rename = "Lambertian")]
    Lambertian(LambertianShaderData),
    #[serde(rename = "BlinnPhong")]
    BlinnPhong(BlinnPhongShaderData),
    #[serde(rename = "Mirror")]
    PerfectMirror(PerfectMirrorShaderData),
    #[serde(rename = "GGXMirror")]
    GGXMirror(GGXMirrorShaderData),
}

#[derive(Deserialize, Debug)]
struct LambertianShaderData {
    #[serde(deserialize_with = "deserialize_vec3")]
    diffuse: Color,
}

#[derive(Deserialize, Debug)]
struct BlinnPhongShaderData {
    #[serde(deserialize_with = "deserialize_vec3")]
    diffuse: Color,
    #[serde(deserialize_with = "deserialize_vec3")]
    specular: Color,
    #[serde(rename = "phongExp")]
    shininess: f32,
}

#[derive(Deserialize, Debug)]
struct PerfectMirrorShaderData {}

#[derive(Deserialize, Debug)]
struct GGXMirrorShaderData {
    roughness: Real,
    samples: u32,
}

#[derive(Deserialize, Debug)]
struct ShaderRef {
    #[serde(rename = "_ref")]
    name: String,
}

#[derive(Deserialize, Debug)]
struct ShapeData {
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "shader")]
    shader: ShaderRef,
    #[serde(flatten)]
    shape: ShapeType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "_type")]
enum ShapeType {
    #[serde(rename = "sphere")]
    Sphere(SphereData),
    #[serde(rename = "box")]
    Box(BoxData),
    #[serde(rename = "triangle")]
    Triangle(TriangleData),
    #[serde(rename = "mesh")]
    Mesh(MeshData),
}

#[derive(Deserialize, Debug)]
struct SphereData {
    #[serde(deserialize_with = "deserialize_vec3")]
    center: Vec3,
    radius: Real,
}

#[derive(Deserialize, Debug)]
struct BoxData {
    #[serde(rename = "minPt", deserialize_with = "deserialize_vec3")]
    min_point: Vec3,
    #[serde(rename = "maxPt", deserialize_with = "deserialize_vec3")]
    max_point: Vec3,
}

#[derive(Deserialize, Debug)]
struct TriangleData {
    #[serde(rename = "v0", deserialize_with = "deserialize_vec3")]
    a: Vec3,
    #[serde(rename = "v1", deserialize_with = "deserialize_vec3")]
    b: Vec3,
    #[serde(rename = "v2", deserialize_with = "deserialize_vec3")]
    c: Vec3,
}

#[derive(Deserialize, Debug)]
struct MeshData {
    #[serde(rename = "file")]
    obj_path: String,
}

pub fn parse_scene(
    scene_json: &str,
    scene_data_path: &str,
    image_width: Option<u32>,
    image_height: Option<u32>,
    aspect_ratio: Option<Real>,
    recursion_depth: Option<u16>,
    disable_shadows: bool,
    render_normals: bool,
) -> Result<Scene, Box<dyn std::error::Error>> {
    let scene_file: SceneFile = serde_json::from_str(scene_json)?;
    let scene = scene_file.scene;

    // print scene data
    #[cfg(debug_assertions)]
    println!("{:#?}", scene);

    // Check that there is exactly one camera
    if scene.camera.len() != 1 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "scene must have exactly one camera",
        )));
    }

    // Image size or default
    let image_width = image_width.unwrap_or(DEFAULT_IMAGE_WIDTH);
    let image_height = image_height.unwrap_or(DEFAULT_IMAGE_HEIGHT);

    // Calculate aspect ratio if not specified
    let aspect_ratio = aspect_ratio.unwrap_or(image_width as Real / image_height as Real);

    // Create camera
    let mut camera: Box<dyn crate::camera::Camera> = match &scene.camera[0].camera_type {
        CameraType::Perspective(perspective) => {
            // View direction or calculate from camera position and lookat point
            let view_dir = match perspective.view_dir {
                Some(v) => v,
                // calculate view dir from camera position and lookat point
                None => {
                    perspective
                        .lookat_point
                        .ok_or("viewDir or lookatPoint must be specified")?
                        - perspective.position
                }
            };

            Box::new(PerspectiveCamera::new(
                perspective.position,
                &view_dir,
                aspect_ratio,
                perspective.focal_length,
            ))
        }
        CameraType::Orthographic(orthographic) => {
            // View direction or calculate from camera position and lookat point
            let view_dir = match orthographic.view_dir {
                Some(v) => v,
                // calculate view dir from camera position and lookat point
                None => {
                    orthographic
                        .lookat_point
                        .ok_or("viewDir or lookatPoint must be specified")?
                        - orthographic.position
                }
            };

            Box::new(OrthographicCamera::new(
                orthographic.position,
                &view_dir,
                aspect_ratio,
            ))
        }
    };

    camera.set_image_pixels(image_width, image_height);

    // Create shaders
    let mut shaders: HashMap<String, Arc<dyn crate::shader::Shader>> = HashMap::new();
    for shader in scene.shader.iter() {
        let shader_name = shader.name.clone();
        let shader: Arc<dyn Shader> = match &shader.shader {
            ShaderType::Lambertian(lambertian) => {
                Arc::new(LambertianShader::new(lambertian.diffuse))
            }
            ShaderType::BlinnPhong(blinn_phong) => Arc::new(BlinnPhongShader::new(
                blinn_phong.diffuse,
                blinn_phong.specular,
                blinn_phong.shininess,
            )),
            ShaderType::PerfectMirror(_) => Arc::new(PerfectMirrorShader::default()),
            ShaderType::GGXMirror(mirror) => {
                Arc::new(GGXMirrorShader::new(mirror.roughness, mirror.samples))
            }
        };
        shaders.insert(shader_name, shader);
    }

    // normal shader
    let normal_shader = Arc::new(NormalShader::default());

    // create a set of names for the shapes to that names are unique
    let mut shape_names: HashSet<&str> = HashSet::new();

    // Create shapes
    let mut shapes: Vec<Arc<dyn Shape>> = Vec::new();
    for shape in scene.shape.iter() {
        // extract shader, or just use normal shader
        let shader = if !render_normals {
            match shaders.get(&shape.shader.name) {
                Some(s) => Arc::clone(s),
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "shape references non-existent shader",
                    )))
                }
            }
        } else {
            Arc::clone(&normal_shader)
        };

        let shape_name = Box::leak(shape.name.clone().into_boxed_str());
        if !shape_names.insert(shape_name) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "shape names must be unique",
            )));
        }
        let shape: Arc<dyn Shape> = match &shape.shape {
            ShapeType::Sphere(sphere) => Arc::new(Sphere::new(
                sphere.center,
                sphere.radius,
                shader,
                shape_name,
            )),
            ShapeType::Box(cuboid) => Arc::new(Cuboid::new(
                cuboid.min_point,
                cuboid.max_point,
                shader,
                shape_name,
            )),
            ShapeType::Triangle(triangle) => Arc::new(Triangle::new(
                triangle.a, triangle.b, triangle.c, shader, shape_name,
            )),
            ShapeType::Mesh(mesh) => Arc::new(Mesh::new(
                Path::new(&scene_data_path)
                    .join(&mesh.obj_path)
                    .to_str()
                    .unwrap(),
                shader,
                shape_name,
            )),
        };
        shapes.push(shape);
    }

    // Create lights
    let mut lights: Vec<Box<dyn crate::light::Light>> = Vec::new();
    for light in scene.light.iter() {
        let light: Box<dyn Light> = match &light.light_type {
            LightType::AmbientLight(ambient_light) => {
                Box::new(AmbientLight::new(ambient_light.intensity))
            }
            LightType::PointLight(point_light) => {
                Box::new(PointLight::new(point_light.position, point_light.intensity))
            }
        };
        lights.push(light);
    }

    let background_color = if render_normals {
        color!(0.0, 0.0, 0.0)
    } else {
        scene.background_color.unwrap_or(DEFAULT_BACKGROUND_COLOR)
    };

    let shape_refs = shapes.clone();
    let bvh = BVH::new(shape_refs);

    let scene = Scene {
        disable_shadows,
        background_color,
        camera,
        shapes,
        shaders,
        lights,
        bvh,
        recursion_depth: recursion_depth.unwrap_or(DEFAULT_RECURSION_DEPTH),
        image_width,
        image_height,
    };
    return Ok(scene);
}

#[derive(Debug)]
pub struct Scene {
    pub disable_shadows: bool,
    pub background_color: Color,
    pub camera: Box<dyn crate::camera::Camera>,
    pub shapes: Vec<Arc<dyn Shape>>,
    pub shaders: HashMap<String, Arc<dyn crate::shader::Shader>>,
    pub lights: Vec<Box<dyn crate::light::Light>>,
    pub bvh: BVH,
    pub recursion_depth: u16,
    pub image_width: u32,
    pub image_height: u32,
}
