import React from 'react';
import {Fade} from "@material-ui/core";
import CircularProgress from "@material-ui/core/CircularProgress";

interface Props {
  delay?: string
}


export class LoadingComponent extends React.PureComponent<Props> {
  render() {
    return (
      <>
        <Fade
          in={true}
          style={{
            transitionDelay: Boolean(this.props.delay) ? this.props.delay : "500ms",
          }}
          unmountOnExit
        >
          <CircularProgress />
        </Fade>
      </>
    )
  }
}