import React from 'react';
import PropTypes from 'prop-types';
import { createStyles, withStyles, WithStyles } from '@material-ui/core/styles';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import AccountCircle from '@material-ui/icons/AccountCircle';
import Switch from '@material-ui/core/Switch';
import FormControlLabel from '@material-ui/core/FormControlLabel';
import FormGroup from '@material-ui/core/FormGroup';
import MenuItem from '@material-ui/core/MenuItem';
import Menu from '@material-ui/core/Menu';
import {isAuthenticated, logout} from "../App";
import {Button} from "@material-ui/core";
import {BrowserRouter, Link} from "react-router-dom";

const styles = createStyles({
  root: {
    flexGrow: 1,
  },
  grow: {
    flexGrow: 1,
  },
  menuButton: {
    marginLeft: -12,
    marginRight: 20,
  },
});

interface LinkResponse {
  link: string
}

export interface Props extends WithStyles<typeof styles> {}

export interface State {
  auth: boolean;
  anchorEl: null | HTMLElement;
  loginLink: null | string
}

class MenuAppBar extends React.Component<Props, State> {
  state: State = {
    auth: true,
    anchorEl: null,
    loginLink: null
  };

  componentDidMount(): void {
    this.getLink();
    this.setState({auth: isAuthenticated()});
  }

  getLink() {
    const url = "/api/auth/link";
    fetch(url)
      .then((response: Response) => {
        return response.json();
      })
      .then((json: LinkResponse) => {
        this.setState({loginLink: json.link})
      })
  }

  handleMenu = (event: React.MouseEvent<HTMLElement>) => {
    this.setState({ anchorEl: event.currentTarget });
  };

  handleClose = () => {
    this.setState({ anchorEl: null });
  };

  handleLogout = () => {
    logout();
    this.setState({auth: isAuthenticated(), anchorEl: null })
  };

  handleLogin = () => {
    console.log("wants to log in");
    if (this.state.loginLink !== null) {
      window.location.href = this.state.loginLink;
    } else {
      console.warn("login link not ready yet")
    }
  };

  render() {
    const { classes } = this.props;
    const { auth, anchorEl } = this.state;
    const open = Boolean(anchorEl);

    return (
      <div className={classes.root}>
        <AppBar position="static">
          <Toolbar>
            <IconButton className={classes.menuButton} color="inherit" aria-label="Menu">
              <MenuIcon />
            </IconButton>
            <Typography variant="h6" color="inherit" className={classes.grow}>
              Bucket Questions
            </Typography>
            {auth
              ? (
              <div>
                <IconButton
                  aria-owns={open ? 'menu-appbar' : undefined}
                  aria-haspopup="true"
                  onClick={this.handleMenu}
                  color="inherit"
                >
                  <AccountCircle />
                </IconButton>
                <Menu
                  id="menu-appbar"
                  anchorEl={anchorEl}
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
                  <MenuItem onClick={this.handleClose}>Profile</MenuItem>
                  <MenuItem onClick={this.handleClose}>My account</MenuItem>
                  <MenuItem onClick={this.handleLogout}>Logout</MenuItem>
                </Menu>
              </div>
              )
              : (
              <div>
                <Button
                  size={"small"}
                  variant={"outlined"}
                  onClick={this.handleLogin}
                >
                  Login
                </Button>
              </div>
              )
            }
          </Toolbar>
        </AppBar>
      </div>
    );
  }
}

(MenuAppBar as React.ComponentClass<Props>).propTypes = {
  classes: PropTypes.object.isRequired,
} as any;

export default withStyles(styles)(MenuAppBar);