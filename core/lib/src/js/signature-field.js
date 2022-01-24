// MIT http://rem.mit-license.org


function trim(c) {
    let ctx = c.getContext('2d'),
        copy = document.createElement('canvas').getContext('2d'),
        pixels = ctx.getImageData(0, 0, c.width, c.height),
        l = pixels.data.length,
        i,
        bound = {
            top: null,
            left: null,
            right: null,
            bottom: null
        },
        x, y;

    for (i = 0; i < l; i += 4) {
        if (pixels.data[i + 3] !== 0) {
            x = (i / 4) % c.width;
            y = ~~((i / 4) / c.width);

            if (bound.top === null) {
                bound.top = y;
            }

            if (bound.left === null) {
                bound.left = x;
            } else if (x < bound.left) {
                bound.left = x;
            }

            if (bound.right === null) {
                bound.right = x;
            } else if (bound.right < x) {
                bound.right = x;
            }

            if (bound.bottom === null) {
                bound.bottom = y;
            } else if (bound.bottom < y) {
                bound.bottom = y;
            }
        }
    }

    let trimHeight = bound.bottom - bound.top,
        trimWidth = bound.right - bound.left,
        trimmed = ctx.getImageData(bound.left, bound.top, trimWidth, trimHeight);

    copy.canvas.width = trimWidth;
    copy.canvas.height = trimHeight;
    copy.putImageData(trimmed, 0, 0);

    // open new window with trimmed image:
    return copy.canvas;
}

window.addEventListener("load", () => {

    const $body = document.querySelector('body');

    const scrollLocker = {
        enable() {
            $body.style.overflow = 'hidden';
            $body.style.position = 'fixed';
            $body.style.top = `0px`;
            $body.style.bottom = `0px`;
            $body.style.width = '100%';
        },
        disable() {
            $body.style.removeProperty('overflow');
            $body.style.removeProperty('position');
            $body.style.removeProperty('top');
            $body.style.removeProperty('bottom');
            $body.style.removeProperty('width');
        }
    };

    document.querySelectorAll(`.signature-field__popup[data-attach-to]`)
        .forEach(signature_popup => {
            const popupId = signature_popup.getAttribute("data-attach-to");
            document.getElementById(popupId).addEventListener("click", e => {
                signature_popup.querySelector(".popup__window-content").requestFullscreen();
            });
        });
    document.querySelectorAll(".signature-field__submit")
        .forEach(signature_popup => {
            signature_popup.addEventListener("click", e => {
                document.exitFullscreen()
                    .then(() => console.log("Document Exited from Full screen mode"));
            });
        });

    document.querySelectorAll(".signature-field")
        .forEach((field) => {
            const id = field.getAttribute("data-signature-field-id");
            let canvas = document.getElementById(`signature-field-${id}__canvas`);
            if (canvas.parentElement) {
                canvas.parentElement.parentElement.parentElement.style.width = "100vw";
                canvas.parentElement.parentElement.parentElement.style.maxWidth = "100vw";
                canvas.parentElement.parentElement.parentElement.style.height = "100vh";
                canvas.parentElement.parentElement.parentElement.style.maxHeight = "100vh";
                canvas.parentElement.parentElement.parentElement.style.overflow = "none";
                canvas.parentElement.parentElement.parentElement.style.borderRadius = "0";
            }
            let input = document.getElementById(`signature-field-${id}__input`);
            let ctx = canvas.getContext("2d");

            const dpr = window.devicePixelRatio || 1;

            //const rect = canvas.getBoundingClientRect();
            // Give the canvas pixel dimensions of their CSS
            // size * the device pixel ratio.

            canvas.width = window.innerWidth * dpr;
            canvas.height = window.innerHeight * dpr;

            window.addEventListener("resize", () => {
                canvas.width = window.innerWidth * dpr;
                canvas.height = window.innerHeight * dpr;

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
                input.value = trim(canvas).toDataURL();
            }, false);

            canvas.addEventListener("mousemove", function (e) {
                mousePos = getMousePos(canvas, e);
            }, false);

            // Add touch event support for mobile
            canvas.addEventListener("touchstart", function (e) {
                scrollLocker.enable();
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
                scrollLocker.disable();
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
                ctx.strokeStyle = getComputedStyle(document.body)
                    .getPropertyValue('--color-text');
                if (drawing) {
                    ctx.lineWidth = 4 * dpr;
                    ctx.lineCap = "round";
                    ctx.lineJoin = "round";
                    ctx.imageSmoothingEnabled = true;
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