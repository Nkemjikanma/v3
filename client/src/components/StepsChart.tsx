import { Tooltip, Bar, ResponsiveContainer, BarChart, XAxis, YAxis } from "recharts"
import { useGetSteps } from "../lib/hooks/useSteps.ts"

export default function StepsChart() {
  const { data: steps, isLoading } = useGetSteps()


  if (isLoading) {
    return <div className="h-48 w-full bg-gray-200 animate-pulse rounded" />;
  }

  if (!steps || steps.length === 0) {
    return <p className="text-gray-400 text-sm">No steps recorded yet</p>
  }

  return (
    <ResponsiveContainer width="100%" height={200}>
      <BarChart data={steps}>
        <XAxis dataKey="date" tick={{ fontSize: 11 }} tickFormatter={(d) => new Date(d).toLocaleDateString("en-GB", { day: "numeric", month: "short" })} />
        <YAxis tick={{ fontSize: 11 }} width={40} />
        <Tooltip
          labelFormatter={(d) => new Date(d).toLocaleDateString("en-GB", { day: "numeric", month: "short", year: "numeric" })}
          formatter={(value: any) => [value.toLocaleString(), "Steps"]}
        />
        <Bar dataKey="step_count" fill="#1f2937" radius={[3, 3, 0, 0]} barSize={6} />
      </BarChart>


    </ResponsiveContainer>
  )
}
