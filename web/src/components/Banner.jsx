import Image from 'next/image'
import { useInView } from 'react-intersection-observer'
import { motion, useAnimation } from 'framer-motion'
import { useTheme } from 'next-themes'
import styles from '@/styles/Banner.module.scss'

export default function Banner() {
  const { theme } = useTheme()
  const controls = useAnimation()
  const [ref, inView] = useInView()

  if (inView) {
    controls.start('visible')
  }

  return (
    <div className={styles.container}>
      <motion.div
        ref={ref}
        animate={controls}
        initial='hidden'
        transition={{ duration: 0.5 }}
        variants={{
          visible: { opacity: 1, y: 0 },
          hidden: { opacity: 0, y: 50 },
        }}
      >
        <Image
          src='/img/artist/banner.png'
          alt='banner'
          width={1920}
          height={1080}
          layout='responsive'
          className={styles.banner}
        />
      </motion.div>
    </div>
  )
}
