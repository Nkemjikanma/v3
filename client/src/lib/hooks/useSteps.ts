import { useQuery } from "@tanstack/react-query";
import { StepsQueryInfo, Steps } from "../types";
import { get } from "../api";

export function useGetSteps(filters?: StepsQueryInfo) {
  return useQuery({
    queryKey: ["steps", filters],
    queryFn: () => {
      const params = new URLSearchParams();

      if (filters?.from) params.set("from", filters.from);
      if (filters?.to) params.set("to", filters.to);

      const query = params.toString();

      return get<Steps[]>(`/steps${query ? `?${query}` : ""}`);
    },
  });
}
