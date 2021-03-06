import React, {ChangeEvent} from 'react';
import {Bucket} from "../DataTypes";
import {Paper} from "@material-ui/core";
import TextField from "@material-ui/core/TextField";
import {authenticatedFetchAndDeserialize} from "../App";
import {Route} from "react-router";
import Button from "@material-ui/core/Button";
import MenuAppBar from "./MenuAppBar";

interface Props {

}

interface State {
  name: string,
  error: null | "string"
}

interface CreateBucketRequest {
  bucket_name: string,
}

export class CreateBucketPage extends React.Component<Props, State> {
  state: State = {
    name: "",
    error: null
  };

  // this any should be History<any> from the history lib (I think?)
  postCreateBucketRequest(history: any) {
    const url = "/api/bucket";
    let request: CreateBucketRequest = {
      bucket_name: this.state.name,
    };

    let options: RequestInit = {
      method: "POST",
      body: JSON.stringify(request)
    };

    authenticatedFetchAndDeserialize<Bucket>(url, options)
      .then((bucket: Bucket) => {
        history.push(`/bucket/${bucket.bucket_slug}`)
      });
  }

  handleNameUpdate = (event: ChangeEvent<HTMLInputElement>) => {
    this.setState({name: event.target.value})
    // TODO update the slug at the same time if its not "dirty"
  };


  submitButton: () => JSX.Element = () => (
    <Route render={({ history }) => (
      <Button
        onClick={() => this.postCreateBucketRequest(history)}
      >
        Create Bucket
      </Button>
    )} />
  );

  render() {
    return (
      <>
        <MenuAppBar/>
        <main>
          <Paper style={styles.smallMargin}>
            <div style={styles.verticalContainer}>
              <div style={styles.constrainedWidth}>
                <div style={styles.verticalSpacing}/>
                <TextField
                  label={"Bucket Name"}
                  fullWidth={true}
                  onChange={this.handleNameUpdate}
                />
              </div>
            <div style={styles.verticalSpacing}/>
            {this.submitButton()}
            </div>
          </Paper>
        </main>
      </>
    )
  }
}

const styles = {
  verticalContainer: {
    display: "flex",
    flexDirection: "column" as "column",
    alignItems: "center",
    padding: "15px"
  },
  smallMargin: {
    margin: "5px"
  },
  verticalSpacing: {
    height: 20
  },
  constrainedWidth: {
    maxWidth: 500
  }
};