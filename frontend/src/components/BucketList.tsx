import React from 'react';
import {ListItem} from "@material-ui/core";
import {Bucket} from "../DataTypes";
import {Loadable} from "../Util";
import {Error} from "../Util";







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

  render() {
    return (
      <div>
      {
        this.state.buckets.match({
          loading: () => {
            return (<>{"Loading"}</>);
          },
          loaded: (buckets: Array<Bucket>) => {
            return (<>{"Buckets"}</>);
          },
          error: (error: Error) => {
            return (<>{error}</>);
          },
        })
      }
      </div>
    );
  }
}