"use client";

import Boid from "./Boid";

export class BoidController {
    private _boids: Boid[] = [];

    constructor(public canvasWidth: number, public canvasHeight: number, numBoids: number) {
        this._boids = Array.from({ length: numBoids }, () => new Boid(canvasWidth, canvasHeight));
    }

    get boids(): Boid[] {
        return this._boids;
    }

    updateAll = (deltaTime: number = 1): void => {
        for (const boid of this._boids) {
            boid.update(this._boids, deltaTime);
        }
    }
}

export class BoidsSimulation {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;
    boidController: BoidController;
    animationFrameId: number | null = null;
    running = false;
    lastFrameTime: number | null = null;

    constructor(canvas: HTMLCanvasElement, numBoids: number) {
        this.canvas = canvas;
        this.ctx = canvas.getContext("2d")!;
        this.boidController = new BoidController(canvas.width, canvas.height, numBoids);
    }

    start = (): void => {
        if (!this.running) {
            this.running = true;
            this.animate();
        }
    }

    stop = (): void => {
        if (this.running) {
            this.running = false;
            if (this.animationFrameId) {
                cancelAnimationFrame(this.animationFrameId);
            }
        }
    }

    animate = (): void => {
        if (!this.running) return;

        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        const now = performance.now();
        if (this.lastFrameTime === null) this.lastFrameTime = now;
        const deltaTime = (now - this.lastFrameTime) / 1000;
        this.lastFrameTime = now;

        this.update(deltaTime);
        this.drawCtx(this.ctx);

        this.animationFrameId = requestAnimationFrame(this.animate);
    }

    update = (deltaTime: number = 1): void => {
        this.boidController.updateAll(deltaTime);
    }


    draw = (): void => {
        for (const boid of this.boidController.boids) {
            this.drawBoid(boid);
        }
    }

    drawCtx = (ctx: CanvasRenderingContext2D): void => {
        for (const boid of this.boidController.boids) {
            this.drawBoidCtx(ctx, boid);
        }

    }

    drawBoid = (boid: Boid): void => {
        // const halfSize = boid.size / 2;

        // this.ctx.beginPath();
        // this.ctx.moveTo(boid.position.x, boid.position.y - halfSize);
        // this.ctx.lineTo(boid.position.x - halfSize, boid.position.y + halfSize);
        // this.ctx.lineTo(boid.position.x + halfSize, boid.position.y + halfSize);
        // this.ctx.closePath();

        // this.ctx.fillStyle = "#ffffff";
        // this.ctx.fill();
    }

    drawBoidCtx = (ctx: CanvasRenderingContext2D, boid: Boid): void => {
        const halfSize = boid.size / 2;

        let rotation = boid.rotation;
        if (rotation < 0) rotation += 2 * Math.PI;

        ctx.save();
        ctx.translate(boid.position.x, boid.position.y);
        ctx.rotate(rotation);
        ctx.translate(-boid.position.x, -boid.position.y);
        ctx.beginPath();
        ctx.moveTo(boid.position.x, boid.position.y - halfSize);
        ctx.lineTo(boid.position.x - halfSize, boid.position.y + halfSize);
        ctx.lineTo(boid.position.x + halfSize, boid.position.y + halfSize);
        ctx.closePath();

        ctx.fillStyle = "#ffffff";
        ctx.fill();
        ctx.restore();
    }
}