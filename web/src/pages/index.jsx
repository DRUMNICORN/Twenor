import React from 'react'
import Simple from '../layout/Simple'
// import Wheel from "@/components/layout/Wheel";
import dynamic from 'next/dynamic'
import Ideas from '@/layout/Ideas';

const Intelligence = dynamic(() => import('@/view/Intelligence'), { ssr: true })

export default function Index() {
  const [page, setPage] = React.useState('simple')

  // add the canvas



  return <>
    {
      (page === 'simple') ? <Simple onClick={() => setPage('wheel')} /> : <Ideas />
    }
  </>
}

Index.canvas = (props) => <Intelligence />


export async function getStaticProps() {
  return { props: { title: 'Nice' } }
}