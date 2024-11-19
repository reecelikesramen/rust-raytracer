mod parse_vec3;

use serde::{Deserialize, Serialize};

use crate::{camera::*, color, geometry::*, light::*, prelude::*, shader::*};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};

/// TODO:
/// - Instanced models [model done]
/// - Transforms for instanced models [model done]
/// - Textures [model done]
/// - Diffuse, specular can be texture or color [model done]
/// - ParsedScene doesn't need a shaders map
/// - SceneArgs container for disable_shadows, recursion_depth, image_width, image_height, etc...
/// - Background structure for either background_color or env_map
/// - Impl scene stuff so its not just public members
/// - MTL parsing into shaders

#[derive(Debug)]
pub struct Scene {
    pub disable_shadows: bool,
    pub background_color: Color,
    pub camera: Box<dyn crate::camera::Camera>,
    pub shapes: Vec<Arc<dyn crate::geometry::Shape>>,
    pub shaders: std::collections::HashMap<String, Arc<dyn crate::shader::Shader>>,
    pub lights: Vec<Box<dyn crate::light::Light>>,
    pub bvh: crate::geometry::BVH,
    pub recursion_depth: u16,
    pub image_width: u32,
    pub image_height: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct SceneModel {
    scene: SceneData,
}

#[derive(Deserialize, Serialize, Debug)]
struct SceneData {
    #[serde(alias = "sceneParameters", default)]
    scene_parameters: SceneParameters,
    #[serde(alias = "camera")]
    cameras: Vec<CameraData>,
    #[serde(alias = "light", default)]
    lights: Vec<LightData>,
    #[serde(alias = "shader")]
    shaders: Vec<ShaderData>,
    #[serde(alias = "shape")]
    shapes: Vec<ShapeData>,
    #[serde(alias = "texture", default)]
    textures: Vec<TextureData>,
    #[serde(alias = "instance", default)]
    instances: Vec<ShapeData>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct SceneParameters {
    #[serde(flatten)]
    background: Option<Background>,
    camera: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Background {
    EnvMap(EnvironmentMap),
    BackgroundColor {
        #[serde(alias = "bgColor")]
        #[serde(alias = "_bgColor")]
        background_color: W<Color>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum EnvironmentMap {
    Prefix {
        #[serde(alias = "envMapPrefix")]
        env_map_prefix: String,
    },
    VertCross {
        #[serde(alias = "envMapVertCross")]
        env_map_vert_cross: String,
    },
}

#[derive(Deserialize, Serialize, Debug)]
struct CameraData {
    #[serde(rename = "_name")]
    name: String,
    #[serde(flatten)]
    camera_type: CameraType,
    #[serde(alias = "imagePlaneWidth", default)]
    image_plane_width: Option<Real>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "_type")]
enum CameraType {
    #[serde(alias = "perspective")]
    Perspective(PerspectiveCameraData),
    #[serde(alias = "orthographic")]
    Orthographic(OrthographicCameraData),
}

#[derive(Deserialize, Serialize, Debug)]
struct PerspectiveCameraData {
    position: W<Vec3>,
    #[serde(flatten)]
    orientation: CameraOrientation,
    #[serde(alias = "focalLength")]
    focal_length: Real,
}

#[derive(Deserialize, Serialize, Debug)]
struct OrthographicCameraData {
    position: W<Vec3>,
    #[serde(flatten)]
    orientation: CameraOrientation,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum CameraOrientation {
    ViewDir {
        #[serde(alias = "viewDir")]
        view_dir: W<Vec3>,
    },
    LookAtPoint {
        #[serde(alias = "lookatPoint")]
        lookat_point: W<Vec3>,
    },
}

impl CameraOrientation {
    pub fn get_view_direction(&self, position: Vec3) -> Vec3 {
        match self {
            CameraOrientation::ViewDir { view_dir } => view_dir.0,
            CameraOrientation::LookAtPoint { lookat_point } => lookat_point.0 - position,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct LightData {
    #[serde(flatten)]
    light_type: LightType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "_type")]
#[serde(rename_all = "lowercase")]
enum LightType {
    Point(PointLightData),
    Area(AreaLightData),
    Shape(ShapeLightData),
    Ambient(AmbientLightData),
}

#[derive(Deserialize, Serialize, Debug)]
struct PointLightData {
    position: W<Vec3>,
    intensity: W<Color>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AmbientLightData {
    intensity: W<Color>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AreaLightData {
    position: W<Vec3>,
    intensity: W<Color>,
    normal: W<Vec3>,
    #[serde(flatten)]
    shape: AreaLightShape,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShapeLightData {
    intensity: W<Color>,
    shape: ShapeData,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum AreaLightShape {
    Rectangular { length: Real, width: Real },
    Circular { radius: Real },
}

#[derive(Deserialize, Serialize, Debug)]
struct ShaderData {
    #[serde(rename = "_name")]
    name: String,
    #[serde(flatten)]
    shader: ShaderType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "_type")]
enum ShaderType {
    Diffuse,
    Lambertian(LambertianShaderData),
    BlinnPhong(BlinnPhongShaderData),
    #[serde(alias = "Mirror")]
    PerfectMirror,
    GGXMirror(GGXMirrorShaderData),
    #[serde(alias = "BlinnPhongMirrored")]
    BlinnPhongMirror,
    Glaze,
    Dielectric,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum MaterialProperty {
    Color(W<Color>),
    Texture {
        #[serde(alias = "tex")]
        texture: String,
        #[serde(alias = "data")]
        tint: W<Color>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
struct LambertianShaderData {
    diffuse: MaterialProperty,
}

#[derive(Deserialize, Serialize, Debug)]
struct BlinnPhongShaderData {
    diffuse: MaterialProperty,
    specular: MaterialProperty,
    #[serde(alias = "phongExp")]
    shininess: f32,
}

#[derive(Deserialize, Serialize, Debug)]
struct GGXMirrorShaderData {
    roughness: Real,
    samples: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShaderRef {
    #[serde(rename = "_ref")]
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ShaderRefType {
    Nested(ShaderRef),
    Inline(String),
}

impl ShaderRefType {
    fn name(&self) -> &String {
        match self {
            ShaderRefType::Nested(ShaderRef { name }) => name,
            ShaderRefType::Inline(name) => name,
        }
    }
}

impl Serialize for ShaderRefType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ShaderRefType::Nested(ShaderRef { name }) => serializer.serialize_str(name),
            ShaderRefType::Inline(name) => serializer.serialize_str(name),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct ShapeData {
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_shader")]
    #[serde(alias = "shader")]
    shader: ShaderRefType,
    #[serde(flatten)]
    shape: ShapeType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "_type")]
#[serde(rename_all = "lowercase")]
enum ShapeType {
    Sphere(SphereData),
    Box(BoxData),
    Triangle(TriangleData),
    Mesh(MeshData),
    Instance(InstanceData),
}

#[derive(Deserialize, Serialize, Debug)]
struct SphereData {
    center: W<Vec3>,
    radius: Real,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum BoxData {
    MinMaxPoint {
        #[serde(alias = "minPt")]
        min: W<Vec3>,
        #[serde(alias = "maxPt")]
        max: W<Vec3>,
    },
    CenterExtent {
        center: W<Vec3>,
        extent: W<Vec3>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
struct TriangleData {
    #[serde(alias = "v0")]
    a: W<Vec3>,
    #[serde(alias = "v1")]
    b: W<Vec3>,
    #[serde(alias = "v2")]
    c: W<Vec3>,
}

#[derive(Deserialize, Serialize, Debug)]
struct MeshData {
    #[serde(alias = "file")]
    model_path: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct InstanceData {
    #[serde(alias = "_id")]
    instance_of: String,
    #[serde(alias = "xform")]
    transform: Vec<TransformData>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum TransformData {
    Translate {
        amount: W<Vec3>,
    },
    Scale {
        amount: W<Vec3>,
    },
    #[serde(alias = "rotation")]
    Rotate {
        axis: RotationAxis,
        #[serde(alias = "amount")]
        degrees: Real,
    },
}

#[derive(Deserialize, Serialize, Debug)]
enum RotationAxis {
    #[serde(alias = "x")]
    X,
    #[serde(alias = "y")]
    Y,
    #[serde(alias = "z")]
    Z,
}

#[derive(Deserialize, Serialize, Debug)]
struct TextureData {
    #[serde(alias = "sourcefile")]
    image_path: String,
    #[serde(alias = "_name")]
    name: String,
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
    let scene_file: SceneModel = serde_json::from_str(scene_json)?;
    let scene = scene_file.scene;

    // print scene data
    #[cfg(debug_assertions)]
    println!("{:#?}", scene);

    // Image size or default
    let image_width = image_width.unwrap_or(DEFAULT_IMAGE_WIDTH);
    let image_height = image_height.unwrap_or(DEFAULT_IMAGE_HEIGHT);

    // Calculate aspect ratio if not specified
    let aspect_ratio = aspect_ratio.unwrap_or(image_width as Real / image_height as Real);

    // Check that there is exactly one camera
    if scene.cameras.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "scene must have at least one camera",
        )));
    }

    // Select camera
    let camera_index = if scene.cameras.len() == 1 {
        0
    } else {
        // camera name specified or default
        let camera_name = scene
            .scene_parameters
            .camera
            .unwrap_or(DEFAULT_CAMERA.to_string());

        // filter scene.cameras by name
        scene
            .cameras
            .iter()
            .position(|c| c.name == camera_name)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("camera {} not found", camera_name),
                )
            })?
    };

    // Create camera
    let mut camera: Box<dyn crate::camera::Camera> = match &scene.cameras[camera_index].camera_type
    {
        CameraType::Perspective(perspective) => Box::new(PerspectiveCamera::new(
            perspective.position.0,
            &perspective
                .orientation
                .get_view_direction(perspective.position.0),
            aspect_ratio,
            perspective.focal_length,
        )),
        CameraType::Orthographic(orthographic) => Box::new(OrthographicCamera::new(
            orthographic.position.0,
            &orthographic
                .orientation
                .get_view_direction(orthographic.position.0),
            aspect_ratio,
        )),
    };

    // Set image size
    camera.set_image_pixels(image_width, image_height);

    // Create shaders
    let mut shaders: HashMap<String, Arc<dyn Shader>> = HashMap::new();
    for shader in scene.shaders.iter() {
        let shader_name = shader.name.clone();
        let shader: Arc<dyn Shader> = match &shader.shader {
            ShaderType::Lambertian(lambertian) => {
                let diffuse = match &lambertian.diffuse {
                    MaterialProperty::Color(color) => color.0,
                    _ => unimplemented!("texture for material property not implemented yet"),
                };
                Arc::new(LambertianShader::new(diffuse))
            }
            ShaderType::BlinnPhong(blinn_phong) => {
                let diffuse = match &blinn_phong.diffuse {
                    MaterialProperty::Color(color) => color.0,
                    _ => unimplemented!("texture for material property not implemented yet"),
                };
                let specular = match &blinn_phong.specular {
                    MaterialProperty::Color(color) => color.0,
                    _ => unimplemented!("texture for material property not implemented yet"),
                };

                Arc::new(BlinnPhongShader::new(
                    diffuse,
                    specular,
                    blinn_phong.shininess,
                ))
            }
            ShaderType::PerfectMirror => Arc::new(PerfectMirrorShader::default()),
            ShaderType::GGXMirror(mirror) => {
                Arc::new(GGXMirrorShader::new(mirror.roughness, mirror.samples))
            }
            _ => Arc::new(NullShader::default()),
        };
        shaders.insert(shader_name, shader);
    }

    // normal shader
    let normal_shader = Arc::new(NormalShader::default());

    // create a set of names for the shapes to that names are unique
    let mut shape_names: HashSet<&str> = HashSet::new();

    // Create shapes
    let mut shapes: Vec<Arc<dyn Shape>> = Vec::new();
    for shape in scene.shapes.iter() {
        // extract shader, or just use normal shader
        let shader = if !render_normals {
            match shaders.get(shape.shader.name()) {
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
                sphere.center.0,
                sphere.radius,
                shader,
                shape_name,
            )),
            ShapeType::Box(cuboid) => Arc::new(match cuboid {
                BoxData::MinMaxPoint {
                    min: min_point,
                    max: max_point,
                } => Cuboid::new(min_point.0, max_point.0, shader, shape_name),
                BoxData::CenterExtent { center, extent } => {
                    let half_extent = extent.0 / 2.0;
                    let min_point = center.0 - half_extent;
                    let max_point = center.0 + half_extent;
                    Cuboid::new(min_point, max_point, shader, shape_name)
                }
            }),
            ShapeType::Triangle(triangle) => Arc::new(Triangle::new(
                triangle.a.0,
                triangle.b.0,
                triangle.c.0,
                shader,
                shape_name,
            )),
            ShapeType::Mesh(mesh) => {
                // TODO: this should be done differently
                let model_path = String::from(
                    Path::new(&scene_data_path)
                        .join(&mesh.model_path)
                        .to_str()
                        .expect("failed to convert model path to string"),
                );
                Arc::new(Mesh::new(model_path, shader, shape_name))
            }
            _ => unimplemented!("shape type not implemented yet"),
        };
        shapes.push(shape);
    }

    // Create lights
    let mut lights: Vec<Box<dyn crate::light::Light>> = Vec::new();
    for light in scene.lights.iter() {
        let light: Box<dyn Light> = match &light.light_type {
            LightType::Ambient(ambient_light) => {
                Box::new(AmbientLight::new(ambient_light.intensity.0))
            }
            LightType::Point(point_light) => Box::new(PointLight::new(
                point_light.position.0,
                point_light.intensity.0,
            )),
            _ => unimplemented!("light type not implemented yet"),
        };
        lights.push(light);
    }

    // // get background color
    // let background_color = if render_normals {
    //     default();
    // } else {
    //     scene.background_color.unwrap_or(DEFAULT_BACKGROUND_COLOR)
    // };

    let shape_refs = shapes.clone();
    let bvh = BVH::new(shape_refs);

    let scene = Scene {
        disable_shadows,
        background_color: color!(0.0, 0.0, 0.0),
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
