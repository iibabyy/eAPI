import React, { FormEvent } from 'react'
import { useFormStatus } from 'react-dom'

function SubmitButton() {

}

export const CreateUser = () => {

	const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();

		const username = (e.currentTarget.elements.namedItem("username") as HTMLInputElement).value;
		const email = (e.currentTarget.elements.namedItem("email") as HTMLInputElement).value;
		const password = (e.currentTarget.elements.namedItem("password") as HTMLInputElement).value;
		
		async function fetchData(username: string, email: string, password: string) {
			try {
				const response = await fetch(
					"http://localhost:8080/users/",
					{
						method: "POST",
						headers: {
							'Accept': 'application/json',
							'Content-Type': 'application/json',
						},
						body: JSON.stringify({
							"username": username,
							"email": email,
							"password": password,
						})
					},
				);

				if (!response.ok) {
					console.log(response)
					throw new Error(await response.text())
				}

				const user = await response.json();
				
				console.log("User created: ", user);
			} catch (error) {
				console.error(error);
			}
		}

		fetchData(username, email, password);
	}

	return (
		<div className='user-create'>
			<form onSubmit={onSubmit} className='user-create-form'>
				<label>
					Name
				</label>
				<input type='text' placeholder='enter your username' name="username" />
				<label>
					Email
				</label>
				<input type='text' placeholder='enter your email' name="email" />
				<label>
					Password
				</label>
				<input type='text' placeholder='enter your password' name="password" />
				<button type="submit">Submit</button>
			</form>
		</div>
	)
}
