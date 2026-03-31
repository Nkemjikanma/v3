import { ReactNode, useEffect } from "react"
import { useNavigate, } from "@tanstack/react-router"
import { useAuth } from "../lib/context/AuthContext"

function AdminGuard({ children }: { children: ReactNode }) {
  const { isAuthenticated } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (!isAuthenticated) navigate({ to: '/admin/login' });
  }, [isAuthenticated]);

  if (!isAuthenticated) return null;
  return <>{children}</>;
}
