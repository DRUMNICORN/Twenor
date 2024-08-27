import { v4 as uuid } from 'uuid';
const Utils = {
  exponentialApproach: (current, target, rate) => {
    if (Math.abs(target - current) < 0.01) {
      return target;
    }
    return current + (target - current) * rate;
  },
  weightToWidth: (weight) => {
    return Math.pow(weight, 2) * 0.1;
  },
  weightToColor: (weight) => {
    return weight > 0 ? 'green' : 'red';
  },
  map: (value, min1, max1, min2, max2) => {
    return (value - min1) * (max2 - min2) / (max1 - min1) + min2;
  },
  minmax: (value, min1, max1, min2, max2) => {
    return (value - min1) * (max2 - min2) / (max1 - min1) + min2;
  },
  uuidv4: () => {
    return uuid();
  },
  distance: (a, b) => {
    return Math.pow((
      Math.pow((a.x - b.x), 2) +
      Math.pow((a.y - b.y), 2) +
      Math.pow((a.z - b.z), 2)), 0.5);
  },
  volume: {
    GRID: (a = 1, b = 1, c = 1, grid = 1) => {
      return () => {
        // grid gibt die genauigkeit an wenn 0.3 errechnet wird, dann entsprit es bei einen 0.5 grid auch 0.5, 
        // wenn 0.3 errechnet wird, dann entsprit es bei einen 0.1 grid auch 0.3
        return {
          x: (Math.floor(Math.random() * a * 2 / grid) * grid - a),
          y: (Math.floor(Math.random() * b * 2 / grid) * grid - b),
          z: (Math.floor(Math.random() * c * 2 / grid) * grid - c)
        };
      }
    },
    CUBE: (a = 1, b = 1, c = 1) => {
      return () => {
        return {
          x: (Math.random() * a * 2 - a),
          y: (Math.random() * b * 2 - b),
          z: (Math.random() * c * 2 - c)
        };
      }
    },
    SPHERE: (diameter = 1) => {
      return () => {
        let d, x, y, z;
        do {
          x = (Math.random() * 2.0 - 1.0);
          y = (Math.random() * 2.0 - 1.0);
          z = (Math.random() * 2.0 - 1.0);
          d = x * x + y * y + z * z;
        } while (d > 1.0);

        x *= diameter;
        y *= diameter;
        z *= diameter;
        return {
          x: x,
          y: y,
          z: z
        };
      }
    },
  },
  activation: {
    SIGMOID: (t) => {
      return 1 / (1 + Math.pow(Math.E, -t))
    }
  },
  connection: {
    // each connection will be validated by a function
    // the params fro the validation function are, position of the neuron a and b, and the distance between them (d), index of the neuron a and b (i, j) 
    // the function should return true if the connection is valid, false otherwise
    ALL: () => {
      return (a, b, d, i, j) => {
        return true;
      }
    },
    OUTPUT: () => {
      return (a, b, d, i, j) => {
        return i < j;
      }
    },
    INPUT: () => {
      return (a, b, d, i, j) => {
        return i > j;
      }
    },
    NONE: () => {
      return (a, b, d, i, j) => {
        return false;
      }
    },
    // this function will create a connection between the neurons a and b if the distance between them is less than the threshold
    DISTANCE: (threshold = 1) => {
      return (a, b, d, i, j) => {
        return d < threshold;
      }
    },
    // this function will create a connection between the neurons a and b if the distance between them is less than the threshold
    RAND_COUNT: (count = 1) => {
      return (a, b, d, i, j) => {
        // check if i and j are less than the count
        if (i < count) {
          // if so, return true
          return true;
        }
        // if not, return false
        return false;
      }
    }
  }
}

export default Utils;