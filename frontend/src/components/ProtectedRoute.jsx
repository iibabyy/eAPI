import { useState, useEffect } from "react";
import { Navigate } from "react-router-dom";
import { ACCESS_TOKEN } from "../constants";
import { jwtDecode } from "jwt-decode";
import api from "../api";


function ProtectedRoute({ children }) {
	const [isAuthorized, setIsAuthorized] = useState(null);
	
	useEffect(() => {
		
		try {
			auth() //.catch(() => setIsAuthorized(false))
		} catch (error) {
			console.log(error);
		}
	})
	
	const refreshToken = async () => {
		try {
			const res = await api.post("/api/auth/refresh");

			if (res.status === 200) {
				localStorage.setItem(ACCESS_TOKEN, res.data.token);
				setIsAuthorized(true);
			} else {
				setIsAuthorized(false);
			}

		} catch (error) {
			console.log(error);
			setIsAuthorized(false);
		}
	}

	const auth = async () => {
		const token = localStorage.getItem(ACCESS_TOKEN);
		if (!token) {
			setIsAuthorized(false);
			return ;
		}

		const decoded = jwtDecode(token);
		const token_expiration = decoded.exp;
		const now = Date.now() / 1000	// ms -> seconds

		if (token_expiration < now) {
			console.log("refreshing token")
			await refreshToken()
		} else {
			console.log("authorized")
			setIsAuthorized(true);
		}
	}



	if (isAuthorized === null) {
		return <div>Loading...</div>
	}

	return isAuthorized ? children : <Navigate to="/login" />;
}

export default ProtectedRoute;