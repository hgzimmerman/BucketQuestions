import React from 'react';
import {ListItem} from "@material-ui/core";
import {Bucket, UuidResponse} from "../DataTypes";
import {Loadable} from "../Util";
import {Error} from "../Util";
import List from "@material-ui/core/List";
import {Route} from "react-router-dom";
import Paper from "@material-ui/core/Paper";
import {LoadingComponent} from "./LoadingComponent";
import Typography from "@material-ui/core/Typography";
import {authenticatedFetchAndDeserialize} from "../App";



interface Props {
  bucketListFetchPromise: () => Promise<Array<Bucket>>
}

interface State {
  buckets: Loadable<Array<Bucket>>
}

export class BucketList extends React.Component<Props, State> {
  state: State = {
    buckets: new Loadable()
  };

  componentDidMount(): void {
    this.props.bucketListFetchPromise()
      .then((buckets: Array<Bucket>) => {
        this.setState({buckets: Loadable.loaded(buckets)})
      })
      .catch((error: any) => {
        this.setState({buckets: Loadable.errored("Couldn't get buckets")})
      });
  }

  remove_self_from_bucket = (bucket: Bucket) => {
    const uuid_url = "/api/user/uuid";
    authenticatedFetchAndDeserialize<UuidResponse>(uuid_url)
      .then((uuidResponse: UuidResponse) => {
        const remove_self_from_bucket_url = `/api/bucket/${bucket.uuid}/user/?user_uuid=${uuidResponse.uuid}`;
        let options: RequestInit = {
          method: "DELETE"
        };
        authenticatedFetchAndDeserialize(remove_self_from_bucket_url, options)
          .then(() => {
            console.log("removed bucket") // TODO actually refresh the bucket list
          })
          .catch(() => {
            console.log("Couldn't remove self from bucket");
          })
      })
      .catch(() => {
        console.log("Couldn't get uuid required to make request to remove self from bucket.");
      })

  };

  static render_bucket(bucket: Bucket, history: any) {
    return (
        <ListItem
          key={bucket.uuid}
          button
          onClick={() => history.push(`/bucket/${bucket.bucket_slug}`)}
        >

          <div style={styles.horizontalContainer}>
            <Typography>
              {bucket.bucket_name}
            </Typography>

            <Typography variant="caption">
              {bucket.bucket_slug}
            </Typography>
          </div>

        </ListItem>
    )
  }

  static render_buckets(buckets: Array<Bucket>) {
    return(
      <Paper style={styles.mainList}>
        <Route render={({ history }) => (
          <List>
            {buckets.map(bucket => BucketList.render_bucket(bucket, history))}
          </List>
        )}/>
      </Paper>
    )
  }

  render() {
    return (
      <div style={styles.verticalContainer}>
      {
        this.state.buckets.match({
          loading: () => <LoadingComponent delay={"200ms"}/>,
          loaded: BucketList.render_buckets,
          error: (error: Error) => {
            return (<>{error}</>);
          },
        })
      }
      </div>
    );
  }
}

const styles = {
  verticalContainer: {
    display: "flex",
    flexDirection: "column" as "column",
    alignItems: "center",
    padding: 10
  },
  mainList: {
    maxWidth: 700,
    width: "100%",
    marginBottom: 15
  },
  horizontalContainer: {
    display: "flex",
    flexDirection: "row" as "row",
    margin: 2,
    justifyContent: "space-between",
    width: "100%"
  }
};