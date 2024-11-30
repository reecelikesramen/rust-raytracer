let raytracer = null

// "ready" if ready for more work, "done" if no more work to claim
self.onmessage = function (e) {
  // check if the message is an array
  if (!Array.isArray(e.data)) {
    return
  }

  if (e.data[0] == "raytracer") {
    raytracer = e.data[1]
    self.postMessage("ready")
  } else if (e.data[0] == "render") {
    raytrace().then(() => self.postMessage("done"))
  }
}

async function raytrace() {
  while (true) {
    let response = await raytracer()
    if (typeof response == "string" && response == "no columns") {
      break
    }
  }
}
