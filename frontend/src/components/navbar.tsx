import React from "react";

const NavBar = () => {
	let nav = document.querySelector("nav");
	
	window.addEventListener("scroll", () => {
		if (document.documentElement.scrollTop > 20) {
			nav?.classList.add("sticky")
		} else {
			nav?.classList.remove("sticky")
		}
	});

	return (
		<>
		<nav>
			<div className="nav-content">
				<div className="logo">
					<a href="#">My App</a>
				</div>
				<ul className="nav-links">
					<li><a href="/users/">Users</a></li>
					<li><a href="#">Products</a></li>
					<li><a href="#">Orders</a></li>
				</ul>
			</div>
		</nav>
		</>
	)
}

export default NavBar;