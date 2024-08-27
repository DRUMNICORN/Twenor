"use client";
// BoidsCanvas.tsx
import React, { useEffect, useRef } from "react";
import { BoidsSimulation } from "./BoidsSimulation";

const BoidsCanvas: React.FC = () => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const boidsSimulation = useRef<BoidsSimulation | null>(null);

    const animate = () => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const boidsSimulation = new BoidsSimulation(canvas, 10); // Adjust the number of boids here
        boidsSimulation.start();

        const render = () => {
            boidsSimulation.update(); // Update the simulation
            boidsSimulation.drawCtx(canvas.getContext("2d")!); // Draw the simulation

            requestAnimationFrame(render);
        };

        render();

        return () => {
            boidsSimulation.stop();
        };
    };

    useEffect(() => {
        animate();
    }, []);

    const render = () => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const context = canvas.getContext("2d");
        if (!context) return;

        const offscreenCanvas = document.createElement("canvas");
        offscreenCanvas.width = canvas.width;
        offscreenCanvas.height = canvas.height;
        const offscreenContext = offscreenCanvas.getContext("2d");
        if (!offscreenContext) return;

        if (!boidsSimulation) return;

        let unloackedBoidsSimulation = boidsSimulation.current;
        if (!unloackedBoidsSimulation) return;

        let boidsSimulationLocal = unloackedBoidsSimulation;

        boidsSimulationLocal.drawCtx(offscreenContext);

        // Copy the off-screen canvas onto the visible canvas
        context.clearRect(0, 0, canvas.width, canvas.height);
        context.drawImage(offscreenCanvas, 0, 0);

        requestAnimationFrame(render);
    };


    return (
        <div style={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            width: "80vw",
        }}>
            <canvas
                ref={canvasRef}
                width={window.innerWidth - 40} // Adjust the canvas width as needed
                height={400} // Adjust the canvas height as needed
                style={{
                    paddingLeft: "14vh",
                }}
            />
        </div>
    );
};

export default BoidsCanvas;
