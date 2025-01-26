import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';

test('renders learn react link', () => {
  window.location.href = "/users/"
  render(<App />);
});
