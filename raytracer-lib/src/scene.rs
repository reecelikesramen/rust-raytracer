use crate::{
    camera::{OrthographicCamera, PerspectiveCamera},
    color,
    geometry::{Cuboid, Shape, Sphere, Triangle},
    light::{AmbientLight, Light, PointLight},
    prelude::*,
    shader::{BlinnPhongShader, Hit, LambertianShader, Shader},
};
use nalgebra::Vector3;
use serde::{de, Deserialize, Deserializer};
use std::collections::{HashMap, HashSet};
use std::{convert::TryInto, str::FromStr, sync::Arc};

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
    light: Vec<LightType>,
    shader: Vec<ShaderType>,
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
    #[serde(rename = "imagePlaneWidth", default)]
    image_plane_width: Option<Real>,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_type")]
    camera_type: String,
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
#[serde(tag = "_type")]
enum ShaderType {
    #[serde(rename = "Lambertian")]
    Lambertian(LambertianShaderData),
    #[serde(rename = "BlinnPhong")]
    BlinnPhong(BlinnPhongShaderData),
    #[serde(rename = "Mirror")]
    Mirror(MirrorShaderData),
}

#[derive(Deserialize, Debug)]
struct LambertianShaderData {
    #[serde(deserialize_with = "deserialize_vec3")]
    diffuse: Color,
    #[serde(rename = "_name")]
    name: String,
}

#[derive(Deserialize, Debug)]
struct BlinnPhongShaderData {
    #[serde(deserialize_with = "deserialize_vec3")]
    diffuse: Color,
    #[serde(deserialize_with = "deserialize_vec3")]
    specular: Color,
    #[serde(rename = "phongExp")]
    phong_exp: f32,
    #[serde(rename = "_name")]
    name: String,
}

#[derive(Deserialize, Debug)]
struct MirrorShaderData {
    roughness: Real,
    #[serde(rename = "_name")]
    name: String,
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

pub fn load_scene(
    path: &str,
    image_width: u32,
    image_height: u32,
    aspect_ratio: Real,
    disable_shadows: bool,
) -> Result<Scene, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let scene_file: SceneFile = serde_json::from_reader(reader)?;
    let scene = scene_file.scene;

    // print scene
    println!("{:#?}", scene);

    // Check that there is exactly one camera
    if scene.camera.len() != 1 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "scene must have exactly one camera",
        )));
    }

    // Check that view_dir or lookat_point is specified
    if scene.camera[0].view_dir.is_none() && scene.camera[0].lookat_point.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "camera must have view_dir or lookat_point specified",
        )));
    }

    // View direction or calculate from camera position and lookat point
    let view_dir = match scene.camera[0].view_dir {
        Some(v) => v,
        // calculate view dir from camera position and lookat point
        None => scene.camera[0].position - scene.camera[0].lookat_point.unwrap(),
    };

    // Create camera
    let mut camera: Box<dyn crate::camera::Camera> = match scene.camera[0].camera_type.as_str() {
        "perspective" => Box::new(PerspectiveCamera::new(
            scene.camera[0].position,
            &view_dir,
            aspect_ratio,
            scene.camera[0].focal_length,
        )),
        "orthographic" => Box::new(OrthographicCamera::new(
            scene.camera[0].position,
            &view_dir,
            aspect_ratio,
        )),
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "camera type not supported",
            )))
        }
    };
    camera.set_image_pixels(image_width, image_height);

    // Create shaders
    let mut shaders: HashMap<String, Arc<dyn crate::shader::Shader>> = HashMap::new();
    for shader in scene.shader.iter() {
        match shader {
            ShaderType::Lambertian(lambertian) => {
                shaders.insert(
                    lambertian.name.clone(),
                    Arc::new(LambertianShader::new(lambertian.diffuse)),
                );
            }
            ShaderType::BlinnPhong(blinn_phong) => {
                shaders.insert(
                    blinn_phong.name.clone(),
                    Arc::new(BlinnPhongShader::new(
                        blinn_phong.diffuse,
                        blinn_phong.specular,
                        blinn_phong.phong_exp,
                    )),
                );
            }
            _ => {
                unimplemented!("shader type not supported yet")
            }
        }
    }

    // create a set of names for the shapes to that names are unique
    let mut shape_names: HashSet<&str> = HashSet::new();

    // Create shapes
    let mut shapes: Vec<Box<dyn crate::geometry::Shape>> = Vec::new();
    for shape in scene.shape.iter() {
        // extract shader
        let shader = match shaders.get(&shape.shader.name) {
            Some(s) => Arc::clone(s),
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "shape references non-existent shader",
                )))
            }
        };

        let shape_name = Box::leak(shape.name.clone().into_boxed_str());
        if !shape_names.insert(shape_name) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "shape names must be unique",
            )));
        }
        let shape: Box<dyn Shape> = match &shape.shape {
            ShapeType::Sphere(sphere) => Box::new(Sphere::new(
                sphere.center,
                sphere.radius,
                shader,
                shape_name,
            )),
            ShapeType::Box(cuboid) => Box::new(Cuboid::new(
                cuboid.min_point,
                cuboid.max_point,
                shader,
                shape_name,
            )),
            ShapeType::Triangle(triangle) => Box::new(Triangle::new(
                triangle.a, triangle.b, triangle.c, shader, shape_name,
            )),
            _ => {
                unimplemented!("shape type not supported yet")
            }
        };
        shapes.push(shape);
    }

    // Create lights
    let mut lights: Vec<Box<dyn crate::light::Light>> = Vec::new();
    for light in scene.light.iter() {
        let light: Box<dyn Light> = match light {
            LightType::AmbientLight(ambient_light) => {
                Box::new(AmbientLight::new(ambient_light.intensity))
            }
            LightType::PointLight(point_light) => {
                Box::new(PointLight::new(point_light.position, point_light.intensity))
            }
            _ => {
                unimplemented!("light type not supported yet")
            }
        };
        lights.push(light);
    }

    let scene = Scene {
        disable_shadows,
        background_color: scene.background_color.unwrap_or(color!(0.0, 0.0, 0.0)),
        camera,
        shapes,
        shaders,
        lights,
    };
    return Ok(scene);
}

#[derive(Debug)]
pub struct Scene {
    pub disable_shadows: bool,
    pub background_color: Color,
    pub camera: Box<dyn crate::camera::Camera>,
    pub shapes: Vec<Box<dyn crate::geometry::Shape>>,
    pub shaders: HashMap<String, Arc<dyn crate::shader::Shader>>,
    pub lights: Vec<Box<dyn crate::light::Light>>,
}

impl Scene {
    pub fn any_hit<'hit>(&'hit self, hit: &'hit mut Hit<'hit>) -> bool {
        for shape in &self.shapes {
            if shape.closest_hit(hit) {
                return true;
            }
        }
        false
    }
}
