import { useState } from 'react'
import api from '../api'
import { useNavigate } from 'react-router-dom';
import { ACCESS_TOKEN } from '../constants';
import '../styles/Form.css'

function LoginForm({route, method}) {
	const [loading, setLoading] = useState(false);
	const navigate = useNavigate();

	const handleSubmit = async (e) => {
		setLoading(true);
		e.preventDefault();

		const email = e.currentTarget.elements.namedItem("email").value;
		const password = e.currentTarget.elements.namedItem("password").value;

		try {
			const res = await api.post(route, {email, password})
			localStorage.setItem(ACCESS_TOKEN, res.data.token);
			console.log("redirecting");
			navigate("/");

		} catch (error) {
			alert(error.response.data.message)
		} finally {
			setLoading(false);
		}
	}


	return <form onSubmit={handleSubmit} className='form-container'>
		<h1>Login</h1>

		<input
			className='form-input'
			type='text'
			name='email'
			placeholder='Email'
		/>
		<input
			className='form-input'
			type='password'
			name='password'
			placeholder='Password'
		/>
		<button className='form-button' type="submit">
			Login
		</button>

	</form>
}

export default LoginForm;