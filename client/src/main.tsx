import React from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import AuthProvider from "./lib/context/AuthContext.tsx"
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import './index.css'
import { useAuth } from "./lib/context/AuthContext"
// Import the generated route tree
import { routeTree } from './routeTree.gen'
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1
    }
  }
});

// Set up a Router instance
const router = createRouter({
  routeTree,
  context: {
    queryClient,
    isAuthenticated: undefined!
  },
  defaultPreload: "intent",
  scrollRestoration: true,
  defaultPreloadStaleTime: 0,
});

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

function InnerApp() {
  const { isAuthenticated } = useAuth();
  return (
    <>
      <RouterProvider router={router} context={{ queryClient, isAuthenticated }} />
      <TanStackRouterDevtools router={router} />
    </>
  );
}
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <AuthProvider>
        <InnerApp />
      </AuthProvider>
    </QueryClientProvider>
  </React.StrictMode>,
)
