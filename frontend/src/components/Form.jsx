import { useState } from 'react'
import api from '../api'
import { useNavigate } from 'react-router-dom';
import { ACCESS_TOKEN } from '../constants';

function CredentialsForm({route, method}) {
	const [loading, setLoading] = useState(false);
	const navigate = useNavigate();

	const name = method === "login" ? "Login" : "Register";

	const handleSubmit = async (e) => {
		setLoading(true);
		e.preventDefault();

		const username = e.currentTarget.elements.namedItem("username").value;
		const email = e.currentTarget.elements.namedItem("email").value;
		const password = e.currentTarget.elements.namedItem("password").value;

		try {
			const res = await api.post(route, {username, password})
			if (method == "login") {
				localStorage.setItem(ACCESS_TOKEN, res.data.token);
				navigate("/");
			} else {
				navigate("/login");
			}

		} catch (error) {
			alert(error)
		} finally {
			setLoading(false);
		}
	}


	return <form onSubmit={handleSubmit} className='form-container'>
		<h1>{name}</h1>
		
		<input
			className='form-input'
			type='text'
			name='username'
			placeholder='Username'
		/>
		<input
			className='form-input'
			type='password'
			name='password'
			placeholder='Password'
		/>
		<button className='form-button' type="submit">
			{name}
		</button>

	</form>
}

export default Form;