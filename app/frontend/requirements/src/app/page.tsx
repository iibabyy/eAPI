"use client";
import React, {useState, useEffect} from "react";

interface User {
	id: number,
	email: string,
	username: string,
	sold: number,
}

export default function Home() {

	const [users, DeserializeUser] = useState<User[] | null>(null);
	const [error, setError] = useState<string | null>(null);

	const getUsers = async () => {
		try {
			const response = await fetch("http://localhost:8080/user/get_all", { method: "GET", });
			if (!response.ok) {
				throw new Error("failed to get users list: " + response.text)
			}

			const data = await response.json();
			// console.log("received :\n", data);
			let i = 32;

			DeserializeUser(data);

			console.log("serialized users:\n", users);
		} catch (e) {
			setError(e instanceof Error ? e.message : "Une erreur est survenu");
			console.log(error);
		}
	}

	useEffect(() => {
		getUsers();
	  }, []);

	return (
		<div>
		{
			users && (
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 p-6">
				{users.map((user, index) => (
					<div key={index}
					className="bg-gray-950 border-2 border-white rounded-[30px] shadow-lg p-6 hover:shadow-xl transition-shadow">
					<h2 className="text-xl font-semibold text-gray-200 mb-2">{user.username}</h2>
					<p className="text-gray-200">id : {user.id}</p>
					<p className="text-gray-200">{user.email}</p>
					<p className="text-gray-200">{user.sold}</p>
					{/* <button onClick={() => {
						router.push(`/posts?user=${user.username}`);
					}}>POSTS</button> */}
					</div>
				))}
				</div>
			)
		}
		</div>
	)
}