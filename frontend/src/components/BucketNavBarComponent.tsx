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
import {BucketManagementModalComponent} from "./BucketManagementModalComponent";
import ListItemIcon from "@material-ui/core/ListItemIcon";
import HomeIcon from '@material-ui/icons/Home';
import SettingsIcon from '@material-ui/icons/Settings';
import bucket from '../bucket_white.png';
import Tooltip from "@material-ui/core/Tooltip";

interface Props {
  title: string,
  bucket_uuid: Uuid | null,
  remaining_questions: number | null
}


interface State {
  anchorEl: null | HTMLElement;
  modalOpen: boolean
}

export class BucketNavBarComponent extends React.Component<Props, State> {
  state: State = {
    anchorEl: null,
    modalOpen: false
  };

  handleMenu = (event: React.MouseEvent<HTMLElement>) => {
    this.setState({ anchorEl: event.currentTarget });
  };

  handleClose = () => {
    this.setState({ anchorEl: null });
  };
  handleOpenModal = () => {
    this.setState({ anchorEl: null, modalOpen: true });
  };
  handleCloseModal = () => {
    this.setState({modalOpen: false})
  };


  render() {
    const open = Boolean(this.state.anchorEl);
    let remainingQuestions = "";
    if (this.props.remaining_questions !== null)  {
      remainingQuestions = this.props.remaining_questions + "";
    }
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
                <MenuItem onClick={() => history.push("/")}>
                  <ListItemIcon>
                    <HomeIcon />
                  </ListItemIcon>
                  Home
                </MenuItem>
                {
                (this.props.bucket_uuid !== null && isAuthenticated()) &&
                  <MenuItem onClick={this.handleOpenModal}>
                    <ListItemIcon>
                      <SettingsIcon />
                    </ListItemIcon>
                    Manage Bucket
                  </MenuItem>
                }
              </Menu>
              <BucketManagementModalComponent open={this.state.modalOpen} handleClose={this.handleCloseModal}/>

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