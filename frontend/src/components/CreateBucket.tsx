import React, {ChangeEvent} from 'react';
import {Bucket} from "../DataTypes";
import {Paper} from "@material-ui/core";
import TextField from "@material-ui/core/TextField";
import {authenticatedFetchAndDeserialize} from "../App";
import {Route} from "react-router";
import Button from "@material-ui/core/Button";

interface Props {

}

interface State {
  name: string,
  slug: string,
  error: null | "string"
}

interface CreateBucketRequest {
  bucket_name: string,
  bucket_slug: string
}

export class CreateBucket extends React.Component<Props, State> {
  state: State = {
    name: "",
    slug: "",
    error: null
  };

  // this any should be History<any> from the history lib (I think?)
  postCreateBucketRequest(history: any) {
    const url = "/api/bucket";
    let request: CreateBucketRequest = {
      bucket_name: this.state.name,
      bucket_slug: this.state.slug
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

  handleSlugUpdate = (event: ChangeEvent<HTMLInputElement>) => {
    this.setState({slug: event.target.value})
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
      <div>
        <Paper>
          <TextField
            label={"Bucket Name"}
            onChange={this.handleNameUpdate}
          />
          <TextField
            label={"URL Slug"}
            onChange={this.handleSlugUpdate}
          />
          {this.submitButton()}
        </Paper>
      </div>
    )
  }
}