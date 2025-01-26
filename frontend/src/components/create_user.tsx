import React, { FormEvent } from 'react'
import { useFormStatus } from 'react-dom'
import Input from './input'

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
					throw new Error(await response.text())
				}

				const user = await response.json();
				
				console.log("User created: ", user);
			} catch (error) {
				console.log(error)
				console.error(error);
			}
		}

		fetchData(username, email, password);
	}

	return (
		<div className='user-create'>
			<form onSubmit={onSubmit} className='user-create-form'>
				<Input name='username' label='Username' type='text' id="1" placeholder='enter your name' />
				<Input name='email' label='Email' type='email' id="2" placeholder='enter your email' />
				<Input name='password' label='Password' type='password' id="3" placeholder='enter your password' />
				<button className='user-create-submit-button' type="submit">Submit</button>
			</form>
		</div>
	)
}
