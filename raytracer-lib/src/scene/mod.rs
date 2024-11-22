mod parse_vec3;

use na::{Rotation3, Scale3, Translation3};
use serde::{de, Deserialize, Serialize};

use crate::{
    camera::*,
    color,
    geometry::*,
    light::*,
    material::{Dielectric, Diffuse, Lambertian, Material, Metal},
    prelude::*,
    shader::*,
    V3,
};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};

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
#[serde(untagged)]
enum PerspectiveCameraType {
    Old(OldPerspectiveCameraData),
    New(NewPerspectiveCameraData),
}

#[derive(Deserialize, Serialize, Debug)]
struct OldPerspectiveCameraData {
    #[serde(alias = "focalLength")]
    focal_length: Real,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewPerspectiveCameraData {
    #[serde(alias = "vfov")]
    vertical_fov: Real,
    focus_distance: Option<Real>,
    defocus_angle: Option<Real>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PerspectiveCameraData {
    position: W<V3>,
    #[serde(flatten)]
    orientation: CameraOrientation,
    #[serde(flatten)]
    camera_type: PerspectiveCameraType,
}

#[derive(Deserialize, Serialize, Debug)]
struct OrthographicCameraData {
    position: W<V3>,
    #[serde(flatten)]
    orientation: CameraOrientation,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum CameraOrientation {
    ViewDir {
        #[serde(alias = "viewDir")]
        view_dir: W<V3>,
    },
    LookAtPoint {
        #[serde(alias = "lookatPoint")]
        lookat_point: W<V3>,
    },
}

impl CameraOrientation {
    pub fn get_view_direction(&self, position: P3) -> V3 {
        match self {
            CameraOrientation::ViewDir { view_dir } => view_dir.0,
            CameraOrientation::LookAtPoint { lookat_point } => P3::from(lookat_point.0) - position,
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
    position: W<V3>,
    intensity: W<Color>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AmbientLightData {
    intensity: W<Color>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AreaLightData {
    position: W<V3>,
    intensity: W<Color>,
    normal: W<V3>,
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
    Diffuse(DiffuseShaderData),
    Lambertian(LambertianShaderData),
    BlinnPhong(BlinnPhongShaderData),
    #[serde(alias = "Mirror")]
    PerfectMirror,
    GGXMirror(GGXMirrorShaderData),
    #[serde(alias = "BlinnPhongMirrored")]
    BlinnPhongMirror,
    Glaze,
    Dielectric(DielectricShaderData),
    Metal(MetalShaderData),
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
    #[serde(default)]
    samples: u32,
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
struct DiffuseShaderData {
    diffuse: MaterialProperty,
    #[serde(default)]
    samples: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct MetalShaderData {
    albedo: MaterialProperty,
    fuzz: Real,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
struct DielectricShaderData {
    attenuation: MaterialProperty,
    #[serde(alias = "refractionIndex")]
    #[serde(alias = "ior")]
    refractive_index: Real,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ShaderRefType {
    Nested {
        #[serde(rename = "_ref")]
        name: String,
    },
    Inline(String),
}

impl ShaderRefType {
    fn name(&self) -> &String {
        match self {
            ShaderRefType::Nested { name } => name,
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
            ShaderRefType::Nested { name } => serializer.serialize_str(name),
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
    center: W<V3>,
    radius: Real,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum BoxData {
    MinMaxPoint {
        #[serde(alias = "minPt")]
        min: W<V3>,
        #[serde(alias = "maxPt")]
        max: W<V3>,
    },
    CenterExtent {
        center: W<V3>,
        extent: W<V3>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
struct TriangleData {
    #[serde(alias = "v0")]
    a: W<V3>,
    #[serde(alias = "v1")]
    b: W<V3>,
    #[serde(alias = "v2")]
    c: W<V3>,
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
        amount: W<V3>,
    },
    Scale {
        amount: W<V3>,
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
        CameraType::Perspective(perspective) => {
            let position = P3::from(perspective.position.0);
            let view_direction = perspective.orientation.get_view_direction(position);

            Box::new(match &perspective.camera_type {
                PerspectiveCameraType::Old(old) => PerspectiveCamera::old(
                    position,
                    &view_direction,
                    aspect_ratio,
                    old.focal_length,
                ),
                PerspectiveCameraType::New(new) => {
                    let defocus_angle = new.defocus_angle.unwrap_or(0.0);
                    let focus_distance = new.focus_distance.unwrap_or(1.0);

                    PerspectiveCamera::new(
                        position,
                        &view_direction,
                        aspect_ratio,
                        new.vertical_fov,
                        focus_distance,
                        defocus_angle,
                    )
                }
            })
        }
        CameraType::Orthographic(orthographic) => {
            let position = P3::from(orthographic.position.0);

            Box::new(OrthographicCamera::new(
                position,
                &orthographic.orientation.get_view_direction(position),
                aspect_ratio,
            ))
        }
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
                Arc::new(LambertianShader::new(diffuse, lambertian.samples))
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
            ShaderType::Diffuse(diffuse) => {
                let albedo = match &diffuse.diffuse {
                    MaterialProperty::Color(color) => color.0,
                    _ => unimplemented!("texture for material property not implemented yet"),
                };
                Arc::new(DiffuseShader::new(albedo, diffuse.samples))
            }
            _ => Arc::new(NullShader::default()),
        };
        shaders.insert(shader_name, shader);
    }

    // Create materials
    let mut materials: HashMap<String, Arc<dyn Material>> = HashMap::new();
    for shader in scene.shaders.iter() {
        let material_name = shader.name.clone();
        let material: Arc<dyn Material> = match &shader.shader {
            ShaderType::Lambertian(lambertian) => {
                let albedo = match &lambertian.diffuse {
                    MaterialProperty::Color(color) => color.0,
                    _ => unimplemented!("texture for material property not implemented yet"),
                };

                Arc::new(Lambertian::new(albedo))
            }
            ShaderType::Diffuse(diffuse) => {
                let albedo = match &diffuse.diffuse {
                    MaterialProperty::Color(color) => color.0,
                    _ => unimplemented!("texture for material property not implemented yet"),
                };

                Arc::new(Diffuse::new(albedo))
            }
            ShaderType::Metal(metal) => {
                let albedo = match &metal.albedo {
                    MaterialProperty::Color(color) => color.0,
                    _ => unimplemented!("texture for material property not implemented yet"),
                };
                Arc::new(Metal::new(albedo, metal.fuzz))
            }
            ShaderType::Dielectric(dielectric) => {
                let attenuation = match &dielectric.attenuation {
                    MaterialProperty::Color(color) => color.0,
                    _ => unimplemented!("texture for material property not implemented yet"),
                };
                Arc::new(Dielectric::new(attenuation, dielectric.refractive_index))
            }
            _ => Arc::new(Lambertian::new(color!(1.0, 0.0, 1.0))),
        };
        materials.insert(material_name, material);
    }

    // Create instances
    let mut instances: HashMap<String, Arc<dyn Shape>> = HashMap::new();
    for shape in scene.instances.iter() {
        let instance_name = Box::leak(shape.name.clone().into_boxed_str());
        let shader = Arc::new(NullShader::default());
        let material = Arc::new(Lambertian::new(color!(0.0, 0.0, 0.0)));
        let shape: Arc<dyn Shape> = match &shape.shape {
            ShapeType::Sphere(sphere) => Arc::new(Sphere::new(
                P3::from(sphere.center.0),
                sphere.radius,
                shader,
                material,
                instance_name,
            )),
            ShapeType::Box(cuboid) => Arc::new(match cuboid {
                BoxData::MinMaxPoint {
                    min: min_point,
                    max: max_point,
                } => Cuboid::new(
                    P3::from(min_point.0),
                    P3::from(max_point.0),
                    shader,
                    material,
                    instance_name,
                ),
                BoxData::CenterExtent { center, extent } => {
                    let center = P3::from(center.0);
                    let half_extent = extent.0 / 2.0;
                    let min_point = center - half_extent;
                    let max_point = center + half_extent;
                    Cuboid::new(min_point, max_point, shader, material, instance_name)
                }
            }),
            ShapeType::Triangle(triangle) => Arc::new(Triangle::new(
                P3::from(triangle.a.0),
                P3::from(triangle.b.0),
                P3::from(triangle.c.0),
                shader,
                material,
                instance_name,
            )),
            ShapeType::Mesh(mesh) => {
                // TODO: this should be done differently
                let model_path = String::from(
                    Path::new(&scene_data_path)
                        .join(&mesh.model_path)
                        .to_str()
                        .expect("failed to convert model path to string"),
                );
                Arc::new(Mesh::new(model_path, shader, material, instance_name))
            }
            ShapeType::Instance(_) => panic!("An instanced shape can not be type instance"),
        };
        instances.insert(instance_name.to_string(), shape);
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

        // extract material
        let material = match materials.get(shape.shader.name()) {
            Some(s) => Arc::clone(s),
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "shape references non-existent material",
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
        let shape: Arc<dyn Shape> = match &shape.shape {
            ShapeType::Sphere(sphere) => Arc::new(Sphere::new(
                P3::from(sphere.center.0),
                sphere.radius,
                shader,
                material,
                shape_name,
            )),
            ShapeType::Box(cuboid) => Arc::new(match cuboid {
                BoxData::MinMaxPoint {
                    min: min_point,
                    max: max_point,
                } => Cuboid::new(
                    P3::from(min_point.0),
                    P3::from(max_point.0),
                    shader,
                    material,
                    shape_name,
                ),
                BoxData::CenterExtent { center, extent } => {
                    let center = P3::from(center.0);
                    let half_extent = extent.0 / 2.0;
                    let min_point = center - half_extent;
                    let max_point = center + half_extent;
                    Cuboid::new(min_point, max_point, shader, material, shape_name)
                }
            }),
            ShapeType::Triangle(triangle) => Arc::new(Triangle::new(
                P3::from(triangle.a.0),
                P3::from(triangle.b.0),
                P3::from(triangle.c.0),
                shader,
                material,
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
                Arc::new(Mesh::new(model_path, shader, material, shape_name))
            }
            ShapeType::Instance(instance) => {
                let shape = instances
                    .get(&instance.instance_of)
                    .expect("instance ID is not a valid instance")
                    .clone();
                let mut translate = V3::default();
                let mut scale = V3::new(1.0, 1.0, 1.0);
                let mut rotate = (
                    Rotation3::identity(),
                    Rotation3::identity(),
                    Rotation3::identity(),
                );
                for transformation in instance.transform.iter() {
                    match transformation {
                        TransformData::Translate { amount } => translate += amount.0,
                        TransformData::Scale { amount } => scale.component_mul_assign(&amount.0),
                        TransformData::Rotate { axis, degrees } => {
                            let angle = PI * degrees / 180.0;
                            match axis {
                                RotationAxis::X => {
                                    rotate.0 = Rotation3::from_axis_angle(&V3::x_axis(), angle)
                                }
                                RotationAxis::Y => {
                                    rotate.1 = Rotation3::from_axis_angle(&V3::y_axis(), angle)
                                }
                                RotationAxis::Z => {
                                    rotate.2 = Rotation3::from_axis_angle(&V3::z_axis(), angle)
                                }
                            };
                        }
                    }
                }

                let rotation = rotate.2 * rotate.1 * rotate.0;

                Arc::new(Instance::new(
                    shape,
                    Translation3::from(translate),
                    rotation,
                    Scale3::from(scale),
                    shader,
                    material,
                    shape_name,
                ))
            }
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
                P3::from(point_light.position.0),
                point_light.intensity.0,
            )),
            _ => unimplemented!("light type not implemented yet"),
        };
        lights.push(light);
    }

    // get background color
    let background_color = if render_normals {
        color!(0.0, 0.0, 0.0)
    } else if let Some(background) = scene.scene_parameters.background {
        match background {
            Background::BackgroundColor { background_color } => background_color.0,
            Background::EnvMap(_) => {
                unimplemented!("environment maps aren't implemented yet")
            }
        }
    } else {
        DEFAULT_BACKGROUND_COLOR
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
