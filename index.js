import wasmInit, { Octocat } from "./pkg/homepage.js"

const runWasm = async () => {
    const rustWasm = await wasmInit("./pkg/homepage_bg.wasm");

    const octocat = Octocat.new();
     
    const octocatCanvas = document.getElementById("octocatCanvas");
    octocatCanvas.height = octocat.height();
    octocatCanvas.width = octocat.width();
    const octocatCtx = octocatCanvas.getContext("2d");
    const octocatImageData = octocatCtx.createImageData(
        octocatCanvas.width,
        octocatCanvas.height
    );
    
    const drawOctocat = () => {
        octocat.gen(); 
        
        const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer);
        const outputPointer = octocat.get_output_ptr();
        
        const imageDataArray = wasmByteMemoryArray.slice(
            outputPointer, 
            outputPointer + octocat.width() * octocat.height() * 4
        );

        octocatImageData.data.set(imageDataArray);
        octocatCtx.clearRect(0, 0, octocatCanvas.width, octocatCanvas.height);

        octocatCtx.putImageData(octocatImageData, 0, 0);
    };
    
    drawOctocat();
    let start = undefined;
    const renderLoop = (timestamp) => {
        if (start === undefined) {
            start = timestamp;
        }
        
        const elapsed = timestamp - start;

        // Stop the animation after 2 seconds
        if (elapsed > 2000) { 
            drawOctocat();

            start = undefined;
            requestAnimationFrame(renderLoop);
        } 
        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
};

runWasm();
