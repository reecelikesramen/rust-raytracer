import init, { RayTracer, initThreadPool } from "./pkg/raytracer_wasm.js"
import { threads } from "wasm-feature-detect"

/**
 *
 * @param {RayTracer} processor
 */
async function runChunkedProcessingWithRAF(processor) {
  return new Promise((resolve) => {
    let pixels_per_chunk = 10
    const TARGET_MIN = 80 // ms
    const TARGET_MAX = 120 // ms
    const TARGET_MID = (TARGET_MIN + TARGET_MAX) / 2 // 50ms
    const processNextChunk = async (start_time) => {
      if (!processor.complete) {
        const progress = await processor.raytrace_next_pixels(pixels_per_chunk)
        let elapsed = performance.now() - start_time

        // Proportional control with dampening
        if (elapsed < TARGET_MIN) {
          // Below target - increase chunk size proportionally to how far we are from target
          const adjustment = 1 + 0.5 * ((TARGET_MIN - elapsed) / TARGET_MIN)
          pixels_per_chunk = Math.ceil(pixels_per_chunk * adjustment)
        } else if (elapsed > TARGET_MAX) {
          // Above target - decrease chunk size proportionally to how far we are from target
          const adjustment = 1 - 0.5 * ((elapsed - TARGET_MAX) / TARGET_MAX)
          pixels_per_chunk = Math.max(1, Math.floor(pixels_per_chunk * adjustment))
        } else {
          // Within target range - make minor adjustments towards the middle
          const adjustment = 1 + 0.1 * ((TARGET_MID - elapsed) / TARGET_MID)
          pixels_per_chunk = Math.round(pixels_per_chunk * adjustment)
        }

        // Use requestAnimationFrame instead of setTimeout
        processor.render_to_canvas()
        requestAnimationFrame(processNextChunk)
      } else {
        resolve()
      }
    }

    // Start processing
    requestAnimationFrame(processNextChunk)
  })
}

let render_to_canvas_id

/**
 * @param {RayTracer} raytracer
 */
function start_render_to_canvas(raytracer) {
  const PERIOD = 1000 / 30
  let last_frame_time = performance.now() - PERIOD
  function animate(current_time) {
    if (current_time - last_frame_time < PERIOD) {
      return
    }

    raytracer.render_to_canvas()

    last_frame_time = current_time

    // Request the next frame
    render_to_canvas_id = requestAnimationFrame(animate)
  }

  // Start the animation
  render_to_canvas_id = requestAnimationFrame(animate)
}

function stop_render_to_canvas() {
  cancelAnimationFrame(render_to_canvas_id)
}

let start = false

;(async function run() {
  Error.stackTraceLimit = 30

  // Initialize the WASM module
  await init()

  const _threads = await threads()
  console.log("We have threads:", _threads)
  if (!_threads) {
    console.warn("WebAssembly multithreading is not supported in this browser")
    return
  }

  await initThreadPool(navigator.hardwareConcurrency)

  const width = 600
  const height = 600

  const canvas = document.getElementById("canvas")
  canvas.style.width = "600px"
  canvas.style.height = "600px"

  // Fetch ./scenes/sphere_scene.json into a string
  const scene_json = await fetch("./scenes/spheres.json").then((r) => r.text())

  const scene_args = {
    width,
    height,
    rays_per_pixel: 100,
  }

  try {
    const raytracer = await RayTracer.init("canvas", scene_json, scene_args)
    console.log("Initialized raytracer")

    // render black canvas
    raytracer.render_to_canvas()

    // start periodic rendering
    start_render_to_canvas(raytracer)

    // parallel processing using rayon
    console.log("Starting raytrace...")
    const date_start = performance.now()
    await raytracer.raytrace_parallel()
    // let i = await raytracer.test_rayon()
    // console.log(i)

    // stop periodic rendering
    stop_render_to_canvas()

    // render final image to canvas
    raytracer.render_to_canvas()

    // log time
    const time_elapsed = performance.now() - date_start
    console.log("Raytraced the scene in", time_elapsed.toFixed(2), "ms!")
  } catch (e) {
    console.error("Error rendering scene:", e)
  }
})()
