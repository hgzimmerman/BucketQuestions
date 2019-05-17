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
      case "public":
        bucket.public_viewable = checked;
        break;
      case "drawing":
        bucket.drawing_enabled= checked;
        break;
      case "exclusive":
        bucket.exclusive = checked;
        break;
      default: console.error("unreachable default")
    }
    this.setState({bucket: bucket, dirty: true});
  };

  saveChanges = () => {
    const changeset: ChangeBucketFlagsRequest = {
      public_viewable: this.state.bucket.public_viewable,
      drawing_enabled: this.state.bucket.drawing_enabled,
      exclusive: this.state.bucket.exclusive
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
            <FormControl style={styles.form}>
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
                      checked={this.state.bucket.public_viewable}
                      onChange={this.handleChange}
                      disabled={!this.props.permissions.set_public_permission}
                      value="public"
                    />
                  }
                  label="Public"
                />
                <FormLabel>Allows the bucket to be seen from the home page.</FormLabel>

                <FormControlLabel
                  control={
                    <Switch
                      checked={this.state.bucket.exclusive}
                      onChange={this.handleChange}
                      disabled={!this.props.permissions.set_exclusive_permission}
                      value="exclusive"
                    />
                  }
                  label="Exclusive"
                />
                <FormLabel>Prevents anyone who hasn't already joined from joining the bucket.</FormLabel>
              </FormGroup>
              <div style={styles.grow}/>
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
  },
  form: {
    width: "100%",
    height: "100%"
  },
  grow: {
    flex: 1
  }
};