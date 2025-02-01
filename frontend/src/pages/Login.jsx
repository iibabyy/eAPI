import LoginForm from '../components/LoginForm'

function Login() {
	return <LoginForm route="/api/auth/login" method="login" />
}

export default Login;