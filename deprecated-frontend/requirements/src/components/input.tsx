import React from "react";
import { Placeholder } from "react-bootstrap";

interface IInputData {
	label: string | undefined,
	type: string,
	id: string | undefined,
	placeholder: string | undefined,
	name: string | undefined,
}

const Input = ({ label = undefined, type, id = undefined, placeholder = undefined, name = undefined }: IInputData) => {
	return (
		<>
		{/* <div className="Input"> */}
		<label className="Input" >{label}</label>
		<input
			name={name}
			type={type}
			id={id}
			placeholder={placeholder}
		/>		
		{/* </div> */}
		</>
	)
}

export default Input;