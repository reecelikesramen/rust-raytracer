import init, { PixelRenderer, test_webgl } from "./pkg/raytracer_wasm.js"

async function run() {
  // Initialize the WASM module
  await init()

  test_webgl()

  // const canvas = document.getElementById("canvas")
  // const ctx = canvas.getContext("2d")

  const width = 800
  const height = 600

  // Fetch ./scenes/sphere_scene.json into a string
  const scene_json = await fetch("./scenes/mirror_scene.json").then((r) => r.text())

  const scene_args = {
    width,
    height,
  }

  // Example of calling your WASM function
  try {
    const renderer = new PixelRenderer(width, height)

    console.log("About to raytrace the scene!")
    const date_start = Date.now()
    renderer.raytrace(scene_json, scene_args)
    const time_elapsed = Date.now() - date_start
    console.log("Raytraced the scene in", time_elapsed, "ms!")

    console.log("About to render the pixels to the canvas!")
    renderer.render_to_canvas()
    console.log("Rendered the pixels to the canvas!")
  } catch (e) {
    console.error("Error rendering scene:", e)
  }
}

run()
