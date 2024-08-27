import { useRef, forwardRef, useImperativeHandle } from 'react'
import styles from '@/styles/Layout.module.scss'

const Layout = forwardRef(({ children, ...props }, ref) => {
  const localRef = useRef()

  useImperativeHandle(ref, () => localRef.current)

  return (
    <div className={styles.layout}
      {...props}
      ref={localRef}>
      {children}
    </div>
  )
})
Layout.displayName = 'Layout'

export default Layout
