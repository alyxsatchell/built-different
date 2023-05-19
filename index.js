import wasmInit, {Space} from "./pkg/built_different.js";

const runWasm = async () => {

	const rustWasm = await wasmInit("./pkg/built_different_bg.wasm");

	const canvasElement = document.querySelector("canvas");

	const canvasContext = canvasElement.getContext("2d");
	const canvasImageData = canvasContext.createImageData(
		canvasElement.width,
		canvasElement.height
	);
  	canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);
  	let space = Space.new();
    let acceleration_information = {"x0" : 0, "x1" : 0, "y1" : 0, "y0": 0};
    let acceleration_information_default = {"x0" : 0, "x1" : 0, "y1" : 0, "y0": 0};
    window.addEventListener("keydown", function (event) {
      if (event.defaultPrevented) {
        return; // Do nothing if the event was already processed
      }
    
      switch (event.key) {

        case "ArrowDown":
          // code for "down arrow" key press.
          //space.accelerate(0, 0, 1);
          acceleration_information["y0"] += 1;
          break;
        case "ArrowUp":
          //space.accelerate(0, 0, -1);
          // code for "up arrow" key press.
          acceleration_information["y0"] -= 1;
          break;
        case "ArrowLeft":
          //space.accelerate(0, -1, 0);
          // code for "left arrow" key press.
          acceleration_information["x0"] -= 1;
          break;
        case "ArrowRight":
          acceleration_information["x0"] += 1;
          //space.accelerate(0, 1, 0);
          // console.log("Right")
          // code for "right arrow" key press.
          break;
        case "w":
          //space.accelerate(1,0,-1);
          break;
        case "a":
          //space.accelerate(1,-1,0)
          break;
        case "s":
          //space.accelerate(1,0,1);
          break;
        case "d":
          //space.accelerate(1,1,0);
          break;
        default:
          return; // Quit when this doesn't handle the key event.
      }
      // Cancel the default action to avoid it being handled twice
      event.preventDefault();
    }, true);

  	const tick = () => {

    	// rustWasm.my_init_function()
    	// space.tick(acceleration_information["x0"],acceleration_information["x1"],acceleration_information["y0"],acceleration_information["y1"]);
      space.tick(0,0,1,0);
      console.log(acceleration_information);
      acceleration_information = {"x0": 0, "y0" : 0, "x1": 0, "y1": 0};
    	const canvasPtr = space.get_canvas();
    	const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer, canvasPtr, 100 * 100 * 4);
    	canvasImageData.data.set(wasmByteMemoryArray);
    	canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);
    	canvasContext.putImageData(canvasImageData, 0, 0);
  	};
  	setInterval(() =>{
    	tick();
      console.log("tick tock")
  	}, 1000)
};
runWasm();