import type { ReactNode } from "react";

type PageHeaderProps = {
  title: string;
  actions?: ReactNode;
  children?: ReactNode;
};

export function PageHeader({ title, actions, children }: PageHeaderProps) {
  return (
    <div className="border-border bg-card sticky top-14 z-10 border-b px-4 py-4 md:px-6">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <h1 className="text-foreground text-xl font-semibold">{title}</h1>
        {actions}
      </div>
      {children}
    </div>
  );
}
