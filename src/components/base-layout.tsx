import { Outlet } from "react-router";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

import PlayerBar from "@/components/player-bar";
import Sidebar from "@/components/sidebar";

const BaseLayout = () => {
  return (
    <div className="flex flex-col h-screen">
      <ResizablePanelGroup direction="horizontal" className="flex-1">
        <ResizablePanel defaultSize={20} minSize={10} maxSize={50}>
          <Sidebar />
        </ResizablePanel>
        <ResizableHandle />
        <ResizablePanel defaultSize={80}>
          <Outlet />
        </ResizablePanel>
      </ResizablePanelGroup>
      <PlayerBar />
    </div>
  );
}

export default BaseLayout;