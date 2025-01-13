"use client";
import React, {useState, useEffect, useActionState} from "react";
import { useForm, SubmitHandler} from "react-hook-form";


interface SignUpForm {
	email: string,
	username: string,
	password: string,
	confirmPassword: string,
}

const SignUp: React.FC = () => {
	const {
		register,
		handleSubmit,
		watch,
		formState: { errors },
	} = useForm<SignUpForm>();

	const onSubmit: SubmitHandler<SignUpForm> = async (data) => {
		console.log("Data submitted: ", data);

		try {
			const response = await fetch("http://localhost:8080/user/create", {
				body: JSON.stringify({
					username: data.username,
					email: data.email,
					password: data.password
				})
			});

			if (!response.ok) {
				throw new Error("" + response.text)
			}
		} catch (e) {
			alert(e)
		}
	};

	const password = watch("password");

	return (
		<div className="flex justify-center items-center min-h-screen bg-gray-100">
			<form
				onSubmit={handleSubmit(onSubmit)}
				className="w-full max-w-md bg-white rounded-lg shadow-md p-6 space-y-4"
			>

			<h1 className="text-2xl font-bold text-gray-800 text-center">Sign Up</h1>
			
				{/* Email */}
				<div>
					<label htmlFor="email" className="block text-sm font-medium text-gray-700">
						Email:
					</label>
					<input
						id="email"
						type="email"
						className={`mt-1 w-full p-2 border ${
							errors.email ? "border-red-500" : "border-gray-300"
						} rounded-md focus:ring-blue-500 focus:border-blue-500`}
						{...register("email", {
							required: "Email is required",
							pattern: {
								value: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
								message: "Invalid email address",
							},
						})}
					/>
					{errors.email && <p className="text-sm text-red-500 mt-1">{errors.email.message}</p>}
				</div>

				{/* Password */}
				<div>
					<label htmlFor="password" className="block text-sm font-medium text-gray-700">Password:</label>
					<input
						id="password"
						type="password"
						className={`mt-1 w-full p-2 border ${
							errors.password ? "border-red-500" : "border-gray-300"
						} rounded-md focus:ring-blue-500 focus:border-blue-500`}		  
						{...register("password", {
							required: "Password is required",
							minLength: {
								value: 6,
								message: "Password must be at lest 6 characters",
							},
						})}
					/>
					{errors.password && <p className="text-sm text-red-500 mt-1">{errors.password.message}</p>}
				</div>


				{/* Confirm Password */}
				<div>
					<label htmlFor="confirmPassword" className="block text-sm font-medium text-gray-700">
						Password:
					</label>
					<input
						id="confirmPassword"
						type="password"
						className={`mt-1 w-full p-2 border ${
							errors.confirmPassword ? "border-red-500" : "border-gray-300"
						} rounded-md focus:ring-blue-500 focus:border-blue-500`}
						{...register("confirmPassword", {
							required: "Please confirm your password",
							validate: (value) => 
								value === password || "Passwords do not match",
						})}
					/>
					{errors.confirmPassword && <p className="text-sm text-red-500 mt-1">
						{errors.confirmPassword.message}
					</p>}
				</div>

				{/* Submit Button */}
				<button type="submit" className="w-full py-2 px-4 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500">
					Sign Up
				</button>
			</form>
		</div>
	)

};

export default SignUp;
