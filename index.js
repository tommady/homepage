import wasmInit from "./pkg/homepage.js"

const runWasm = async () => {
    const rustWasm = await wasmInit("./pkg/homepage_bg.wasm");
    // const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer);

    const canvasElement = document.getElementById("octocatCanvas");
    const canvasContext = canvasElement.getContext("2d");
    const canvasImageData = canvasContext.createImageData(
        canvasElement.width,
        canvasElement.height
    );

    const drawCheckerBoard = () => {
        const width = 49;
        const height = 22;

        rustWasm.gen_octocat();
        
        const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer);
        const outputPointer = rustWasm.get_octocat_output_buffer_ptr();
        
        const imageDataArray = wasmByteMemoryArray.slice(
            outputPointer, 
            outputPointer + width * height * 4
        );

        canvasImageData.data.set(imageDataArray);
        canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

        canvasContext.putImageData(canvasImageData, 0, 0);
    };

    drawCheckerBoard();
    setInterval(() => {
        drawCheckerBoard();
    }, 1000);
};

runWasm();
