window.addEventListener("load", () => {
    document.querySelectorAll(".signature-field")
        .forEach((field) => {
            let canvas = field.querySelector("canvas");
            let input = field.querySelector("input");
            let ctx = canvas.getContext("2d");


            const dpr = window.devicePixelRatio || 1;
            const rect = canvas.getBoundingClientRect();
            // Give the canvas pixel dimensions of their CSS
            // size * the device pixel ratio.
            canvas.width = rect.width * dpr;
            canvas.height = rect.height * dpr;

            ctx.lineWidth = 4 * dpr;
            ctx.lineCap = "round";
            ctx.lineJoin = "round";
            ctx.imageSmoothingEnabled = true;

            ctx.strokeStyle = getComputedStyle(document.body)
                .getPropertyValue('--color-text');

            console.log(ctx.strokeStyle);

            window.matchMedia("(prefers-color-scheme: dark)")
                .addEventListener("change", e => {
                    ctx.strokeStyle = getComputedStyle(document.body)
                        .getPropertyValue('--color-text');
                });

            let drawing = false;
            let mousePos = {
                x: 0,
                y: 0
            };
            let lastPos = mousePos;

            canvas.addEventListener("mousedown", function (e) {
                drawing = true;
                lastPos = getMousePos(canvas, e);
            }, false);

            canvas.addEventListener("mouseup", function (e) {
                drawing = false;
                input.value = canvas.toDataURL();
            }, false);

            canvas.addEventListener("mousemove", function (e) {
                mousePos = getMousePos(canvas, e);
            }, false);

            // Add touch event support for mobile
            canvas.addEventListener("touchstart", function (e) {

            }, false);

            canvas.addEventListener("touchmove", function (e) {
                let touch = e.touches[0];
                let me = new MouseEvent("mousemove", {
                    clientX: touch.clientX,
                    clientY: touch.clientY
                });
                canvas.dispatchEvent(me);
            }, false);

            canvas.addEventListener("touchstart", function (e) {
                mousePos = getTouchPos(canvas, e);
                let touch = e.touches[0];
                let me = new MouseEvent("mousedown", {
                    clientX: touch.clientX,
                    clientY: touch.clientY
                });
                canvas.dispatchEvent(me);
            }, false);

            canvas.addEventListener("touchend", function (e) {
                let me = new MouseEvent("mouseup", {});
                canvas.dispatchEvent(me);
            }, false);

            canvas.addEventListener("mouseleave", function (e) {
                let me = new MouseEvent("mouseup", {});
                canvas.dispatchEvent(me);
            }, false);

            function getMousePos(canvasDom, mouseEvent) {
                let rect = canvasDom.getBoundingClientRect();
                return {
                    x: (mouseEvent.clientX - rect.left) * dpr,
                    y: (mouseEvent.clientY - rect.top) * dpr
                }
            }

            function getTouchPos(canvasDom, touchEvent) {
                let rect = canvasDom.getBoundingClientRect();
                return {
                    x: (touchEvent.touches[0].clientX - rect.left) * dpr,
                    y: (touchEvent.touches[0].clientY - rect.top) * dpr
                }
            }

            function renderCanvas() {
                if (drawing) {
                    ctx.moveTo(lastPos.x, lastPos.y);
                    ctx.lineTo(mousePos.x, mousePos.y);
                    ctx.stroke();
                    lastPos = mousePos;
                }
            }

            // Prevent scrolling when touching the canvas
            document.body.addEventListener("touchstart", function (e) {
                if (e.target == canvas) {
                    e.preventDefault();
                }
            }, false);
            document.body.addEventListener("touchend", function (e) {
                if (e.target == canvas) {
                    e.preventDefault();
                }
            }, false);
            document.body.addEventListener("touchmove", function (e) {
                if (e.target == canvas) {
                    e.preventDefault();
                }
            }, false);

            function drawLoop() {
                requestAnimationFrame(drawLoop);
                renderCanvas();
            }

            drawLoop();
        });
});