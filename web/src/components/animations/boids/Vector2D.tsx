export default class Vector2D {
    constructor(public x: number, public y: number) { }

    add(vector: Vector2D): Vector2D {
        return new Vector2D(this.x + vector.x, this.y + vector.y);
    }

    subtract(vector: Vector2D): Vector2D {
        return new Vector2D(this.x - vector.x, this.y - vector.y);
    }

    multiply(scalar: number): Vector2D {
        return new Vector2D(this.x * scalar, this.y * scalar);
    }

    divide(scalar: number): Vector2D {
        return new Vector2D(this.x / scalar, this.y / scalar);
    }

    magnitude(): number {
        return Math.sqrt(this.x * this.x + this.y * this.y);
    }

    normalize(): Vector2D {
        const mag = this.magnitude();
        return this.divide(mag);
    }

    limit(max: number): Vector2D {
        const mag = this.magnitude();
        if (mag > max) {
            return this.normalize().multiply(max);
        }
        return this;
    }

    setMagnitude(mag: number): Vector2D {
        return this.normalize().multiply(mag);
    }

    distance(vector: Vector2D): number {
        const dx = this.x - vector.x;
        const dy = this.y - vector.y;
        return Math.sqrt(dx * dx + dy * dy);
    }

    angle(): number {
        return Math.atan2(this.y, this.x);
    }
}   
