// about.jsx -> <Welcome>
import ChatPrompt from '@/layout/ChatPrompt';

export default function Page() {
  return (
    <ChatPrompt />
  );
}

import dynamic from 'next/dynamic'
const Globe = dynamic(() => import('@/view/Globe'), { ssr: true })

Page.canvas = (props) => <Globe />