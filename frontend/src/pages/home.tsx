import React, { useEffect, useState } from 'react';
import { Users } from '../components/user_list'
import '../App.css';
import { CreateUser } from '../components/create_user';

enum UserMode {
  ShowUsers = "show",
  CreateUsers = "create",
}

function Home() {
  const [actualMode, setMode] = useState(UserMode.ShowUsers)
  const [content, setContent] = useState(<Users />)

  useEffect(() => {
	if (actualMode === UserMode.ShowUsers) {
	  setContent(<Users />);
	} else if (actualMode === UserMode.CreateUsers) {
	  setContent(<CreateUser />);
	}
  }, [actualMode])

  function DefaultMode() {
	setMode(UserMode.ShowUsers);
  }

  function CreateUserMode() {
	setMode(UserMode.CreateUsers);
  }

  const UnderlineIfSelected = (mode: UserMode) => {
	return (mode === actualMode)? {
		// textDecoration: 'underline #d0c9c8',
		textShadow: "0 5px 10px rgba(0, 0, 0, 0.5)",
	} : {}
  }

  return (
	<div className="App">
	  <ul className='menu-bar'>
		<li key="default">
		  <a onClick={DefaultMode} style={UnderlineIfSelected(UserMode.ShowUsers)}>Show</a>
		</li>
		<li key="create">
		  <a onClick={CreateUserMode} style={UnderlineIfSelected(UserMode.CreateUsers)}>Create</a>
		</li>
	  </ul>
	  {content}
	</div>
  );
}

export default Home;
