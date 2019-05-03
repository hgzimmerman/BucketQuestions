import React, {ChangeEvent} from 'react';
import logo from './../logo.svg';
import {AppBar} from "@material-ui/core";
import Tabs from "@material-ui/core/Tabs";
import Tab from "@material-ui/core/Tab";
import TextField from "@material-ui/core/TextField";
import classes from 'classnames';
import {BucketList} from "./BucketList";
import {Bucket} from "../DataTypes";
import {authenticatedFetchAndDeserialize} from "../App";
import Icon from "@material-ui/core/Icon";
import {Add} from "@material-ui/icons";
import Button from "@material-ui/core/Button";

interface Props {

}

type TabState = "public" | "joined"

interface State {
  tabPage: number
  bucketSearch: string
}

export class Home extends React.Component<Props, State> {
  state: State = {
    tabPage: 0,
    bucketSearch: ""
  };

  handleTabSelected = (event: any, value: number) => {
    this.setState({tabPage: value})
  };

  handleSearchText = (event: ChangeEvent<HTMLInputElement>) => {
    this.setState({bucketSearch: event.target.value})
  };

  render() {
    return (
      <div>

        <AppBar
          position="static"
          color={"primary"}
        >
          <div style={styles.horizontal_container}>

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
            <div style={styles.grow}/>
            <div style={styles.vertically_centered}>
              <Button
                // style={{height: 60}}
                size={"medium"}
                variant={"outlined"}
              >
                <Add/>
                New Bucket
              </Button>
            </div>

            <TextField
              id="outlined-search"
              label="Find Bucket"
              type="search"
              value={this.state.bucketSearch}
              onChange={this.handleSearchText}
              color={"white"}
              // className={classes.textField}
              margin="dense"
              variant="filled"
              // fullWidth={true}
            />
          </div>

        </AppBar>
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
    )
  }
}

const styles = {
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
  }
};

const get_public_buckets: () => Promise<Array<Bucket>> = () => {
  const url: string = "/api/bucket/public";
  return authenticatedFetchAndDeserialize(url);
};

const get_joined_buckets: () => Promise<Array<Bucket>> = () => {
  const url: string = "/api/bucket/in";
  return authenticatedFetchAndDeserialize(url);
};