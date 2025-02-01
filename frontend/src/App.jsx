import React from 'react'
import {BrowserRouter, Routes, Route, Navigate} from 'react-router-dom'
import api from './api';
import Register from './pages/Register';
import ProtectedRoute from './components/ProtectedRoute';
import Login from './pages/Login';
import NotFound from './pages/NotFound';
import Home from './pages/Home';

function Logout() {
  localStorage.clear();

  try {
    api.get("/api/auth/logout");
  } catch {}

  return <Navigate to="/login" />;
}

function RegisterAndLogout() {
  Logout();

  return <Register/>;
}

function App() {

  return (
    <BrowserRouter>
      <Routes>
        <Route
          path='/'
          element={
            <ProtectedRoute>
              <Home />
            </ProtectedRoute>
          }
        />
        <Route path='/login' element={<Login />} />
        <Route path='/logout' element={<Logout />} />
        <Route path='/register' element={<RegisterAndLogout />} />
        <Route path='*' element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App