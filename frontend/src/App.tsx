import React from "react";
import {
	BrowserRouter as Router,
	Route,
	Routes,
} from 'react-router-dom'

import Home from "./pages/home"
import NavBar from './components/navbar'

const App = () => {
	return (
		<>
			<section className="space"></section>
			<NavBar />
			<Routes>
				<Route path="/users" element={<Home />} />
			</ Routes>
			<section className="space"></section>
		</>
	)
}

export default App;