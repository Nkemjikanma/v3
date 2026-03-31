import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { useAuth } from "../context/AuthContext";
import { get, post, patch, del } from "../api";
import {
  Book,
  UpdateBookFormData,
  BookFormData,
  BookQueryInfo,
} from "../types";

export function useGetBooks(filters?: BookQueryInfo) {
  return useQuery({
    queryKey: ["books", filters],
    queryFn: () => {
      const params = new URLSearchParams();

      if (filters?.category) params.set("category", filters.category);
      if (filters?.status) params.set("status", filters.status);
      if (filters?.year_read)
        params.set("year_read", String(filters.year_read));

      const query = params.toString();
      return get<Book[]>(`/books${query ? `?${query}` : ""}`);
    },
  });
}

export function useGetBook(id: string) {
  return useQuery({
    queryKey: ["books", id],
    queryFn: () => get<Book>(`/books/${id}`),
  });
}

export function useAddBook() {
  const { token } = useAuth();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (book: BookFormData) => post<string>("/books", token!, book),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["books"] }),
  });
}

export function useUpdateBook() {
  const { token } = useAuth();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, book }: { id: string; book: UpdateBookFormData }) =>
      patch<string>(`/books/${id}`, token!, book),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["books"] }),
  });
}

export function useDeleteBook() {
  const { token } = useAuth();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => del<string>(`/books/${id}`, token!),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["books"] }),
  });
}
