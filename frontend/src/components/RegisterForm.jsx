import { useState } from 'react'
import api from '../api'
import { useNavigate } from 'react-router-dom';
import { ACCESS_TOKEN } from '../constants';
import '../styles/Form.css'

function RegisterForm({route, method}) {
	const [loading, setLoading] = useState(false);
	const navigate = useNavigate();

	const handleSubmit = async (e) => {
		setLoading(true);
		e.preventDefault();

		const name = e.currentTarget.elements.namedItem("name").value;
		const email = e.currentTarget.elements.namedItem("email").value;
		const password = e.currentTarget.elements.namedItem("password").value;
		const passwordConfirm = e.currentTarget.elements.namedItem("password").value;

		try {
			const res = await api.post(route, {name, email, password, passwordConfirm})

			navigate("/login");

		} catch (error) {
			alert(error.response.data.message)
		} finally {
			setLoading(false);
		}
	}


	return <form onSubmit={handleSubmit} className='form-container'>
		<h1>Register</h1>
		
		<input
			className='form-input'
			type='text'
			name='name'
			placeholder='Username'
		/>
		<input
			className='form-input'
			type='email'
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
			Register
		</button>

	</form>
}

export default RegisterForm;