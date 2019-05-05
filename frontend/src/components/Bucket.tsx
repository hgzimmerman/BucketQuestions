import React, {ChangeEvent} from 'react';
import {Loadable, Error} from "../Util";
import {authenticatedFetchAndDeserialize, isAuthenticated} from "../App";
import {
  ArchiveQuestionRequest,
  Bucket,
  BucketUserPermissions,
  ErrorResponse,
  NewQuestionRequest,
  Question, SetPermissionsRequest, User, Uuid
} from "../DataTypes";
import {Button} from "@material-ui/core";
import Paper from "@material-ui/core/Paper";
import TextField from "@material-ui/core/TextField";
import {BucketNavBarComponent} from "./BucketNavBarComponent";
import {BucketManagementModalComponent} from "./BucketManagementModalComponent";

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
  bucket: Loadable<Bucket>,
  question: Loadable<Question | null>,
  questions_remaining_in_bucket: Loadable<number>,
  permissions: Loadable<BucketUserPermissions>,
  user: Loadable<User>,
  newQuestionText: string,
  // anchorElModal: null | HTMLElement;
  modalOpen: boolean
}

export class BucketComponent extends React.Component<Props, State> {
  state: State = {
    bucket: Loadable.loading(),
    question: Loadable.unloaded(),
    questions_remaining_in_bucket: Loadable.loading(),
    permissions: Loadable.unloaded(),
    user: Loadable.unloaded(),
    newQuestionText: "",
    // anchorElModal: null,
    modalOpen: false
  };

  componentWillReceiveProps(nextProps: Readonly<Props>, nextContext: any): void {
    this.getBucket()
      .then((bucket: Bucket | void) => {
        if (bucket != null) {
          this.getNumberOfQuestions(bucket);
          if (isAuthenticated()) {
            Promise.all([
              this.getPermissions(bucket.uuid),
              // this.getSelf()
            ])
          }
        }
      });
  }

  componentDidMount(): void {
    this.getBucket()
      .then((bucket: Bucket | void) => {
        if (bucket != null) {
          this.getNumberOfQuestions(bucket);
          if (isAuthenticated()) {
            this.getPermissions(bucket.uuid)
          }
        }
      });
  }

  /*
   * Handle UI Events
   */

  handleCloseModal = () => {
    this.setState({modalOpen: false})
  };

  handleOpenModal = () => {
    this.setState({ modalOpen: true });
  };


  handleNewQuestionTextUpdate = (event: ChangeEvent<HTMLInputElement>) => {
    this.setState({newQuestionText: event.target.value})
  };

  /* === Network requests */

  getPermissions: (bucket_uuid: Uuid) => Promise<void> = (bucket_uuid: Uuid) => {
    const url = `/api/bucket/${bucket_uuid}/user`;
    return authenticatedFetchAndDeserialize<BucketUserPermissions>(url)
      .then((permissions: BucketUserPermissions) => {
        this.setState({permissions: Loadable.loaded(permissions)})
      })
  };

  // getSelf = () => {
  //   const url = `/api/user`;
  //   return authenticatedFetchAndDeserialize<User>(url)
  //     .then((user: User) => {
  //       this.setState({user: Loadable.loaded(user)})
  //
  //     })
  // };

  setPermissions = (permissions: SetPermissionsRequest, bucket_uuid: Uuid) => {
    const user_uuid = permissions.target_user_uuid;
    const url = `/api/bucket/${bucket_uuid}/user`;
    const body = JSON.stringify(permissions);
    const options = {
      method: "PUT",
      body
    };
    authenticatedFetchAndDeserialize<BucketUserPermissions>(url, options)
      .then((response: BucketUserPermissions) => {
        console.log(JSON.stringify(response)) // TODO remove this.
        // TODO Might need some logic for setting own permissions vs others
      });
  };

  getBucket: () => Promise<Bucket | undefined> = () => {
    const url = `/api/bucket/slug/${this.props.match.params.slug}`;
    return authenticatedFetchAndDeserialize<Bucket>(url)
      .then((bucket: Bucket) => {
        this.setState({bucket: Loadable.loaded(bucket)});
        return bucket
      })
      .catch((error: ErrorResponse) => {
        this.setState({bucket: Loadable.errored(error.message)});
        return undefined
      })
  };

  getNumberOfQuestions: (bucket: Bucket) => Promise<void> = (bucket: Bucket) => {
    const url = `/api/question/number?bucket_uuid=${bucket.uuid}`;
    return authenticatedFetchAndDeserialize<number>(url)
      .then((num: number) => {
        this.setState({questions_remaining_in_bucket: Loadable.loaded(num)})
      })
      .catch((error: ErrorResponse) => {
        this.setState({questions_remaining_in_bucket: Loadable.errored(error.message)})
      })
  };

  getRandomQuestion: (bucket: Bucket) => Promise<void> = (bucket: Bucket) => {
    const url = `/api/question/random?bucket_uuid=${bucket.uuid}`;
    return authenticatedFetchAndDeserialize<Question>(url)
      .then((question: Question) => {
        this.setState({question: Loadable.loaded(question)})
      })
      .catch((error: ErrorResponse) => {
        this.setState({bucket: Loadable.errored(error.message)})
      });
  };

  markQuestionAsArchived: (question: Question) => Promise<void> = (question: Question) => {
    const url = `/api/question/archive`;
    const bodyData: ArchiveQuestionRequest = {
      question_uuid: question.uuid,
      archived: true
    };
    const body = JSON.stringify(bodyData);
    const options = {
      method: "PUT",
      body
    };
    return authenticatedFetchAndDeserialize(url, options)
      .then((response: any) => {
        console.log("Put question on floor" + JSON.stringify(response))
      });
  };

  addQuestionToBucket: (bucket: Bucket) => Promise<void> = (bucket: Bucket) => {
    if (this.state.newQuestionText.length <= 0) {
      console.log("Submission of empty question prevented");
      return Promise.resolve()
    }

    const url = `/api/question/`;
    const bodyData: NewQuestionRequest = {
      bucket_uuid: bucket.uuid,
      question_text: this.state.newQuestionText
    };
    const body = JSON.stringify(bodyData);
    const options = {
      method: "POST",
      body
    };
    return authenticatedFetchAndDeserialize(url, options)
      .then((response: any) => {
        this.setState({newQuestionText: ""})
      });
  };


  /*
   * === Rendering ===
   */

  render_bucket: (bucket: Bucket) => JSX.Element = (bucket: Bucket) => {
    return (
      <>
        <Paper style={styles.smallMargin}>
          <div style={styles.questionCard}>
          {
            this.state.question.match({
              unloaded: () => <>
                <div style={styles.grow}/>
                <Button
                  onClick={() => {
                    this.getRandomQuestion(bucket)
                      .then(() => this.getNumberOfQuestions(bucket))
                  }}
                >
                  Draw Random Question
                </Button>
              </>,
              loading: () => <>Loading</>,
              loaded: (question: Question | null) => this.render_question(question, bucket),
              error: (error: Error) => <>{error}</>
            })
          }
          </div>
        </Paper>
        {this.render_new_question(bucket)}
      </>
    )
  };

  /**
   *
   * @param question The question to render
   * @param bucket The bucket object is needed to get the uuid required to fetch another random question from the bucket.
   */
  render_question: (question: Question | null, bucket: Bucket) => JSX.Element = (question: Question | null, bucket: Bucket) => {
    return (
      <>
        {
          (question !== null)
          ? <>
              <h4>{question.question_text}</h4>

              <div style={styles.grow}/>

              <div>
                <Button onClick={() => this.setState({question: Loadable.unloaded()})}>
                  Put Back
                </Button>

                <Button onClick={() => {
                  this.markQuestionAsArchived(question)
                    .then(() => this.getNumberOfQuestions(bucket))
                    .then(() => this.setState({question: Loadable.unloaded()}))
                }}>
                  Discard
                </Button>
              </div>
            </>
          : <>
              <h5>No Questions Available</h5>
              <Button onClick={() =>
                this.getRandomQuestion(bucket)
                  .then(() => this.getNumberOfQuestions(bucket))
              }>
                Draw
              </Button>
            </>
        }
      </>
    )
  };

  /**
   *
   * @param bucket Bucket is required when adding a new question to the bucket
   */
  render_new_question(bucket: Bucket) {
    return (
      <Paper style={styles.smallMargin}>
        <div style={styles.padded}>
          <TextField
            label="New Question"
            multiline
            rows={5}
            rowsMax={8}
            variant={"filled"}
            fullWidth={true}
            value={this.state.newQuestionText}
            onChange={this.handleNewQuestionTextUpdate}
          />
        </div>
          <Button
            onClick={() => this.addQuestionToBucket(bucket).then(() => this.getNumberOfQuestions(bucket))}
          >
            Add to Bucket
          </Button>
      </Paper>
    )
  }

  render() {
    let title = "";
    let bucket_uuid = null;
    let bucket = this.state.bucket.getLoaded();
    if (bucket != null) {
      title = bucket.bucket_name;
      bucket_uuid = bucket.uuid;
    }
    const remaining_questions = this.state.questions_remaining_in_bucket.getLoaded();

    return (
      <>
        <BucketNavBarComponent title={title} bucket_uuid={bucket_uuid} remaining_questions={remaining_questions} handleOpenModal={this.handleOpenModal}/>
        <div style={styles.container}>
          <div style={styles.constrainedWidth}>
            {
              this.state.bucket.match({
                loading: () => <>Loading</>,
                loaded: this.render_bucket,
                error: (error: Error) => <>Could not get bucket - {error}</>
              })
            }
          </div>
        </div>
        <BucketManagementModalComponent open={this.state.modalOpen} handleClose={this.handleCloseModal}/>
      </>
    )
  }
}


const styles = {
  padded: {
    padding: 10
  },
  smallMargin: {
    margin: 10
  },
  container: {
    display: "flex",
    flexDirection: "column" as "column",
    alignItems: "center",
  },
  constrainedWidth: {
    maxWidth: 700,
    width: "100%"
  },
  questionCard: {
    display: "flex",
    flexDirection: "column" as "column",
    minHeight: 200
  },
  bottom: {
    marginTop: "auto"
  },
  grow: {
    flexGrow: 1
  }

};