import React from 'react';
import PropTypes from 'prop-types';
import { createStyles, withStyles, WithStyles } from '@material-ui/core/styles';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import IconButton, {IconButtonProps} from '@material-ui/core/IconButton';
import HomeIcon from '@material-ui/icons/Home';
import {LoginIconComponent} from "./LoginIconComponent";
import {Link} from "react-router-dom";

const styles = createStyles({
  root: {
    flexGrow: 1,
  },
  grow: {
    flexGrow: 1,
  },
  menuButton: {
    marginLeft: -12,
    marginRight: 20,
  },
});



export interface Props extends WithStyles<typeof styles> {}

export interface State {
}

interface LinkIconButtonProps extends IconButtonProps {
  to: string;
  replace?: boolean;
}

const LinkIconButton = (props: LinkIconButtonProps) => (
  <IconButton {...props} component={Link as any} />
);

class MenuAppBar extends React.Component<Props, State> {
  state: State = {
  };

  componentDidMount(): void {
  }


  render() {
    const { classes } = this.props;

    return (
        <AppBar style={{"position": "static"}}>
          <Toolbar>
            <LinkIconButton
              to={"/"}
              className={classes.menuButton}
              color="inherit"
              aria-label="Home"
            >
              <HomeIcon />
            </LinkIconButton>
            <Typography variant="h6" color="inherit" className={classes.grow}>
              Create Bucket
            </Typography>
            <LoginIconComponent/>
          </Toolbar>
        </AppBar>
    );
  }
}

(MenuAppBar as React.ComponentClass<Props>).propTypes = {
  classes: PropTypes.object.isRequired,
} as any;

export default withStyles(styles)(MenuAppBar);