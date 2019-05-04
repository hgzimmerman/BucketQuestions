import React from 'react';
import {Loadable, Error} from "../Util";
import {authenticatedFetchAndDeserialize} from "../App";
import {Bucket, ErrorResponse} from "../DataTypes";

interface Props {
  match: Match
}
interface Match {
  params: Params
}
interface Params {
  slug: string
}

interface State {
  // slug: string
  bucket: Loadable<Bucket>
}

export class BucketComponent extends React.Component<Props, State> {
  state: State = {
    // slug: this.props.match.params.slug,
    bucket: Loadable.loading()
  };

  componentWillReceiveProps(nextProps: Readonly<Props>, nextContext: any): void {
    this.getBucket();
  }
  componentDidMount(): void {
    this.getBucket();
  }

  getBucket: () => void = () => {
    const url = `/api/bucket/slug/${this.props.match.params.slug}`;
    authenticatedFetchAndDeserialize<Bucket>(url)
      .then((bucket: Bucket) => {
        this.setState({bucket: Loadable.loaded(bucket)})
      })
      .catch((error: ErrorResponse) => {
        this.setState({bucket: Loadable.errored(error.message)})
      })
  };

  render() {
    return (
      <div>
        {
          this.state.bucket.match({
            loading: () => <>Loading</>,
            loaded: (bucket: Bucket) => <>{bucket.uuid}</>,
            error: (error: Error) => <>{error}</>
          })
        }
      </div>
    )
  }
}