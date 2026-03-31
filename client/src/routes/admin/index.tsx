import { createFileRoute, Outlet, redirect } from '@tanstack/react-router'

export const Route = createFileRoute("/admin/")({
  beforeLoad: ({ context }) => {
    if (!context.isAuthenticated) {
      throw redirect({ to: '/admin/login' })
    }
  },
  component: AdminComponent,
})

function AdminComponent() {
  return (
    <div>
      <h1>Admin Page</h1>
      <Outlet /> {/* This is where child routes will render */}
    </div>
  )
}
