import { Library, Settings } from "lucide-react";

import SidebarMenuButton from "@/components/sidebar-menu-button";

const Sidebar = () => {
  return (
    <nav className="flex flex-col h-full w-full p-1.5">
      <SidebarMenuButton
        to="/library"
        text="Library"
        icon={<Library />}
      />
      <SidebarMenuButton
        to="/settings"
        text="Settings"
        icon={<Settings />}
      />
    </nav>
  );
}

export default Sidebar;