import React from 'react';
import {AppBar} from "@material-ui/core";
import Toolbar from "@material-ui/core/Toolbar";
import Typography from "@material-ui/core/Typography";
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import Menu from "@material-ui/core/Menu";
import MenuItem from "@material-ui/core/MenuItem";
import {Route} from "react-router";
import {isAuthenticated} from "../App";
import {LoginIconComponent} from "./LoginIconComponent";
import ListItemIcon from "@material-ui/core/ListItemIcon";
import HomeIcon from '@material-ui/icons/Home';
import SettingsIcon from '@material-ui/icons/Settings';
import bucket from '../bucket_white.png';
import Tooltip from "@material-ui/core/Tooltip";

interface Props {
  // The title displayed in the nav bar.
  title: string,
  // In order for the modal menu option to be shown,
  // the bucket component needs to have confirmed the user has permissions to do so,
  // and that the requisite data is fetched.
  permissionsModalReady: boolean,
  // How many questions are in the bucket?
  remaining_questions: number | null,
  // A callback to open the modal from the parent component.
  handleOpenModal: () => void
}


interface State {
  anchorEl: null | HTMLElement;
}

export class BucketNavBarComponent extends React.Component<Props, State> {
  state: State = {
    anchorEl: null,
  };

  handleMenu = (event: React.MouseEvent<HTMLElement>) => {
    this.setState({ anchorEl: event.currentTarget });
  };

  handleClose = () => {
    this.setState({ anchorEl: null });
  };

  handleOpenModal = () => {
    this.props.handleOpenModal();
    this.setState({ anchorEl: null});
  };



  render() {
    const open = Boolean(this.state.anchorEl);
    let remainingQuestions = "";
    if (this.props.remaining_questions !== null)  {
      remainingQuestions = this.props.remaining_questions + "";
    }
    return (
      <AppBar style={{"position": "static"}}>
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
                <MenuItem onClick={() => history.push("/")}>
                  <ListItemIcon>
                    <HomeIcon />
                  </ListItemIcon>
                  Home
                </MenuItem>
                {
                (this.props.permissionsModalReady && isAuthenticated()) &&
                  <MenuItem onClick={this.handleOpenModal}>
                    <ListItemIcon>
                      <SettingsIcon />
                    </ListItemIcon>
                    Manage Bucket
                  </MenuItem>
                }
              </Menu>

              <Typography variant="h6" color="inherit">
                {this.props.title}
              </Typography>


              <div style={styles.horizSpacing}/>

              <Tooltip title="Remaining Questions" aria-label="Remaining Questions">
                <div>
                  <img src={bucket} alt={"Remaining Questions"} style={styles.bucketImage}/>: {remainingQuestions}
                </div>
              </Tooltip>

              <div style={styles.grow}/>

              <LoginIconComponent/>
            </>
            )}
          />

        </Toolbar>
      </AppBar>
    )
  }
}

const styles  = {
  grow: {
    flexGrow: 1
  },
  horizSpacing: {
    width: 20
  },
  bucketImage: {
    paddingTop: 7,
    height: 18
  }
};