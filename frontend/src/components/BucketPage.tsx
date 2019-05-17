import React, {ChangeEvent} from 'react';
import {Loadable, Error} from "../Util";
import {authenticatedFetchAndDeserialize, isAuthenticated} from "../App";
import {
  ArchiveQuestionRequest,
  Bucket,
  BucketUserPermissions, BucketUserRelation, ChangeBucketFlagsRequest,
  ErrorResponse,
  NewQuestionRequest,
  Question, SetPermissionsRequest, User, Uuid
} from "../DataTypes";
import {Button} from "@material-ui/core";
import Paper from "@material-ui/core/Paper";
import TextField from "@material-ui/core/TextField";
import {BucketNavBarComponent} from "./BucketNavBarComponent";
import {BucketManagementModalComponent} from "./BucketManagementModalComponent";
import {LoadingComponent} from "./LoadingComponent";

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
  modalOpen: boolean
}

export class BucketPage extends React.Component<Props, State> {
  state: State = {
    bucket: Loadable.loading(),
    question: Loadable.unloaded(),
    questions_remaining_in_bucket: Loadable.loading(),
    permissions: Loadable.unloaded(),
    user: Loadable.unloaded(),
    newQuestionText: "",
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
          this.getPermissions(bucket.uuid);
          this.addSelfToBucket(bucket);
        }
      });
  }

  /*
   * Handle UI Events
   */

  handleCloseModal = () => {
    console.log("handling closing the modal");
    this.setState({modalOpen: false})
  };

  handleOpenModal = () => {
    console.log("handling opening the modal");
    this.setState({ modalOpen: true });
  };

  handleNewQuestionTextUpdate = (event: ChangeEvent<HTMLInputElement>) => {
    this.setState({newQuestionText: event.target.value})
  };

  /* === Network requests */

  getPermissions = (bucket_uuid: Uuid) => {
    const url = `/api/bucket/${bucket_uuid}/user`;
    if (isAuthenticated()) {
      return authenticatedFetchAndDeserialize<BucketUserPermissions>(url)
        .then((permissions: BucketUserPermissions) => {
          this.setState({permissions: Loadable.loaded(permissions)})
        })
        .catch((error: ErrorResponse) => {
          this.setState({permissions: Loadable.errored(error.message)})
        });
    }
  };

  changeBucketFlagsRequest = (bucket_uuid: Uuid, changes: ChangeBucketFlagsRequest) => {
    const url = `/api/bucket/${bucket_uuid}`;
    const body = JSON.stringify(changes);
    const options = {
      method: "PUT",
      body
    };
    authenticatedFetchAndDeserialize<Bucket>(url, options)
      .then((bucket: Bucket) => {
        this.setState({bucket: Loadable.loaded(bucket)})
      })
  };



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

  addSelfToBucket = (bucket: Bucket) => {
    const url = `/api/bucket/${bucket.uuid}/user`;
    // If the user is logged in and the bucket is not exclusive, add the user to the bucket.
    if (isAuthenticated() && !bucket.exclusive) {
      const options = {
        method: "POST",
      };
      authenticatedFetchAndDeserialize<BucketUserRelation>(url, options)
        .then(() => {
          console.log("Added self to bucket.")
        })
        .catch(() => {
          console.log("User already added to bucket. The 412 response is expected behavior.")
        })
    }
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
              loading: () => <LoadingComponent/>,
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
    let bucket = this.state.bucket.getLoaded();
    if (bucket != null) {
      title = bucket.bucket_name;
    }
    const remaining_questions = this.state.questions_remaining_in_bucket.getLoaded();
    const permissionsModalReady = this.state.permissions.isLoaded() && this.state.bucket.isLoaded();

    const permissions =  this.state.permissions.getLoaded();

    return (
      <>
        <BucketNavBarComponent title={title}  remaining_questions={remaining_questions} handleOpenModal={this.handleOpenModal} permissionsModalReady={permissionsModalReady}/>
        <main>
          <div style={styles.container}>
            <div style={styles.constrainedWidth}>
              {
                this.state.bucket.match({
                  loading: () => <LoadingComponent/>,
                  loaded: this.render_bucket,
                  error: (error: Error) => <>Could not get bucket - {error}</>
                })
              }
            </div>
          </div>
        </main>
        {
          (bucket !== null && permissions !== null) &&
          <BucketManagementModalComponent
            open={this.state.modalOpen}
            handleClose={this.handleCloseModal}
            bucket={bucket as Bucket}
            permissions={permissions as BucketUserPermissions}
            setBucketStateCallback={this.changeBucketFlagsRequest}
          />
        }
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