import React from 'react';
import {AppBar} from "@material-ui/core";
import Toolbar from "@material-ui/core/Toolbar";
import Typography from "@material-ui/core/Typography";
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import Menu from "@material-ui/core/Menu";
import MenuItem from "@material-ui/core/MenuItem";
import {Route} from "react-router";
import {Uuid} from "../DataTypes";
import {isAuthenticated} from "../App";
import {LoginIconComponent} from "./LoginIconComponent";

interface Props {
  title: string,
  bucket_uuid: Uuid | null,
}


interface State {
  anchorEl: null | HTMLElement;
}

export class BucketNavBarComponent extends React.Component<Props, State> {
  state: State = {
    anchorEl: null
  };

  handleMenu = (event: React.MouseEvent<HTMLElement>) => {
    this.setState({ anchorEl: event.currentTarget });
  };

  handleClose = () => {
    this.setState({ anchorEl: null });
  };

  render() {

    const open = Boolean(this.state.anchorEl);
    return (
      <AppBar>
        <Toolbar>

          <Route render={({ history }) => (
            <>
              <IconButton
                color="inherit"
                aria-label="Menu"
                onClick={this.handleMenu}
              >
                <MenuIcon />
              </IconButton>
              <Menu
                anchorEl={this.state.anchorEl}
                anchorOrigin={{
                  vertical: 'top',
                  horizontal: 'right',
                }}
                transformOrigin={{
                  vertical: 'top',
                  horizontal: 'right',
                }}
                open={open}
                onClose={this.handleClose}
              >
                <MenuItem onClick={() => history.push("/")}>Home</MenuItem>
                {
                (this.props.bucket_uuid !== null && isAuthenticated()) &&
                  <MenuItem onClick={this.handleClose}>Manage Bucket</MenuItem>
                }
              </Menu>

              <Typography variant="h6" color="inherit">
                {this.props.title}
              </Typography>

              <LoginIconComponent/>
            </>
            )}
          />

        </Toolbar>
      </AppBar>
    )
  }
}