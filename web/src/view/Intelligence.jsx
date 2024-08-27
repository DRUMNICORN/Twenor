import { useState, useEffect } from 'react'
import Network from './Network'
import Utils from './intelligence/Utils'

export default function Intelligence({ route, ...props }) {
  return (
    <group {...props}>
      <Network
        maxConnections={8}
        clusters={[
          {
            size: 8,
            name: 'input',
            offset: { x: 0, y: 0, z: 0 },
            volume_func: Utils.volume.SPHERE(8),
            color: 'red',
          },
          {
            size: 8,
            name: 'box',
            offset: { x: 0, y: -10, z: 0 },
            volume_func: Utils.volume.GRID(1, 1, 1, 1),
          },
        ]}
      />
    </group>
  )
}
