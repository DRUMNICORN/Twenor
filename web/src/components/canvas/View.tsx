'use client'

import { forwardRef, Suspense, useImperativeHandle, useRef } from 'react'
import { OrbitControls, PerspectiveCamera, View as ViewImpl } from '@react-three/drei'
import { Three } from '@/components/canvas/Three'

export const Common = ({ color }) => (
  <Suspense fallback={null}>
    {color && <color attach='background' args={[color]} />}
    {/* <ambientLight intensity={0.5} /> */}
    {/* <pointLight position={[20, 30, 10]} intensity={1} />
    <pointLight position={[-10, -10, -10]} color="red" /> */}
    <PerspectiveCamera makeDefault fov={40} position={[0, 0, 6]} view={undefined} visible={undefined} type={undefined} id={undefined} uuid={undefined} name={undefined} parent={undefined} modelViewMatrix={undefined} normalMatrix={undefined} matrixWorld={undefined} matrixAutoUpdate={undefined} matrixWorldAutoUpdate={undefined} matrixWorldNeedsUpdate={undefined} castShadow={undefined} receiveShadow={undefined} frustumCulled={undefined} renderOrder={undefined} animations={undefined} userData={undefined} customDepthMaterial={undefined} customDistanceMaterial={undefined} isObject3D={undefined} onBeforeRender={undefined} onAfterRender={undefined} applyMatrix4={undefined} applyQuaternion={undefined} setRotationFromAxisAngle={undefined} setRotationFromEuler={undefined} setRotationFromMatrix={undefined} setRotationFromQuaternion={undefined} rotateOnAxis={undefined} rotateOnWorldAxis={undefined} rotateX={undefined} rotateY={undefined} rotateZ={undefined} translateOnAxis={undefined} translateX={undefined} translateY={undefined} translateZ={undefined} localToWorld={undefined} worldToLocal={undefined} lookAt={undefined} add={undefined} remove={undefined} removeFromParent={undefined} clear={undefined} getObjectById={undefined} getObjectByName={undefined} getObjectByProperty={undefined} getObjectsByProperty={undefined} getWorldPosition={undefined} getWorldQuaternion={undefined} getWorldScale={undefined} getWorldDirection={undefined} raycast={undefined} traverse={undefined} traverseVisible={undefined} traverseAncestors={undefined} updateMatrix={undefined} updateMatrixWorld={undefined} updateWorldMatrix={undefined} toJSON={undefined} clone={undefined} copy={undefined} addEventListener={undefined} hasEventListener={undefined} removeEventListener={undefined} dispatchEvent={undefined} zoom={undefined} matrixWorldInverse={undefined} projectionMatrix={undefined} projectionMatrixInverse={undefined} isCamera={undefined} near={undefined} far={undefined} isPerspectiveCamera={undefined} aspect={undefined} focus={undefined} filmGauge={undefined} filmOffset={undefined} setFocalLength={undefined} getFocalLength={undefined} getEffectiveFOV={undefined} getFilmWidth={undefined} getFilmHeight={undefined} setViewOffset={undefined} clearViewOffset={undefined} updateProjectionMatrix={undefined} setLens={undefined} key={undefined} quaternion={undefined} attach={undefined} args={undefined} onUpdate={undefined} up={undefined} scale={undefined} rotation={undefined} matrix={undefined} layers={undefined} dispose={undefined} onClick={undefined} onContextMenu={undefined} onDoubleClick={undefined} onPointerUp={undefined} onPointerDown={undefined} onPointerOver={undefined} onPointerOut={undefined} onPointerEnter={undefined} onPointerLeave={undefined} onPointerMove={undefined} onPointerMissed={undefined} onPointerCancel={undefined} onWheel={undefined} />
  </Suspense>
)

type ViewProps = {
  children: React.ReactNode;
  orbit: boolean;
};


const View = forwardRef(({ children, orbit }: ViewProps, ref) => {
  const controls = useRef(null)
  const view = useRef(null)

  useImperativeHandle(ref, () => ({
    get controls() {
      return controls.current
    },
    get view() {
      return view.current
    }
  }))

  return (
    <Three>
      <ViewImpl ref={view}>
        {children}
        {orbit && <OrbitControls ref={controls} />}
      </ViewImpl>
    </Three>
  )
}

export { View }
