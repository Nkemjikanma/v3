import { createFileRoute, Outlet, redirect, Link } from '@tanstack/react-router'

export const Route = createFileRoute("/admin/")({
  beforeLoad: ({ context }) => {
    if (!context.isAuthenticated) {
      throw redirect({ to: '/admin/login' })
    }
  },
  component: AdminLayout,
})

function AdminLayout() {
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-outfitSemiBold">Admin</h1>
        <nav className="flex gap-4 text-sm">
          <Link
            to="/admin/books"
            className="hover:underline"
            activeProps={{ className: "font-outfitBold underline" }}
          >
            Books
          </Link>
          <Link
            to="/admin/songs"
            className="hover:underline"
            activeProps={{ className: "font-outfitBold underline" }}
          >
            Songs
          </Link>
        </nav>
      </div>
      <Outlet />
    </div>
  )
}
