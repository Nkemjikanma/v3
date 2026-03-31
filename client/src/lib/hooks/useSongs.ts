import { useQuery, useQueryClient, useMutation } from "@tanstack/react-query";
import { useAuth } from "../context/AuthContext";
import {
  Song,
  SongFormData,
  SongQueryInfo,
  UpdateSongFormData,
} from "../types";

import { get, post, patch, del } from "../api";

export function useGetSongs(filters?: SongQueryInfo) {
  return useQuery({
    queryKey: ["songs", filters],
    queryFn: () => {
      const params = new URLSearchParams();

      if (filters?.instrument) params.set("instrument", filters.instrument);

      const query = params.toString();
      return get<Song[]>(`/songs${query ? `?${query}` : ""}`);
    },
  });
}

export function useGetSong(id: string) {
  return useQuery({
    queryKey: ["songs", id],
    queryFn: () => get<Song>(`/songs/${id}`),
  });
}

export function useAddSong() {
  const { token } = useAuth();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (song: SongFormData) => post<string>("/songs", token!, song),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["songs"] }),
  });
}

export function useUpdateSong(id: string) {
  const { token } = useAuth();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (song: UpdateSongFormData) =>
      patch<string>(`/songs/${id}`, token!, song),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["songs"] }),
  });
}

export function useDeleteSong(id: string) {
  const { token } = useAuth();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => del<string>(`/songs/${id}`, token!),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["songs"] }),
  });
}
