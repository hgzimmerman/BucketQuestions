import React from 'react';
import {Button} from "@material-ui/core";
import MenuItem from "@material-ui/core/MenuItem";
import {isAuthenticated, logout} from "../App";
import IconButton from "@material-ui/core/IconButton";
import {AccountCircle} from "@material-ui/icons";
import Menu from "@material-ui/core/Menu";
import {LinkResponse} from "../DataTypes";
import Tooltip from "@material-ui/core/Tooltip";

interface Props {

}

interface State {
  auth: boolean,
  anchorEl: null | HTMLElement;
  loginLink: null | string
}

export class LoginIconComponent extends React.Component<Props, State> {
  state: State = {
    auth: isAuthenticated(),
    anchorEl: null,
    loginLink: null
  };

  componentDidMount(): void {
    if (!isAuthenticated()) {
      this.getLink();
    }
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
    this.getLink()
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
    const { auth, anchorEl } = this.state;
    const open = Boolean(anchorEl);

    return (
      <>
        {auth
          ? (
          <div>
            <Tooltip title={"Account"} placement={"bottom"}>
              <IconButton
                aria-owns={open ? 'menu-appbar' : undefined}
                aria-haspopup="true"
                onClick={this.handleMenu}
                color="inherit"
              >
                <AccountCircle />
              </IconButton>
            </Tooltip>
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
        </>
    )
  }
}