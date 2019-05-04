import React from 'react';
import {Modal} from "@material-ui/core";
import Typography from "@material-ui/core/Typography";
import Paper from "@material-ui/core/Paper";

interface Props {
  open: boolean,
  handleClose: () => void
}

interface State {

}

export class BucketManagementModalComponent extends React.Component<Props, State> {

  render() {
    return (
      <Modal
        open={this.props.open}
        onClose={this.props.handleClose}
      >
        <Paper style={styles.floating}>
          <Typography variant="h6" id="modal-title">
            Hello World
          </Typography>
        </Paper>
      </Modal>
    )
  }
}


const styles = {
  floating : {
    padding: 10,
    margin: 20
  }
};