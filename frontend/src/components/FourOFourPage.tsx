import React from 'react';
import MenuAppBar from "./MenuAppBar";
import {Typography} from "@material-ui/core";

interface Props {

}

interface State {

}

export class FourOFourPage extends React.Component<Props, State> {
  render() {
    return (
      <>
        <MenuAppBar/>
        <main>
          <Typography>
            Page Not Found
          </Typography>
        </main>
      </>
    )
  }
}