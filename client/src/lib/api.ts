import { APIResponse } from "./types";
const API_URL = import.meta.env.VITE_API_URL;

async function request<T>(
  endpoint: string,
  token?: string,
  options?: RequestInit,
): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const response = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    headers: {
      ...headers,
      ...options?.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json();

    throw new Error(error.error || "Something went wrong");
  }

  const data: APIResponse<T> = await response.json();

  return data.response_data;
}

export async function get<T>(endpoint: string): Promise<T> {
  return request<T>(endpoint);
}

// TODO: Set type of body
export async function post<T>(
  endpoint: string,
  token: string,
  body: any,
): Promise<T> {
  return request<T>(endpoint, token, {
    method: "POST",
    body: JSON.stringify(body),
  });
}

export async function patch<T>(
  endpoint: string,
  token: string,
  body: unknown,
): Promise<T> {
  return request<T>(endpoint, token, {
    method: "PATCH",
    body: JSON.stringify(body),
  });
}

export async function del<T>(endpoint: string, token: string): Promise<T> {
  return request<T>(endpoint, token, { method: "DELETE" });
}
