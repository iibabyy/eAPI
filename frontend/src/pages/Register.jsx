import RegisterForm from "../components/RegisterForm";

function Register() {
	return <RegisterForm route="/api/auth/register" method="register" />
}

export default Register;