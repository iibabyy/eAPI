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
				<div className='user-profile-details'>
					<p>{user.email}</p>
					<p>sold: {user.sold}</p>
				</div>
			);

		} else {
			setShown(false);
			setDetails(<></>);
		}
	}

	return (
		<div className='user-profile'>
			<a onClick={ShowDetails}>{user.username}</a>
			{details}
		</div>
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
			<div className='user-list'>
				{/* <div className='user-list-header'>
					<span><strong>username</strong></span>
					<span>email</span>
					<span>sold</span>
				</div> */}
				{users.map((user) => <Profile {...user} />)}
			</div>
		</>
	)
}
