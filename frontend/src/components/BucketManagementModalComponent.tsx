import React, {ChangeEvent} from 'react';
import {Modal} from "@material-ui/core";
import Typography from "@material-ui/core/Typography";
import Paper from "@material-ui/core/Paper";
import {Bucket, BucketUserPermissions, ChangeBucketFlagsRequest, Uuid} from "../DataTypes";
import {isEqual} from 'lodash';
import Switch from "@material-ui/core/Switch";
import FormControlLabel from "@material-ui/core/FormControlLabel";
import FormLabel from "@material-ui/core/FormLabel";
import FormControl from "@material-ui/core/FormControl";
import FormGroup from "@material-ui/core/FormGroup";
import Button from "@material-ui/core/Button";

interface Props {
  open: boolean,
  handleClose: () => void
  bucket: Bucket,
  permissions: BucketUserPermissions,
  setBucketStateCallback: (bucket_uuid: Uuid, request: ChangeBucketFlagsRequest) => void
}

interface State {
  bucket: Bucket,
  dirty: boolean

}

export class BucketManagementModalComponent extends React.Component<Props, State> {
  state: State = {
    bucket: this.props.bucket,
    dirty: false
  };

  componentWillReceiveProps(nextProps: Readonly<Props>, nextContext: any): void {
    if (isEqual(nextProps.bucket, this.state.bucket)) {
      this.setState({bucket: nextProps.bucket, dirty: false})
    } else {
      this.setState({dirty: false})
    }
  }

  handleChange = (event: React.ChangeEvent<HTMLInputElement>, checked: boolean) => {
    let bucket = this.state.bucket;
    switch (event.target.value) {
      case "visible":
        bucket.visible = checked;
        break;
      case "drawing":
        bucket.drawing_enabled= checked;
        break;
      case "private":
        bucket.private = checked;
        break;
      default: console.error("unreachable default")
    }
    this.setState({bucket: bucket, dirty: true});
  };

  saveChanges = () => {
    const changeset: ChangeBucketFlagsRequest = {
      visible: this.state.bucket.visible,
      drawing_enabled: this.state.bucket.drawing_enabled,
      private: this.state.bucket.private
    };
    this.props.setBucketStateCallback(this.state.bucket.uuid, changeset)
  };

  render() {
    return (
      <Modal
        open={this.props.open}
        onClose={this.props.handleClose}
      >
        <div style={styles.flex}>
          <Paper style={styles.floating}>
            <FormControl>
              <FormLabel>Bucket Settings</FormLabel>
              <FormGroup>
                <FormControlLabel
                  control={
                    <Switch
                      checked={this.state.bucket.drawing_enabled}
                      onChange={this.handleChange}
                      disabled={!this.props.permissions.set_drawing_permission}
                      value="drawing"
                    />
                  }
                  label="Drawing"
                />
                <FormLabel>Allows Questions to be drawn from this bucket</FormLabel>

                <FormControlLabel
                  control={
                    <Switch
                      checked={this.state.bucket.visible}
                      onChange={this.handleChange}
                      disabled={!this.props.permissions.set_visibility_permission}
                      value="visible"
                    />
                  }
                  label="Visible"
                />
                <FormLabel>Allows the bucket to be seen from the home page.</FormLabel>

                <FormControlLabel
                  control={
                    <Switch
                      checked={this.state.bucket.private}
                      onChange={this.handleChange}
                      disabled={!this.props.permissions.set_private_permission}
                      value="private"
                    />
                  }
                  label="Private"
                />
                <FormLabel>Prevents anyone who hasn't already joined from joining the bucket.</FormLabel>
              </FormGroup>
              <Button
                onClick={() => this.saveChanges()}
                disabled={!this.state.dirty}
              >
               Save
              </Button>
            </FormControl>


          </Paper>
        </div>
      </Modal>
    )
  }
}


const styles = {
  floating : {
    flexGrow: 1,
    padding: 10,
    margin: 20,
    maxWidth: 600,
    minWidth: 300,
    maxHeight: 400,
    pointerEvents: "auto" as "auto",
  },
  flex: {
    pointerEvents: "none" as "none",
    height: "100%",
    display: "flex",
    justifyContent: "center",
    alignContent: "center",
  }
};