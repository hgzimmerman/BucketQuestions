import React from 'react';
import './App.css';
import { BrowserRouter, Route, Switch } from "react-router-dom";
import {HomePage} from "./components/HomePage";
import {FourOFourPage} from "./components/FourOFourPage";
import {ErrorResponse} from "./DataTypes";
import {CreateBucketPage} from "./components/CreateBucketPage";
import {BucketPage} from "./components/BucketPage";

const App: React.FC = () => {
  return (
    <div className="App">
      <BrowserRouter>
        <Switch>
          <Route path={"/"} exact component={HomePage}/>
          <Route path={"/create_bucket"} exact component={CreateBucketPage}/>
          <Route path={"/bucket/:slug"} component={BucketPage}/>
          <Route component={FourOFourPage}/>
        </Switch>
      </BrowserRouter>

    </div>
  );
};

export function logout() {
  console.log("Logging out");
  window.localStorage.removeItem('jwt')
}

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

   let newInit: RequestInit = {
    headers,
    ...init
  };
  return fetch(url, newInit);
}

/**
 * Is the user authenticated
 */
export function isAuthenticated(): boolean {
  const jwt = getJwt();
  return (jwt !== null)
}


/**
 * Performs a fetch after trying to attach the bearer JWT
 * @param url The url to hit.
 * @param init The additional parameters.
 */
export async function authenticatedFetchAndDeserialize<T>(url: string, init?: RequestInit): Promise<T> {
  const response = await authenticatedFetch(url, init);
  if (response.ok) {
    return response.json().then((value: T) => value);
  } else {
    return response.json().then((err: ErrorResponse) => {
      throw err;
    });
  }
}



export default App;
