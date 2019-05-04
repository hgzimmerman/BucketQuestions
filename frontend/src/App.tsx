import React from 'react';
import './App.css';
import MenuAppBar from "./components/MenuAppBar";
import { BrowserRouter, Route, Switch } from "react-router-dom";

import {Home} from "./components/Home"
import {FourOFour} from "./components/FourOFour"
import {ErrorResponse} from "./DataTypes";
import {CreateBucket} from "./components/CreateBucket";
import {BucketComponent} from "./components/Bucket";

const App: React.FC = () => {
  return (
    <div className="App">
      <BrowserRouter>
        <MenuAppBar/>
        <Switch>
          <Route path={"/"} exact component={Home}/>
          <Route path={"/create_bucket"} exact component={CreateBucket}/>
          <Route path={"/bucket/:slug"} component={BucketComponent}/>
          <Route component={FourOFour}/>
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
