import React, {ChangeEvent} from 'react';
import {AppBar, Theme, WithStyles} from "@material-ui/core";
import Tabs from "@material-ui/core/Tabs";
import Tab from "@material-ui/core/Tab";
import {BucketList} from "./BucketList";
import {Bucket} from "../DataTypes";
import {authenticatedFetchAndDeserialize, isAuthenticated} from "../App";
import {Link} from "react-router-dom";
import Tooltip from "@material-ui/core/Tooltip";
import Fab from "@material-ui/core/Fab";
import AddIcon from '@material-ui/icons/Add';
import createStyles from "@material-ui/core/styles/createStyles";
import withStyles from "@material-ui/core/styles/withStyles";



const styles = (theme: Theme) => createStyles({
  horizontal_container: {
    display: "flex",
    flexDirection: "row" as "row",
  },
  grow: {
    flexGrow: 1
  },
  vertically_centered: {
    display: "flex",
    flexDirection: "column" as "column",
    justifyContent: "center"
  },
  create_bucket_icon: {
    position: "absolute"
  },
  absolute: {
    position: 'absolute',
    bottom: theme.spacing.unit * 2,
    right: theme.spacing.unit * 3,
  },
});

interface Props extends WithStyles<typeof styles>{

}


interface State {
  tabPage: number
  bucketSearch: string
}

export const Home = withStyles(styles)(
  class extends React.Component<Props, State> {
    state: State = {
      tabPage: 0,
      bucketSearch: ""
    };
    componentDidMount(): void {
    }

    handleTabSelected = (event: any, value: number) => {
      this.setState({tabPage: value})
    };

    handleSearchTextUpdate = (event: ChangeEvent<HTMLInputElement>) => {
      this.setState({bucketSearch: event.target.value})
    };

    render() {
      const {classes} = this.props;
      const auth: boolean = isAuthenticated();
      return (
        <div>

          {(auth) &&
          <AppBar
            position="static"
            color={"primary"}
          >
            <div className={classes.horizontal_container}>
              <Tabs
                value={this.state.tabPage}
                onChange={this.handleTabSelected}
              >
                <Tab
                  label="Joined"
                  style={{height: 60}}
                />
                <Tab
                  label="Public"
                  style={{height: 60}}
                />
              </Tabs>
              <div className={classes.grow}/>
            </div>
          </AppBar>
          }
          {(auth)
            ? <>
              {this.state.tabPage === 0 &&
                <BucketList
                  bucketListFetchPromise={get_joined_buckets}
                />
              }
              {this.state.tabPage === 1 &&
                <BucketList
                  bucketListFetchPromise={get_public_buckets}
                />
              }
              </>
            :  <BucketList
                  bucketListFetchPromise={get_public_buckets}
                />
          }

          {(auth) &&
            <Link to={"/create_bucket"}>
              <Tooltip title="Create Bucket" aria-label="Add">
                <Fab
                  color="secondary"
                  className={classes.absolute}
                >
                  <AddIcon />
                </Fab>
              </Tooltip>
            </Link>
          }
        </div>
      )
    }
  }
);




const get_public_buckets: () => Promise<Array<Bucket>> = () => {
  const url: string = "/api/bucket/public";
  return authenticatedFetchAndDeserialize(url);
};

const get_joined_buckets: () => Promise<Array<Bucket>> = () => {
  const url: string = "/api/bucket/in";
  return authenticatedFetchAndDeserialize(url);
};