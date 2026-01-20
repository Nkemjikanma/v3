import { getCurrentlyReadingBooks } from "../utils"
import { useQuery } from "@tanstack/react-query";

export const useGetReads = () => {
	return useQuery({
		queryKey: ["get-current-reads"],
		queryFn: async () => { return await getCurrentlyReadingBooks() },
	});
}
