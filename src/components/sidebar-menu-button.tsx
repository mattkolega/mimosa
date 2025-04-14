import { Link, useLocation } from "react-router";

import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

type Props = {
  to: string;
  text: string;
  icon?: React.ReactNode
}

const SidebarMenuButton = (props: Props) => {
  const location = useLocation();
  const isActive = location.pathname === props.to;

  const btnClass = cn(
    "justify-start",
    isActive && "bg-accent"
  );

  return (
    <Button variant="ghost" className={btnClass} asChild>
      <Link to={props.to}>
        {props.icon} {props.text}
      </Link>
    </Button>
  );
}

export default SidebarMenuButton;