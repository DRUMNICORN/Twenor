import { useState, useEffect } from 'react'
import Network from './Network'
import Utils from './intelligence/Utils'

export default function Globe({ route, ...props }) {
  return (
    <group {...props}>
      <Network
        maxConnections={5}
        clusters={[
          {
            size: 20,
            name: 'box',
            offset: { x: 0, y: 0, z: 0 },
            volume_func: Utils.volume.GRID(6, 6, 6, 2),
          },
        ]}
      />
    </group>
  )
}
