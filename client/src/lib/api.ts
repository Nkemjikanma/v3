import { APIResponse } from "./types";
const API_URL = import.meta.env.VITE_API_URL;

async function request<T>(endpoint: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_URL}${endpoint}`, {
    headers: {
      "Content-Type": "application/json",
    },
    ...options,
  });

  if (!response.ok) {
    const error = await response.json();

    throw new Error(error.error || "Soemthing went wrong");
  }

  const data: APIResponse<T> = await response.json();

  return data.response_data;
}

export async function get<T>(endpoint: string): Promise<T> {
  return request<T>(endpoint);
}

// TODO: Set type of body
export async function post<T>(endpoint: string, body: any): Promise<T> {
  return request<T>(endpoint, {
    method: "POST",
    body: JSON.stringify(body),
  });
}

export async function patch<T>(endpoint: string, body: unknown): Promise<T> {
  return request<T>(endpoint, {
    method: "PATCH",
    body: JSON.stringify(body),
  });
}

export async function del<T>(endpoint: string): Promise<T> {
  return request<T>(endpoint, { method: "DELETE" });
}
