import wasmInit, { Animation  } from "./pkg/homepage.js"

const runWasm = async () => {
    const rustWasm = await wasmInit("./pkg/homepage_bg.wasm");

    const octocat = Animation.new("octocat");
     
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

    const email = Animation.new("email");
     
    const emailCanvas = document.getElementById("emailCanvas");
    emailCanvas.height = email.height();
    emailCanvas.width = email.width();
    const emailCtx = emailCanvas.getContext("2d");
    const emailImageData = emailCtx.createImageData(
        emailCanvas.width,
        emailCanvas.height
    );
    
    const drawEmail = () => {
        email.gen(); 
        
        const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer);
        const outputPointer = email.get_output_ptr();
        
        const imageDataArray = wasmByteMemoryArray.slice(
            outputPointer, 
            outputPointer + email.width() * email.height() * 4
        );

        emailImageData.data.set(imageDataArray);
        emailCtx.clearRect(0, 0, emailCanvas.width, emailCanvas.height);

        emailCtx.putImageData(emailImageData, 0, 0);
    };

    const linkedin = Animation.new("linkedin");
     
    const linkedinCanvas = document.getElementById("linkedinCanvas");
    linkedinCanvas.height = linkedin.height();
    linkedinCanvas.width = linkedin.width();
    const linkedinCtx = linkedinCanvas.getContext("2d");
    const linkedinImageData = linkedinCtx.createImageData(
        linkedinCanvas.width,
        linkedinCanvas.height
    );
    
    const drawLinkedin = () => {
        linkedin.gen(); 
        
        const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer);
        const outputPointer = linkedin.get_output_ptr();
        
        const imageDataArray = wasmByteMemoryArray.slice(
            outputPointer, 
            outputPointer + linkedin.width() * linkedin.height() * 4
        );

        linkedinImageData.data.set(imageDataArray);
        linkedinCtx.clearRect(0, 0, linkedinCanvas.width, linkedinCanvas.height);

        linkedinCtx.putImageData(linkedinImageData, 0, 0);
    };
    
    drawOctocat();
    drawEmail(); 
    drawLinkedin();

    let start = undefined;
    const renderLoop = (timestamp) => {
        if (start === undefined) {
            start = timestamp;
        }
        
        const elapsed = timestamp - start;

        // Stop the animation after 2 seconds
        if (elapsed > 2000) { 
            drawOctocat();
            drawEmail();
            drawLinkedin();

            start = undefined;
            requestAnimationFrame(renderLoop);
        } 
        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
};

runWasm();
