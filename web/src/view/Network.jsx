
import React, { useEffect, useMemo, useState } from 'react';
import Connection from './intelligence/Connection';
import Neuron from './intelligence/Neuron';

export default function Network({ clusters, maxConnections, ...props }) {
  const [connections, setConnections] = useState([]);
  const [neurons, setNeurons] = useState([]);

  const memoizedClusters = useMemo(() => clusters, [clusters]);

  const memoizedNeurons = useMemo(() => {
    let new_neurons = [];
    let tryAndError = 0;
    memoizedClusters.forEach((cluster, i) => {
      let cluster_id = cluster.id || i;
      for (let ii = 0; ii < cluster.size; ii++) {
        // get random position in volume function and add offset
        let position = cluster.volume_func();
        if (cluster.offset) {
          position.x += cluster.offset.x;
          position.y += cluster.offset.y;
          position.z += cluster.offset.z;
        }

        // check if neuron already exists
        let exists = false;
        new_neurons.forEach((neuron) => {
          if (neuron.x === position.x && neuron.y === position.y && neuron.z === position.z) {
            exists = true;
          }
        })


        if (exists) {
          ii--;
          tryAndError++;
          if (tryAndError > cluster.size * cluster.size) {
            break;
          }
          continue;
        }

        new_neurons.push({
          id: cluster_id,
          x: position.x,
          y: position.y,
          z: position.z,
        });
      }
    })
    return new_neurons;
  }, [memoizedClusters]);

  const memoizedConnections = useMemo(() => {
    let new_connections = [];
    let tryAndError = 0;
    memoizedNeurons.forEach((from, i) => {
      for (let ii = 0; ii < maxConnections; ii++) {
        let to = memoizedNeurons[Math.floor(Math.random() * memoizedNeurons.length)];
        // check if connection already exists

        if (from === to) {
          ii--;
          continue;
        }

        // check if neuron is in the same cluster
        if (from.id != to.id) {
          tryAndError++;
          if (tryAndError > maxConnections * maxConnections) {
            break;
          }
          continue;
        }

        let exists = false;
        new_connections.forEach((connection) => {
          if (connection.from === from && connection.to === to) {
            exists = true;
          }
        })

        if (exists) {
          ii--;
          continue;
        }

        new_connections.push({
          from: from,
          to: to,
        })
      }
    })
    return new_connections;
  }, [memoizedNeurons, maxConnections]);

  useEffect(() => {
    setNeurons(memoizedNeurons);
  }, [memoizedNeurons]);

  useEffect(() => {
    setConnections(memoizedConnections);
  }, [memoizedConnections]);

  return (
    <group {...props}>
      {memoizedConnections.map((connection, i) => <Connection from={connection.from} to={connection.to} key={'connection-' + i} />)}
      {memoizedNeurons.map((neuron, i) => {
        let position = {
          x: neuron.x || 0,
          y: neuron.y || 0,
          z: neuron.z || 0,
        }
        return <Neuron position={position} key={'neuron-' + i} />
      })}
    </group>
  );
}

// Path: src\components\canvas\intelligence\Network.jsx
