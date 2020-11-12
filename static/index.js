import wasmInit, { Animation  } from "./pkg/wasm.js"

const runWasm = async () => {
    const rustWasm = await wasmInit("./pkg/wasm_bg.wasm");

    const draw = ({ instance, href }) => {
        const currentUL = document.getElementById("social-links");

        const li = document.createElement("li");
        const a = document.createElement("a");
        a.setAttribute("href", href);
        a.setAttribute("target", "_blank");

        const canvas = document.createElement("canvas");
        canvas.height = instance.height();
        canvas.width = instance.width();

        a.appendChild(canvas);
        li.appendChild(a);
        currentUL.appendChild(li);
        
        const ctx = canvas.getContext("2d");
        const imageData = ctx.createImageData(
                canvas.width,
                canvas.height
        );
        return () => {
                instance.gen();
                const wasmByteMemoryArray = new Uint8Array(
                        rustWasm.memory.buffer
                );
                const outputPointer = instance.get_output_ptr();
                const imageDataArray = wasmByteMemoryArray.slice(
                        outputPointer,
                        outputPointer + instance.width() * instance.height() * 4
                );
                imageData.data.set(imageDataArray);
                ctx.clearRect(
                        0,
                        0,
                        canvas.width,
                        canvas.height
                );
                ctx.putImageData(imageData, 0, 0);
        };
    };

    const octocat = Animation.new("octocat");
    const email = Animation.new("email");
    const linkedin = Animation.new("linkedin");
     
    const drawFuncs = [
        { instance: octocat, href: "https://github.com/tommady" },
        { instance: email, href: "mailto:lindroos.hsu@tommady.com" },
        { instance: linkedin, href: "https://www.linkedin.com/in/lindrooshsu" }
    ].map(draw);
    
    drawFuncs.forEach(function(func){
        func();
    });
    let start = undefined;

    const renderLoop = (timestamp) => {
        if (start === undefined) {
            start = timestamp;
        }
        
        const elapsed = timestamp - start;

        // Stop the animation after 2 seconds
        if (elapsed > 2000) { 
            drawFuncs.forEach(function(func){
                func();
            });

            start = undefined;
            requestAnimationFrame(renderLoop);
        } 
        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
};

runWasm();
