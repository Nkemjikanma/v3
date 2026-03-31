import { createContext, useState, useContext, ReactNode } from 'react';
import { useRouter } from "@tanstack/react-router"

type AuthContextType = {
	token: string | null;
	login: (token: string) => void;
	logout: () => void;
	isAuthenticated: boolean;
};

const AuthContext = createContext<AuthContextType | null>(null);

const AuthProvider = ({ children }: { children: ReactNode }) => {
	const router = useRouter()


	const [token, setToken] = useState<string | null>(null);

	const login = (authToken: string) => {
		setToken(authToken)
	};

	const logout = () => {
		setToken(null)
		router.invalidate()
	}

	const isAuthenticated = token !== null;

	return (
		<AuthContext.Provider value={{ token, login, logout, isAuthenticated }}> {children}</AuthContext.Provider>
	);
}

export const useAuth = () => {
	const context = useContext(AuthContext);

	if (!context) throw new Error("useAuth must be used within AuthProvider");

	return context;
}
export default AuthProvider; 
