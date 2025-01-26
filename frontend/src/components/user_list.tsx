import axios from 'axios';
import React, { useEffect, useState } from 'react'

interface IUser {
	user_id: number,
	username: string,
	email: string,
	sold: number,
}

function initialState(): IUser[] {
	return []
}

function Profile(user: IUser) {
	const [details, setDetails] = useState(<></>);
	const [shown, setShown] = useState(false);

	const ShowDetails = () => {
		if (!shown) {
			setShown(true);

			setDetails(
				<>
				<p className='user-profile-details'>{user.email}</p>
				<p className='user-profile-details'>sold: {user.sold}</p>
				</>
			);

		} else {
			setShown(false);
			setDetails(<></>);
		}
	}

	return (
		<li className='user-profile'>
			<h3 className='user-profile-username'>{user.username}</h3>
			{details}
			<button onClick={ShowDetails}>Details</button>
		</li>
	)
}

export const Users = () =>{
	
	const [users, setUsers] = useState(() => initialState());

	useEffect(() =>  {
		fetch('http://localhost:8080/users/')
			.then((data) => data.json())
			.then((json: IUser[]) => {
				// const data = JSON.stringify(json);
				setUsers(json);
				console.log("data received: ", json);
			})
	}, [])

	return (
		<>
			<ul className='user-list'>
				{/* <div className='user-list-header'>
					<span><strong>username</strong></span>
					<span>email</span>
					<span>sold</span>
				</div> */}
				{users.map((user) => <Profile {...user} />)}
			</ul>
		</>
	)
}
