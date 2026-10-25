import { Binary, Search, type LucideIcon } from "lucide-react";

export type NavItem = {
  id: string;
  path: string;
  label: string;
  icon: LucideIcon;
};

export type NavGroup = {
  id: string;
  label: string;
  items: NavItem[];
};

export const APP_NAME = "Hash Hunter";

export const NAV_GROUPS: NavGroup[] = [
  {
    id: "hunt",
    label: "探す",
    items: [
      { id: "hunt", path: "/", label: "探す", icon: Search },
      { id: "detectors", path: "/detectors", label: "検出器", icon: Binary },
    ],
  },
];

export function findNavItem(pathname: string): NavItem | undefined {
  return NAV_GROUPS.flatMap((group) => group.items).find(
    (item) => item.path === pathname,
  );
}
