import { Link, useLocation } from "react-router-dom";

import { ThemeToggle } from "@/components/theme/ThemeToggle";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { SidebarTrigger } from "@/components/ui/sidebar";
import { APP_NAME, findNavItem } from "@/config/navigation";

export function AppHeader() {
  const location = useLocation();
  const current = findNavItem(location.pathname);

  return (
    <header className="bg-sidebar sticky top-0 z-20 flex h-14 shrink-0 items-center gap-3 border-b border-transparent px-3">
      <SidebarTrigger className="cursor-pointer" />
      <Breadcrumb className="min-w-0">
        <BreadcrumbList className="flex-nowrap overflow-hidden">
          <BreadcrumbItem>
            <BreadcrumbLink asChild>
              <Link to="/" className="cursor-pointer">
                {APP_NAME}
              </Link>
            </BreadcrumbLink>
          </BreadcrumbItem>
          {current ? (
            <>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbPage>{current.label}</BreadcrumbPage>
              </BreadcrumbItem>
            </>
          ) : null}
        </BreadcrumbList>
      </Breadcrumb>
      <div className="ml-auto">
        <ThemeToggle />
      </div>
    </header>
  );
}
