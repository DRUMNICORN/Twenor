"use client";

import Vector2D from "./Vector2D";

export default class Boid {
    private _position: Vector2D;
    private _velocity: Vector2D;
    private _acceleration: Vector2D;
    private _rotation: number = 0;

    maxSpeed = 1;
    maxForce = 0.5;
    size = 10;

    separationThreshold = 2;
    cohesionThreshold = 4;
    alignmentThreshold = 10;
    borderThreshold = 2;

    constructor(private canvasWidth: number, private canvasHeight: number) {
        this._position = new Vector2D(Math.random() * canvasWidth, Math.random() * canvasHeight);
        this._velocity = new Vector2D(Math.random() * 2 - 1, Math.random() * 2 - 1);
        this._acceleration = new Vector2D(0, 0);
        this._rotation = Math.random() * 360;
    }

    get position(): Vector2D {
        return this._position;
    }

    get rotation(): number {
        return this._rotation;
    }

    update = (boids: Boid[], deltaTime: number): void => {
        const separationForce = this.calculateForce(this._position, boids, this.separationThreshold, true);
        const alignmentForce = this.calculateForce(this._velocity, boids, this.alignmentThreshold);
        const cohesionForce = this.calculateForce(this._position, boids, this.cohesionThreshold);

        this.applyForce(separationForce, alignmentForce, cohesionForce);
        this.limitSpeed(deltaTime);
        // this.wrapEdges();
        this.stayAwayFromEdges();
        this._velocity = this._velocity.add(this._acceleration).limit(this.maxSpeed);
        this._position = this._position.add(this._velocity);
        this._acceleration = this._acceleration.multiply(0);
        this._rotation = this._velocity.angle();
    }

    wrapEdges = (): void => {
        const buffer = this.size * this.borderThreshold;
        const { x, y } = this._position;
        const { canvasWidth, canvasHeight } = this;

        if (x < -buffer) this._position.x = canvasWidth + buffer;
        else if (x > canvasWidth + buffer) this._position.x = -buffer;

        if (y < -buffer) this._position.y = canvasHeight + buffer;
        else if (y > canvasHeight + buffer) this._position.y = -buffer;
    }

    stayAwayFromEdges = (): void => {
        const buffer = this.size * this.borderThreshold;
        const { x, y } = this._position;
        const { canvasWidth, canvasHeight } = this;

        // Check top left corner
        if (x < buffer && y < buffer) {
            this._acceleration.x = this.maxSpeed * 0.5;
            this._acceleration.y = this.maxSpeed * 0.5;
        }
        // Check top right corner
        else if (x > canvasWidth - buffer && y < buffer) {
            this._acceleration.x = -this.maxSpeed * 0.5;
            this._acceleration.y = this.maxSpeed * 0.5;
        }
        // Check bottom left corner
        else if (x < buffer && y > canvasHeight - buffer) {
            this._acceleration.x = this.maxSpeed * 0.5;
            this._acceleration.y = -this.maxSpeed * 0.5;
        }
        // Check bottom right corner
        else if (x > canvasWidth - buffer && y > canvasHeight - buffer) {
            this._acceleration.x = -this.maxSpeed * 0.5;
            this._acceleration.y = -this.maxSpeed * 0.5;
        }
        // Check top edge
        else if (y < buffer) {
            this._acceleration.y = this.maxSpeed;
        }
        // Check bottom edge
        else if (y > canvasHeight - buffer) {
            this._acceleration.y = -this.maxSpeed;
        }
        // Check left edge
        else if (x < buffer) {
            this._acceleration.x = this.maxSpeed;
        }
        // Check right edge
        else if (x > canvasWidth - buffer) {
            this._acceleration.x = -this.maxSpeed;
        }
    }

    calculateForce = (target: Vector2D, boids: Boid[], threshold: number, normalize: boolean = false): Vector2D => {
        let force = new Vector2D(0, 0);
        let count = 0;

        for (const otherBoid of boids) {
            if (otherBoid !== this) {
                const distance = target.distance(otherBoid.position);

                if (distance < threshold) {
                    const direction = target.subtract(otherBoid.position);
                    force = force.add(normalize ? direction.normalize().divide(distance) : direction);
                    count++;
                }
            }
        }

        if (count > 0) {
            force = force.divide(count);
            force = force.setMagnitude(this.maxSpeed);
            force = force.subtract(normalize ? this._velocity : target).limit(this.maxForce);
        }

        return force;
    }

    applyForce = (...forces: Vector2D[]): void => {
        this._acceleration = forces.reduce((acc, force) => acc.add(force), this._acceleration);
    }

    limitSpeed = (deltaTime: number): void => {
        const adjustedMaxSpeed = this.maxSpeed / deltaTime;
        const speed = this._velocity.magnitude();

        if (speed > adjustedMaxSpeed) {
            this._velocity = this._velocity.setMagnitude(adjustedMaxSpeed);
        }
    }

}
