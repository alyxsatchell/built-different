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
    console.log("bean")
  	let space = Space.new();
    console.log("bean2")
  	const tick = () => {
        window.addEventListener("keydown", function (event) {
            if (event.defaultPrevented) {
              return; // Do nothing if the event was already processed
            }
          
            switch (event.key) {

              case "ArrowDown":
                // code for "down arrow" key press.
                space.accelerate(0, 0, 1);
                break;
              case "ArrowUp":
                space.accelerate(0, 0, -1);
                // code for "up arrow" key press.
                break;
              case "ArrowLeft":
                space.accelerate(0, -1, 0);
                // code for "left arrow" key press.
                break;
              case "ArrowRight":
                space.accelerate(0, 1, 0);
                console.log("Right")
                // code for "right arrow" key press.
                break;
              case "w":
                space.accelerate(1,0,-1);
                break;
              case "a":
                space.accelerate(1,1,0)
                break;
              case "s":
                space.accelerate(1,0,1);
                break;
              case "d":
                space.accelerate(1,-1,0);
                console.log("d")
                break;
              default:
                // console.log(event.key)
                return; // Quit when this doesn't handle the key event.
            }
          
            // Cancel the default action to avoid it being handled twice
            event.preventDefault();
          }, true);

    	// rustWasm.my_init_function()
    	space.tick();
    	const canvasPtr = space.get_canvas();
    	const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer, canvasPtr, 100 * 100 * 4);
    	canvasImageData.data.set(wasmByteMemoryArray);
    	canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);
    	canvasContext.putImageData(canvasImageData, 0, 0);
  	};
  	setInterval(() =>{
    	tick();
  	}, 50)
};
runWasm();