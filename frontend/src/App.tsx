import React from 'react';
import logo from './logo.svg';
import './App.css';
import MenuAppBar from "./components/MenuAppBar";

const App: React.FC = () => {
  return (
    <div className="App">
      <MenuAppBar/>
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
};

function getJwt(): string | null {
    return window.localStorage.getItem('jwt');
}

function getJwtBearer(): string | null {
    let jwt = getJwt();
    if (jwt !== null) {
        return 'bearer ' + getJwt();
    } else {
        return null
    }
}

function authenticatedFetch(url: string, init?: RequestInit): Promise<Response> {
  let jwt: string | null = getJwtBearer();
  let headers: HeadersInit = {
    "content-type": 'application/json',
  };

  // Add the bearer JWT to the headers
  if (jwt != null) {
    headers["Authorization"] = jwt;
  }

  if (init !== undefined &&init.method !== undefined && init.body !== undefined) {
    console.log("making a request with a body");
    let newInit: RequestInit = {
      headers,
      ...init
    };
    return fetch(url, newInit)
  } else {
    console.log("making request without body");
    return fetch(url, {headers})
  }
}


export function isAuthenticated(): boolean {
  const jwt = getJwt();
  return (jwt !== null)
}

/**
 * If the api doesn't fulfill a request, it will return one of these.
 */
export interface ApiError {
  message: string,
  canonical_reason: string,
  error_code: number,
}

export function authenticatedFetchAndDeserialize<T>(url: string, init?: RequestInit): Promise<T> {
  return authenticatedFetch(url, init)
    .then(response => {
      if (response.ok) {
        return response.json().then((value: T) => value);
      } else {
        return response.json().then((err: ApiError) => {throw err;});
      }
    });
}



export default App;
